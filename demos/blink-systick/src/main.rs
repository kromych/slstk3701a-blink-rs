//! Blink LEDs using the SysTick interrupt on the SLSTK3701A.
//!
//! LED0 = PH10, LED1 = PH13 (active-high push-pull).
//! SysTick fires every 1 ms; LEDs toggle every 500 ms.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::exception;

use core::cell::RefCell;
use cortex_m::peripheral::syst::SystClkSource;
use critical_section::Mutex;
use efm32gg11b_pac as pac;
use portable_atomic::{AtomicU32, Ordering};

static GPIO: Mutex<RefCell<Option<pac::Gpio>>> = Mutex::new(RefCell::new(None));
static TICK: AtomicU32 = AtomicU32::new(0);

const LED0: u32 = 1 << 10;
const LED1: u32 = 1 << 13;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock on HFBUS.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // Configure PH10 and PH13 as push-pull outputs.
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());

    critical_section::with(|cs| {
        GPIO.borrow(cs).replace(Some(p.gpio));
    });

    // Configure SysTick for 1 ms ticks.
    // Default HFRCO on GG11 is 19 MHz.
    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(19_000 - 1);
    cp.SYST.clear_current();
    cp.SYST.enable_interrupt();
    cp.SYST.enable_counter();

    defmt::info!("SysTick blink started");

    loop {
        cortex_m::asm::wfe();
    }
}

fn set_leds(gpio: &pac::Gpio, led0_on: bool) {
    let val = if led0_on {
        LED0 // LED0 on, LED1 off
    } else {
        LED1 // LED0 off, LED1 on
    };
    let clear = if led0_on { LED1 } else { LED0 };
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits((r.bits() | val) & !clear) });
}

#[exception]
fn SysTick() {
    let tick = TICK.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    if !tick.is_multiple_of(500) {
        return;
    }

    critical_section::with(|cs| {
        if let Some(gpio) = GPIO.borrow(cs).borrow().as_ref() {
            set_leds(gpio, (tick / 1000).is_multiple_of(2));
            defmt::info!("tick {}", tick);
        }
    });
}
