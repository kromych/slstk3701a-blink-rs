//! Composite USB CDC-ECM + CDC-ACM class driver.
//!
//! Combines a CDC-ECM function (USB Ethernet adapter) with a CDC-ACM function
//! (virtual serial port) in a single composite USB device using Interface
//! Association Descriptors (IAD).
//!
//! Endpoint allocation:
//!   EP1 IN/OUT  -- ECM bulk data (Ethernet frames)
//!   EP2 IN      -- ECM interrupt notifications
//!   EP3 IN/OUT  -- ACM bulk data (serial console)
//!
//! Interface layout:
//!   IF0 -- ECM Communication (interrupt IN EP2)
//!   IF1 -- ECM Data (alt0 inactive, alt1 bulk IN/OUT EP1)
//!   IF2 -- ACM Communication (no notification endpoint)
//!   IF3 -- ACM Data (bulk IN/OUT EP3)

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use core::sync::atomic::{AtomicU16, Ordering};
use portable_atomic::AtomicBool;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const EP1_MPS: u16 = 64; // ECM bulk max packet size
const EP2_MPS: u16 = 16; // ECM interrupt max packet size
const EP3_MPS: u16 = 64; // ACM bulk max packet size
const MAX_FRAME: usize = 1536;

// ---------------------------------------------------------------------------
// ECM shared frame buffers (ISR <-> main loop)
// ---------------------------------------------------------------------------

static mut H2D_FRAME: [u8; MAX_FRAME] = [0; MAX_FRAME];
static mut H2D_POS: usize = 0;
static H2D_LEN: AtomicU16 = AtomicU16::new(0);

static mut D2H_FRAME: [u8; MAX_FRAME] = [0; MAX_FRAME];
static D2H_LEN: AtomicU16 = AtomicU16::new(0);
static mut D2H_POS: usize = 0;

pub static CONFIGURED: AtomicBool = AtomicBool::new(false);
static ECM_DATA_ACTIVE: AtomicBool = AtomicBool::new(false);
static NOTIFY_STATE: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

// ---------------------------------------------------------------------------
// ACM shared buffers (ISR <-> main loop)
// ---------------------------------------------------------------------------

/// ACM receive ring buffer (host -> device serial data).
const ACM_RX_BUF_SIZE: usize = 256;
static mut ACM_RX_BUF: [u8; ACM_RX_BUF_SIZE] = [0; ACM_RX_BUF_SIZE];
static ACM_RX_HEAD: AtomicU16 = AtomicU16::new(0); // ISR writes
static ACM_RX_TAIL: AtomicU16 = AtomicU16::new(0); // main reads

/// ACM transmit buffer (device -> host serial data).
const ACM_TX_BUF_SIZE: usize = 256;
static mut ACM_TX_BUF: [u8; ACM_TX_BUF_SIZE] = [0; ACM_TX_BUF_SIZE];
static ACM_TX_HEAD: AtomicU16 = AtomicU16::new(0); // main writes
static ACM_TX_TAIL: AtomicU16 = AtomicU16::new(0); // ISR reads
static ACM_TX_BUSY: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// ECM public API
// ---------------------------------------------------------------------------

pub fn is_configured() -> bool {
    CONFIGURED.load(Ordering::Acquire)
}

pub fn is_data_active() -> bool {
    ECM_DATA_ACTIVE.load(Ordering::Acquire)
}

pub fn take_rx_frame(buf: &mut [u8; MAX_FRAME]) -> Option<usize> {
    let len = H2D_LEN.load(Ordering::Acquire);
    if len == 0 {
        return None;
    }
    let n = len as usize;
    unsafe {
        buf[..n].copy_from_slice(&H2D_FRAME[..n]);
    }
    H2D_LEN.store(0, Ordering::Release);
    Some(n)
}

pub fn submit_tx_frame(data: &[u8], usb: &UsbBus) -> bool {
    if data.is_empty() || data.len() > MAX_FRAME {
        return false;
    }
    if !ECM_DATA_ACTIVE.load(Ordering::Acquire) {
        return false;
    }
    if D2H_LEN.load(Ordering::Acquire) != 0 {
        return false;
    }
    let len = data.len();
    unsafe {
        D2H_FRAME[..len].copy_from_slice(data);
        D2H_POS = 0;
    }
    D2H_LEN.store(len as u16, Ordering::Release);
    send_next_ecm_chunk(usb);
    true
}

