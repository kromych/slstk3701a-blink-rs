//! USB NIC demo for the SLSTK3701A board.
//!
//! Bridges the onboard KSZ8091RNA Ethernet PHY to the USB host as a
//! CDC-ECM network adapter. The board appears as a standard USB Ethernet
//! device -- no special drivers needed on Linux, macOS, or Windows 10+.
//!
//! Data flow:
//!   Host -> wire:  USB bulk OUT -> frame reassembly -> eth::tx_packet()
//!   Wire -> host:  eth::rx_packet() -> USB bulk IN -> host
//!
//! The main loop is fully interrupt-driven: SysTick (1 kHz) provides
//! timing, the ETH RXCMPLT/TXCMPLT interrupts signal Ethernet events,
//! and the USB ISR handles enumeration and bulk transfers. Between
//! events the CPU sleeps via WFI.
//!
//! The Color Memory LCD displays live stats: link state, IP address
//! (reported by the host's DHCP), frame counters, and uptime.
//!
//! LEDs:
//!   LED0 (PH10) -- Ethernet link is up
//!   LED1 (PH13) -- USB configured and data interface active
//!
//! NOTE: The board power switch must be in the USB position and an
//! Ethernet cable must be connected. Two USB cables are needed: one
//! for the J-Link debugger and one for the Micro-AB USB connector.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use efm32gg11b_pac as pac;
use efm32gg11b_usb::cdc_ecm::{self, CdcEcmClass};
use efm32gg11b_usb::UsbDevice;
use pac::interrupt;
use slstk3701a::display;

// ---------- millisecond timer ----------

static MILLIS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

