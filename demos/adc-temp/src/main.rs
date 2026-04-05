//! Read the internal temperature sensor via ADC0 on the SLSTK3701A.
//!
//! The EFM32GG11B internal temperature sensor is at POSSEL = 0xF3 (TEMP).
//! Factory calibration from DEVINFO converts the raw ADC reading to °C.
//! Results are logged via defmt RTT every second.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;

/// Initialise ADC0 for single-conversion reads from the temperature sensor.
fn adc_init(p: &pac::Peripherals) {
    // Enable ADC0 clock on HFPER bus.
    p.cmu.hfperclken0().modify(|_, w| w.adc0().set_bit());

    // Warm-up mode: keep ADC warm between conversions.
    // Timebase ≈ 1 µs at 19 MHz → timebase = 18.
    // Prescaler: default (no division).
    p.adc0.ctrl().write(|w| unsafe {
        w.warmupmode()
            .keepadcwarm()
            .timebase()
            .bits(18)
            .presc()
            .bits(0)
    });

    // Single conversion: POSSEL = 0xF3 (TEMP), reference = 1.25V internal, 12-bit.
    p.adc0.singlectrl().write(|w| unsafe {
        w.possel().bits(0xF3);
        w.ref_()._1v25().res()._12bit().adj().clear_bit()
    });
}

/// Trigger a single ADC conversion and return the 12-bit result.
fn adc_read(adc: &pac::Adc0) -> u16 {
    adc.cmd().write(|w| w.singlestart().set_bit());
    while adc.status().read().singledv().bit_is_clear() {}
    adc.singledata().read().data().bits() as u16
}

/// Convert raw 12-bit ADC reading to temperature in °C × 10 (one decimal).
///
/// Uses factory calibration from DEVINFO:
///   T = T_cal - (ADC_raw - ADC_cal) × 1.25 / (4096 × 6.27e-3)
/// Scaled to ×10: delta_T_x10 = -(raw - cal) × 125000 / 25681
fn adc_to_temp_x10(raw: u16, cal_temp: i32, cal_adc: i32) -> i32 {
    let delta = raw as i32 - cal_adc;
    cal_temp * 10 - (delta * 12500) / 25681 * 10
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock (for potential LED indication).
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());

    // Read factory calibration from DEVINFO.
    let cal_temp = p.devinfo.cal().read().temp().bits() as i32;
    let cal_adc = p.devinfo.adc0cal2().read().temp1v25().bits() as i32;

    defmt::info!("DEVINFO: cal_temp={}°C, cal_adc={}", cal_temp, cal_adc,);

    adc_init(&p);

    defmt::info!("ADC temperature demo started");

    loop {
        let raw = adc_read(&p.adc0);
        let temp_x10 = adc_to_temp_x10(raw, cal_temp, cal_adc);
        let integer = temp_x10 / 10;
        let frac = (temp_x10 % 10).unsigned_abs();

        defmt::info!("ADC raw={}, temp={}.{}°C", raw, integer, frac,);

        cortex_m::asm::delay(19_000_000); // ~1s at 19 MHz
    }
}
