//! DWC2 USB device driver for EFM32GG11B.
//!
//! Provides a generic USB device state machine ([`UsbDevice`]) that delegates
//! class-specific behaviour to a [`UsbClass`] trait object.
//!
//! Ported from the EFM32HG309 driver with adjustments for:
//! - GG11B clock tree (USHFRCO via CMU USBCTRL, HFBUS clock gate)
//! - GG11B-specific init (GOTGCTL, DATTRIM1 ENDLYPULLUP)
//! - 7 device endpoints (EP0-EP6) and 512-word FIFO RAM

#![no_std]

pub mod bus;
pub mod cdc_acm;

pub use bus::UsbBus;
use efm32gg11b_pac as pac;
use pac::interrupt;
use portable_atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct SetupPacket {
    pub bm_request_type: u8,
    pub b_request: u8,
    pub w_value: u16,
    pub w_index: u16,
    pub w_length: u16,
}

pub enum SetupResult {
    Handled,
    DataIn,
    DataOut,
    Unhandled,
}

#[derive(Copy, Clone)]
pub enum EpType {
    Bulk,
    Interrupt,
    Isochronous,
}

#[derive(Copy, Clone)]
pub struct EpConfig {
    pub ep_type: EpType,
    pub mps: u16,
    pub has_in: bool,
    pub has_out: bool,
}

pub struct UsbConfig {
    pub rx_fifo_words: u16,
    pub tx0_fifo_words: u16,
    pub tx1_fifo_words: u16,
    pub tx2_fifo_words: u16,
    pub ep1: Option<EpConfig>,
    pub ep2: Option<EpConfig>,
}

// ---------------------------------------------------------------------------
// UsbClass trait
// ---------------------------------------------------------------------------

pub trait UsbClass {
    fn device_descriptor(&self) -> &[u8];
    fn config_descriptor(&self) -> &[u8];
    fn string_descriptor(&self, index: u8) -> Option<&[u8]>;
    fn handle_setup(&mut self, setup: &SetupPacket, usb: &UsbBus) -> SetupResult;
    fn ep0_data_out(&mut self, _data: &[u8], _usb: &UsbBus) {}
    fn data_out(&mut self, _ep: u8, _data: &[u8], _usb: &UsbBus) {}
    fn in_complete(&mut self, _ep: u8, _usb: &UsbBus) {}
    fn configured(&mut self, _usb: &UsbBus) {}
    fn reset(&mut self) {}
    fn suspended(&mut self) {}
    fn set_interface(&mut self, _iface: u8, _alt: u8, _usb: &UsbBus) {}
    fn get_interface(&self, _iface: u8) -> u8 {
        0
    }
}

// ---------------------------------------------------------------------------
// USB string descriptor helper
// ---------------------------------------------------------------------------

/// Compile-time UTF-16LE string descriptor from an ASCII literal.
#[macro_export]
macro_rules! usb_string {
    ($s:expr) => {{
        const N: usize = $s.len();
        const LEN: usize = 2 + N * 2;
        let mut buf = [0u8; LEN];
        buf[0] = LEN as u8;
        buf[1] = 0x03; // STRING descriptor type
        let bytes = $s.as_bytes();
        let mut i = 0;
        while i < N {
            buf[2 + i * 2] = bytes[i];
            buf[2 + i * 2 + 1] = 0;
            i += 1;
        }
        buf
    }};
}

// ---------------------------------------------------------------------------
// ISR dispatch
// ---------------------------------------------------------------------------

static POLL_FN: AtomicUsize = AtomicUsize::new(0);

/// Register the poll function. Called by [`usb_device!`] generated code.
pub fn register_poll(f: fn()) {
    POLL_FN.store(f as usize, Ordering::Release);
}

/// USB ISR entry point — dispatches to the registered poll function.
#[interrupt]
fn USB() {
    let f = POLL_FN.load(Ordering::Acquire);
    if f != 0 {
        let poll: fn() = unsafe { core::mem::transmute(f) };
        poll();
    }
}

/// Enter an idle WFI loop (for after USB is started).
pub fn idle() -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

// ---------------------------------------------------------------------------
// usb_device! macro
// ---------------------------------------------------------------------------

