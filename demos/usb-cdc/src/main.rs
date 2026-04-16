//! USB CDC ACM (virtual serial port) demo for the SLSTK3701A.
//!
//! Echoes data received from the host.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;
use efm32gg11b_usb::cdc_acm::{self, CdcAcmClass, CdcAcmHandler};
use efm32gg11b_usb::{UsbBus, UsbDevice};

struct EchoHandler;

impl CdcAcmHandler for EchoHandler {
    fn data_received(&mut self, data: &[u8], usb: &UsbBus) {
        usb.ep_write(1, data);
    }

    fn tx_complete(&mut self, usb: &UsbBus) {
        usb.ep_prepare_out(1, 64, 0);
    }
}

efm32gg11b_usb::usb_device!(CdcAcmClass<EchoHandler>);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // LEDs: PH10 (LED0), PH13 (LED1) push-pull, initially off.
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());
    p.gpio
        .ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !((1 << 10) | (1 << 13))) });

    // VBUSEN: PF5 push-pull, driven low (device mode — don't supply VBUS).
    p.gpio.pf_model().modify(|_, w| w.mode5().pushpull());
    p.gpio
        .pf_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        CdcAcmClass::new(EchoHandler),
        cdc_acm::usb_config(),
    );

    defmt::info!(
        "USB CDC ACM serial port ready (DMA={})",
        efm32gg11b_usb::DMA_MODE
    );
    defmt::info!("NOTE: Set power switch to USB and connect cable to Micro-AB connector");
    usb_start(dev);

    loop {
        cortex_m::asm::wfi();
    }
}
