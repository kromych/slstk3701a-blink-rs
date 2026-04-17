//! SD card driver for the SLSTK3701A board using the native SDIO controller.
//!
//! The EFM32GG11B integrates an SD Host Controller (SDHCI-compliant) at
//! 0x400F_1000.  On the SLSTK3701A starter kit the microSD card slot is
//! wired to SDIO Route #1:
//!
//! ```text
//!   PE14 - SDIO_CLK        PA0 - DAT0
//!   PE15 - SDIO_CMD        PA1 - DAT1
//!   PB10 - Card Detect     PA2 - DAT2
//!   PB9  - Write Protect   PA3 - DAT3
//!   PE7  - SD Power Enable (active high)
//! ```
//!
//! Routing locations: CLK/CMD/DAT = LOC1, CD = LOC3, WP = LOC3.
//!
//! # Usage
//!
//! Call [`init()`] once after enabling the GPIO clock.  Use [`card_detect()`]
//! to check physical insertion.  [`card_init()`] performs the full SD card
//! enumeration sequence (CMD0 -> CMD8 -> ACMD41 -> CMD2 -> CMD3 -> CMD9 ->
//! CMD7 -> ACMD6) and returns a [`CardInfo`] on success.  Then use
//! [`read_block()`] and [`write_block()`] for 512-byte block I/O.

use efm32gg11b_pac as pac;

// ---------- public types ----------

#[derive(Clone, Copy, Debug, defmt::Format)]
pub enum SdError {
    Timeout,
    CmdError,
    DataError,
    NoCard,
    InitFailed,
}

#[derive(Clone, Copy, Debug)]
pub struct CardInfo {
    /// Raw CID register (128 bits, big-endian word order).
    pub cid: [u32; 4],
    /// Raw CSD register (128 bits, big-endian word order).
    pub csd: [u32; 4],
    /// Relative Card Address.
    pub rca: u16,
    /// True if the card is SDHC/SDXC (block-addressed).
    pub sdhc: bool,
    /// Card capacity in 512-byte blocks.
    pub blocks: u32,
}

impl CardInfo {
    /// Manufacturer ID (MID) from CID.
    pub fn manufacturer_id(&self) -> u8 {
        (self.cid[3] >> 24) as u8
    }

    /// OEM/Application ID from CID (2 ASCII characters).
    pub fn oem_id(&self) -> [u8; 2] {
        [
            ((self.cid[3] >> 16) & 0xFF) as u8,
            ((self.cid[3] >> 8) & 0xFF) as u8,
        ]
    }

    /// Product name from CID (5 ASCII characters).
    pub fn product_name(&self) -> [u8; 5] {
        [
            (self.cid[3] & 0xFF) as u8,
            (self.cid[2] >> 24) as u8,
            ((self.cid[2] >> 16) & 0xFF) as u8,
            ((self.cid[2] >> 8) & 0xFF) as u8,
            (self.cid[2] & 0xFF) as u8,
        ]
    }

    /// Product revision (major.minor) from CID.
    pub fn product_revision(&self) -> (u8, u8) {
        let rev = (self.cid[1] >> 24) as u8;
        (rev >> 4, rev & 0x0F)
    }

    /// Card capacity in megabytes.
    pub fn capacity_mb(&self) -> u32 {
        self.blocks / 2048
    }
}

// ---------- constants ----------

const SD_BLOCK_SIZE: u16 = 512;

// SD command indices
const CMD0: u8 = 0; // GO_IDLE_STATE
const CMD2: u8 = 2; // ALL_SEND_CID
const CMD3: u8 = 3; // SEND_RELATIVE_ADDR
const CMD6: u8 = 6; // SWITCH_FUNC (ACMD6 = SET_BUS_WIDTH)
const CMD7: u8 = 7; // SELECT/DESELECT_CARD
const CMD8: u8 = 8; // SEND_IF_COND
const CMD9: u8 = 9; // SEND_CSD
const _CMD12: u8 = 12; // STOP_TRANSMISSION
const _CMD13: u8 = 13; // SEND_STATUS
const CMD16: u8 = 16; // SET_BLOCKLEN
const CMD17: u8 = 17; // READ_SINGLE_BLOCK
const CMD24: u8 = 24; // WRITE_BLOCK
const CMD55: u8 = 55; // APP_CMD
const ACMD41: u8 = 41; // SD_SEND_OP_COND