/// Generate the global USB device state, poll function, and start helper.
///
/// Usage: `efm32gg11b_usb::usb_device!(MyCdcClass);`
#[macro_export]
macro_rules! usb_device {
    ($class_ty:ty) => {
        use core::cell::RefCell;
        use critical_section::Mutex;

        static USB_DEV: Mutex<RefCell<Option<$crate::UsbDevice<$class_ty>>>> =
            Mutex::new(RefCell::new(None));

        fn usb_poll() {
            critical_section::with(|cs| {
                if let Some(dev) = USB_DEV.borrow(cs).borrow_mut().as_mut() {
                    dev.poll();
                }
            });
        }

        fn usb_start(dev: $crate::UsbDevice<$class_ty>) {
            critical_section::with(|cs| {
                USB_DEV.borrow(cs).replace(Some(dev));
            });
            $crate::register_poll(usb_poll);
            unsafe { cortex_m::peripheral::NVIC::unmask(efm32gg11b_pac::Interrupt::USB) };
        }

        #[allow(dead_code)]
        fn usb_with_bus<R>(f: impl FnOnce(&$crate::bus::UsbBus) -> R) -> Option<R> {
            critical_section::with(|cs| {
                USB_DEV.borrow(cs).borrow().as_ref().map(|dev| f(dev.bus()))
            })
        }
    };
}

// ---------------------------------------------------------------------------
// String descriptor 0
// ---------------------------------------------------------------------------

/// String descriptor 0 - language ID (English US).
static STRING0: [u8; 4] = [4, 0x03, 0x09, 0x04];

// ---------------------------------------------------------------------------
// UsbDevice
// ---------------------------------------------------------------------------

pub struct UsbDevice<C: UsbClass> {
    bus: UsbBus,
    pub class: C,
    config: UsbConfig,
    ep0_out_buf: [u8; 64],
    ep0_out_len: usize,
    pending_data_out: bool,
    ep0_in_ptr: *const u8,
    ep0_in_remaining: usize,
}

unsafe impl<C: UsbClass + Send> Send for UsbDevice<C> {}

impl<C: UsbClass> UsbDevice<C> {
    /// Initialize USB clocks, the DWC2 peripheral, and connect to the bus.
    pub fn init(
        cmu: &pac::cmu::RegisterBlock,
        usb: &pac::usb::RegisterBlock,
        class: C,
        config: UsbConfig,
    ) -> Self {
        // ---- Clock setup (USHFRCO 48 MHz + USB clock recovery) ----

        // Enable USB clock on HFBUS.
        cmu.hfbusclken0().modify(|_, w| w.usb().set_bit());
        let _ = cmu.hfbusclken0().read();

        // Enable USHFRCO.
        cmu.oscencmd().write(|w| w.ushfrcoen().set_bit());
        while cmu.status().read().ushfrcordy().bit_is_clear() {}

        // Select USHFRCO as USB clock and enable it.
        cmu.usbctrl()
            .write(|w| w.usbclksel().ushfrco().usbclken().set_bit());

        // Enable USB clock recovery from SOF packets.
        cmu.usbcrctrl().modify(|_, w| w.usbcren().set_bit());

        // ---- GG11B-specific USB init ----

        // Clear VBUS override bits in GOTGCTL.
        usb.gotgctl().modify(|_, w| {
            w.bvalidoven()
                .clear_bit()
                .bvalidovval()
                .clear_bit()
                .vbvalidoven()
                .clear_bit()
                .vbvalidovval()
                .clear_bit()
        });

        // Enable delayed pull-up for signal integrity.
        usb.dattrim1().modify(|_, w| w.endlypullup().set_bit());

        // ---- USB peripheral init ----

        // Disable LEM oscillator control during init.
        usb.ctrl().modify(|_, w| w.lemoscctrl().none());
        // PHY pins enable (GG11B ROUTE has phypen only, no dmpupen).
        usb.route().write(|w| w.phypen().set_bit());
        // Power/clock gating off.
        usb.pcgcctl().write(|w| unsafe { w.bits(0) });

        // Soft-reset.
        while usb.gahbcfg().read().bits() != 0 || usb.grstctl().read().ahbidle().bit_is_clear() {}
        usb.grstctl().modify(|_, w| w.csftrst().set_bit());
        while usb.grstctl().read().csftrst().bit_is_set() {}
        while usb.grstctl().read().ahbidle().bit_is_clear() {}

        usb.gahbcfg().write(|w| w.glblintrmsk().set_bit());
        usb.dctl().modify(|_, w| w.sftdiscon().set_bit());
        usb.dcfg()
            .write(|w| unsafe { w.devspd().fs().devaddr().bits(0) });

        // Endpoint interrupt masks.
        usb.diepmsk().write(|w| w.xfercomplmsk().set_bit());
        usb.doepmsk()
            .write(|w| w.xfercomplmsk().set_bit().setupmsk().set_bit());

        // ---- FIFO allocation ----
        usb.grxfsiz()
            .write(|w| unsafe { w.rxfdep().bits(config.rx_fifo_words) });
        let tx0_start = config.rx_fifo_words;
        usb.gnptxfsiz().write(|w| unsafe {
            w.nptxfstaddr()
                .bits(tx0_start)
                .nptxfineptxf0dep()
                .bits(config.tx0_fifo_words)
        });
        let tx1_start = tx0_start + config.tx0_fifo_words;
        usb.dieptxf1().write(|w| unsafe {
            w.inepntxfstaddr()
                .bits(tx1_start)
                .inepntxfdep()
                .bits(config.tx1_fifo_words)
        });
        let tx2_start = tx1_start + config.tx1_fifo_words;
        usb.dieptxf2().write(|w| unsafe {
            w.inepntxfstaddr()
                .bits(tx2_start)
                .inepntxfdep()
                .bits(config.tx2_fifo_words)
        });

        // ---- Global interrupt mask ----
        usb.gintmsk().write(|w| {
            w.usbrstmsk()
                .set_bit()
                .enumdonemsk()
                .set_bit()
                .usbsuspmsk()
                .set_bit()
                .wkupintmsk()
                .set_bit()
                .iepintmsk()
                .set_bit()
                .oepintmsk()
                .set_bit()
                .rxflvlmsk()
                .set_bit()
        });

        // Clear all pending interrupts.
        usb.gintsts().write(|w| unsafe { w.bits(0xFFFF_FFFF) });

        // Power-on programming done handshake.
        usb.dctl().modify(|_, w| w.pwronprgdone().set_bit());
        cortex_m::asm::delay(800); // ~10 µs at 48 MHz
        usb.dctl().modify(|_, w| w.pwronprgdone().clear_bit());

        // Connect (clear soft-disconnect).
        usb.dctl().modify(|_, w| w.sftdiscon().clear_bit());

        Self {
            bus: UsbBus::new(),
            class,
            config,
            ep0_out_buf: [0u8; 64],
            ep0_out_len: 0,
            pending_data_out: false,
            ep0_in_ptr: core::ptr::null(),
            ep0_in_remaining: 0,
        }
    }

