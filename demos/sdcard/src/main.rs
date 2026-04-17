//! MicroSD card reader demo for the SLSTK3701A starter kit.
//!
//! Detects a microSD card via the on-board slot, initialises it through the
//! native SDIO controller, reads the card identity (CID) and capacity, then
//! displays the information on the Color Memory LCD.  Performs a read-back
//! test of block 0 (MBR) and displays the first 16 bytes as a hex dump.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32gg11b_pac as pac;
use slstk3701a::{display, sdio};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock on HFBUS.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // Initialise the LCD.
    display::init();
    let mut vcom = false;
    display::clear(&mut vcom);

    display::draw_text(0, 1, "SD Card Demo", &mut vcom);
    display::draw_text(1, 1, "SLSTK3701A", &mut vcom);
    display::fill_rect(0, 20, 176, 1, true, &mut vcom);

    // Initialise the SDIO controller.
    sdio::init();

    // Wait for card insertion.
    display::draw_text(3, 1, "Insert microSD...", &mut vcom);
    defmt::info!("Waiting for SD card...");

    loop {
        // Poll for card detect.
        while !sdio::card_detect() {
            cortex_m::asm::delay(1_000_000);
            display::toggle_vcom();
        }

        display::draw_text(3, 1, "Card detected!   ", &mut vcom);
        defmt::info!("Card detected, initialising...");

        // Small debounce delay.
        cortex_m::asm::delay(5_000_000);

        match sdio::card_init() {
            Ok(info) => {
                show_card_info(&info, &mut vcom);
                read_and_show_mbr(&info, &mut vcom);

                // Idle while card is inserted.
                while sdio::card_detect() {
                    cortex_m::asm::delay(19_000_000);
                    display::toggle_vcom();
                }
                // Card removed.
                display::clear(&mut vcom);
                display::draw_text(0, 1, "SD Card Demo", &mut vcom);
                display::draw_text(1, 1, "SLSTK3701A", &mut vcom);
                display::fill_rect(0, 20, 176, 1, true, &mut vcom);
                display::draw_text(3, 1, "Card removed.    ", &mut vcom);
                sdio::power_off();
                defmt::info!("Card removed");
                cortex_m::asm::delay(19_000_000);
                display::draw_text(3, 1, "Insert microSD...", &mut vcom);
            }
            Err(e) => {
                display::draw_text(3, 1, "Init failed!     ", &mut vcom);
                defmt::error!("Card init failed: {:?}", e);
                sdio::power_off();
                cortex_m::asm::delay(19_000_000 * 3);
                display::draw_text(3, 1, "Insert microSD...", &mut vcom);
            }
        }
    }
}

fn show_card_info(info: &sdio::CardInfo, vcom: &mut bool) {
    let mut line = [b' '; 22];

    // Product name.
    let name = info.product_name();
    copy_str(&mut line, b"Name: ");
    line[6..11].copy_from_slice(&name);
    display::draw_text(4, 1, core::str::from_utf8(&line[..11]).unwrap_or("?"), vcom);

    // Manufacturer ID.
    let mid = info.manufacturer_id();
    let mut buf = [0u8; 22];
    let n = fmt_line(&mut buf, "MID: 0x", mid as u32, true);
    display::draw_text(5, 1, core::str::from_utf8(&buf[..n]).unwrap_or("?"), vcom);

    // OEM ID.
    let oem = info.oem_id();
    let mut line2 = [b' '; 22];
    copy_str(&mut line2, b"OEM: ");
    line2[5] = oem[0];
    line2[6] = oem[1];
    display::draw_text(6, 1, core::str::from_utf8(&line2[..7]).unwrap_or("?"), vcom);

    // Revision.
    let (maj, min) = info.product_revision();
    let mut rev_buf = [0u8; 22];
    copy_str(&mut rev_buf, b"Rev: ");
    rev_buf[5] = b'0' + maj;
    rev_buf[6] = b'.';
    rev_buf[7] = b'0' + min;
    display::draw_text(
        7,
        1,
        core::str::from_utf8(&rev_buf[..8]).unwrap_or("?"),
        vcom,
    );

    // Capacity.
    let mb = info.capacity_mb();
    let mut cap_buf = [0u8; 22];
    let cn = fmt_line(&mut cap_buf, "Size: ", mb, false);
    cap_buf[cn] = b' ';
    cap_buf[cn + 1] = b'M';
    cap_buf[cn + 2] = b'B';
    display::draw_text(
        8,
        1,
        core::str::from_utf8(&cap_buf[..cn + 3]).unwrap_or("?"),
        vcom,
    );

    // Type.
    if info.sdhc {
        display::draw_text(9, 1, "Type: SDHC", vcom);
    } else {
        display::draw_text(9, 1, "Type: SDSC", vcom);
    }

    // Block count.
    let mut blk_buf = [0u8; 22];
    let bn = fmt_line(&mut blk_buf, "Blocks: ", info.blocks, false);
    display::draw_text(
        10,
        1,
        core::str::from_utf8(&blk_buf[..bn]).unwrap_or("?"),
        vcom,
    );

    defmt::info!(
        "Card: {} {} MB, {} blocks, SDHC={}",
        core::str::from_utf8(&name).unwrap_or("?"),
        mb,
        info.blocks,
        info.sdhc
    );
}

