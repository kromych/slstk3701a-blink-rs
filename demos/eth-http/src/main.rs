//! Ethernet DHCP + HTTP demo for the SLSTK3701A.
//!
//! Obtains an IPv4 address via DHCP and serves a simple web page on port 80.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use efm32gg11b_pac as pac;
use smoltcp::iface::{Config, Interface, SocketSet, SocketStorage};
use smoltcp::socket::dhcpv4;
use smoltcp::socket::tcp;
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, HardwareAddress, IpCidr};

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

// ---------- millisecond timer ----------

static MILLIS: AtomicU32 = AtomicU32::new(0);

fn now() -> Instant {
    Instant::from_millis(MILLIS.load(Ordering::Relaxed) as i64)
}

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

fn systick_init(syst: &mut cortex_m::peripheral::SYST) {
    syst.set_clock_source(SystClkSource::Core);
    // HFCLK stays on HFRCO (19 MHz) -- eth::init() does NOT switch to HFXO.
    syst.set_reload(19_000 - 1); // 19 MHz -> 1 ms tick
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}

// ---------- smoltcp PHY device ----------

struct EthDevice;

struct EthRxToken;
struct EthTxToken;

impl smoltcp::phy::Device for EthDevice {
    type RxToken<'a> = EthRxToken;
    type TxToken<'a> = EthTxToken;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        if slstk3701a::eth::rx_available() {
            Some((EthRxToken, EthTxToken))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        if slstk3701a::eth::tx_available() {
            Some(EthTxToken)
        } else {
            None
        }
    }

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        let mut caps = smoltcp::phy::DeviceCapabilities::default();
        caps.max_transmission_unit = 1514;
        caps.medium = smoltcp::phy::Medium::Ethernet;
        caps
    }
}

impl smoltcp::phy::RxToken for EthRxToken {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        if let Some(data) = slstk3701a::eth::rx_packet() {
            let r = f(data);
            slstk3701a::eth::rx_release();
            r
        } else {
            f(&[])
        }
    }
}

impl smoltcp::phy::TxToken for EthTxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buf = [0u8; 1536];
        let result = f(&mut buf[..len]);
        slstk3701a::eth::tx_packet(&buf[..len]);
        result
    }
}

// ---------- HTTP response ----------

const HTTP_HEADER: &[u8] =
    b"HTTP/1.0 200 OK\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n";

const HTTP_BODY_PREFIX: &[u8] =
    b"<html><body><h1>EFM32GG11B SLSTK3701A</h1><p>Hello from bare-metal Rust!</p><p>IP: ";

const HTTP_BODY_SUFFIX: &[u8] = b"</p></body></html>";