    pub fn bus(&self) -> &UsbBus {
        &self.bus
    }

    /// Poll for and handle USB interrupts.
    pub fn poll(&mut self) {
        let gintsts = self.bus.regs().gintsts().read();

        if gintsts.usbrst().bit_is_set() {
            self.bus.regs().gintsts().write(|w| w.usbrst().set_bit());
            self.handle_usb_reset();
        }

        if gintsts.enumdone().bit_is_set() {
            let usb = self.bus.regs();
            usb.gintsts().write(|w| w.enumdone().set_bit());
            usb.gusbcfg()
                .modify(|_, w| unsafe { w.usbtrdtim().bits(5) });
            usb.diep0ctl().modify(|_, w| w.mps()._64b());
            usb.dctl().modify(|_, w| w.cgnpinnak().set_bit());
            self.bus.ep0_prepare_out();
            defmt::info!("Speed negotiation complete");
        }

        if gintsts.usbsusp().bit_is_set() {
            self.bus.regs().gintsts().write(|w| w.usbsusp().set_bit());
            defmt::info!("USB suspended");
            self.class.suspended();
        }

        if gintsts.wkupint().bit_is_set() {
            self.bus.regs().gintsts().write(|w| w.wkupint().set_bit());
            defmt::info!("USB wakeup");
        }

        if gintsts.rxflvl().bit_is_set() {
            self.handle_rxflvl();
        }

        if gintsts.iepint().bit_is_set() {
            self.handle_iepint();
        }

        if gintsts.oepint().bit_is_set() {
            self.handle_oepint();
        }
    }

    // -----------------------------------------------------------------------
    // Internal handlers
    // -----------------------------------------------------------------------