fn read_and_show_mbr(info: &sdio::CardInfo, vcom: &mut bool) {
    display::fill_rect(0, 96, 176, 1, true, vcom);
    display::draw_text(12, 1, "MBR (block 0):", vcom);

    let mut block = [0u8; 512];
    match sdio::read_block(info, 0, &mut block) {
        Ok(()) => {
            defmt::info!("Block 0 read OK");

            // Show first 16 bytes as hex on the LCD.
            for row in 0..2u8 {
                let mut hex = [b' '; 22];
                for col in 0..8 {
                    let byte = block[row as usize * 8 + col];
                    let off = col * 3;
                    hex[off] = hex_digit(byte >> 4);
                    hex[off + 1] = hex_digit(byte & 0x0F);
                }
                let text_row = 13 + row;
                display::draw_text(
                    text_row,
                    1,
                    core::str::from_utf8(&hex[..23]).unwrap_or("?"),
                    vcom,
                );
            }

            // Check for MBR signature (0x55AA at offset 510).
            if block[510] == 0x55 && block[511] == 0xAA {
                display::draw_text(16, 1, "MBR signature OK", vcom);
                defmt::info!("MBR signature valid (0x55AA)");
            } else {
                display::draw_text(16, 1, "No MBR signature", vcom);
                defmt::warn!("MBR signature: {:02x}{:02x}", block[510], block[511]);
            }
        }
        Err(e) => {
            display::draw_text(13, 1, "Read error!", vcom);
            defmt::error!("Block 0 read failed: {:?}", e);
        }
    }
}

// ---------- formatting helpers ----------

fn hex_digit(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'A' + nibble - 10
    }
}

fn copy_str(dst: &mut [u8], src: &[u8]) {
    let len = src.len().min(dst.len());
    dst[..len].copy_from_slice(&src[..len]);
}

/// Format "prefix<number>" into buf.  Returns total length written.
/// If `hex` is true, formats the number as 0xNN.
fn fmt_line(buf: &mut [u8; 22], prefix: &str, val: u32, hex: bool) -> usize {
    let pb = prefix.as_bytes();
    let plen = pb.len().min(22);
    buf[..plen].copy_from_slice(&pb[..plen]);
    if hex {
        // Format as hex (up to 8 digits).
        let mut tmp = [0u8; 8];
        let mut v = val;
        let mut len = 0;
        if v == 0 {
            tmp[0] = b'0';
            len = 1;
        } else {
            while v > 0 && len < 8 {
                tmp[len] = hex_digit((v & 0xF) as u8);
                v >>= 4;
                len += 1;
            }
            // Reverse.
            for i in 0..len / 2 {
                tmp.swap(i, len - 1 - i);
            }
        }
        buf[plen] = b'0';
        buf[plen + 1] = b'x';
        let start = plen + 2;
        buf[start..start + len].copy_from_slice(&tmp[..len]);
        start + len
    } else {
        // Format as decimal.
        let mut tmp = [0u8; 10];
        let mut v = val;
        let mut len = 0;
        if v == 0 {
            tmp[0] = b'0';
            len = 1;
        } else {
            while v > 0 && len < 10 {
                tmp[len] = b'0' + (v % 10) as u8;
                v /= 10;
                len += 1;
            }
            for i in 0..len / 2 {
                tmp.swap(i, len - 1 - i);
            }
        }
        buf[plen..plen + len].copy_from_slice(&tmp[..len]);
        plen + len
    }
}
