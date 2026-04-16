//! USB CDC-ECM (Ethernet Control Model) class driver.
//!
//! Presents the device as a USB Ethernet adapter. Raw Ethernet frames are
//! exchanged over Bulk IN/OUT endpoints (EP1) with no encapsulation -- this
//! is the simplest Ethernet-over-USB protocol.
//!
//! The class uses three static frame buffers for ISR <-> main-loop bridging:
//!
//!   Host -> Device (USB RX -> ETH TX):
//!     USB ISR reassembles 64-byte bulk OUT packets into a complete frame
//!     in `H2D_FRAME`. When a short packet arrives (end of frame),
//!     `H2D_LEN` is set. The main loop calls `take_rx_frame()` to retrieve
//!     it and forward to `eth::tx_packet()`.
//!
//!   Device -> Host (ETH RX -> USB TX):
//!     The main loop calls `submit_tx_frame()` to copy a frame into
//!     `D2H_FRAME` and kick the first 64-byte chunk over Bulk IN.
//!     The ISR's `in_complete` callback sends subsequent chunks until
//!     the frame is fully delivered.
//!
//! OS support: Linux and macOS load the native `cdc_ether` / `ECMControl`
//! driver automatically. Windows 10+ supports CDC-ECM via `UsbNcm`.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use core::sync::atomic::{AtomicU16, Ordering};
use portable_atomic::AtomicBool;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const EP1_MPS: u16 = 64; // Bulk endpoint max packet size (Full Speed)
const EP2_MPS: u16 = 16; // Interrupt endpoint max packet size
const MAX_FRAME: usize = 1536; // Max Ethernet frame (1514 rounded up)

// ---------------------------------------------------------------------------
// Shared frame buffers (ISR <-> main loop)
// ---------------------------------------------------------------------------

/// Host-to-device reassembly buffer.
static mut H2D_FRAME: [u8; MAX_FRAME] = [0; MAX_FRAME];
/// Current write position in H2D_FRAME (ISR only).
static mut H2D_POS: usize = 0;
/// Frame length when complete (0 = buffer empty/assembling).
static H2D_LEN: AtomicU16 = AtomicU16::new(0);

/// Device-to-host frame buffer.
static mut D2H_FRAME: [u8; MAX_FRAME] = [0; MAX_FRAME];
/// Total frame length (0 = buffer empty).
static D2H_LEN: AtomicU16 = AtomicU16::new(0);
/// Current read position for chunked TX (ISR only).
static mut D2H_POS: usize = 0;

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Set when the data interface alt setting is 1 (active).
static DATA_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Notification state: 0=idle, 1=connection sent (send speed next), 2=done.
static NOTIFY_STATE: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

// ---------------------------------------------------------------------------
// Public API for the main loop
// ---------------------------------------------------------------------------

pub fn is_configured() -> bool {
    CONFIGURED.load(Ordering::Acquire)
}

pub fn is_data_active() -> bool {
    DATA_ACTIVE.load(Ordering::Acquire)
}

/// Retrieve a reassembled Ethernet frame received from the USB host.
///
/// Returns `Some(len)` if a frame was copied into `buf`, or `None` if no
/// frame is available. Caller should forward the frame to `eth::tx_packet()`.
pub fn take_rx_frame(buf: &mut [u8; MAX_FRAME]) -> Option<usize> {
    let len = H2D_LEN.load(Ordering::Acquire);
    if len == 0 {
        return None;
    }
    let n = len as usize;
    // SAFETY: ISR will not touch H2D_FRAME while H2D_LEN != 0 (it only
    // writes when H2D_LEN is 0, i.e. buffer is free for reassembly).
    unsafe {
        buf[..n].copy_from_slice(&H2D_FRAME[..n]);
    }
    H2D_LEN.store(0, Ordering::Release);
    Some(n)
}

/// Queue an Ethernet frame for transmission to the USB host and start
/// sending the first chunk.
///
/// Returns `true` if the frame was accepted, `false` if the TX path is busy.
/// Must be called with USB bus access (via `usb_with_bus` or similar).
pub fn submit_tx_frame(data: &[u8], usb: &UsbBus) -> bool {
    if data.is_empty() || data.len() > MAX_FRAME {
        return false;
    }
    if !DATA_ACTIVE.load(Ordering::Acquire) {
        return false;
    }
    // Check if previous frame is still being sent.
    if D2H_LEN.load(Ordering::Acquire) != 0 {
        return false;
    }
    let len = data.len();
    // SAFETY: D2H_LEN == 0 guarantees the ISR is not reading D2H_FRAME.
    unsafe {
        D2H_FRAME[..len].copy_from_slice(data);
        D2H_POS = 0;
    }
    D2H_LEN.store(len as u16, Ordering::Release);
    // Kick the first chunk.
    send_next_chunk(usb);
    true
}