// Response types (maps to RESPTYPESEL field)
const RESP_NONE: u8 = 0;
const RESP_136: u8 = 1; // R2 (CID, CSD)
const RESP_48: u8 = 2; // R1, R3, R6, R7
const RESP_48_BUSY: u8 = 3; // R1b

// ---------- PAC accessors ----------

fn sdio() -> &'static pac::sdio::RegisterBlock {
    unsafe { &*pac::Sdio::ptr() }
}

fn cmu() -> &'static pac::cmu::RegisterBlock {
    unsafe { &*pac::Cmu::ptr() }
}

fn gpio() -> &'static pac::gpio::RegisterBlock {
    unsafe { &*pac::Gpio::ptr() }
}

// ---------- delay ----------

fn delay_ms(ms: u32) {
    cortex_m::asm::delay(ms * 19_000); // ~19 MHz HFRCO
}

fn delay_us(us: u32) {
    cortex_m::asm::delay(us * 19);
}

// ---------- GPIO helpers ----------

fn gpio_set(
    dout: &pac::generic::Reg<
        impl pac::generic::Writable + pac::generic::Readable + pac::generic::RegisterSpec<Ux = u32>,
    >,
    mask: u32,
) {
    dout.modify(|r, w| unsafe { w.bits(r.bits() | mask) });
}

fn gpio_clear(
    dout: &pac::generic::Reg<
        impl pac::generic::Writable + pac::generic::Readable + pac::generic::RegisterSpec<Ux = u32>,
    >,
    mask: u32,
) {
    dout.modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
}

// ---------- public API ----------

/// Check if a card is physically inserted (CD pin is active low).
pub fn card_detect() -> bool {
    gpio().pb_din().read().bits() & (1 << 10) == 0
}

