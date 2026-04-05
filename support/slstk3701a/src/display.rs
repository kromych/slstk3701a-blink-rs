//! Driver for the Sharp Memory LCD (LS013B7DH06) on the SLSTK3701A board.
//!
//! The display is 128x128 pixels, 3-bit RGB color, driven via USART1 SPI.
//! Each pixel is 3 bits (R, G, B), so each row is 128 × 3 / 8 = 48 bytes.
//!
//! Pin mapping (SLSTK3701A):
//!   PA14 – USART1_TX  (SPI MOSI / SI),  location 6
//!   PC15 – USART1_CLK (SPI SCK / SCLK), location 3
//!   PC14 – LCD chip-select (SCS, active HIGH, manual GPIO)
//!   PA11 – EXTCOMIN (VCOM inversion toggle)
//!   PA9  – LCD display-select (DISP_ENABLE)

use crate::font8x8::{reverse_bits, FONT};
use efm32gg11b_pac as pac;

// ---------- display geometry ----------

/// Display width in pixels.
pub const WIDTH: u8 = 128;
/// Display height in pixels.
pub const HEIGHT: u8 = 128;
/// Bits per pixel (RGB 3-bit).
pub const BPP: usize = 3;
/// Bytes per pixel row (128 × 3 / 8 = 48).
pub const BYTES_PER_ROW: usize = (WIDTH as usize * BPP) / 8;
/// Number of 8-pixel-tall text rows (128 / 8).
pub const TEXT_ROWS: u8 = 16;
/// Number of 8-pixel-wide text columns (128 / 8).
pub const TEXT_COLS: u8 = 16;

// ---------- GPIO bit positions ----------

const PA_DISP_SEL: u32 = 1 << 9;
const PA_EXTCOMIN: u32 = 1 << 11;
const PC_CS: u32 = 1 << 14;

// ---------- raw PAC accessors ----------

fn gpio() -> &'static pac::gpio::RegisterBlock {
    unsafe { &*pac::Gpio::ptr() }
}

fn usart1() -> &'static pac::usart1::RegisterBlock {
    unsafe { &*pac::Usart1::ptr() }
}

// ---------- GPIO helpers ----------

#[inline]
fn cs_high() {
    gpio()
        .pc_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | PC_CS) });
}

#[inline]
fn cs_low() {
    gpio()
        .pc_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !PC_CS) });
}

// ---------- SPI helpers ----------

#[inline]
fn spi_write(byte: u8) {
    let u = usart1();
    while u.status().read().txbl().bit_is_clear() {}
    u.txdata().write(|w| unsafe { w.bits(byte as u32) });
}

fn spi_wait_done() {
    while usart1().status().read().txc().bit_is_clear() {}
}

#[inline]
fn delay_us(us: u32) {
    cortex_m::asm::delay(us * 19);
}

#[inline]
fn scs_setup_delay() {
    delay_us(6);
}

#[inline]
fn scs_hold_delay() {
    delay_us(2);
}

// ---------- 3-bit pixel helpers ----------

/// Set a pixel in a 3-bpp row buffer.
///
/// Each pixel occupies 3 consecutive bits, LSB first within each byte.
/// `color` is a 3-bit value: bit 0 = blue, bit 1 = green, bit 2 = red.
/// White = 0b111, black = 0b000.
#[inline]
fn set_pixel(row_buf: &mut [u8; BYTES_PER_ROW], x: usize, color: u8) {
    let bit_offset = x * BPP;
    let byte_idx = bit_offset / 8;
    let bit_idx = bit_offset % 8;

    // The 3 pixel bits may span two bytes.
    let bits_in_first = (8 - bit_idx).min(3);
    let mask1 = ((1u8 << bits_in_first) - 1) << bit_idx;
    row_buf[byte_idx] = (row_buf[byte_idx] & !mask1) | ((color << bit_idx) & mask1);

    if bits_in_first < 3 {
        let remaining = 3 - bits_in_first;
        let mask2 = (1u8 << remaining) - 1;
        row_buf[byte_idx + 1] =
            (row_buf[byte_idx + 1] & !mask2) | ((color >> bits_in_first) & mask2);
    }
}

// ---------- public API ----------