/// Send the next 64-byte chunk of D2H_FRAME over Bulk IN.
fn send_next_chunk(usb: &UsbBus) {
    let total = D2H_LEN.load(Ordering::Relaxed) as usize;
    if total == 0 {
        return;
    }
    // SAFETY: We are the only writer of D2H_POS (ISR context or
    // critical section via usb_with_bus). D2H_FRAME is stable while
    // D2H_LEN != 0.
    let pos = unsafe { D2H_POS };
    if pos >= total {
        // Frame fully sent. If length was a multiple of MPS, send ZLP.
        if total % (EP1_MPS as usize) == 0 {
            usb.ep_write(1, &[]);
        }
        // Mark TX buffer as free.
        D2H_LEN.store(0, Ordering::Release);
        return;
    }
    let remaining = total - pos;
    let chunk = remaining.min(EP1_MPS as usize);
    let mut pkt = [0u8; EP1_MPS as usize];
    unsafe {
        pkt[..chunk].copy_from_slice(&D2H_FRAME[pos..pos + chunk]);
        D2H_POS = pos + chunk;
    }
    usb.ep_write(1, &pkt[..chunk]);
}

// ---------------------------------------------------------------------------
// Descriptors
// ---------------------------------------------------------------------------

// MAC address as a USB string descriptor (12 hex ASCII chars).
// Must match the MAC returned by slstk3701a::eth::mac_address().
// MAC: 02:00:00:47:11:BB -> "020000471188"
static STRING_MAC: [u8; 2 + 12 * 2] = usb_string!("0200004711BB");