/// One-time hardware initialisation: GPIO, clocks, SDIO controller.
///
/// The caller must have enabled the GPIO clock on HFBUS before calling.
pub fn init() {
    let gpio = gpio();
    let cmu = cmu();
    let s = sdio();

    // ---- GPIO configuration ----

    // PE7: SD power enable (push-pull output, active high, initially off).
    gpio.pe_model().modify(|_, w| w.mode7().pushpull());
    gpio_clear(gpio.pe_dout(), 1 << 7);

    // PE14: SDIO_CLK, PE15: SDIO_CMD (push-pull for output pins).
    gpio.pe_modeh()
        .modify(|_, w| w.mode14().pushpull().mode15().pushpull());

    // PA0-PA3: DAT0-DAT3 (push-pull with input for bidirectional).
    gpio.pa_model().modify(|_, w| {
        w.mode0()
            .pushpull()
            .mode1()
            .pushpull()
            .mode2()
            .pushpull()
            .mode3()
            .pushpull()
    });

    // PB10: Card Detect (input with pull-up).
    gpio.pb_modeh().modify(|_, w| w.mode10().input());
    gpio_set(gpio.pb_dout(), 1 << 10); // enable pull-up

    // PB9: Write Protect (input).
    gpio.pb_modeh().modify(|_, w| w.mode9().input());

    // ---- Enable SDIO peripheral clock ----

    // Select HFRCO as SDIO reference clock (default, ~19 MHz).
    cmu.sdioctrl().write(|w| w.sdioclksel().hfrco());

    // Enable SDIO bus clock.
    cmu.hfbusclken0().modify(|_, w| w.sdio().set_bit());
    let _ = cmu.hfbusclken0().read(); // sync

    // ---- Configure SDIO pin routing ----
    // Route #1: CLK/CMD/DAT = LOC1, CD = LOC3, WP = LOC3.
    s.routeloc0().write(|w| {
        w.datloc()
            .loc1()
            .cdloc()
            .loc3()
            .wploc()
            .loc3()
            .clkloc()
            .loc1()
    });
    s.routeloc1().write(|w| w.cmdloc().loc1());
    s.routepen().write(|w| {
        w.clkpen()
            .set_bit()
            .cmdpen()
            .set_bit()
            .d0pen()
            .set_bit()
            .d1pen()
            .set_bit()
            .d2pen()
            .set_bit()
            .d3pen()
            .set_bit()
    });

    // ---- Configure capabilities ----
    // Tell the controller about our hardware capabilities.
    // Base clock = 19 MHz, timeout clock = 19 MHz (unit = MHz).
    s.cfg0().write(|w| unsafe {
        w.bits(
            (19 << 13)  // BASECLKFREQ = 19 MHz
            | (1 << 12) // TOUTCLKUNIT = MHz
            | (19 << 6) // TOUTCLKFREQ = 19
            | (1 << 26) // CSDMASUP
            | (1 << 25) // CHSSUP (high speed)
            | (1 << 28), // C3P3VSUP
        )
    });
    s.cfg1().write(|w| w.slottype().rmsdslot());

    // ---- Software reset ----
    s.clockctrl().write(|w| w.sftrsta().set_bit());
    while s.clockctrl().read().sftrsta().bit_is_set() {}
    delay_ms(1);

    // ---- Enable internal clock ----
    s.clockctrl().write(|w| w.intclken().set_bit());
    while s.clockctrl().read().intclkstable().bit_is_clear() {}

    // ---- Enable status flags ----
    // Enable all normal + error status flags in IFENC so they appear in IFCR.
    s.ifenc().write(|w| unsafe { w.bits(0x10FF_00FF) });

    defmt::info!("SDIO controller initialised");
}

/// Power on the SD card slot.
pub fn power_on() {
    let gpio = gpio();
    let s = sdio();

    // Drive PE7 high to enable power to the SD card slot.
    gpio_set(gpio.pe_dout(), 1 << 7);
    delay_ms(10);

    // Set bus voltage to 3.3V and enable bus power.
    s.hostctrl1()
        .modify(|_, w| w.sdbusvoltsel()._3p3v().sdbuspower().set_bit());
    delay_ms(10);
}

/// Power off the SD card slot.
pub fn power_off() {
    let s = sdio();
    s.hostctrl1().modify(|_, w| w.sdbuspower().clear_bit());
    gpio_clear(gpio().pe_dout(), 1 << 7);
}

/// Set the SD clock frequency.
/// `div` is the divider value: 0 = base clock, 1 = base/2, 2 = base/4, N = base/(2*N).
fn set_clock(div: u16) {
    let s = sdio();

    // Disable SD clock.
    s.clockctrl().modify(|_, w| w.sdclken().clear_bit());

    // Set frequency divider.
    let upper = ((div >> 8) & 0x3) as u8;
    let lower = (div & 0xFF) as u8;
    s.clockctrl()
        .modify(|_, w| unsafe { w.sdclkfreqsel().bits(lower).uppsdclkfre().bits(upper) });

    // Enable internal clock.
    s.clockctrl().modify(|_, w| w.intclken().set_bit());
    while s.clockctrl().read().intclkstable().bit_is_clear() {}

    // Enable SD clock output.
    s.clockctrl().modify(|_, w| w.sdclken().set_bit());
    delay_us(100);
}

/// Set the data timeout counter value.
fn set_timeout(val: u8) {
    let s = sdio();
    s.clockctrl()
        .modify(|_, w| unsafe { w.dattoutcntval().bits(val & 0x0F) });
}

