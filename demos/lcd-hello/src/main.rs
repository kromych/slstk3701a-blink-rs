//! Display a greeting and simple geometric patterns on the Color Memory LCD.

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
    display::draw_text(2, 1, "EFM32 Giant Gecko 11", &mut vcom);
    display::draw_text(3, 1, "SLSTK3701A", &mut vcom);
    display::draw_text(5, 1, "176x176 8-color LCD", &mut vcom);

    // Draw a horizontal rule.
    display::fill_rect(8, 52, 160, 2, true, &mut vcom);

    // Draw a checkerboard pattern (8x8 pixel blocks).
    for block_row in 0..5u8 {
        let mut rows = [[0xFFu8; display::BYTES_PER_ROW]; 8];
        for row in &mut rows {
            for bx in 0..22u8 {
                let checker = ((block_row as usize + bx as usize) & 1) != 0;
                if checker {
                    for px in 0..8 {
                        set_pixel_3bpp(row, bx as usize * 8 + px, 0b000);
                    }
                }
            }
        }
        let start = 60 + block_row * 8;
        display::write_rows(start, &rows, &mut vcom);
    }

    // Draw a border rectangle.
    display::fill_rect(0, 110, 176, 1, true, &mut vcom); // top
    display::fill_rect(0, 155, 176, 1, true, &mut vcom); // bottom
    display::fill_rect(0, 110, 1, 46, true, &mut vcom); // left
    display::fill_rect(175, 110, 1, 46, true, &mut vcom); // right

    // Label inside the border.
    display::draw_text(15, 3, "JDI LPM013M126A", &mut vcom);
    display::draw_text(16, 3, "Color Memory LCD", &mut vcom);

    // Draw color blocks to demonstrate 8 colors.
    let colors: [(u8, &str); 8] = [
        (0b000, "Blk"),
        (0b001, "Blu"),
        (0b010, "Grn"),
        (0b011, "Cyn"),
        (0b100, "Red"),
        (0b101, "Mag"),
        (0b110, "Yel"),
        (0b111, "Wht"),
    ];
    {
        let mut rows = [[0xFFu8; display::BYTES_PER_ROW]; 12];
        for (i, &(color, _)) in colors.iter().enumerate() {
            let x0 = 8 + i * 20;
            for row in rows.iter_mut() {
                for px in x0..x0 + 16 {
                    set_pixel_3bpp(row, px, color);
                }
            }
        }
        display::write_rows(140, &rows, &mut vcom);
    }

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