#[rustfmt::skip]
static DEVICE_DESC: [u8; 18] = [
    18,         // bLength
    0x01,       // bDescriptorType (Device)
    0x00, 0x02, // bcdUSB 2.00
    0x02,       // bDeviceClass    (Communications)
    0x06,       // bDeviceSubClass (ECM)
    0x00,       // bDeviceProtocol
    64,         // bMaxPacketSize0
    0xFE, 0xCA, // idVendor  0xCAFE (test VID)
    0x10, 0x00, // idProduct 0x0010
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

// Total configuration descriptor length:
//   Config(9) + IAD(8) +
//   Iface0(9) + CDC_Header(5) + CDC_Union(5) + CDC_Ethernet(13) + EP2_IN(7) +
//   Iface1_Alt0(9) +
//   Iface1_Alt1(9) + EP1_IN(7) + EP1_OUT(7)
const CONFIG_TOTAL_LEN: u16 = 9 + 8 + 9 + 5 + 5 + 13 + 7 + 9 + 9 + 7 + 7;

#[rustfmt::skip]
static CONFIG_DESC: [u8; CONFIG_TOTAL_LEN as usize] = [
    // ---- Configuration Descriptor ----
    9, 0x02,
    (CONFIG_TOTAL_LEN & 0xFF) as u8, (CONFIG_TOTAL_LEN >> 8) as u8,
    2,              // bNumInterfaces
    1,              // bConfigurationValue
    0,              // iConfiguration
    0x80,           // bmAttributes (bus-powered)
    250,            // bMaxPower (500 mA)

    // ---- Interface Association Descriptor ----
    8, 0x0B,        // bLength, bDescriptorType (IAD)
    0,              // bFirstInterface
    2,              // bInterfaceCount
    0x02,           // bFunctionClass (Communications)
    0x06,           // bFunctionSubClass (ECM)
    0x00,           // bFunctionProtocol
    0,              // iFunction

    // ---- Interface 0: CDC Communication (ECM) ----
    9, 0x04,
    0,              // bInterfaceNumber
    0,              // bAlternateSetting
    1,              // bNumEndpoints (interrupt IN)
    0x02,           // bInterfaceClass    (Communications)
    0x06,           // bInterfaceSubClass (Ethernet Control Model)
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // CDC Header Functional Descriptor
    5, 0x24, 0x00,
    0x20, 0x01,     // bcdCDC 1.20

    // CDC Union Functional Descriptor
    5, 0x24, 0x06,
    0,              // bControlInterface
    1,              // bSubordinateInterface0

    // CDC Ethernet Networking Functional Descriptor
    13, 0x24, 0x0F,
    3,              // iMACAddress (string descriptor index)
    0x00, 0x00, 0x00, 0x00, // bmEthernetStatistics (none)
    0xEA, 0x05,     // wMaxSegmentSize = 1514
    0x00, 0x00,     // wNumberMCFilters = 0
    0,              // bNumberPowerFilters = 0

    // Endpoint 2 IN -- Interrupt (notifications)
    7, 0x05,
    0x82,           // bEndpointAddress (EP2 IN)
    0x03,           // bmAttributes     (Interrupt)
    (EP2_MPS & 0xFF) as u8, (EP2_MPS >> 8) as u8,
    255,            // bInterval (ms)

    // ---- Interface 1 Alt 0: CDC Data (inactive, 0 endpoints) ----
    9, 0x04,
    1,              // bInterfaceNumber
    0,              // bAlternateSetting
    0,              // bNumEndpoints
    0x0A,           // bInterfaceClass    (CDC Data)
    0x00,           // bInterfaceSubClass
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // ---- Interface 1 Alt 1: CDC Data (active, 2 endpoints) ----
    9, 0x04,
    1,              // bInterfaceNumber
    1,              // bAlternateSetting
    2,              // bNumEndpoints
    0x0A,           // bInterfaceClass    (CDC Data)
    0x00,           // bInterfaceSubClass
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // Endpoint 1 IN -- Bulk (device -> host)
    7, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x02,           // bmAttributes     (Bulk)
    (EP1_MPS & 0xFF) as u8, (EP1_MPS >> 8) as u8,
    0,              // bInterval

    // Endpoint 1 OUT -- Bulk (host -> device)
    7, 0x05,
    0x01,           // bEndpointAddress (EP1 OUT)
    0x02,           // bmAttributes     (Bulk)
    (EP1_MPS & 0xFF) as u8, (EP1_MPS >> 8) as u8,
    0,              // bInterval
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3701A");
static STRING2: [u8; 2 + 15 * 2] = usb_string!("EFM32 USB-ETH  ");

// ---------------------------------------------------------------------------
// Notification helpers
// ---------------------------------------------------------------------------

/// NETWORK_CONNECTION notification (connected = true).
#[rustfmt::skip]
static NOTIFY_CONNECTED: [u8; 8] = [
    0xA1,       // bmRequestType (class, interface, device-to-host)
    0x00,       // bNotification (NETWORK_CONNECTION)
    0x01, 0x00, // wValue = 1 (connected)
    0x00, 0x00, // wIndex = 0 (interface 0)
    0x00, 0x00, // wLength = 0
];

/// CONNECTION_SPEED_CHANGE notification for 100 Mbps full-duplex.
#[rustfmt::skip]
static NOTIFY_SPEED: [u8; 16] = [
    0xA1,       // bmRequestType
    0x2A,       // bNotification (CONNECTION_SPEED_CHANGE)
    0x00, 0x00, // wValue = 0
    0x00, 0x00, // wIndex = 0 (interface 0)
    0x08, 0x00, // wLength = 8
    // Downlink speed: 100,000,000 bps = 0x05F5E100
    0x00, 0xE1, 0xF5, 0x05,
    // Uplink speed: 100,000,000 bps
    0x00, 0xE1, 0xF5, 0x05,
];

/// Send NETWORK_CONNECTION and CONNECTION_SPEED_CHANGE notifications
/// on the interrupt endpoint (EP2 IN). Call after link is up.
pub fn notify_link_up(usb: &UsbBus) {
    usb.ep_write(2, &NOTIFY_CONNECTED);
}

/// Send the speed change notification. Call from `in_complete(2)` after
/// the connection notification has been sent.
pub fn notify_speed(usb: &UsbBus) {
    usb.ep_write(2, &NOTIFY_SPEED);
}

// ---------------------------------------------------------------------------
// CDC-ECM class driver
// ---------------------------------------------------------------------------

pub struct CdcEcmClass;

impl CdcEcmClass {
    pub fn new() -> Self {
        Self
    }
}

/// Recommended USB peripheral / FIFO configuration for CDC-ECM.
pub fn usb_config() -> UsbConfig {
    UsbConfig {
        rx_fifo_words: 64,
        tx0_fifo_words: 24,
        tx1_fifo_words: 64, // EP1 bulk data
        tx2_fifo_words: 16, // EP2 interrupt notifications
        ep1: Some(EpConfig {
            ep_type: EpType::Bulk,
            mps: EP1_MPS,
            has_in: true,
            has_out: true,
            out_max_xfer: 1536,
        }),
        ep2: Some(EpConfig {
            ep_type: EpType::Interrupt,
            mps: EP2_MPS,
            has_in: true,
            has_out: false,
            out_max_xfer: 0,
        }),
        tx3_fifo_words: 0,
        ep3: None,
    }
}

impl UsbClass for CdcEcmClass {
    fn device_descriptor(&self) -> &[u8] {
        &DEVICE_DESC
    }

    fn config_descriptor(&self) -> &[u8] {
        &CONFIG_DESC
    }

    fn string_descriptor(&self, index: u8) -> Option<&[u8]> {
        match index {
            1 => Some(&STRING1),
            2 => Some(&STRING2),
            3 => Some(&STRING_MAC),
            _ => None,
        }
    }

    fn handle_setup(&mut self, setup: &SetupPacket, _usb: &UsbBus) -> SetupResult {
        const SET_ETHERNET_PACKET_FILTER: u8 = 0x43;

        match (setup.bm_request_type, setup.b_request) {
            // SET_ETHERNET_PACKET_FILTER -- Linux always sends this.
            // Accept and ignore (we receive everything from the wire).
            (0x21, SET_ETHERNET_PACKET_FILTER) => {
                defmt::info!("SET_ETHERNET_PACKET_FILTER: {:04x}", setup.w_value);
                SetupResult::Handled
            }
            _ => SetupResult::Unhandled,
        }
    }

    fn data_out(&mut self, ep: u8, data: &[u8], _usb: &UsbBus) {
        if ep != 1 || !DATA_ACTIVE.load(Ordering::Relaxed) {
            return;
        }

        // In DMA mode, multi-packet transfers deliver the complete frame
        // in one XFERCOMPL (short packet terminates the DMA transfer).
        #[cfg(feature = "dma")]
        unsafe {
            if H2D_LEN.load(Ordering::Relaxed) != 0 {
                return; // Previous frame not yet consumed.
            }
            let len = data.len();
            if len > 0 && len <= MAX_FRAME {
                H2D_FRAME[..len].copy_from_slice(data);
                H2D_LEN.store(len as u16, Ordering::Release);
            }
        }

        // In FIFO (slave) mode, reassemble frame from 64-byte bulk OUT
        // packets. A short packet (< 64 bytes) or ZLP marks end of frame.
        #[cfg(not(feature = "dma"))]
        unsafe {
            let pos = H2D_POS;
            if H2D_LEN.load(Ordering::Relaxed) != 0 {
                // Previous frame not yet consumed by main loop -- drop.
                return;
            }
            if !data.is_empty() && pos + data.len() <= MAX_FRAME {
                H2D_FRAME[pos..pos + data.len()].copy_from_slice(data);
                H2D_POS = pos + data.len();
            }
            // End of frame: short packet or ZLP.
            if data.len() < EP1_MPS as usize {
                let frame_len = if data.is_empty() {
                    pos
                } else {
                    pos + data.len()
                };
                if frame_len > 0 {
                    H2D_LEN.store(frame_len as u16, Ordering::Release);
                }
                H2D_POS = 0;
            }
        }
    }

    fn in_complete(&mut self, ep: u8, usb: &UsbBus) {
        match ep {
            1 => {
                // Continue sending the current D2H frame.
                send_next_chunk(usb);
            }
            2 => {
                // Notification endpoint: send speed after connection.
                if NOTIFY_STATE.load(Ordering::Relaxed) == 1 {
                    NOTIFY_STATE.store(2, Ordering::Relaxed);
                    notify_speed(usb);
                }
            }
            _ => {}
        }
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("CDC-ECM configured");
    }

    fn set_interface(&mut self, iface: u8, alt: u8, usb: &UsbBus) {
        if iface == 1 {
            let active = alt == 1;
            DATA_ACTIVE.store(active, Ordering::Release);
            defmt::info!("CDC-ECM data interface alt={}", alt);
            if active {
                // Prepare EP1 OUT to receive frames from the host.
                usb.ep_prepare_out(1);
            }
        }
    }

    fn get_interface(&self, iface: u8) -> u8 {
        if iface == 1 && DATA_ACTIVE.load(Ordering::Relaxed) {
            1
        } else {
            0
        }
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        DATA_ACTIVE.store(false, Ordering::Release);
        D2H_LEN.store(0, Ordering::Release);
        H2D_LEN.store(0, Ordering::Release);
        unsafe {
            H2D_POS = 0;
            D2H_POS = 0;
        }
        NOTIFY_STATE.store(0, Ordering::Relaxed);
    }

    fn suspended(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        DATA_ACTIVE.store(false, Ordering::Release);
    }
}

/// Trigger the NETWORK_CONNECTION notification sequence.
/// Call once after USB is configured and Ethernet link is up.
/// The speed notification follows automatically via `in_complete(2)`.
pub fn send_link_up_notifications(usb: &UsbBus) {
    NOTIFY_STATE.store(1, Ordering::Release);
    notify_link_up(usb);
}