fn send_next_ecm_chunk(usb: &UsbBus) {
    let total = D2H_LEN.load(Ordering::Relaxed) as usize;
    if total == 0 {
        return;
    }
    let pos = unsafe { D2H_POS };
    if pos >= total {
        if total % (EP1_MPS as usize) == 0 {
            usb.ep_write(1, &[]);
        }
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
// ACM public API
// ---------------------------------------------------------------------------

/// Read bytes from the ACM serial receive buffer. Returns number of bytes read.
pub fn acm_read(buf: &mut [u8]) -> usize {
    let head = ACM_RX_HEAD.load(Ordering::Acquire) as usize;
    let tail = ACM_RX_TAIL.load(Ordering::Relaxed) as usize;
    if head == tail {
        return 0;
    }
    let mut count = 0;
    let mut t = tail;
    while t != head && count < buf.len() {
        buf[count] = unsafe { ACM_RX_BUF[t] };
        t = (t + 1) % ACM_RX_BUF_SIZE;
        count += 1;
    }
    ACM_RX_TAIL.store(t as u16, Ordering::Release);
    count
}

/// Write bytes to the ACM serial transmit buffer. Returns number of bytes written.
pub fn acm_write(data: &[u8]) -> usize {
    let head = ACM_TX_HEAD.load(Ordering::Relaxed) as usize;
    let tail = ACM_TX_TAIL.load(Ordering::Acquire) as usize;
    let mut count = 0;
    let mut h = head;
    for &b in data {
        let next = (h + 1) % ACM_TX_BUF_SIZE;
        if next == tail {
            break; // full
        }
        unsafe {
            ACM_TX_BUF[h] = b;
        }
        h = next;
        count += 1;
    }
    ACM_TX_HEAD.store(h as u16, Ordering::Release);
    count
}

/// Flush the ACM TX buffer by kicking the first chunk if not already sending.
/// Must be called with USB bus access.
pub fn acm_flush(usb: &UsbBus) {
    if !ACM_TX_BUSY.load(Ordering::Relaxed) {
        send_next_acm_chunk(usb);
    }
}

fn send_next_acm_chunk(usb: &UsbBus) {
    let head = ACM_TX_HEAD.load(Ordering::Acquire) as usize;
    let tail = ACM_TX_TAIL.load(Ordering::Relaxed) as usize;
    if head == tail {
        ACM_TX_BUSY.store(false, Ordering::Release);
        return;
    }
    ACM_TX_BUSY.store(true, Ordering::Release);
    let mut pkt = [0u8; EP3_MPS as usize];
    let mut count = 0;
    let mut t = tail;
    while t != head && count < EP3_MPS as usize {
        pkt[count] = unsafe { ACM_TX_BUF[t] };
        t = (t + 1) % ACM_TX_BUF_SIZE;
        count += 1;
    }
    ACM_TX_TAIL.store(t as u16, Ordering::Release);
    usb.ep_write(3, &pkt[..count]);
}

/// Write a string to the ACM serial port. Returns number of bytes written.
pub fn acm_write_str(s: &str) -> usize {
    acm_write(s.as_bytes())
}

// ---------------------------------------------------------------------------
// ECM notification helpers
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static NOTIFY_CONNECTED: [u8; 8] = [
    0xA1, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[rustfmt::skip]
static NOTIFY_SPEED: [u8; 16] = [
    0xA1, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00,
    0x00, 0xE1, 0xF5, 0x05,
    0x00, 0xE1, 0xF5, 0x05,
];

pub fn notify_link_up(usb: &UsbBus) {
    usb.ep_write(2, &NOTIFY_CONNECTED);
}

pub fn notify_speed(usb: &UsbBus) {
    usb.ep_write(2, &NOTIFY_SPEED);
}

pub fn send_link_up_notifications(usb: &UsbBus) {
    NOTIFY_STATE.store(1, Ordering::Release);
    notify_link_up(usb);
}

// ---------------------------------------------------------------------------
// Descriptors
// ---------------------------------------------------------------------------

static STRING_MAC: [u8; 2 + 12 * 2] = usb_string!("0200004711BB");

// Composite device: class 0xEF (Misc), subclass 0x02, protocol 0x01 (IAD).
#[rustfmt::skip]
static DEVICE_DESC: [u8; 18] = [
    18, 0x01,
    0x00, 0x02, // bcdUSB 2.00
    0xEF,       // bDeviceClass    (Misc -- IAD)
    0x02,       // bDeviceSubClass (Common Class)
    0x01,       // bDeviceProtocol (IAD)
    64,         // bMaxPacketSize0
    0xFE, 0xCA, // idVendor  0xCAFE
    0x11, 0x00, // idProduct 0x0011
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

// Configuration descriptor total length:
//   Config(9)
//   + IAD_ECM(8) + IF0(9) + CDC_Header(5) + CDC_Union(5) + CDC_Ether(13) + EP2_IN(7)
//   + IF1_Alt0(9) + IF1_Alt1(9) + EP1_IN(7) + EP1_OUT(7)
//   + IAD_ACM(8) + IF2(9) + CDC_Header(5) + CDC_CallMgmt(5) + CDC_ACM(4) + CDC_Union(5)
//   + IF3(9) + EP3_IN(7) + EP3_OUT(7)
//   = 9 + (8+9+5+5+13+7) + (9+9+7+7) + (8+9+5+5+4+5) + (9+7+7)
//   = 9 + 47 + 32 + 36 + 23 = 147
const CONFIG_TOTAL_LEN: u16 = 147;

#[rustfmt::skip]
static CONFIG_DESC: [u8; CONFIG_TOTAL_LEN as usize] = [
    // ---- Configuration Descriptor ----
    9, 0x02,
    (CONFIG_TOTAL_LEN & 0xFF) as u8, (CONFIG_TOTAL_LEN >> 8) as u8,
    4,              // bNumInterfaces
    1,              // bConfigurationValue
    0,              // iConfiguration
    0x80,           // bmAttributes (bus-powered)
    250,            // bMaxPower (500 mA)

    // ================================================================
    // Function 1: CDC-ECM (Ethernet)
    // ================================================================

    // ---- IAD for ECM ----
    8, 0x0B,
    0,              // bFirstInterface (IF0)
    2,              // bInterfaceCount (IF0 + IF1)
    0x02,           // bFunctionClass (Communications)
    0x06,           // bFunctionSubClass (ECM)
    0x00,           // bFunctionProtocol
    0,              // iFunction

    // ---- IF0: ECM Communication ----
    9, 0x04,
    0, 0,           // bInterfaceNumber, bAlternateSetting
    1,              // bNumEndpoints
    0x02, 0x06, 0x00, // CDC, ECM, None
    0,              // iInterface

    // CDC Header
    5, 0x24, 0x00, 0x20, 0x01,

    // CDC Union
    5, 0x24, 0x06, 0, 1,

    // CDC Ethernet Networking
    13, 0x24, 0x0F,
    3,              // iMACAddress
    0x00, 0x00, 0x00, 0x00,
    0xEA, 0x05,     // wMaxSegmentSize = 1514
    0x00, 0x00,     // wNumberMCFilters
    0,              // bNumberPowerFilters

    // EP2 IN -- Interrupt (ECM notifications)
    7, 0x05,
    0x82, 0x03,
    (EP2_MPS & 0xFF) as u8, (EP2_MPS >> 8) as u8,
    255,

    // ---- IF1 Alt 0: ECM Data (inactive) ----
    9, 0x04,
    1, 0,           // bInterfaceNumber, bAlternateSetting
    0,              // bNumEndpoints
    0x0A, 0x00, 0x00,
    0,

    // ---- IF1 Alt 1: ECM Data (active) ----
    9, 0x04,
    1, 1,           // bInterfaceNumber, bAlternateSetting
    2,              // bNumEndpoints
    0x0A, 0x00, 0x00,
    0,

    // EP1 IN -- Bulk
    7, 0x05,
    0x81, 0x02,
    (EP1_MPS & 0xFF) as u8, (EP1_MPS >> 8) as u8,
    0,

    // EP1 OUT -- Bulk
    7, 0x05,
    0x01, 0x02,
    (EP1_MPS & 0xFF) as u8, (EP1_MPS >> 8) as u8,
    0,

    // ================================================================
    // Function 2: CDC-ACM (Serial Console)
    // ================================================================

    // ---- IAD for ACM ----
    8, 0x0B,
    2,              // bFirstInterface (IF2)
    2,              // bInterfaceCount (IF2 + IF3)
    0x02,           // bFunctionClass (Communications)
    0x02,           // bFunctionSubClass (ACM)
    0x00,           // bFunctionProtocol
    0,              // iFunction

    // ---- IF2: ACM Communication ----
    9, 0x04,
    2, 0,           // bInterfaceNumber, bAlternateSetting
    0,              // bNumEndpoints (no notification EP)
    0x02, 0x02, 0x00, // CDC, ACM, None
    0,              // iInterface

    // CDC Header
    5, 0x24, 0x00, 0x20, 0x01,

    // CDC Call Management
    5, 0x24, 0x01, 0x00, 3,  // bDataInterface = 3

    // CDC ACM
    4, 0x24, 0x02, 0x02,

    // CDC Union
    5, 0x24, 0x06, 2, 3,     // control=IF2, subordinate=IF3

    // ---- IF3: ACM Data ----
    9, 0x04,
    3, 0,           // bInterfaceNumber, bAlternateSetting
    2,              // bNumEndpoints
    0x0A, 0x00, 0x00,
    0,

    // EP3 IN -- Bulk (device -> host serial)
    7, 0x05,
    0x83, 0x02,
    (EP3_MPS & 0xFF) as u8, (EP3_MPS >> 8) as u8,
    0,

    // EP3 OUT -- Bulk (host -> device serial)
    7, 0x05,
    0x03, 0x02,
    (EP3_MPS & 0xFF) as u8, (EP3_MPS >> 8) as u8,
    0,
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3701A");
static STRING2: [u8; 2 + 9 * 2] = usb_string!("Disco NIC");

// ---------------------------------------------------------------------------
// Composite class driver
// ---------------------------------------------------------------------------

pub struct CdcEcmAcmClass {
    acm_line_coding: [u8; 7],
}

impl CdcEcmAcmClass {
    pub fn new() -> Self {
        Self {
            // 115200 8N1
            acm_line_coding: [0x00, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x08],
        }
    }
}

pub fn usb_config() -> UsbConfig {
    UsbConfig {
        rx_fifo_words: 64,
        tx0_fifo_words: 24,
        tx1_fifo_words: 64, // EP1 ECM bulk
        tx2_fifo_words: 16, // EP2 ECM interrupt
        tx3_fifo_words: 32, // EP3 ACM bulk
        ep1: Some(EpConfig {
            ep_type: EpType::Bulk,
            mps: EP1_MPS,
            has_in: true,
            has_out: true,
            out_max_xfer: 1536, // Ethernet frame size
        }),
        ep2: Some(EpConfig {
            ep_type: EpType::Interrupt,
            mps: EP2_MPS,
            has_in: true,
            has_out: false,
            out_max_xfer: 0,
        }),
        ep3: Some(EpConfig {
            ep_type: EpType::Bulk,
            mps: EP3_MPS,
            has_in: true,
            has_out: true,
            out_max_xfer: 0,
        }),
    }
}

impl UsbClass for CdcEcmAcmClass {
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

    fn handle_setup(&mut self, setup: &SetupPacket, usb: &UsbBus) -> SetupResult {
        const SET_ETHERNET_PACKET_FILTER: u8 = 0x43;
        const SET_LINE_CODING: u8 = 0x20;
        const GET_LINE_CODING: u8 = 0x21;
        const SET_CONTROL_LINE_STATE: u8 = 0x22;

        let iface = setup.w_index as u8;

        match (setup.bm_request_type, setup.b_request) {
            // ECM: SET_ETHERNET_PACKET_FILTER
            (0x21, SET_ETHERNET_PACKET_FILTER) if iface == 0 || iface == 1 => {
                defmt::info!("SET_ETHERNET_PACKET_FILTER: {:04x}", setup.w_value);
                SetupResult::Handled
            }

            // ACM: SET_LINE_CODING
            (0x21, SET_LINE_CODING) if iface == 2 || iface == 3 => SetupResult::DataOut,

            // ACM: GET_LINE_CODING
            (0xA1, GET_LINE_CODING) if iface == 2 || iface == 3 => {
                let len = (setup.w_length as usize).min(self.acm_line_coding.len());
                usb.ep0_write_packet(&self.acm_line_coding[..len]);
                SetupResult::DataIn
            }

            // ACM: SET_CONTROL_LINE_STATE
            (0x21, SET_CONTROL_LINE_STATE) if iface == 2 || iface == 3 => {
                let dtr = setup.w_value & 0x01 != 0;
                defmt::info!("ACM SET_CONTROL_LINE_STATE DTR={}", dtr);
                SetupResult::Handled
            }

            _ => SetupResult::Unhandled,
        }
    }

    fn ep0_data_out(&mut self, data: &[u8], _usb: &UsbBus) {
        // ACM SET_LINE_CODING data stage.
        if data.len() >= 7 {
            self.acm_line_coding.copy_from_slice(&data[..7]);
        }
    }

    fn data_out(&mut self, ep: u8, data: &[u8], _usb: &UsbBus) {
        if ep == 1 && ECM_DATA_ACTIVE.load(Ordering::Relaxed) {
            // ECM: DMA multi-packet delivers the complete frame at once.
            #[cfg(feature = "dma")]
            unsafe {
                if H2D_LEN.load(Ordering::Relaxed) != 0 {
                    return;
                }
                let len = data.len();
                if len > 0 && len <= MAX_FRAME {
                    H2D_FRAME[..len].copy_from_slice(data);
                    H2D_LEN.store(len as u16, Ordering::Release);
                }
            }
            // ECM FIFO mode: reassemble from 64-byte bulk OUT packets.
            #[cfg(not(feature = "dma"))]
            unsafe {
                let pos = H2D_POS;
                if H2D_LEN.load(Ordering::Relaxed) != 0 {
                    return;
                }
                if !data.is_empty() && pos + data.len() <= MAX_FRAME {
                    H2D_FRAME[pos..pos + data.len()].copy_from_slice(data);
                    H2D_POS = pos + data.len();
                }
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
        } else if ep == 3 && !data.is_empty() {
            // ACM: push received serial data into the ring buffer.
            let head = ACM_RX_HEAD.load(Ordering::Relaxed) as usize;
            let tail = ACM_RX_TAIL.load(Ordering::Acquire) as usize;
            let mut h = head;
            for &b in data {
                let next = (h + 1) % ACM_RX_BUF_SIZE;
                if next == tail {
                    break; // ring full, drop remaining
                }
                unsafe {
                    ACM_RX_BUF[h] = b;
                }
                h = next;
            }
            ACM_RX_HEAD.store(h as u16, Ordering::Release);
        }
    }

    fn in_complete(&mut self, ep: u8, usb: &UsbBus) {
        match ep {
            1 => send_next_ecm_chunk(usb),
            2 => {
                if NOTIFY_STATE.load(Ordering::Relaxed) == 1 {
                    NOTIFY_STATE.store(2, Ordering::Relaxed);
                    notify_speed(usb);
                }
            }
            3 => send_next_acm_chunk(usb),
            _ => {}
        }
    }

    fn configured(&mut self, usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        // ACM data interface is always active (no alt-setting dance).
        usb.ep_prepare_out(3);
        defmt::info!("Composite ECM+ACM configured");
    }

    fn set_interface(&mut self, iface: u8, alt: u8, usb: &UsbBus) {
        // ECM data interface (IF1) alt setting.
        if iface == 1 {
            let active = alt == 1;
            ECM_DATA_ACTIVE.store(active, Ordering::Release);
            defmt::info!("ECM data interface alt={}", alt);
            if active {
                usb.ep_prepare_out(1);
            }
        }
    }

    fn get_interface(&self, iface: u8) -> u8 {
        if iface == 1 && ECM_DATA_ACTIVE.load(Ordering::Relaxed) {
            1
        } else {
            0
        }
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        ECM_DATA_ACTIVE.store(false, Ordering::Release);
        D2H_LEN.store(0, Ordering::Release);
        H2D_LEN.store(0, Ordering::Release);
        unsafe {
            H2D_POS = 0;
            D2H_POS = 0;
        }
        NOTIFY_STATE.store(0, Ordering::Relaxed);
        ACM_RX_HEAD.store(0, Ordering::Release);
        ACM_RX_TAIL.store(0, Ordering::Release);
        ACM_TX_HEAD.store(0, Ordering::Release);
        ACM_TX_TAIL.store(0, Ordering::Release);
        ACM_TX_BUSY.store(false, Ordering::Release);
    }

    fn suspended(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        ECM_DATA_ACTIVE.store(false, Ordering::Release);
    }
}