/// Wait for command inhibit to clear.
fn wait_cmd_ready() -> Result<(), SdError> {
    let s = sdio();
    let mut timeout = 100_000u32;
    while s.prsstat().read().cmdinhibitcmd().bit_is_set() {
        timeout -= 1;
        if timeout == 0 {
            return Err(SdError::Timeout);
        }
    }
    Ok(())
}

/// Wait for data inhibit to clear.
fn wait_dat_ready() -> Result<(), SdError> {
    let s = sdio();
    let mut timeout = 100_000u32;
    while s.prsstat().read().cmdinhibitdat().bit_is_set() {
        timeout -= 1;
        if timeout == 0 {
            return Err(SdError::Timeout);
        }
    }
    Ok(())
}

/// Send an SD command and wait for completion.  Returns the response word(s).
fn send_cmd(cmd: u8, arg: u32, resp_type: u8, data: bool) -> Result<u32, SdError> {
    let s = sdio();

    wait_cmd_ready()?;
    if data {
        wait_dat_ready()?;
    }

    // Clear all pending status flags.
    s.ifcr().write(|w| unsafe { w.bits(0xFFFF_FFFF) });

    // Write command argument.
    s.cmdarg1().write(|w| unsafe { w.bits(arg) });

    // Build TFRMODE + command register value.
    let mut val: u32 = 0;
    val |= (cmd as u32) << 24; // CMDINDEX
    val |= (resp_type as u32) << 16; // RESPTYPESEL

    // CRC and index check for most responses.
    if resp_type == RESP_48 || resp_type == RESP_48_BUSY {
        val |= 1 << 19; // CMDCRCCHKEN
        val |= 1 << 20; // CMDINDXCHKEN
    } else if resp_type == RESP_136 {
        val |= 1 << 19; // CMDCRCCHKEN (R2 has CRC but no index check)
    }

    if data {
        val |= 1 << 21; // DATPRESSEL
    }

    // CMD8 and CMD55+ACMD41 don't use index check.
    if cmd == CMD8 {
        // CMD8: R7, check CRC but not index (some cards are picky).
        val &= !(1 << 20);
    }

    s.tfrmode().write(|w| unsafe { w.bits(val) });

    // Wait for command complete.
    let mut timeout = 1_000_000u32;
    loop {
        let status = s.ifcr().read();
        if status.cmdcom().bit_is_set() {
            // Clear command complete flag.
            s.ifcr().write(|w| w.cmdcom().set_bit());
            break;
        }
        if status.cmdtouterr().bit_is_set()
            || status.cmdcrcerr().bit_is_set()
            || status.cmdendbiterr().bit_is_set()
            || status.cmdindexerr().bit_is_set()
        {
            // Clear error flags.
            s.ifcr().write(|w| unsafe { w.bits(0xFFFF_0000) });
            // Reset CMD line.
            s.clockctrl().modify(|_, w| w.sftrstcmd().set_bit());
            while s.clockctrl().read().sftrstcmd().bit_is_set() {}
            return Err(SdError::CmdError);
        }
        timeout -= 1;
        if timeout == 0 {
            return Err(SdError::Timeout);
        }
    }

    Ok(s.resp0().read().bits())
}

/// Send CMD55 (APP_CMD prefix) + an application command.
fn send_acmd(rca: u16, cmd: u8, arg: u32, resp_type: u8) -> Result<u32, SdError> {
    send_cmd(CMD55, (rca as u32) << 16, RESP_48, false)?;
    send_cmd(cmd, arg, resp_type, false)
}