fn systick_init(syst: &mut cortex_m::peripheral::SYST) {
    syst.set_clock_source(SystClkSource::Core);
    // HFCLK is 19 MHz HFRCO (eth::init does NOT switch HFCLK to HFXO).
    syst.set_reload(19_000 - 1); // 19 MHz -> 1 ms tick
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

// ---------- LED helpers ----------

const LED0: u32 = 1 << 10;
const LED1: u32 = 1 << 13;

fn led_set(gpio: &pac::Gpio, mask: u32) {
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
}

fn led_clear(gpio: &pac::Gpio, mask: u32) {
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
}

// ---------- LCD display helpers ----------

// Colors (3-bit RGB: bit2=R, bit1=G, bit0=B).
const BLACK: u8 = 0b000;
const BLUE: u8 = 0b001;
const GREEN: u8 = 0b010;
const RED: u8 = 0b100;
const WHITE: u8 = 0b111;

/// Format a u32 as decimal right-aligned in a fixed-width buffer.
/// Returns number of chars written.
fn fmt_u32(val: u32, buf: &mut [u8]) -> usize {
    if val == 0 {
        let last = buf.len() - 1;
        buf[last] = b'0';
        return 1;
    }
    let mut v = val;
    let mut i = buf.len();
    while v > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    buf.len() - i
}

struct LcdState {
    vcom: bool,
    last_vcom_ms: u32,
    last_draw_ms: u32,
}

impl LcdState {
    fn new() -> Self {
        Self {
            vcom: false,
            last_vcom_ms: 0,
            last_draw_ms: 0,
        }
    }

    /// Toggle VCOM at ~60 Hz to prevent DC bias damage.
    fn tick_vcom(&mut self, now_ms: u32) {
        if now_ms.wrapping_sub(self.last_vcom_ms) >= 16 {
            self.last_vcom_ms = now_ms;
            display::toggle_vcom();
        }
    }

    /// Redraw stats on the LCD at ~2 Hz.
    fn draw_stats(&mut self, now_ms: u32, link: bool, usb_active: bool, h2d: u32, d2h: u32) {
        if now_ms.wrapping_sub(self.last_draw_ms) < 500 {
            return;
        }
        self.last_draw_ms = now_ms;

        // Row 0: title
        display::draw_text_colored(0, 0, "      USB NIC       ", BLUE, WHITE, &mut self.vcom);

        // Row 2: link status
        if link {
            display::draw_text_colored(2, 0, "ETH: LINK UP  100M  ", GREEN, WHITE, &mut self.vcom);
        } else {
            display::draw_text_colored(2, 0, "ETH: LINK DOWN       ", RED, WHITE, &mut self.vcom);
        }

        // Row 3: USB status
        if usb_active {
            display::draw_text_colored(3, 0, "USB: ACTIVE (ECM)   ", GREEN, WHITE, &mut self.vcom);
        } else if cdc_ecm::is_configured() {
            display::draw_text_colored(3, 0, "USB: CONFIGURED     ", BLUE, WHITE, &mut self.vcom);
        } else {
            display::draw_text_colored(3, 0, "USB: WAITING...     ", RED, WHITE, &mut self.vcom);
        }

        // Row 4: MAC address
        display::draw_text(4, 0, "MAC:02004711BB      ", &mut self.vcom);

        // Row 5-6: frame counters
        let mut line = [b' '; 22];
        line[0] = b'T';
        line[1] = b'X';
        line[2] = b':';
        let mut nbuf = [b' '; 10];
        let n = fmt_u32(h2d, &mut nbuf);
        line[3..3 + n].copy_from_slice(&nbuf[10 - n..]);
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(5, 0, s, BLACK, WHITE, &mut self.vcom);

        let mut line = [b' '; 22];
        line[0] = b'R';
        line[1] = b'X';
        line[2] = b':';
        let n = fmt_u32(d2h, &mut nbuf);
        line[3..3 + n].copy_from_slice(&nbuf[10 - n..]);
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(6, 0, s, BLACK, WHITE, &mut self.vcom);

        // Row 8: uptime
        let secs = now_ms / 1000;
        let mins = secs / 60;
        let s_rem = secs % 60;
        let mut line = [b' '; 22];
        line[0] = b'U';
        line[1] = b'p';
        line[2] = b':';
        line[3] = b' ';
        // Format MM:SS
        line[4] = b'0' + (mins / 10 % 10) as u8;
        line[5] = b'0' + (mins % 10) as u8;
        line[6] = b':';
        line[7] = b'0' + (s_rem / 10) as u8;
        line[8] = b'0' + (s_rem % 10) as u8;
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(8, 0, s, BLACK, WHITE, &mut self.vcom);
    }
}

// ---------- ETH interrupt handler ----------

#[interrupt]
fn ETH() {
    slstk3701a::eth::irq_handler();
}

// ---------- USB device setup ----------

efm32gg11b_usb::usb_device!(CdcEcmClass);

// ---------- entry ----------

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // LEDs: PH10 (LED0), PH13 (LED1) push-pull.
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());
    led_clear(&p.gpio, LED0 | LED1);

    // VBUSEN: PF5 push-pull, driven low (device mode -- don't supply VBUS).
    p.gpio.pf_model().modify(|_, w| w.mode5().pushpull());
    p.gpio
        .pf_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });

    // ---- Initialize display ----
    display::init();
    let mut lcd = LcdState::new();
    display::clear(&mut lcd.vcom);
    display::draw_text_colored(0, 0, "      USB NIC       ", BLUE, WHITE, &mut lcd.vcom);
    display::draw_text(2, 0, "Initializing...", &mut lcd.vcom);

    // ---- Initialize Ethernet hardware ----
    slstk3701a::eth::init();

    // Start SysTick -- HFCLK is still 19 MHz HFRCO.
    systick_init(&mut cp.SYST);

    display::draw_text(2, 0, "ETH: waiting link...", &mut lcd.vcom);
    defmt::info!("USB NIC demo -- waiting for Ethernet link...");
    while !slstk3701a::eth::link_up() {
        cortex_m::asm::delay(19_000); // ~1 ms
    }
    led_set(&p.gpio, LED0);
    defmt::info!("Ethernet link up");

    // Enable ETH interrupts (RXCMPLT + TXCMPLT).
    slstk3701a::eth::enable_interrupts();
    unsafe { pac::NVIC::unmask(pac::Interrupt::ETH) };

    // ---- Initialize USB CDC-ECM ----
    let dev = UsbDevice::init(&p.cmu, &p.usb, CdcEcmClass::new(), cdc_ecm::usb_config());

    defmt::info!("USB CDC-ECM initialized (DMA={})", efm32gg11b_usb::DMA_MODE);
    defmt::info!("NOTE: Set power switch to USB and connect Micro-AB cable");
    usb_start(dev);

    display::draw_text(3, 0, "USB: waiting...     ", &mut lcd.vcom);

    // Wait for USB configuration.
    while !cdc_ecm::is_configured() {
        let ms = MILLIS.load(Ordering::Relaxed);
        lcd.tick_vcom(ms);
        cortex_m::asm::wfi();
    }
    defmt::info!("USB configured, waiting for data interface...");

    // Wait for the host to activate the data interface (SET_INTERFACE alt=1).
    while !cdc_ecm::is_data_active() {
        let ms = MILLIS.load(Ordering::Relaxed);
        lcd.tick_vcom(ms);
        cortex_m::asm::wfi();
    }
    led_set(&p.gpio, LED1);
    defmt::info!("Data interface active -- bridging started");

    // Send link-up notifications to the host.
    usb_with_bus(|usb| {
        cdc_ecm::send_link_up_notifications(usb);
    });

    // ---- Event-driven bridge loop ----
    //
    // Three interrupt sources wake the CPU from WFI:
    //   - SysTick (1 kHz)    -- timing for LCD updates, heartbeat, VCOM
    //   - ETH RXCMPLT/TXCMPLT -- Ethernet frame events
    //   - USB ISR             -- USB enumeration, bulk transfers
    //
    // On each wake we process all pending work, then sleep again.
    // This is cooperative async without a runtime: each iteration handles
    // whatever events have accumulated, in priority order.

    let mut rx_buf = [0u8; 1536];
    let mut last_heartbeat: u32 = 0;
    let mut stats_h2d: u32 = 0;
    let mut stats_d2h: u32 = 0;
    let mut link_was_up = true;

    loop {
        // Sleep until any interrupt fires.
        cortex_m::asm::wfi();

        let ms = MILLIS.load(Ordering::Relaxed);

        // ---- Process ETH events ----
        // The ISR sets event flags; we also poll descriptors directly
        // because multiple frames can arrive per interrupt.
        let _eth_rx = slstk3701a::eth::take_rx_event();
        let _eth_tx = slstk3701a::eth::take_tx_event();

        // Host -> Wire: take reassembled frame from USB and send to ETH.
        if let Some(len) = cdc_ecm::take_rx_frame(&mut rx_buf) {
            if slstk3701a::eth::tx_available() {
                slstk3701a::eth::tx_packet(&rx_buf[..len]);
                stats_h2d += 1;
            }
        }

        // Wire -> Host: drain all available ETH frames into USB.
        while slstk3701a::eth::rx_available() {
            if let Some(data) = slstk3701a::eth::rx_packet() {
                let sent = usb_with_bus(|usb| cdc_ecm::submit_tx_frame(data, usb));
                if sent == Some(true) {
                    stats_d2h += 1;
                }
                slstk3701a::eth::rx_release();
            } else {
                break;
            }
        }

        // ---- Link monitoring ----
        let link = slstk3701a::eth::link_up();
        if link != link_was_up {
            link_was_up = link;
            if link {
                led_set(&p.gpio, LED0);
                defmt::info!("Ethernet link restored");
            } else {
                led_clear(&p.gpio, LED0);
                defmt::warn!("Ethernet link lost");
            }
        }

        // ---- Periodic heartbeat (defmt log) ----
        if ms.wrapping_sub(last_heartbeat) >= 10_000 {
            last_heartbeat = ms;
            defmt::info!("heartbeat t={}ms h2d={} d2h={}", ms, stats_h2d, stats_d2h);
            slstk3701a::eth::dump_status();
        }

        // ---- LCD updates ----
        lcd.tick_vcom(ms);
        lcd.draw_stats(ms, link, cdc_ecm::is_data_active(), stats_h2d, stats_d2h);
    }
}
