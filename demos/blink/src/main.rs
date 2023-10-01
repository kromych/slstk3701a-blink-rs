//! This demo shows how to blink an LED and use the SysTick delay.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // If the Watchdog is not reset/disabled, the board will reboot.
    p.WDOG0.ctrl.modify(|_, w| w.en().clear_bit());
    p.WDOG1.ctrl.modify(|_, w| w.en().clear_bit());

    defmt::info!("Alive");

    loop {
        cortex_m::asm::wfi();
    }
}