/// Perform the full SD card initialisation sequence.
///
/// Returns [`CardInfo`] on success, describing the card's identity and capacity.
pub fn card_init() -> Result<CardInfo, SdError> {
    if !card_detect() {
        return Err(SdError::NoCard);
    }

    power_on();

    // Start with slow clock (~400 kHz from 19 MHz base: div = 24 -> 19M/48 ~ 396 kHz).
    set_clock(24);
    set_timeout(0x0E); // max timeout

    // 1-bit bus width initially.
    sdio().hostctrl1().modify(|_, w| w.dattranwd().clear_bit());

    delay_ms(10);

    // ---- CMD0: GO_IDLE_STATE ----
    // Send a few times with delays to ensure the card enters idle.
    for _ in 0..3 {
        let _ = send_cmd(CMD0, 0, RESP_NONE, false);
        delay_ms(2);
    }

    // ---- CMD8: SEND_IF_COND ----
    // Check voltage support (0x1AA = 3.3V + check pattern).
    let v2 = match send_cmd(CMD8, 0x0000_01AA, RESP_48, false) {
        Ok(resp) => {
            if resp & 0xFF != 0xAA {
                defmt::warn!("CMD8: unexpected echo {:08x}", resp);
            }
            true // SD v2.0+
        }
        Err(_) => false, // SD v1.x or MMC
    };

    // ---- ACMD41: SD_SEND_OP_COND ----
    // HCS bit (bit 30) = 1 for SDHC support if v2.
    let acmd41_arg = if v2 { 0x40FF_8000 } else { 0x00FF_8000 };
    let mut ocr = 0u32;
    for _ in 0..1000 {
        match send_acmd(0, ACMD41, acmd41_arg, RESP_48) {
            Ok(resp) => {
                ocr = resp;
                if ocr & (1 << 31) != 0 {
                    break; // card power-up complete
                }
            }
            Err(_) => {}
        }
        delay_ms(1);
    }
    if ocr & (1 << 31) == 0 {
        defmt::error!("ACMD41 timeout: card did not become ready");
        power_off();
        return Err(SdError::InitFailed);
    }
    let sdhc = ocr & (1 << 30) != 0;
    defmt::info!("OCR={:08x} SDHC={}", ocr, sdhc);

    // ---- CMD2: ALL_SEND_CID ----
    send_cmd(CMD2, 0, RESP_136, false)?;
    let s = sdio();
    let cid = [
        s.resp0().read().bits(),
        s.resp2().read().bits(),
        s.resp4().read().bits(),
        s.resp6().read().bits(),
    ];
    defmt::info!(
        "CID: {:08x} {:08x} {:08x} {:08x}",
        cid[3],
        cid[2],
        cid[1],
        cid[0]
    );

    // ---- CMD3: SEND_RELATIVE_ADDR ----
    let resp3 = send_cmd(CMD3, 0, RESP_48, false)?;
    let rca = (resp3 >> 16) as u16;
    defmt::info!("RCA={:04x}", rca);

    // ---- CMD9: SEND_CSD ----
    send_cmd(CMD9, (rca as u32) << 16, RESP_136, false)?;
    let csd = [
        s.resp0().read().bits(),
        s.resp2().read().bits(),
        s.resp4().read().bits(),
        s.resp6().read().bits(),
    ];
    defmt::info!(
        "CSD: {:08x} {:08x} {:08x} {:08x}",
        csd[3],
        csd[2],
        csd[1],
        csd[0]
    );

    // Parse capacity from CSD.
    let blocks = parse_capacity(&csd, sdhc);

    // ---- CMD7: SELECT_CARD ----
    send_cmd(CMD7, (rca as u32) << 16, RESP_48_BUSY, false)?;

    // ---- Set block length to 512 for non-SDHC cards ----
    if !sdhc {
        send_cmd(CMD16, SD_BLOCK_SIZE as u32, RESP_48, false)?;
    }

    // ---- ACMD6: SET_BUS_WIDTH (4-bit) ----
    send_acmd(rca, CMD6, 0x0000_0002, RESP_48)?;
    s.hostctrl1().modify(|_, w| w.dattranwd().set_bit());

    // ---- Increase clock speed ----
    // div=0 -> base clock (19 MHz). Safe for most SD cards.
    set_clock(0);

    defmt::info!(
        "SD card init complete: {} blocks ({} MB)",
        blocks,
        blocks / 2048
    );

    Ok(CardInfo {
        cid,
        csd,
        rca,
        sdhc,
        blocks,
    })
}

