//! Count seconds using SysTick and display the value on the Color Memory LCD.
//! Push button 0 (PC8) resets the counter.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use efm32gg11b_pac as pac;
use pac::interrupt;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use slstk3701a::display;

static MILLIS: AtomicU32 = AtomicU32::new(0);
static RESET_REQ: AtomicBool = AtomicBool::new(false);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // Initialise the LCD.
    display::init();
    let mut vcom = false;
    display::clear(&mut vcom);

    display::draw_text(0, 0, "RTCC Counter", &mut vcom);
    display::draw_text(2, 0, "Seconds:", &mut vcom);
    display::draw_text(6, 0, "BTN0 = reset", &mut vcom);

    // Configure push button 0 (PC8) for interrupt on press.
    p.gpio.pc_modeh().modify(|_, w| w.mode8().inputpullfilter());
    p.gpio
        .pc_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8)) }); // pull-up
    p.gpio.extipselh().modify(|_, w| w.extipsel8().portc());
    p.gpio
        .extifall()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8)) });
    p.gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 8) });
    p.gpio
        .ien()
        .modify(|r, w| unsafe { w.ext().bits(r.ext().bits() | (1 << 8)) });

    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN) };

    // SysTick at 1 kHz (19 MHz / 19000 = 1 kHz).
    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(19_000 - 1);
    cp.SYST.clear_current();
    cp.SYST.enable_counter();
    cp.SYST.enable_interrupt();

    defmt::info!("LCD counter started");

    let mut last_sec = u32::MAX;
    let mut last_vcom: u32 = 0;
    loop {
        cortex_m::asm::wfi();

        if RESET_REQ.swap(false, Ordering::Relaxed) {
            MILLIS.store(0, Ordering::Relaxed);
            last_sec = u32::MAX;
        }

        let ms = MILLIS.load(Ordering::Relaxed);
        let secs = ms / 1000;

        if secs != last_sec {
            last_sec = secs;

            let mut buf = [0u8; 10];
            let digits = display::format_u32(secs, &mut buf);
            let mut line = [b' '; 16];
            line[..digits.len()].copy_from_slice(digits);

            let s = core::str::from_utf8(&line).unwrap_or("");
            display::draw_text(3, 0, s, &mut vcom);

            defmt::info!("Counter: {}", secs);
        }

        // Toggle VCOM at ~60 Hz.
        if ms.wrapping_sub(last_vcom) >= 16 {
            last_vcom = ms;
            display::toggle_vcom();
        }
    }
}

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

#[interrupt]
fn GPIO_EVEN() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 8) });
    RESET_REQ.store(true, Ordering::Relaxed);
}
