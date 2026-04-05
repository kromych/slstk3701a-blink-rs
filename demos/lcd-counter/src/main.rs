//! Count seconds using the RTCC and display the value on the Sharp Memory LCD.
//! Push button 0 (PC8) resets the counter.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use critical_section::Mutex;
use efm32gg11b_pac as pac;
use pac::interrupt;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use slstk3701a::display;

static RTCC: Mutex<RefCell<Option<pac::Rtcc>>> = Mutex::new(RefCell::new(None));
static SECONDS: AtomicU32 = AtomicU32::new(0);
static RESET_REQ: AtomicBool = AtomicBool::new(false);

/// LFRCO ticks per second.
const TICKS_PER_SEC: u32 = 32_768;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

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

    // --- RTCC setup ---
    // Enable LFRCO.
    p.cmu.oscencmd().write(|w| w.lfrcoen().set_bit());
    // Wait for LFRCO ready.
    while p.cmu.status().read().lfrcordy().bit_is_clear() {}

    // Select LFRCO as LFE clock (for RTCC).
    p.cmu.lfeclksel().write(|w| w.lfe().lfrco());

    // Enable RTCC clock.
    p.cmu.lfeclken0().modify(|_, w| w.rtcc().set_bit());

    // Configure RTCC: enable, no prescaler.
    p.rtcc.ctrl().write(|w| w.enable().set_bit());

    // CC0 in output-compare mode, compare value = 1 second.
    p.rtcc.cc0_ctrl().write(|w| w.mode().outputcompare());
    p.rtcc
        .cc0_ccv()
        .write(|w| unsafe { w.ccv().bits(TICKS_PER_SEC) });

    // Enable CC0 interrupt.
    p.rtcc.ien().write(|w| w.cc0().set_bit());
    p.rtcc.ifc().write(|w| w.cc0().set_bit());

    unsafe { pac::NVIC::unmask(pac::Interrupt::RTCC) };

    critical_section::with(|cs| {
        RTCC.borrow(cs).replace(Some(p.rtcc));
    });

    let mut last_displayed = u32::MAX;
    loop {
        if RESET_REQ.swap(false, Ordering::Relaxed) {
            SECONDS.store(0, Ordering::Relaxed);
        }

        let secs = SECONDS.load(Ordering::Relaxed);
        if secs != last_displayed {
            last_displayed = secs;

            let mut buf = [0u8; 10];
            let digits = display::format_u32(secs, &mut buf);
            let mut line = [b' '; 16];
            line[..digits.len()].copy_from_slice(digits);

            let s = core::str::from_utf8(&line).unwrap_or("");
            display::draw_text(3, 0, s, &mut vcom);

            defmt::info!("Counter: {}", secs);
        }

        display::toggle_vcom();
        cortex_m::asm::wfe();
    }
}

#[interrupt]
fn RTCC() {
    SECONDS.fetch_add(1, Ordering::Relaxed);
    critical_section::with(|cs| {
        if let Some(rtcc) = RTCC.borrow(cs).borrow().as_ref() {
            // Advance compare value by one second.
            let next = rtcc
                .cc0_ccv()
                .read()
                .ccv()
                .bits()
                .wrapping_add(TICKS_PER_SEC);
            rtcc.cc0_ccv().write(|w| unsafe { w.ccv().bits(next) });
            rtcc.ifc().write(|w| w.cc0().set_bit());
        }
    });
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_EVEN() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 8) });
    RESET_REQ.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}
