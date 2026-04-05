//! Display a greeting and simple geometric patterns on the Sharp Memory LCD.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;
use slstk3701a::display;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock on HFBUS.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    display::init();

    let mut vcom = false;

    display::clear(&mut vcom);

    // Draw a title.
    display::draw_text(1, 1, "Hello, World!", &mut vcom);
    display::draw_text(2, 1, "EFM32 Giant", &mut vcom);
    display::draw_text(3, 1, "Gecko SLSTK3701A", &mut vcom);

    // Draw a horizontal rule.
    display::fill_rect(8, 36, 112, 2, true, &mut vcom);

    // Draw a checkerboard pattern (8x8 pixel blocks) in the lower half.
    for block_row in 0..5u8 {
        let mut rows = [[0xFFu8; display::BYTES_PER_ROW]; 8];
        for row in &mut rows {
            for bx in 0..16u8 {
                let checker = ((block_row as usize + bx as usize) & 1) != 0;
                if checker {
                    for px in 0..8 {
                        set_pixel_3bpp(row, bx as usize * 8 + px, 0b000);
                    }
                }
            }
        }
        let start = 48 + block_row * 8;
        display::write_rows(start, &rows, &mut vcom);
    }

    // Draw a border rectangle outline (1-pixel lines).
    for x in 0..128usize {
        set_pixel_line(x, 96, true, &mut vcom);
        set_pixel_line(x, 123, true, &mut vcom);
    }
    for y in 96..124u8 {
        set_pixel_line(0, y, true, &mut vcom);
        set_pixel_line(127, y, true, &mut vcom);
    }

    // Label inside the border.
    display::draw_text(13, 3, "Sharp LCD", &mut vcom);
    display::draw_text(14, 3, "128x128 RGB3", &mut vcom);

    defmt::info!("LCD hello demo running");

    loop {
        cortex_m::asm::delay(19_000_000);
        display::toggle_vcom();
    }
}

/// Set a single pixel in a 3-bpp row buffer.
fn set_pixel_3bpp(row_buf: &mut [u8; display::BYTES_PER_ROW], x: usize, color: u8) {
    let bit_offset = x * display::BPP;
    let byte_idx = bit_offset / 8;
    let bit_idx = bit_offset % 8;

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

/// Write a single pixel on a row (overwrites rest of row with white).
fn set_pixel_line(x: usize, y: u8, black: bool, vcom: &mut bool) {
    let mut row = [0xFFu8; display::BYTES_PER_ROW];
    if black {
        set_pixel_3bpp(&mut row, x, 0b000);
    }
    display::write_row(y, &row, vcom);
}