// ---------- entry ----------

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // LEDs: PH10 (LED0), PH13 (LED1) push-pull.
    p.gpio
        .ph_modeh()
        .modify(|_, w| w.mode10().pushpull().mode13().pushpull());
    // Both off initially.
    led_clear(&p.gpio, LED0 | LED1);

    // Init Ethernet hardware (enables HFXO for REFCLK but keeps HFCLK on 19 MHz HFRCO).
    slstk3701a::eth::init();

    // Start SysTick -- HFCLK is still 19 MHz HFRCO.
    systick_init(&mut cp.SYST);

    defmt::info!("Waiting for link...");
    while !slstk3701a::eth::link_up() {
        cortex_m::asm::delay(19_000);
    }
    // LED0 on = link up.
    led_set(&p.gpio, LED0);
    defmt::info!("Link up");

    // ---- smoltcp setup ----
    let mac = slstk3701a::eth::mac_address();
    let hw_addr = HardwareAddress::Ethernet(EthernetAddress(mac));

    let mut device = EthDevice;
    let config = Config::new(hw_addr);
    let mut iface = Interface::new(config, &mut device, now());

    // Socket storage.
    let mut sockets_storage: [SocketStorage; 2] = Default::default();
    let mut sockets = SocketSet::new(&mut sockets_storage[..]);

    // DHCP socket.
    let dhcp_handle = sockets.add(dhcpv4::Socket::new());

    // TCP socket for HTTP.
    let mut tcp_rx_buf = [0u8; 1024];
    let mut tcp_tx_buf = [0u8; 2048];
    let tcp_rx = tcp::Socket::new(
        smoltcp::socket::tcp::SocketBuffer::new(&mut tcp_rx_buf[..]),
        smoltcp::socket::tcp::SocketBuffer::new(&mut tcp_tx_buf[..]),
    );
    let tcp_handle = sockets.add(tcp_rx);

    let mut got_ip = false;
    let mut ip_str_buf = [0u8; 16];
    let mut ip_str_len = 0;
    let mut last_heartbeat: u32 = 0;

    loop {
        let timestamp = now();
        iface.poll(timestamp, &mut device, &mut sockets);

        // Periodic heartbeat.
        let ms = MILLIS.load(Ordering::Relaxed);
        if ms.wrapping_sub(last_heartbeat) >= 10000 {
            last_heartbeat = ms;
            defmt::info!("heartbeat t={}ms", ms);
            slstk3701a::eth::dump_status();
        }

        // Handle DHCP.
        let dhcp_socket = sockets.get_mut::<dhcpv4::Socket>(dhcp_handle);
        if let Some(event) = dhcp_socket.poll() {
            match event {
                dhcpv4::Event::Configured(config) => {
                    let addr = config.address;
                    defmt::info!(
                        "DHCP: {}.{}.{}.{}/{}",
                        addr.address().octets()[0],
                        addr.address().octets()[1],
                        addr.address().octets()[2],
                        addr.address().octets()[3],
                        addr.prefix_len()
                    );
                    iface.update_ip_addrs(|addrs| {
                        if addrs.is_empty() {
                            addrs.push(IpCidr::Ipv4(addr)).ok();
                        } else {
                            addrs[0] = IpCidr::Ipv4(addr);
                        }
                    });
                    if let Some(router) = config.router {
                        iface.routes_mut().add_default_ipv4_route(router).ok();
                    }
                    // Format IP into buffer.
                    ip_str_len = fmt_ipv4(addr.address().octets(), &mut ip_str_buf);

                    // LED1 on = got IP.
                    led_set(&p.gpio, LED1);
                    got_ip = true;
                }
                dhcpv4::Event::Deconfigured => {
                    defmt::info!("DHCP: deconfigured");
                    iface.update_ip_addrs(|addrs| addrs.clear());
                    iface.routes_mut().remove_default_ipv4_route();
                    // LED1 off.
                    led_clear(&p.gpio, LED1);
                    got_ip = false;
                }
            }
        }

        // Handle TCP (HTTP server on port 80).
        if got_ip {
            let socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
            if !socket.is_open() {
                socket.listen(80).ok();
            }
            if socket.may_recv() {
                // Read (and discard) the HTTP request.
                let _n = socket.recv(|data| (data.len(), data.len())).unwrap_or(0);
                if socket.may_send() && _n > 0 {
                    // Send response.
                    let _ = socket.send_slice(HTTP_HEADER);
                    let _ = socket.send_slice(HTTP_BODY_PREFIX);
                    let _ = socket.send_slice(&ip_str_buf[..ip_str_len]);
                    let _ = socket.send_slice(HTTP_BODY_SUFFIX);
                    socket.close();
                }
            }
        }
    }
}

/// Format an IPv4 address into a byte buffer. Returns number of bytes written.
fn fmt_ipv4(octets: [u8; 4], buf: &mut [u8; 16]) -> usize {
    let mut pos = 0;
    for (i, &octet) in octets.iter().enumerate() {
        if i > 0 {
            buf[pos] = b'.';
            pos += 1;
        }
        if octet >= 100 {
            buf[pos] = b'0' + octet / 100;
            pos += 1;
        }
        if octet >= 10 {
            buf[pos] = b'0' + (octet / 10) % 10;
            pos += 1;
        }
        buf[pos] = b'0' + octet % 10;
        pos += 1;
    }
    pos
}
