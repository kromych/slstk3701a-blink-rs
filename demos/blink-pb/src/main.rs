//! Toggle LEDs with push buttons on the SLSTK3701A via GPIO interrupts.
//!
//! BTN0 = PC8 (even interrupt) → LED0 on, LED1 off
//! BTN1 = PC9 (odd interrupt)  → LED1 on, LED0 off
//! LED0 = PH10, LED1 = PH13 (active-high push-pull).

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use critical_section::Mutex;
use efm32gg11b_pac as pac;

use pac::interrupt;

static GPIO: Mutex<RefCell<Option<pac::Gpio>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock on HFBUS.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // Configure LEDs: PH10 and PH13 as push-pull outputs (both off).
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());
    p.gpio
        .ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !((1 << 10) | (1 << 13))) });

    // Configure buttons: PC8 and PC9 as inputs with pull-up (DOUT=1 for pull-up).
    p.gpio
        .pc_modeh()
        .modify(|_, w| w.mode8().inputpullfilter().mode9().inputpullfilter());
    p.gpio
        .pc_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8) | (1 << 9)) });

    // Route pins 8-9 to port C for external interrupts.
    p.gpio
        .extipselh()
        .modify(|_, w| w.extipsel8().portc().extipsel9().portc());

    // Trigger on falling edge (button press, active low).
    p.gpio
        .extifall()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8) | (1 << 9)) });

    // Clear pending and enable interrupts for pins 8 and 9.
    p.gpio
        .ifc()
        .write(|w| unsafe { w.ext().bits((1 << 8) | (1 << 9)) });
    p.gpio
        .ien()
        .modify(|r, w| unsafe { w.ext().bits(r.ext().bits() | (1 << 8) | (1 << 9)) });

    critical_section::with(|cs| {
        GPIO.borrow(cs).replace(Some(p.gpio));
    });

    // Unmask GPIO interrupts in NVIC.
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN);
        pac::NVIC::unmask(pac::Interrupt::GPIO_ODD);
    }

    defmt::info!("Push button demo ready");

    loop {
        cortex_m::asm::wfi();
    }
}

/// BTN0 (PC8, even pin) → LED0 on, LED1 off.
#[interrupt]
fn GPIO_EVEN() {
    critical_section::with(|cs| {
        if let Some(gpio) = GPIO.borrow(cs).borrow().as_ref() {
            gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 8) });
            gpio.ph_dout()
                .modify(|r, w| unsafe { w.bits((r.bits() | (1 << 10)) & !(1 << 13)) });
            defmt::info!("BTN0 pressed");
        }
    });
}

/// BTN1 (PC9, odd pin) → LED1 on, LED0 off.
#[interrupt]
fn GPIO_ODD() {
    critical_section::with(|cs| {
        if let Some(gpio) = GPIO.borrow(cs).borrow().as_ref() {
            gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 9) });
            gpio.ph_dout()
                .modify(|r, w| unsafe { w.bits((r.bits() | (1 << 13)) & !(1 << 10)) });
            defmt::info!("BTN1 pressed");
        }
    });
}
