//! USB HID keyboard demo for the SLSTK3701A.
//!
//! Types "Hello from EFM32!\n" every 5 seconds.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;
use efm32gg11b_usb::hid_keyboard::{self, HidKeyboardClass, HidKeyboardHandler};
use efm32gg11b_usb::UsbDevice;

struct Greeting;

impl HidKeyboardHandler for Greeting {
    fn message(&self) -> &[u8] {
        b"Hello from EFM32!\n"
    }
}

efm32gg11b_usb::usb_device!(HidKeyboardClass);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // VBUSEN: PF5 push-pull, driven low (device mode — don't supply VBUS).
    p.gpio.pf_model().modify(|_, w| w.mode5().pushpull());
    p.gpio
        .pf_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });

    let dev = UsbDevice::init(&p.cmu, &p.usb, HidKeyboardClass, hid_keyboard::usb_config());

    defmt::info!("USB HID keyboard ready (DMA={})", efm32gg11b_usb::DMA_MODE);
    defmt::info!("NOTE: Set power switch to USB and connect cable to Micro-AB connector");
    usb_start(dev);
    hid_keyboard::run(&Greeting, |report| {
        usb_with_bus(|bus| bus.ep_write(1, report));
    });
}
