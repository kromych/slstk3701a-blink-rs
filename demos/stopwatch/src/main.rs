//! Stopwatch demo for the SLSTK3701A board.
//!
//! Uses SysTick at 1 kHz for timekeeping and the Color Memory LCD
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

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use efm32gg11b_pac as pac;
use pac::interrupt;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use slstk3701a::display;

// Colors (3-bit RGB: bit2=R, bit1=G, bit0=B).
const BLACK: u8 = 0b000;
const BLUE: u8 = 0b001;
const GREEN: u8 = 0b010;
const CYAN: u8 = 0b011;
const RED: u8 = 0b100;
const WHITE: u8 = 0b111;

static MILLIS: AtomicU32 = AtomicU32::new(0);

/// Button event flags.
static BTN0_PRESSED: AtomicBool = AtomicBool::new(false);
static BTN1_PRESSED: AtomicBool = AtomicBool::new(false);

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
    let mut cp = pac::CorePeripherals::take().unwrap();

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

    // --- SysTick at 1 kHz ---
    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(19_000 - 1);
    cp.SYST.clear_current();
    cp.SYST.enable_counter();
    cp.SYST.enable_interrupt();

    // --- State ---
    let mut running = false;
    let mut start_ms: u32 = 0;
    let mut elapsed_ms: u32 = 0;
    let mut laps: [u32; 4] = [0; 4];
    let mut lap_count: usize = 0;
    let mut last_tenths: u32 = u32::MAX;
    let mut last_vcom: u32 = 0;

    // Draw static UI with color.
    display::draw_text_colored(0, 0, "    STOPWATCH       ", BLUE, WHITE, &mut vcom);
    display::draw_text(2, 0, "BTN0:Start/Stop", &mut vcom);
    display::draw_text(3, 0, "BTN1: Lap/Reset", &mut vcom);
    draw_time(5, 0, &mut vcom);
    display::draw_text_colored(7, 0, "    [STOPPED]       ", RED, WHITE, &mut vcom);

    defmt::info!("Stopwatch demo started");

    loop {
        cortex_m::asm::wfi();

        let now = MILLIS.load(Ordering::Relaxed);

        // Update elapsed time.
        if running {
            elapsed_ms = now.wrapping_sub(start_ms);
        }

        // BTN0: start/stop.
        if BTN0_PRESSED.swap(false, Ordering::Relaxed) {
            running = !running;
            if running {
                start_ms = now.wrapping_sub(elapsed_ms);
                led_on(LED0);
                display::draw_text_colored(7, 0, "    [RUNNING]       ", GREEN, WHITE, &mut vcom);
            } else {
                led_off(LED0);
                display::draw_text_colored(7, 0, "    [STOPPED]       ", RED, WHITE, &mut vcom);
            }
            last_tenths = u32::MAX;
            cortex_m::asm::delay(3_800_000); // ~200ms debounce
            BTN0_PRESSED.store(false, Ordering::Relaxed); // discard bounce
        }

        // BTN1: lap (if running) or reset (if stopped).
        if BTN1_PRESSED.swap(false, Ordering::Relaxed) {
            if running {
                let tenths = elapsed_ms / 100;
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
                elapsed_ms = 0;
                start_ms = now;
                lap_count = 0;
                laps = [0; 4];
                for r in 9..13 {
                    display::draw_text(r, 0, "                    ", &mut vcom);
                }
                last_tenths = u32::MAX;
            }
            cortex_m::asm::delay(3_800_000); // debounce
            BTN1_PRESSED.store(false, Ordering::Relaxed); // discard bounce
        }

        let tenths = elapsed_ms / 100;
        if tenths != last_tenths {
            last_tenths = tenths;

            draw_time(5, tenths, &mut vcom);

            if lap_count > 0 {
                display::draw_text_colored(8, 0, "Laps:               ", BLUE, WHITE, &mut vcom);
                for (i, &lap) in laps.iter().enumerate().take(lap_count.min(4)) {
                    draw_lap(9 + i as u8, i as u8, lap, &mut vcom);
                }
            }
        }

        // Toggle VCOM at ~60 Hz.
        if now.wrapping_sub(last_vcom) >= 16 {
            last_vcom = now;
            display::toggle_vcom();
        }
    }
}

fn draw_time(row: u8, tenths: u32, vcom: &mut bool) {
    let total_secs = tenths / 10;
    let frac = tenths % 10;
    let mins = total_secs / 60;
    let secs = total_secs % 60;

    let mut buf = [b' '; 22];
    buf[4] = b'0' + (mins / 10 % 10) as u8;
    buf[5] = b'0' + (mins % 10) as u8;
    buf[6] = b':';
    buf[7] = b'0' + (secs / 10) as u8;
    buf[8] = b'0' + (secs % 10) as u8;
    buf[9] = b'.';
    buf[10] = b'0' + frac as u8;

    let s = core::str::from_utf8(&buf).unwrap_or("");
    display::draw_text_colored(row, 0, s, BLACK, WHITE, vcom);
}

fn draw_lap(row: u8, idx: u8, tenths: u32, vcom: &mut bool) {
    let total_secs = tenths / 10;
    let frac = tenths % 10;
    let mins = total_secs / 60;
    let secs = total_secs % 60;

    let mut buf = [b' '; 22];
    buf[1] = b'L';
    buf[2] = b'1' + idx;
    buf[4] = b'0' + (mins / 10 % 10) as u8;
    buf[5] = b'0' + (mins % 10) as u8;
    buf[6] = b':';
    buf[7] = b'0' + (secs / 10) as u8;
    buf[8] = b'0' + (secs % 10) as u8;
    buf[9] = b'.';
    buf[10] = b'0' + frac as u8;

    let color = if idx % 2 == 0 { BLUE } else { CYAN };
    let s = core::str::from_utf8(&buf).unwrap_or("");
    display::draw_text_colored(row, 0, s, color, WHITE, vcom);
}

// --- Interrupt handlers ---

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

#[interrupt]
fn GPIO_EVEN() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 8) });
    BTN0_PRESSED.store(true, Ordering::Relaxed);
}

#[interrupt]
fn GPIO_ODD() {
    let gpio = unsafe { &*pac::Gpio::ptr() };
    gpio.ifc().write(|w| unsafe { w.ext().bits(1 << 9) });
    BTN1_PRESSED.store(true, Ordering::Relaxed);
}
