//! Disco -- a USB NIC with a serial management console.
//!
//!
//! Composite USB device:
//!   - CDC-ECM: USB Ethernet adapter (same bridge as usb-nic)
//!   - CDC-ACM: Serial management console with login + IP blocklist
//!
//! The serial console presents a CLI::
//!   - Login with username/password (default: root / gg11b)
//!   - `deny <ip>`   -- block traffic to an IPv4 address
//!   - `permit <ip>` -- unblock an IPv4 address
//!   - `show acl`    -- display the current access control list
//!   - `show status` -- display bridge stats
//!   - `passwd`      -- change the login password
//!   - `logout`      -- return to login prompt
//!   - `help` / `?`  -- show available commands
//!
//! Credentials are stored in RAM (flash storage is left as an exercise).
//! Blocked IP addresses are checked on both TX and RX paths.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use efm32gg11b_pac as pac;
use efm32gg11b_usb::cdc_ecm_acm::{self, CdcEcmAcmClass};
use efm32gg11b_usb::UsbDevice;
use pac::interrupt;
use slstk3701a::display;

// ---------- millisecond timer ----------

static MILLIS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

fn systick_init(syst: &mut cortex_m::peripheral::SYST) {
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(19_000 - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

// ---------- LED helpers ----------

const LED0: u32 = 1 << 10;
const LED1: u32 = 1 << 13;

fn led_set(gpio: &pac::Gpio, mask: u32) {
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
}

fn led_clear(gpio: &pac::Gpio, mask: u32) {
    gpio.ph_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
}

// ---------- LCD helpers ----------

const BLACK: u8 = 0b000;
const BLUE: u8 = 0b001;
const GREEN: u8 = 0b010;
const RED: u8 = 0b100;
const WHITE: u8 = 0b111;

fn fmt_u32(val: u32, buf: &mut [u8]) -> usize {
    if val == 0 {
        let last = buf.len() - 1;
        buf[last] = b'0';
        return 1;
    }
    let mut v = val;
    let mut i = buf.len();
    while v > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    buf.len() - i
}

struct LcdState {
    vcom: bool,
    last_vcom_ms: u32,
    last_draw_ms: u32,
}

impl LcdState {
    fn new() -> Self {
        Self {
            vcom: false,
            last_vcom_ms: 0,
            last_draw_ms: 0,
        }
    }

    fn tick_vcom(&mut self, now_ms: u32) {
        if now_ms.wrapping_sub(self.last_vcom_ms) >= 16 {
            self.last_vcom_ms = now_ms;
            display::toggle_vcom();
        }
    }

    fn draw_stats(
        &mut self,
        now_ms: u32,
        link: bool,
        usb_active: bool,
        h2d: u32,
        d2h: u32,
        blocked: u32,
        acl_count: usize,
    ) {
        if now_ms.wrapping_sub(self.last_draw_ms) < 500 {
            return;
        }
        self.last_draw_ms = now_ms;

        display::draw_text_colored(0, 0, " Disco Management  ", WHITE, BLUE, &mut self.vcom);

        if link {
            display::draw_text_colored(2, 0, "ETH: LINK UP  100M  ", GREEN, WHITE, &mut self.vcom);
        } else {
            display::draw_text_colored(2, 0, "ETH: LINK DOWN       ", RED, WHITE, &mut self.vcom);
        }

        if usb_active {
            display::draw_text_colored(3, 0, "USB: ECM+ACM ACTIVE ", GREEN, WHITE, &mut self.vcom);
        } else if cdc_ecm_acm::is_configured() {
            display::draw_text_colored(3, 0, "USB: CONFIGURED     ", BLUE, WHITE, &mut self.vcom);
        } else {
            display::draw_text_colored(3, 0, "USB: WAITING...     ", RED, WHITE, &mut self.vcom);
        }

        let mut nbuf = [b' '; 10];
        let mut line = [b' '; 22];
        line[0] = b'T';
        line[1] = b'X';
        line[2] = b':';
        let n = fmt_u32(h2d, &mut nbuf);
        line[3..3 + n].copy_from_slice(&nbuf[10 - n..]);
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(5, 0, s, BLACK, WHITE, &mut self.vcom);

        let mut line = [b' '; 22];
        line[0] = b'R';
        line[1] = b'X';
        line[2] = b':';
        let n = fmt_u32(d2h, &mut nbuf);
        line[3..3 + n].copy_from_slice(&nbuf[10 - n..]);
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(6, 0, s, BLACK, WHITE, &mut self.vcom);

        let mut line = [b' '; 22];
        line[0] = b'D';
        line[1] = b'r';
        line[2] = b'o';
        line[3] = b'p';
        line[4] = b':';
        let n = fmt_u32(blocked, &mut nbuf);
        line[5..5 + n].copy_from_slice(&nbuf[10 - n..]);
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(7, 0, s, RED, WHITE, &mut self.vcom);

        let mut line = [b' '; 22];
        line[0] = b'A';
        line[1] = b'C';
        line[2] = b'L';
        line[3] = b':';
        let n = fmt_u32(acl_count as u32, &mut nbuf);
        line[4..4 + n].copy_from_slice(&nbuf[10 - n..]);
        line[4 + n..4 + n + 6].copy_from_slice(b" rules");
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text(8, 0, s, &mut self.vcom);

        // Uptime
        let secs = now_ms / 1000;
        let mins = secs / 60;
        let s_rem = secs % 60;
        let mut line = [b' '; 22];
        line[0] = b'U';
        line[1] = b'p';
        line[2] = b':';
        line[3] = b' ';
        line[4] = b'0' + (mins / 10 % 10) as u8;
        line[5] = b'0' + (mins % 10) as u8;
        line[6] = b':';
        line[7] = b'0' + (s_rem / 10) as u8;
        line[8] = b'0' + (s_rem % 10) as u8;
        let s = core::str::from_utf8(&line).unwrap_or("");
        display::draw_text_colored(10, 0, s, BLACK, WHITE, &mut self.vcom);
    }
}

// ---------- IP blocklist + credentials ----------

const MAX_ACL_ENTRIES: usize = 16;

struct Firewall {
    /// Blocked IPv4 addresses (network byte order).
    acl: [[u8; 4]; MAX_ACL_ENTRIES],
    acl_len: usize,
    /// Login credentials.
    username: [u8; 16],
    username_len: usize,
    password: [u8; 16],
    password_len: usize,
    /// Stats.
    blocked_count: u32,
}

impl Firewall {
    fn new() -> Self {
        let mut fw = Self {
            acl: [[0; 4]; MAX_ACL_ENTRIES],
            acl_len: 0,
            username: [0; 16],
            username_len: 4,
            password: [0; 16],
            password_len: 4,
            blocked_count: 0,
        };
        fw.username[..4].copy_from_slice(b"root");
        fw.password[..4].copy_from_slice(b"gg11b"[..4].as_ref());
        fw.password[4] = b'b';
        // Actually let's just do it cleanly:
        fw.password_len = 5;
        fw.password[..5].copy_from_slice(b"gg11b");
        fw
    }

    fn is_blocked(&self, ip: [u8; 4]) -> bool {
        for i in 0..self.acl_len {
            if self.acl[i] == ip {
                return true;
            }
        }
        false
    }

    fn deny(&mut self, ip: [u8; 4]) -> bool {
        if self.is_blocked(ip) {
            return false;
        }
        if self.acl_len >= MAX_ACL_ENTRIES {
            return false;
        }
        self.acl[self.acl_len] = ip;
        self.acl_len += 1;
        true
    }

    fn permit(&mut self, ip: [u8; 4]) -> bool {
        for i in 0..self.acl_len {
            if self.acl[i] == ip {
                self.acl[i] = self.acl[self.acl_len - 1];
                self.acl_len -= 1;
                return true;
            }
        }
        false
    }

    fn check_and_count(&mut self, frame: &[u8]) -> bool {
        if let Some(ip) = extract_dst_ipv4(frame) {
            if self.is_blocked(ip) {
                self.blocked_count += 1;
                return true;
            }
        }
        if let Some(ip) = extract_src_ipv4(frame) {
            if self.is_blocked(ip) {
                self.blocked_count += 1;
                return true;
            }
        }
        false
    }

    fn check_username(&self, input: &[u8]) -> bool {
        input == &self.username[..self.username_len]
    }

    fn check_password(&self, input: &[u8]) -> bool {
        input == &self.password[..self.password_len]
    }

    fn set_password(&mut self, new_pw: &[u8]) {
        let len = new_pw.len().min(16);
        self.password[..len].copy_from_slice(&new_pw[..len]);
        self.password_len = len;
    }
}

/// Extract destination IPv4 from an Ethernet frame (EtherType 0x0800).
fn extract_dst_ipv4(frame: &[u8]) -> Option<[u8; 4]> {
    if frame.len() < 34 {
        return None;
    }
    // EtherType at offset 12-13.
    if frame[12] != 0x08 || frame[13] != 0x00 {
        return None;
    }
    // Destination IP at offset 30-33 in IPv4 header (14 + 16).
    Some([frame[30], frame[31], frame[32], frame[33]])
}

/// Extract source IPv4 from an Ethernet frame.
fn extract_src_ipv4(frame: &[u8]) -> Option<[u8; 4]> {
    if frame.len() < 34 {
        return None;
    }
    if frame[12] != 0x08 || frame[13] != 0x00 {
        return None;
    }
    // Source IP at offset 26-29 (14 + 12).
    Some([frame[26], frame[27], frame[28], frame[29]])
}

// ---------- CLI state machine ----------

const CMD_BUF_SIZE: usize = 64;

#[derive(PartialEq)]
enum CliState {
    Username,
    Password,
    Authenticated,
}

struct Cli {
    state: CliState,
    buf: [u8; CMD_BUF_SIZE],
    pos: usize,
    username_buf: [u8; 16],
    username_len: usize,
}

impl Cli {
    fn new() -> Self {
        Self {
            state: CliState::Username,
            buf: [0; CMD_BUF_SIZE],
            pos: 0,
            username_buf: [0; 16],
            username_len: 0,
        }
    }

    fn reset(&mut self) {
        self.state = CliState::Username;
        self.pos = 0;
        self.username_len = 0;
    }

    /// Process incoming serial bytes. Returns responses to send back.
    /// `out` is a closure that sends text to the serial port.
    fn process(&mut self, data: &[u8], fw: &mut Firewall, out: &mut impl FnMut(&[u8])) {
        for &b in data {
            match b {
                // Backspace / DEL
                0x08 | 0x7F => {
                    if self.pos > 0 {
                        self.pos -= 1;
                        out(b"\x08 \x08");
                    }
                }
                // Enter
                b'\r' | b'\n' => {
                    out(b"\r\n");
                    let mut line_copy = [0u8; CMD_BUF_SIZE];
                    let line_len = self.pos;
                    line_copy[..line_len].copy_from_slice(&self.buf[..line_len]);
                    self.handle_line(&line_copy[..line_len], fw, out);
                    self.pos = 0;
                }
                // Printable
                0x20..=0x7E => {
                    if self.pos < CMD_BUF_SIZE {
                        self.buf[self.pos] = b;
                        self.pos += 1;
                        if self.state == CliState::Password {
                            out(b"*");
                        } else {
                            out(&[b]);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_line(&mut self, line: &[u8], fw: &mut Firewall, out: &mut impl FnMut(&[u8])) {
        match self.state {
            CliState::Username => {
                self.username_buf[..line.len().min(16)]
                    .copy_from_slice(&line[..line.len().min(16)]);
                self.username_len = line.len().min(16);
                self.state = CliState::Password;
                out(b"Password: ");
            }
            CliState::Password => {
                if fw.check_username(&self.username_buf[..self.username_len])
                    && fw.check_password(line)
                {
                    self.state = CliState::Authenticated;
                    out(b"\r\n");
                    out(b"Welcome to Disco OS v1.0\r\n");
                    out(b"\r\n");
                    out(b"disco# ");
                } else {
                    out(b"% Login failed.\r\n\r\n");
                    self.state = CliState::Username;
                    out(b"Username: ");
                }
            }
            CliState::Authenticated => {
                self.dispatch_command(line, fw, out);
            }
        }
    }

    fn dispatch_command(&mut self, line: &[u8], fw: &mut Firewall, out: &mut impl FnMut(&[u8])) {
        let line = trim(line);
        if line.is_empty() {
            out(b"disco# ");
            return;
        }

        if starts_with(line, b"deny ") {
            let arg = trim(&line[5..]);
            if let Some(ip) = parse_ipv4(arg) {
                if fw.deny(ip) {
                    out(b"% Access list updated: deny ");
                    fmt_ipv4(ip, out);
                    out(b"\r\n");
                } else {
                    out(b"% Already denied or ACL full.\r\n");
                }
            } else {
                out(b"% Invalid IP address.\r\n");
            }
        } else if starts_with(line, b"permit ") {
            let arg = trim(&line[7..]);
            if let Some(ip) = parse_ipv4(arg) {
                if fw.permit(ip) {
                    out(b"% Removed deny rule for ");
                    fmt_ipv4(ip, out);
                    out(b"\r\n");
                } else {
                    out(b"% No deny rule for that address.\r\n");
                }
            } else {
                out(b"% Invalid IP address.\r\n");
            }
        } else if line == b"show acl" {
            if fw.acl_len == 0 {
                out(b"% Access list is empty.\r\n");
            } else {
                for i in 0..fw.acl_len {
                    out(b"  deny ");
                    fmt_ipv4(fw.acl[i], out);
                    out(b"\r\n");
                }
            }
        } else if line == b"show status" {
            out(b"Disco NIC Status\r\n");
            out(b"  Link: ");
            if slstk3701a::eth::link_up() {
                out(b"UP\r\n");
            } else {
                out(b"DOWN\r\n");
            }
            out(b"  Blocked packets: ");
            let mut nbuf = [b' '; 10];
            let n = fmt_u32(fw.blocked_count, &mut nbuf);
            out(&nbuf[10 - n..]);
            out(b"\r\n");
            out(b"  ACL entries: ");
            let n = fmt_u32(fw.acl_len as u32, &mut nbuf);
            out(&nbuf[10 - n..]);
            out(b"\r\n");
        } else if starts_with(line, b"passwd") {
            out(b"New password: ");
            // We'll handle this in a simplified way: passwd <newpw>
            let arg = trim(&line[6..]);
            if arg.is_empty() {
                out(b"\r\n% Usage: passwd <new-password>\r\n");
            } else {
                fw.set_password(arg);
                out(b"\r\n% Password changed.\r\n");
            }
        } else if line == b"logout" || line == b"exit" || line == b"quit" {
            out(b"% Goodbye.\r\n\r\n");
            self.reset();
            out(b"Username: ");
            return;
        } else if line == b"help" || line == b"?" {
            out(b"Available commands:\r\n");
            out(b"  deny <ip>    - Block traffic to/from IPv4 address\r\n");
            out(b"  permit <ip>  - Remove block for IPv4 address\r\n");
            out(b"  show acl     - Display access control list\r\n");
            out(b"  show status  - Display bridge statistics\r\n");
            out(b"  passwd <pw>  - Change login password\r\n");
            out(b"  logout       - End session\r\n");
            out(b"  help         - Show this message\r\n");
        } else {
            out(b"% Unknown command: \"");
            out(line);
            out(b"\"\r\n");
            out(b"% Type 'help' for available commands.\r\n");
        }

        out(b"disco# ");
    }
}

// ---------- CLI parsing helpers ----------

fn trim(s: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = s.len();
    while start < end && s[start] == b' ' {
        start += 1;
    }
    while end > start && s[end - 1] == b' ' {
        end -= 1;
    }
    &s[start..end]
}

fn starts_with(s: &[u8], prefix: &[u8]) -> bool {
    s.len() >= prefix.len() && &s[..prefix.len()] == prefix
}

fn parse_ipv4(s: &[u8]) -> Option<[u8; 4]> {
    let mut octets = [0u8; 4];
    let mut octet_idx = 0;
    let mut val: u16 = 0;
    let mut digits = 0;
    for &b in s {
        if b == b'.' {
            if digits == 0 || val > 255 || octet_idx >= 3 {
                return None;
            }
            octets[octet_idx] = val as u8;
            octet_idx += 1;
            val = 0;
            digits = 0;
        } else if b.is_ascii_digit() {
            val = val * 10 + (b - b'0') as u16;
            digits += 1;
            if digits > 3 {
                return None;
            }
        } else {
            return None;
        }
    }
    if digits == 0 || val > 255 || octet_idx != 3 {
        return None;
    }
    octets[3] = val as u8;
    Some(octets)
}

fn fmt_ipv4(ip: [u8; 4], out: &mut impl FnMut(&[u8])) {
    for (i, &octet) in ip.iter().enumerate() {
        if i > 0 {
            out(b".");
        }
        let mut nbuf = [b' '; 3];
        let n = fmt_u32(octet as u32, &mut nbuf);
        out(&nbuf[3 - n..]);
    }
}

// ---------- ETH interrupt handler ----------

#[interrupt]
fn ETH() {
    slstk3701a::eth::irq_handler();
}

// ---------- USB device setup ----------

efm32gg11b_usb::usb_device!(CdcEcmAcmClass);

// ---------- entry ----------

// TODO: Add saving to the SD card
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // LEDs
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());
    led_clear(&p.gpio, LED0 | LED1);

    // VBUSEN: PF5 push-pull, low (device mode).
    p.gpio.pf_model().modify(|_, w| w.mode5().pushpull());
    p.gpio
        .pf_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });

    // ---- Display ----
    display::init();
    let mut lcd = LcdState::new();
    display::clear(&mut lcd.vcom);
    display::draw_text_colored(0, 0, " Disco Management  ", WHITE, BLUE, &mut lcd.vcom);
    display::draw_text(2, 0, "Initializing...", &mut lcd.vcom);

    // ---- Ethernet ----
    slstk3701a::eth::init();
    systick_init(&mut cp.SYST);

    display::draw_text(2, 0, "ETH: waiting link...", &mut lcd.vcom);
    defmt::info!("Disco -- waiting for Ethernet link...");
    while !slstk3701a::eth::link_up() {
        cortex_m::asm::delay(19_000);
    }
    led_set(&p.gpio, LED0);
    defmt::info!("Ethernet link up");

    slstk3701a::eth::enable_interrupts();
    unsafe { pac::NVIC::unmask(pac::Interrupt::ETH) };

    // ---- USB CDC-ECM + CDC-ACM ----
    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        CdcEcmAcmClass::new(),
        cdc_ecm_acm::usb_config(),
    );
    defmt::info!("Disco USB composite device initialized");
    usb_start(dev);

    // Wait for configuration.
    while !cdc_ecm_acm::is_configured() {
        let ms = MILLIS.load(Ordering::Relaxed);
        lcd.tick_vcom(ms);
        cortex_m::asm::wfi();
    }
    defmt::info!("USB configured");

    // Wait for ECM data interface.
    while !cdc_ecm_acm::is_data_active() {
        let ms = MILLIS.load(Ordering::Relaxed);
        lcd.tick_vcom(ms);
        cortex_m::asm::wfi();
    }
    led_set(&p.gpio, LED1);
    defmt::info!("ECM data active -- bridging started");

    usb_with_bus(|usb| {
        cdc_ecm_acm::send_link_up_notifications(usb);
    });

    // Send initial banner on ACM.
    let banner = b"\r\n\
        =============================================\r\n\
        |          Disco OS v1.0                    |\r\n\
        =============================================\r\n\
        \r\n\
        Username: ";
    cdc_ecm_acm::acm_write(banner);
    usb_with_bus(|usb| cdc_ecm_acm::acm_flush(usb));

    // ---- Main loop ----

    let mut rx_buf = [0u8; 1536];
    let mut acm_in = [0u8; 64];
    let mut last_heartbeat: u32 = 0;
    let mut stats_h2d: u32 = 0;
    let mut stats_d2h: u32 = 0;
    let mut link_was_up = true;
    let mut firewall = Firewall::new();
    let mut cli = Cli::new();

    loop {
        cortex_m::asm::wfi();

        let ms = MILLIS.load(Ordering::Relaxed);

        let _eth_rx = slstk3701a::eth::take_rx_event();
        let _eth_tx = slstk3701a::eth::take_tx_event();

        // Host -> Wire (with ACL check).
        if let Some(len) = cdc_ecm_acm::take_rx_frame(&mut rx_buf) {
            if !firewall.check_and_count(&rx_buf[..len]) {
                if slstk3701a::eth::tx_available() {
                    slstk3701a::eth::tx_packet(&rx_buf[..len]);
                    stats_h2d += 1;
                }
            }
        }

        // Wire -> Host (with ACL check).
        while slstk3701a::eth::rx_available() {
            if let Some(data) = slstk3701a::eth::rx_packet() {
                if !firewall.check_and_count(data) {
                    let sent = usb_with_bus(|usb| cdc_ecm_acm::submit_tx_frame(data, usb));
                    if sent == Some(true) {
                        stats_d2h += 1;
                    }
                }
                slstk3701a::eth::rx_release();
            } else {
                break;
            }
        }

        // ---- ACM serial console ----
        let n = cdc_ecm_acm::acm_read(&mut acm_in);
        if n > 0 {
            let mut tx_pending = false;
            cli.process(&acm_in[..n], &mut firewall, &mut |data: &[u8]| {
                cdc_ecm_acm::acm_write(data);
                tx_pending = true;
            });
            if tx_pending {
                usb_with_bus(|usb| cdc_ecm_acm::acm_flush(usb));
            }
        }

        // ---- Link monitoring ----
        let link = slstk3701a::eth::link_up();
        if link != link_was_up {
            link_was_up = link;
            if link {
                led_set(&p.gpio, LED0);
            } else {
                led_clear(&p.gpio, LED0);
            }
        }

        // ---- Heartbeat ----
        if ms.wrapping_sub(last_heartbeat) >= 10_000 {
            last_heartbeat = ms;
            defmt::info!(
                "heartbeat t={}ms h2d={} d2h={} blocked={}",
                ms,
                stats_h2d,
                stats_d2h,
                firewall.blocked_count
            );
        }

        // ---- LCD ----
        lcd.tick_vcom(ms);
        lcd.draw_stats(
            ms,
            link,
            cdc_ecm_acm::is_data_active(),
            stats_h2d,
            stats_d2h,
            firewall.blocked_count,
            firewall.acl_len,
        );
    }
}