/// Read a single 512-byte block from the card.
///
/// `block` is the block address (LBA).  `buf` must be exactly 512 bytes.
pub fn read_block(info: &CardInfo, block: u32, buf: &mut [u8; 512]) -> Result<(), SdError> {
    let s = sdio();
    let addr = if info.sdhc { block } else { block * 512 };

    wait_dat_ready()?;

    // Set block size and count.
    s.blksize().write(|w| unsafe {
        w.tfrblksize()
            .bits(SD_BLOCK_SIZE)
            .blkscntforcurrtfr()
            .bits(1)
    });

    // Configure transfer mode: read, single block, no DMA.
    // Then send CMD17.
    let tfrmode: u32 = (1 << 4)            // DATDIRSEL = 1 (read)
        | (1 << 1)          // BLKCNTEN
        | (CMD17 as u32) << 24
        | (RESP_48 as u32) << 16
        | (1 << 19)         // CMDCRCCHKEN
        | (1 << 20)         // CMDINDXCHKEN
        | (1 << 21); // DATPRESSEL

    // Clear pending flags.
    s.ifcr().write(|w| unsafe { w.bits(0xFFFF_FFFF) });

    s.cmdarg1().write(|w| unsafe { w.bits(addr) });
    s.tfrmode().write(|w| unsafe { w.bits(tfrmode) });

    // Wait for command complete.
    wait_for_flag(|st| st.cmdcom().bit_is_set(), |st| st.errint().bit_is_set())?;
    s.ifcr().write(|w| w.cmdcom().set_bit());

    // Wait for buffer read ready.
    wait_for_flag(
        |st| st.bfrrdrdy().bit_is_set(),
        |st| st.errint().bit_is_set(),
    )?;
    s.ifcr().write(|w| w.bfrrdrdy().set_bit());

    // Read 512 bytes from the buffer data port (32 bits at a time).
    for i in 0..128 {
        let word = s.bufdatport().read().bits();
        let off = i * 4;
        buf[off] = word as u8;
        buf[off + 1] = (word >> 8) as u8;
        buf[off + 2] = (word >> 16) as u8;
        buf[off + 3] = (word >> 24) as u8;
    }

    // Wait for transfer complete.
    wait_for_flag(
        |st| st.trancom().bit_is_set(),
        |st| st.errint().bit_is_set(),
    )?;
    s.ifcr().write(|w| w.trancom().set_bit());

    Ok(())
}

/// Write a single 512-byte block to the card.
///
/// `block` is the block address (LBA).  `buf` must be exactly 512 bytes.
pub fn write_block(info: &CardInfo, block: u32, buf: &[u8; 512]) -> Result<(), SdError> {
    let s = sdio();
    let addr = if info.sdhc { block } else { block * 512 };

    wait_dat_ready()?;

    // Set block size and count.
    s.blksize().write(|w| unsafe {
        w.tfrblksize()
            .bits(SD_BLOCK_SIZE)
            .blkscntforcurrtfr()
            .bits(1)
    });

    // Configure transfer mode: write, single block, no DMA.
    let tfrmode: u32 = (0 << 4)            // DATDIRSEL = 0 (write)
        | (1 << 1)          // BLKCNTEN
        | (CMD24 as u32) << 24
        | (RESP_48 as u32) << 16
        | (1 << 19)         // CMDCRCCHKEN
        | (1 << 20)         // CMDINDXCHKEN
        | (1 << 21); // DATPRESSEL

    s.ifcr().write(|w| unsafe { w.bits(0xFFFF_FFFF) });

    s.cmdarg1().write(|w| unsafe { w.bits(addr) });
    s.tfrmode().write(|w| unsafe { w.bits(tfrmode) });

    // Wait for command complete.
    wait_for_flag(|st| st.cmdcom().bit_is_set(), |st| st.errint().bit_is_set())?;
    s.ifcr().write(|w| w.cmdcom().set_bit());

    // Wait for buffer write ready.
    wait_for_flag(
        |st| st.bfrwrrdy().bit_is_set(),
        |st| st.errint().bit_is_set(),
    )?;
    s.ifcr().write(|w| w.bfrwrrdy().set_bit());

    // Write 512 bytes to the buffer data port (32 bits at a time).
    for i in 0..128 {
        let off = i * 4;
        let word = (buf[off] as u32)
            | ((buf[off + 1] as u32) << 8)
            | ((buf[off + 2] as u32) << 16)
            | ((buf[off + 3] as u32) << 24);
        s.bufdatport().write(|w| unsafe { w.bits(word) });
    }

    // Wait for transfer complete.
    wait_for_flag(
        |st| st.trancom().bit_is_set(),
        |st| st.errint().bit_is_set(),
    )?;
    s.ifcr().write(|w| w.trancom().set_bit());

    Ok(())
}

