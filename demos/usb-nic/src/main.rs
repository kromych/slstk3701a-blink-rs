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

    // ---- Initialize Ethernet hardware ----
    slstk3701a::eth::init();

    // Start SysTick -- HFCLK is still 19 MHz HFRCO.
    systick_init(&mut cp.SYST);

    defmt::info!("USB NIC demo -- waiting for Ethernet link...");
    while !slstk3701a::eth::link_up() {
        cortex_m::asm::delay(19_000); // ~1 ms
    }
    led_set(&p.gpio, LED0);
    defmt::info!("Ethernet link up");

    // ---- Initialize USB CDC-ECM ----
    let dev = UsbDevice::init(&p.cmu, &p.usb, CdcEcmClass::new(), cdc_ecm::usb_config());

    defmt::info!("USB CDC-ECM initialized");
    defmt::info!("NOTE: Set power switch to USB and connect Micro-AB cable");
    usb_start(dev);

    // Wait for USB configuration.
    while !cdc_ecm::is_configured() {
        cortex_m::asm::wfi();
    }
    defmt::info!("USB configured, waiting for data interface...");

    // Wait for the host to activate the data interface (SET_INTERFACE alt=1).
    while !cdc_ecm::is_data_active() {
        cortex_m::asm::wfi();
    }
    led_set(&p.gpio, LED1);
    defmt::info!("Data interface active -- bridging started");

    // Send link-up notifications to the host.
    usb_with_bus(|usb| {
        cdc_ecm::send_link_up_notifications(usb);
    });

    // ---- Bridge loop ----
    let mut rx_buf = [0u8; 1536];
    let mut last_heartbeat: u32 = 0;
    let mut stats_h2d: u32 = 0;
    let mut stats_d2h: u32 = 0;

    loop {
        let ms = MILLIS.load(Ordering::Relaxed);

        // Periodic heartbeat.
        if ms.wrapping_sub(last_heartbeat) >= 10_000 {
            last_heartbeat = ms;
            defmt::info!("heartbeat t={}ms h2d={} d2h={}", ms, stats_h2d, stats_d2h);
            slstk3701a::eth::dump_status();
        }

        // Check link status.
        if !slstk3701a::eth::link_up() {
            led_clear(&p.gpio, LED0);
            defmt::warn!("Ethernet link lost -- waiting...");
            while !slstk3701a::eth::link_up() {
                cortex_m::asm::delay(190_000); // ~10 ms
            }
            led_set(&p.gpio, LED0);
            defmt::info!("Ethernet link restored");
        }

        // Host -> Wire: take reassembled frame from USB and send to ETH.
        if let Some(len) = cdc_ecm::take_rx_frame(&mut rx_buf) {
            if slstk3701a::eth::tx_available() {
                slstk3701a::eth::tx_packet(&rx_buf[..len]);
                stats_h2d += 1;
            }
        }

        // Wire -> Host: take frame from ETH and send to USB.
        if slstk3701a::eth::rx_available() {
            if let Some(data) = slstk3701a::eth::rx_packet() {
                let sent = usb_with_bus(|usb| cdc_ecm::submit_tx_frame(data, usb));
                if sent == Some(true) {
                    stats_d2h += 1;
                }
                slstk3701a::eth::rx_release();
            }
        }

        // Brief idle to avoid busy-spinning at full speed.
        cortex_m::asm::wfi();
    }
}
