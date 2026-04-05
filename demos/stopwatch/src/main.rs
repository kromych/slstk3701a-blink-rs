//! Stopwatch demo for the SLSTK3701A board.
//!
//! Uses the RTCC at 32768 Hz for accurate timekeeping and the Sharp Memory LCD
//! to display elapsed time in MM:SS.d format (1/10 second resolution).
//!
//! Controls:
//!   BTN0 (PC8) - Start / Stop
//!   BTN1 (PC9) - Lap (while running) / Reset (while stopped)
//!
//! Up to 4 lap times are shown on screen.

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

/// RTCC ticks accumulated (each tick = 1/10 second).
static TICKS: AtomicU32 = AtomicU32::new(0);

/// Button event flags.
static BTN0_PRESSED: AtomicBool = AtomicBool::new(false);
static BTN1_PRESSED: AtomicBool = AtomicBool::new(false);

/// RTCC compare ticks for 1/10 second (32768 / 10 ≈ 3277).
const TENTH_SEC_TICKS: u32 = 3277;

const LED0: u32 = 1 << 10;
const LED1: u32 = 1 << 13;

fn led_on(led: u32) {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | led) });
}

fn led_off(led: u32) {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !led) });
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // --- Display ---
    display::init();
    let mut vcom = false;
    display::clear(&mut vcom);

    // --- LEDs ---
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());
    led_off(LED0 | LED1);

    // --- Buttons: PC8 (BTN0, even), PC9 (BTN1, odd) ---
    p.gpio
        .pc_modeh()
        .modify(|_, w| w.mode8().inputpullfilter().mode9().inputpullfilter());
    p.gpio
        .pc_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8) | (1 << 9)) });
    p.gpio
        .extipselh()
        .modify(|_, w| w.extipsel8().portc().extipsel9().portc());
    p.gpio
        .extifall()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8) | (1 << 9)) });
    p.gpio
        .ifc()
        .write(|w| unsafe { w.ext().bits((1 << 8) | (1 << 9)) });
    p.gpio
        .ien()
        .modify(|r, w| unsafe { w.ext().bits(r.ext().bits() | (1 << 8) | (1 << 9)) });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO_EVEN);
        pac::NVIC::unmask(pac::Interrupt::GPIO_ODD);
    }

    // --- RTCC setup ---
    p.cmu.oscencmd().write(|w| w.lfrcoen().set_bit());
    while p.cmu.status().read().lfrcordy().bit_is_clear() {}
    p.cmu.lfeclksel().write(|w| w.lfe().lfrco());
    p.cmu.lfeclken0().modify(|_, w| w.rtcc().set_bit());

    // Configure RTCC but don't enable yet.
    p.rtcc.ctrl().write(|w| w.enable().clear_bit());
    p.rtcc.cc0_ctrl().write(|w| w.mode().outputcompare());
    p.rtcc
        .cc0_ccv()
        .write(|w| unsafe { w.ccv().bits(TENTH_SEC_TICKS) });
    p.rtcc.ien().write(|w| w.cc0().set_bit());
    p.rtcc.ifc().write(|w| w.cc0().set_bit());

    unsafe { pac::NVIC::unmask(pac::Interrupt::RTCC) };

    critical_section::with(|cs| {
        RTCC.borrow(cs).replace(Some(p.rtcc));
    });

    // --- State ---
    let mut running = false;
    let mut tenths: u32 = 0;
    let mut laps: [u32; 4] = [0; 4];
    let mut lap_count: usize = 0;
    let mut display_dirty = true;

    // Draw static UI.
    display::draw_text(0, 0, "  STOPWATCH", &mut vcom);
    display::draw_text(2, 0, "BTN0:Start/Stop", &mut vcom);
    display::draw_text(3, 0, "BTN1: Lap/Reset", &mut vcom);
    draw_time(5, 0, &mut vcom);
    display::draw_text(7, 0, "  [STOPPED]", &mut vcom);

    defmt::info!("Stopwatch demo started");

    loop {
        cortex_m::asm::wfe();

        // Accumulate RTCC ticks.
        let new_ticks = TICKS.swap(0, Ordering::Relaxed);
        if running && new_ticks > 0 {
            tenths += new_ticks;
            display_dirty = true;
        }

        // BTN0: start/stop.
        if BTN0_PRESSED.swap(false, Ordering::Relaxed) {
            running = !running;
            if running {
                // Reset RTCC counter and start.
                critical_section::with(|cs| {
                    if let Some(rtcc) = RTCC.borrow(cs).borrow().as_ref() {
                        rtcc.cnt().write(|w| unsafe { w.bits(0) });
                        rtcc.cc0_ccv()
                            .write(|w| unsafe { w.ccv().bits(TENTH_SEC_TICKS) });
                        rtcc.ifc().write(|w| w.cc0().set_bit());
                        rtcc.ctrl().write(|w| w.enable().set_bit());
                    }
                });
                led_on(LED0);
                display::draw_text(7, 0, "  [RUNNING] ", &mut vcom);
            } else {
                critical_section::with(|cs| {
                    if let Some(rtcc) = RTCC.borrow(cs).borrow().as_ref() {
                        rtcc.ctrl().write(|w| w.enable().clear_bit());
                    }
                });
                led_off(LED0);
                display::draw_text(7, 0, "  [STOPPED] ", &mut vcom);
            }
            display_dirty = true;
            cortex_m::asm::delay(3_800_000); // ~200ms debounce
        }

        // BTN1: lap (if running) or reset (if stopped).
        if BTN1_PRESSED.swap(false, Ordering::Relaxed) {
            if running {
                if lap_count < 4 {
                    laps[lap_count] = tenths;
                    lap_count += 1;
                } else {
                    laps[0] = laps[1];
                    laps[1] = laps[2];
                    laps[2] = laps[3];
                    laps[3] = tenths;
                }
                led_on(LED1);
                cortex_m::asm::delay(1_900_000); // flash ~100ms
                led_off(LED1);
            } else {
                tenths = 0;
                lap_count = 0;
                laps = [0; 4];
                for r in 9..13 {
                    display::draw_text(r, 0, "                ", &mut vcom);
                }
            }
            display_dirty = true;
            cortex_m::asm::delay(3_800_000); // debounce
        }

        if display_dirty {
            display_dirty = false;

            draw_time(5, tenths, &mut vcom);

            if lap_count > 0 {
                display::draw_text(8, 0, "Laps:", &mut vcom);
                for (i, &lap) in laps.iter().enumerate().take(lap_count.min(4)) {
                    draw_lap(9 + i as u8, i as u8, lap, &mut vcom);
                }
            }

            display::toggle_vcom();
        }
    }
}