    fn handle_usb_reset(&mut self) {
        defmt::info!("USB reset");
        let usb = self.bus.regs();

        usb.dcfg().modify(|_, w| unsafe { w.devaddr().bits(0) });

        // Flush all FIFOs.
        usb.grstctl()
            .write(|w| w.txfflsh().set_bit().txfnum().fall().rxfflsh().set_bit());
        while usb.grstctl().read().txfflsh().bit_is_set()
            || usb.grstctl().read().rxfflsh().bit_is_set()
        {}

        // Configure EP0.
        usb.diep0ctl().write(|w| w.mps()._64b().snak().set_bit());
        usb.doep0ctl().write(|w| w.snak().set_bit());

        self.activate_endpoints();

        usb.doep0tsiz().write(|w| unsafe { w.supcnt().bits(3) });

        self.pending_data_out = false;
        self.class.reset();
    }

    fn activate_endpoints(&self) {
        let usb = self.bus.regs();
        let mut daintmsk: u32 = 0x0001_0001; // EP0 IN + OUT

        if let Some(ref ep) = self.config.ep1 {
            if ep.has_in {
                daintmsk |= 1 << 1;
                usb.diep0_ctl().write(|w| unsafe {
                    let w = w
                        .mps()
                        .bits(ep.mps)
                        .usbactep()
                        .set_bit()
                        .txfnum()
                        .bits(1)
                        .snak()
                        .set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                        EpType::Isochronous => w.eptype().iso(),
                    }
                });
            }
            if ep.has_out {
                daintmsk |= 1 << 17;
                usb.doep0_ctl().write(|w| unsafe {
                    let w = w.mps().bits(ep.mps).usbactep().set_bit().snak().set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                        EpType::Isochronous => w.eptype().iso(),
                    }
                });
            }
        }

        if let Some(ref ep) = self.config.ep2 {
            if ep.has_in {
                daintmsk |= 1 << 2;
                usb.diep1_ctl().write(|w| unsafe {
                    let w = w
                        .mps()
                        .bits(ep.mps)
                        .usbactep()
                        .set_bit()
                        .txfnum()
                        .bits(2)
                        .snak()
                        .set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                        EpType::Isochronous => w.eptype().iso(),
                    }
                });
            }
            if ep.has_out {
                daintmsk |= 1 << 18;
                usb.doep1_ctl().write(|w| unsafe {
                    let w = w.mps().bits(ep.mps).usbactep().set_bit().snak().set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                        EpType::Isochronous => w.eptype().iso(),
                    }
                });
            }
        }

        usb.daintmsk().write(|w| unsafe { w.bits(daintmsk) });
        usb.diepmsk().write(|w| w.xfercomplmsk().set_bit());
        usb.doepmsk()
            .write(|w| w.xfercomplmsk().set_bit().setupmsk().set_bit());
    }

    fn handle_rxflvl(&mut self) {
        loop {
            if !self.bus.regs().gintsts().read().rxflvl().bit_is_set() {
                break;
            }
            let grxstsp = self.bus.regs().grxstsp().read().bits();
            let epnum = (grxstsp & 0xF) as u8;
            let bcnt = ((grxstsp >> 4) & 0x7FF) as usize;
            let pktsts = (grxstsp >> 17) & 0xF;

            match (epnum, pktsts) {
                (0, 0x6) => {
                    self.bus.flush_ep0_tx_if_pending();
                    self.bus.clear_ep0_setup_int();
                    self.ep0_in_remaining = 0;

                    let w0 = bus::read_rx_word();
                    let w1 = bus::read_rx_word();

                    let setup = SetupPacket {
                        bm_request_type: (w0 & 0xFF) as u8,
                        b_request: ((w0 >> 8) & 0xFF) as u8,
                        w_value: ((w0 >> 16) & 0xFFFF) as u16,
                        w_index: (w1 & 0xFFFF) as u16,
                        w_length: ((w1 >> 16) & 0xFFFF) as u16,
                    };

                    defmt::debug!(
                        "SETUP: type={:02x} req={:02x} val={:04x} idx={:04x} len={}",
                        setup.bm_request_type,
                        setup.b_request,
                        setup.w_value,
                        setup.w_index,
                        setup.w_length,
                    );

                    self.handle_setup(setup);
                }
                (0, 0x4) => self.bus.ep0_prepare_out(),

                (0, 0x2) => {
                    self.ep0_out_len = bus::read_rx_fifo(&mut self.ep0_out_buf, bcnt);
                }
                (0, 0x3) => {}

                (ep, 0x2) if ep > 0 => {
                    let mut buf = [0u8; 64];
                    let len = bus::read_rx_fifo(&mut buf, bcnt);
                    self.class.data_out(ep, &buf[..len], &self.bus);
                }
                (_, 0x3) => {}

                (_, 0x2) => {
                    bus::drain_rx_fifo(bcnt);
                }
                _ => {}
            }
        }
    }

    fn handle_iepint(&mut self) {
        // EP0 IN.
        let diep0int = self.bus.regs().diep0int().read();
        self.bus
            .regs()
            .diep0int()
            .write(|w| unsafe { w.bits(diep0int.bits()) });
        if diep0int.xfercompl().bit_is_set() {
            if self.ep0_in_remaining > 0 {
                self.ep0_continue_in();
            } else {
                self.bus.ep0_prepare_out();
            }
        }

        // EP1 IN.
        if self.config.ep1.as_ref().is_some_and(|e| e.has_in) {
            let int = self.bus.regs().diep0_int().read();
            self.bus
                .regs()
                .diep0_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                self.class.in_complete(1, &self.bus);
            }
        }

        // EP2 IN.
        if self.config.ep2.as_ref().is_some_and(|e| e.has_in) {
            let int = self.bus.regs().diep1_int().read();
            self.bus
                .regs()
                .diep1_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                self.class.in_complete(2, &self.bus);
            }
        }
    }

    fn handle_oepint(&mut self) {
        // EP0 OUT.
        let doep0int = self.bus.regs().doep0int().read();
        self.bus
            .regs()
            .doep0int()
            .write(|w| unsafe { w.bits(doep0int.bits()) });
        if doep0int.xfercompl().bit_is_set() {
            if self.pending_data_out {
                self.pending_data_out = false;
                let len = self.ep0_out_len;
                let mut buf = [0u8; 64];
                buf[..len].copy_from_slice(&self.ep0_out_buf[..len]);
                self.class.ep0_data_out(&buf[..len], &self.bus);
                self.bus.ep0_write_packet(&[]);
            } else {
                self.bus.ep0_prepare_out();
            }
        }

        // EP1 OUT.
        if self.config.ep1.as_ref().is_some_and(|e| e.has_out) {
            let int = self.bus.regs().doep0_int().read();
            self.bus
                .regs()
                .doep0_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                if let Some(ref ep) = self.config.ep1 {
                    self.bus.ep_prepare_out(1, ep.mps);
                }
            }
        }

        // EP2 OUT.
        if self.config.ep2.as_ref().is_some_and(|e| e.has_out) {
            let int = self.bus.regs().doep1_int().read();
            self.bus
                .regs()
                .doep1_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                if let Some(ref ep) = self.config.ep2 {
                    self.bus.ep_prepare_out(2, ep.mps);
                }
            }
        }
    }

    fn ep0_start_in(&mut self, data: &[u8], max_len: usize) {
        let total = data.len().min(max_len);
        let chunk = total.min(64);
        self.bus.ep0_write_packet(&data[..chunk]);
        if total > chunk {
            self.ep0_in_ptr = unsafe { data.as_ptr().add(chunk) };
            self.ep0_in_remaining = total - chunk;
        } else {
            self.ep0_in_ptr = core::ptr::null();
            self.ep0_in_remaining = 0;
        }
    }

    fn ep0_continue_in(&mut self) {
        if self.ep0_in_remaining == 0 {
            return;
        }
        let chunk = self.ep0_in_remaining.min(64);
        let data = unsafe { core::slice::from_raw_parts(self.ep0_in_ptr, chunk) };
        self.bus.ep0_write_packet(data);
        if self.ep0_in_remaining > chunk {
            self.ep0_in_ptr = unsafe { self.ep0_in_ptr.add(chunk) };
            self.ep0_in_remaining -= chunk;
        } else {
            self.ep0_in_ptr = core::ptr::null();
            self.ep0_in_remaining = 0;
        }
    }

    fn handle_setup(&mut self, setup: SetupPacket) {
        const GET_STATUS: u8 = 0x00;
        const SET_ADDRESS: u8 = 0x05;
        const GET_DESCRIPTOR: u8 = 0x06;
        const SET_CONFIGURATION: u8 = 0x09;
        const GET_INTERFACE: u8 = 0x0A;
        const SET_INTERFACE: u8 = 0x0B;

        const DESC_DEVICE: u8 = 0x01;
        const DESC_CONFIGURATION: u8 = 0x02;
        const DESC_STRING: u8 = 0x03;

        match (setup.bm_request_type, setup.b_request) {
            // GET_STATUS (device).
            (0x80, GET_STATUS) => {
                self.ep0_start_in(&[0x00, 0x00], setup.w_length as usize);
            }

            // GET_DESCRIPTOR.
            (0x80, GET_DESCRIPTOR) => {
                let desc_type = (setup.w_value >> 8) as u8;
                let desc_index = (setup.w_value & 0xFF) as u8;
                match desc_type {
                    DESC_DEVICE => {
                        defmt::info!("GET_DESCRIPTOR Device");
                        let desc = self.class.device_descriptor();
                        let ptr = desc.as_ptr();
                        let len = desc.len();
                        self.ep0_start_in(
                            unsafe { core::slice::from_raw_parts(ptr, len) },
                            setup.w_length as usize,
                        );
                    }
                    DESC_CONFIGURATION => {
                        defmt::info!("GET_DESCRIPTOR Configuration");
                        let desc = self.class.config_descriptor();
                        let ptr = desc.as_ptr();
                        let len = desc.len();
                        self.ep0_start_in(
                            unsafe { core::slice::from_raw_parts(ptr, len) },
                            setup.w_length as usize,
                        );
                    }
                    DESC_STRING => {
                        if desc_index == 0 {
                            self.ep0_start_in(&STRING0, setup.w_length as usize);
                        } else if let Some(desc) = self.class.string_descriptor(desc_index) {
                            let ptr = desc.as_ptr();
                            let len = desc.len();
                            self.ep0_start_in(
                                unsafe { core::slice::from_raw_parts(ptr, len) },
                                setup.w_length as usize,
                            );
                        } else {
                            self.bus.stall_ep0();
                        }
                    }
                    _ => {
                        defmt::warn!("Unsupported descriptor type {}", desc_type);
                        self.bus.stall_ep0();
                    }
                }
            }

            // SET_ADDRESS.
            (0x00, SET_ADDRESS) => {
                let addr = (setup.w_value & 0x7F) as u8;
                defmt::info!("SET_ADDRESS {}", addr);
                self.bus
                    .regs()
                    .dcfg()
                    .modify(|_, w| unsafe { w.devaddr().bits(addr) });
                self.bus.ep0_write_packet(&[]);
            }

            // SET_CONFIGURATION.
            (0x00, SET_CONFIGURATION) => {
                defmt::info!("SET_CONFIGURATION {}", setup.w_value);
                if let Some(ref ep) = self.config.ep1 {
                    if ep.has_out {
                        self.bus.ep_prepare_out(1, ep.mps);
                    }
                }
                if let Some(ref ep) = self.config.ep2 {
                    if ep.has_out {
                        self.bus.ep_prepare_out(2, ep.mps);
                    }
                }
                // Enable Low Energy Mode.
                self.bus.regs().ctrl().modify(|_, w| {
                    w.lemoscctrl()
                        .gate()
                        .lemidleen()
                        .set_bit()
                        .lemphyctrl()
                        .set_bit()
                });
                self.bus.ep0_write_packet(&[]);
                self.class.configured(&self.bus);
            }

            // GET_STATUS (interface / endpoint).
            (0x81, GET_STATUS) | (0x82, GET_STATUS) => {
                self.ep0_start_in(&[0x00, 0x00], setup.w_length as usize);
            }

            // SET_INTERFACE.
            (0x01, SET_INTERFACE) => {
                let iface = setup.w_index as u8;
                let alt = setup.w_value as u8;
                defmt::info!("SET_INTERFACE iface={} alt={}", iface, alt);
                self.class.set_interface(iface, alt, &self.bus);
                self.bus.ep0_write_packet(&[]);
            }

            // GET_INTERFACE.
            (0x81, GET_INTERFACE) => {
                let iface = setup.w_index as u8;
                let alt = self.class.get_interface(iface);
                self.ep0_start_in(&[alt], setup.w_length as usize);
            }

            // Delegate to class.
            _ => match self.class.handle_setup(&setup, &self.bus) {
                SetupResult::Handled => {
                    self.bus.ep0_write_packet(&[]);
                }
                SetupResult::DataIn => {}
                SetupResult::DataOut => {
                    self.pending_data_out = true;
                }
                SetupResult::Unhandled => {
                    defmt::warn!(
                        "Unhandled SETUP: type={:02x} req={:02x} val={:04x} idx={:04x}",
                        setup.bm_request_type,
                        setup.b_request,
                        setup.w_value,
                        setup.w_index,
                    );
                    self.bus.stall_ep0();
                }
            },
        }
    }
}
