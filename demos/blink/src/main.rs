//! Blink LED0 and LED1 alternately on the SLSTK3701A.
//!
//! LED0 = PH10, LED1 = PH13 (active-high push-pull).

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock on HFBUS.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // Configure PH10 and PH13 as push-pull outputs.
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());

    defmt::info!("Blinking LEDs");

    loop {
        // LED0 on, LED1 off
        p.gpio
            .ph_dout()
            .modify(|r, w| unsafe { w.bits((r.bits() | (1 << 10)) & !(1 << 13)) });
        cortex_m::asm::delay(19_000_000); // ~1s at 19 MHz HFRCO default

        // LED0 off, LED1 on
        p.gpio
            .ph_dout()
            .modify(|r, w| unsafe { w.bits((r.bits() | (1 << 13)) & !(1 << 10)) });
        cortex_m::asm::delay(19_000_000);
    }
}