fn draw_time(row: u8, tenths: u32, vcom: &mut bool) {
    let total_secs = tenths / 10;
    let frac = tenths % 10;
    let mins = total_secs / 60;
    let secs = total_secs % 60;

    let mut buf = [b' '; 16];
    buf[2] = b'0' + (mins / 10 % 10) as u8;
    buf[3] = b'0' + (mins % 10) as u8;
    buf[4] = b':';
    buf[5] = b'0' + (secs / 10) as u8;
    buf[6] = b'0' + (secs % 10) as u8;
    buf[7] = b'.';
    buf[8] = b'0' + frac as u8;

    let s = core::str::from_utf8(&buf).unwrap_or("");
    display::draw_text(row, 0, s, vcom);
}

fn draw_lap(row: u8, idx: u8, tenths: u32, vcom: &mut bool) {
    let total_secs = tenths / 10;
    let frac = tenths % 10;
    let mins = total_secs / 60;
    let secs = total_secs % 60;

    let mut buf = [b' '; 16];
    buf[0] = b'L';
    buf[1] = b'1' + idx;
    buf[3] = b'0' + (mins / 10 % 10) as u8;
    buf[4] = b'0' + (mins % 10) as u8;
    buf[5] = b':';
    buf[6] = b'0' + (secs / 10) as u8;
    buf[7] = b'0' + (secs % 10) as u8;
    buf[8] = b'.';
    buf[9] = b'0' + frac as u8;

    let s = core::str::from_utf8(&buf).unwrap_or("");
    display::draw_text(row, 0, s, vcom);
}

// --- Interrupt handlers ---

#[interrupt]
fn RTCC() {
    TICKS.fetch_add(1, Ordering::Relaxed);
    critical_section::with(|cs| {
        if let Some(rtcc) = RTCC.borrow(cs).borrow().as_ref() {
            let next = rtcc
                .cc0_ccv()
                .read()
                .ccv()
                .bits()
                .wrapping_add(TENTH_SEC_TICKS);
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
    BTN0_PRESSED.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}

#[interrupt]
fn GPIO_ODD() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 9) });
    BTN1_PRESSED.store(true, Ordering::Relaxed);
    cortex_m::asm::sev();
}