/// One-time initialisation: clocks, USART1 SPI mode, GPIO pins, display power.
pub fn init() {
    let cmu = unsafe { &*pac::Cmu::ptr() };
    let u = usart1();
    let gpio = gpio();

    // Enable USART1 peripheral clock on HFPER bus.
    cmu.hfperclken0().modify(|_, w| w.usart1().set_bit());

    // PA9 (DISP_SEL), PA11 (EXTCOMIN), PA14 (USART1_TX / MOSI) → push-pull.
    gpio.pa_modeh()
        .modify(|_, w| w.mode9().pushpull().mode11().pushpull().mode14().pushpull());

    // PC14 (CS), PC15 (USART1_CLK / SCLK) → push-pull.
    gpio.pc_modeh()
        .modify(|_, w| w.mode14().pushpull().mode15().pushpull());

    // CS low initially.
    cs_low();

    // Synchronous mode, LSB first (MSBF=0), CPOL=0, CPHA=0.
    u.ctrl().write(|w| w.sync().set_bit());
    u.frame().write(|w| w.databits().eight());

    // ~1 MHz SPI clock at 19 MHz HFPERCLK.
    u.clkdiv().write(|w| unsafe { w.bits(2176) });

    // Route TX to location 6 (PA14), CLK to location 3 (PC15).
    u.routeloc0().write(|w| w.txloc().loc6().clkloc().loc3());
    u.routepen()
        .write(|w| w.txpen().set_bit().clkpen().set_bit());

    // Enable master mode and TX.
    u.cmd()
        .write(|w| w.masteren().set_bit().txen().set_bit().rxen().set_bit());

    // Power on the display.
    gpio.pa_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | PA_DISP_SEL) });

    cortex_m::asm::delay(200_000);
}

/// Clear the entire display to white.
pub fn clear(vcom: &mut bool) {
    let cmd = 0x04 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    scs_setup_delay();
    spi_write(cmd);
    spi_write(0x00);
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

/// Toggle EXTCOMIN pin (call at ~1 Hz to prevent DC bias damage).
pub fn toggle_vcom() {
    gpio()
        .pa_douttgl()
        .write(|w| unsafe { w.bits(PA_EXTCOMIN) });
}

/// Write a single pixel row (0-127) with 48 bytes of 3-bpp data.
pub fn write_row(row: u8, data: &[u8; BYTES_PER_ROW], vcom: &mut bool) {
    let cmd = 0x01 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    scs_setup_delay();
    spi_write(cmd);
    spi_write(row + 1);
    for &b in data.iter() {
        spi_write(b);
    }
    spi_write(0x00);
    spi_write(0x00);
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

/// Write multiple consecutive pixel rows starting at `start_row`.
pub fn write_rows(start_row: u8, rows: &[[u8; BYTES_PER_ROW]], vcom: &mut bool) {
    if rows.is_empty() {
        return;
    }
    let cmd = 0x01 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    scs_setup_delay();
    spi_write(cmd);
    for (i, row_data) in rows.iter().enumerate() {
        spi_write(start_row + i as u8 + 1);
        for &b in row_data.iter() {
            spi_write(b);
        }
        spi_write(0x00); // line trailer
    }
    spi_write(0x00); // final trailer
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

/// Render a text string at the given text-row and text-column position.
///
/// Characters are 8x8 pixels. Background is white, text is black.
pub fn draw_text(row: u8, col: u8, text: &str, vcom: &mut bool) {
    if row >= TEXT_ROWS {
        return;
    }

    // White row: all pixel bits set (0xFF).
    let mut pixel_rows = [[0xFFu8; BYTES_PER_ROW]; 8];

    for (ci, ch) in text.bytes().enumerate() {
        let cx = col as usize + ci;
        if cx >= TEXT_COLS as usize {
            break;
        }
        let glyph_idx = if (32..=126).contains(&ch) {
            (ch - 32) as usize
        } else {
            0
        };
        let glyph = &FONT[glyph_idx];
        for py in 0..8 {
            // Font: MSB = leftmost pixel. Reverse to get LSB = leftmost.
            let font_byte = reverse_bits(glyph[py]);
            // Each character is 8 pixels at column cx*8.
            for px in 0..8 {
                let on = (font_byte >> px) & 1 != 0;
                if on {
                    // Black pixel: color = 0b000.
                    set_pixel(&mut pixel_rows[py], cx * 8 + px, 0b000);
                }
                // White pixels are already 0xFF.
            }
        }
    }

    let start = row * 8;
    write_rows(start, &pixel_rows, vcom);
}

/// Fill a rectangular region (in pixel coordinates) with black or white.
pub fn fill_rect(x: u8, y: u8, w: u8, h: u8, black: bool, vcom: &mut bool) {
    let x0 = x.min(WIDTH) as usize;
    let y0 = y.min(HEIGHT);
    let x1 = (x as u16 + w as u16).min(WIDTH as u16) as usize;
    let y1 = (y as u16 + h as u16).min(HEIGHT as u16) as u8;

    let color = if black { 0b000 } else { 0b111 };

    for row in y0..y1 {
        let mut data = [0xFFu8; BYTES_PER_ROW]; // white
        if black {
            for px in x0..x1 {
                set_pixel(&mut data, px, color);
            }
        }
        write_row(row, &data, vcom);
    }
}

/// Format a `u32` as decimal into a caller-provided buffer.
pub fn format_u32(value: u32, buf: &mut [u8; 10]) -> &[u8] {
    if value == 0 {
        buf[0] = b'0';
        return &buf[..1];
    }
    let mut v = value;
    let mut i = 10;
    while v > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    &buf[i..]
}