// ---------- internal ----------

/// Poll IFCR until `done` predicate is true, or `err` predicate indicates failure.
fn wait_for_flag(
    done: impl Fn(&pac::sdio::ifcr::R) -> bool,
    err: impl Fn(&pac::sdio::ifcr::R) -> bool,
) -> Result<(), SdError> {
    let s = sdio();
    let mut timeout = 1_000_000u32;
    loop {
        let status = s.ifcr().read();
        if done(&status) {
            return Ok(());
        }
        if err(&status) {
            let bits = status.bits();
            defmt::warn!("SDIO error status: {:08x}", bits);
            // Clear error flags and reset DAT line.
            s.ifcr().write(|w| unsafe { w.bits(0xFFFF_0000) });
            s.clockctrl().modify(|_, w| w.sftrstdat().set_bit());
            while s.clockctrl().read().sftrstdat().bit_is_set() {}
            return Err(SdError::DataError);
        }
        timeout -= 1;
        if timeout == 0 {
            return Err(SdError::Timeout);
        }
    }
}

/// Parse card capacity in 512-byte blocks from the CSD register.
///
/// CSD register is returned by the SDHCI with bits shifted down by 8 (the
/// CRC is stripped), so the 128-bit register occupies resp[3:0] with
/// resp[3] holding the MSBs.
fn parse_capacity(csd: &[u32; 4], sdhc: bool) -> u32 {
    if sdhc {
        // CSD v2.0 (SDHC/SDXC): C_SIZE is bits [69:48] of the 128-bit CSD.
        // After the 8-bit right shift by SDHCI: bits [61:40] in our resp array.
        // resp[1] bits [21:0] contain C_SIZE[21:0] (22-bit field).
        let c_size = ((csd[1] >> 8) & 0x003F_FFFF) as u32;
        // Capacity = (C_SIZE + 1) * 1024 blocks (each block = 512 bytes).
        (c_size + 1) * 1024
    } else {
        // CSD v1.0 (SDSC): C_SIZE [73:62], C_SIZE_MULT [49:47], READ_BL_LEN [83:80].
        // After 8-bit shift: READ_BL_LEN = resp[2] bits [11:8]
        let read_bl_len = ((csd[2] >> 8) & 0x0F) as u32;
        // C_SIZE = resp[2] bits [1:0] << 10 | resp[1] bits [31:22]
        let c_size = (((csd[2] & 0x03) << 10) | ((csd[1] >> 22) & 0x3FF)) as u32;
        // C_SIZE_MULT = resp[1] bits [9:7]
        let c_size_mult = ((csd[1] >> 7) & 0x07) as u32;
        let block_len = 1u32 << read_bl_len;
        let mult = 1u32 << (c_size_mult + 2);
        let capacity_bytes = (c_size + 1) * mult * block_len;
        capacity_bytes / 512
    }
}
