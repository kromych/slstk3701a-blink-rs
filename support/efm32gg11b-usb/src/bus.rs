use efm32gg11b_pac as pac;

// ---------------------------------------------------------------------------
// FIFO helpers (slave / non-DMA mode)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "dma"))]
const USB_BASE: u32 = 0x4002_2000;

/// DWC2 FIFO base address for endpoint `ep`.
///
/// DWC2 core sits at USB_BASE + 0xDE000 = 0x4010_0000.
/// FIFOs are at DWC2_BASE + 0x1000 * (ep + 1).
#[cfg(not(feature = "dma"))]
#[inline]
const fn fifo_addr(ep: u8) -> u32 {
    USB_BASE + 0xDF000 + (ep as u32) * 0x1000
}

#[cfg(not(feature = "dma"))]
#[inline]
fn fifo_read(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

#[cfg(not(feature = "dma"))]
#[inline]
fn fifo_write(addr: u32, value: u32) {
    unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
}

#[cfg(not(feature = "dma"))]
fn write_fifo(addr: u32, data: &[u8], len: usize) {
    let mut i = 0;
    while i < len {
        let mut word = 0u32;
        for b in 0..4 {
            if i + b < len {
                word |= (data[i + b] as u32) << (b * 8);
            }
        }
        fifo_write(addr, word);
        i += 4;
    }
}

/// Read `bcnt` bytes from the shared RX FIFO into `buf`. Returns bytes read.
#[cfg(not(feature = "dma"))]
pub fn read_rx_fifo(buf: &mut [u8], bcnt: usize) -> usize {
    let addr = fifo_addr(0); // RX FIFO read port is always EP0 address
    let len = bcnt.min(buf.len());
    let mut off = 0usize;
    for _ in 0..bcnt.div_ceil(4) {
        let w = fifo_read(addr);
        for &b in w.to_le_bytes().iter() {
            if off < len {
                buf[off] = b;
            }
            off += 1;
        }
    }
    len
}

/// Drain `bcnt` bytes from the RX FIFO without storing.
#[cfg(not(feature = "dma"))]
pub fn drain_rx_fifo(bcnt: usize) {
    let addr = fifo_addr(0);
    for _ in 0..bcnt.div_ceil(4) {
        fifo_read(addr);
    }
}

/// Read a raw 32-bit word from the RX FIFO.
#[cfg(not(feature = "dma"))]
pub fn read_rx_word() -> u32 {
    fifo_read(fifo_addr(0))
}

// ---------------------------------------------------------------------------
// DMA-aligned buffers (DMA mode)
// ---------------------------------------------------------------------------

/// DWORD-aligned buffer for DWC2 DMA transfers.
#[cfg(feature = "dma")]
#[repr(C, align(4))]
struct DmaBuf<const N: usize>([u8; N]);

#[cfg(feature = "dma")]
static mut EP0_IN_BUF: DmaBuf<64> = DmaBuf([0; 64]);
#[cfg(feature = "dma")]
static mut EP1_IN_BUF: DmaBuf<1024> = DmaBuf([0; 1024]);
/// EP1 OUT DMA buffer — sized for a full Ethernet frame so the DWC2
/// can receive an entire CDC-ECM transfer (multi-packet) in one shot.
#[cfg(feature = "dma")]
static mut EP1_OUT_BUF: DmaBuf<EP1_OUT_BUF_SIZE> = DmaBuf([0; EP1_OUT_BUF_SIZE]);
/// EP1 OUT DMA buffer size (max Ethernet frame, rounded up).
#[cfg(feature = "dma")]
pub const EP1_OUT_BUF_SIZE: usize = 1536;
#[cfg(feature = "dma")]
static mut EP2_IN_BUF: DmaBuf<1024> = DmaBuf([0; 1024]);
#[cfg(feature = "dma")]
static mut EP2_OUT_BUF: DmaBuf<1024> = DmaBuf([0; 1024]);
#[cfg(feature = "dma")]
static mut EP3_IN_BUF: DmaBuf<1024> = DmaBuf([0; 1024]);
#[cfg(feature = "dma")]
static mut EP3_OUT_BUF: DmaBuf<1024> = DmaBuf([0; 1024]);

/// Read received EP0 OUT data from the DMA buffer.
///
/// DATA OUT payloads share the same buffer as SETUP packets
/// (`EP0_SETUP_DMA_BUF`). This avoids a race where the DWC2 DMA
/// might write a DATA OUT packet before the firmware has switched
/// the DMA address to a separate buffer.
#[cfg(feature = "dma")]
pub fn ep0_out_data(len: usize) -> &'static [u8] {
    unsafe { &super::EP0_SETUP_DMA_BUF.0[..len.min(64)] }
}

/// Read received EP1 OUT data from the DMA buffer.
#[cfg(feature = "dma")]
pub fn ep1_out_data(len: usize) -> &'static [u8] {
    unsafe { &EP1_OUT_BUF.0[..len.min(EP1_OUT_BUF_SIZE)] }
}

/// Read received EP2 OUT data from the DMA buffer.
#[cfg(feature = "dma")]
pub fn ep2_out_data(len: usize) -> &'static [u8] {
    unsafe { &EP2_OUT_BUF.0[..len.min(1024)] }
}

/// Read received EP3 OUT data from the DMA buffer.
#[cfg(feature = "dma")]
pub fn ep3_out_data(len: usize) -> &'static [u8] {
    unsafe { &EP3_OUT_BUF.0[..len.min(1024)] }
}

// ---------------------------------------------------------------------------
// UsbBus
// ---------------------------------------------------------------------------

/// Thin wrapper around `&pac::usb::RegisterBlock` providing endpoint operations.
///
/// When the `dma` feature is enabled (default), all data transfers use DWC2
/// Buffer DMA mode: the CPU writes data into aligned static buffers and
/// programs the per-endpoint DMA address registers; the DWC2 core moves data
/// between the buffers and the internal FIFOs autonomously.
///
/// Without `dma`, the driver operates in slave (FIFO) mode: the CPU reads and
/// writes the DWC2 FIFOs directly, which uses less static memory at the cost
/// of higher ISR overhead.
pub struct UsbBus {
    usb: &'static pac::usb::RegisterBlock,
}

// SAFETY: UsbBus holds a reference to a fixed MMIO register block.
// Access is mediated by critical sections in the ISR / main-loop pattern.
unsafe impl Send for UsbBus {}
unsafe impl Sync for UsbBus {}

impl Default for UsbBus {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbBus {
    pub fn new() -> Self {
        Self {
            usb: unsafe { &*pac::Usb::ptr() },
        }
    }

    #[inline]
    pub fn regs(&self) -> &pac::usb::RegisterBlock {
        self.usb
    }

    /// Write up to 64 bytes (one MPS) to EP0 IN and start the transfer.
    pub fn ep0_write_packet(&self, data: &[u8]) {
        defmt::assert!(
            data.len() <= 64,
            "EP0 IN: len exceeds 64-byte MPS (DIEP0TSIZ.xfersize is 7 bits)"
        );
        let len = data.len();
        #[cfg(feature = "dma")]
        unsafe {
            EP0_IN_BUF.0[..len].copy_from_slice(&data[..len]);
            cortex_m::asm::dsb();
            self.usb
                .diep0dmaaddr()
                .write(|w| w.bits(core::ptr::addr_of!(EP0_IN_BUF) as u32));
        }
        self.usb
            .diep0tsiz()
            .write(|w| unsafe { w.xfersize().bits(len as u8).pktcnt().bits(1) });
        self.usb
            .diep0ctl()
            .modify(|_, w| w.epena().set_bit().cnak().set_bit());
        #[cfg(not(feature = "dma"))]
        write_fifo(fifo_addr(0), data, len);
    }

    /// Prepare EP0 OUT to receive SETUP packets.
    ///
    /// In DMA mode the endpoint is armed **without** clearing NAK so that
    /// only SETUP packets (which bypass NAK) can be received.  This
    /// prevents a DATA OUT from racing ahead and overwriting the SETUP
    /// data in the shared DMA buffer before the ISR can read it.
    ///
    /// Call [`ep0_clear_out_nak`] afterwards when the firmware is ready
    /// to accept a DATA OUT or status-stage ZLP.
    pub fn ep0_prepare_out(&self) {
        #[cfg(feature = "dma")]
        unsafe {
            self.usb
                .doep0dmaaddr()
                .write(|w| w.bits(core::ptr::addr_of!(super::EP0_SETUP_DMA_BUF) as u32));
        }
        self.usb
            .doep0tsiz()
            .write(|w| unsafe { w.supcnt().bits(3).pktcnt().set_bit().xfersize().bits(64) });
        #[cfg(feature = "dma")]
        self.usb
            .doep0ctl()
            .modify(|_, w| w.epena().set_bit().snak().set_bit());
        #[cfg(not(feature = "dma"))]
        self.usb
            .doep0ctl()
            .modify(|_, w| w.epena().set_bit().cnak().set_bit());
    }

    /// Clear NAK on EP0 OUT so the next DATA OUT or status ZLP can be
    /// received.  Only meaningful in DMA mode; in slave mode CNAK is
    /// already set by [`ep0_prepare_out`].
    #[cfg(feature = "dma")]
    pub fn ep0_clear_out_nak(&self) {
        self.usb.doep0ctl().modify(|_, w| w.cnak().set_bit());
    }

    /// STALL EP0 (both directions).
    pub fn stall_ep0(&self) {
        self.usb.diep0ctl().modify(|_, w| w.stall().set_bit());
        self.usb.doep0ctl().modify(|_, w| w.stall().set_bit());
    }

    /// Write data to a non-EP0 IN endpoint (1 or 2).
    pub fn ep_write(&self, ep: u8, data: &[u8]) {
        match ep {
            1 => {
                let len = data.len();
                #[cfg(feature = "dma")]
                let len = len.min(1024);
                #[cfg(feature = "dma")]
                unsafe {
                    EP1_IN_BUF.0[..len].copy_from_slice(&data[..len]);
                    cortex_m::asm::dsb();
                    self.usb
                        .diep0_dmaaddr()
                        .write(|w| w.bits(core::ptr::addr_of!(EP1_IN_BUF) as u32));
                }
                self.usb
                    .diep0_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                let is_iso = self.usb.diep0_ctl().read().eptype().is_iso();
                self.usb.diep0_ctl().modify(|_, w| {
                    let w = w.usbactep().set_bit().cnak().set_bit().epena().set_bit();
                    if is_iso {
                        let even_now = self.usb.dsts().read().soffn().bits() & 1 == 0;
                        if even_now {
                            w.setd1pidof().set_bit()
                        } else {
                            w.setd0pidef().set_bit()
                        }
                    } else {
                        w
                    }
                });
                #[cfg(not(feature = "dma"))]
                write_fifo(fifo_addr(1), data, len);
            }
            2 => {
                let len = data.len();
                #[cfg(feature = "dma")]
                let len = len.min(1024);
                #[cfg(feature = "dma")]
                unsafe {
                    EP2_IN_BUF.0[..len].copy_from_slice(&data[..len]);
                    cortex_m::asm::dsb();
                    self.usb
                        .diep1_dmaaddr()
                        .write(|w| w.bits(core::ptr::addr_of!(EP2_IN_BUF) as u32));
                }
                self.usb
                    .diep1_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                let is_iso = self.usb.diep1_ctl().read().eptype().is_iso();
                self.usb.diep1_ctl().modify(|_, w| {
                    let w = w.usbactep().set_bit().cnak().set_bit().epena().set_bit();
                    if is_iso {
                        let even_now = self.usb.dsts().read().soffn().bits() & 1 == 0;
                        if even_now {
                            w.setd1pidof().set_bit()
                        } else {
                            w.setd0pidef().set_bit()
                        }
                    } else {
                        w
                    }
                });
                #[cfg(not(feature = "dma"))]
                write_fifo(fifo_addr(2), data, len);
            }
            3 => {
                let len = data.len();
                #[cfg(feature = "dma")]
                let len = len.min(1024);
                #[cfg(feature = "dma")]
                unsafe {
                    EP3_IN_BUF.0[..len].copy_from_slice(&data[..len]);
                    cortex_m::asm::dsb();
                    self.usb
                        .diep2_dmaaddr()
                        .write(|w| w.bits(core::ptr::addr_of!(EP3_IN_BUF) as u32));
                }
                self.usb
                    .diep2_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                self.usb
                    .diep2_ctl()
                    .modify(|_, w| w.usbactep().set_bit().cnak().set_bit().epena().set_bit());
                #[cfg(not(feature = "dma"))]
                write_fifo(fifo_addr(3), data, len);
            }
            _ => {}
        }
    }

    /// Prepare a bulk/interrupt OUT endpoint (1, 2, or 3) to receive data.
    ///
    /// `max_xfer`: when non-zero (DMA mode only), the endpoint is armed
    /// for **multi-packet** reception (`pktcnt = max_xfer / mps`) so
    /// the host can send an entire payload without per-packet NAK.
    /// XFERCOMPL fires once a short packet (or ZLP) terminates the
    /// transfer.  When zero the endpoint receives a single packet.
    #[allow(unused_variables)]
    pub fn ep_prepare_out(&self, ep: u8, mps: u16, max_xfer: u16) {
        match ep {
            1 => {
                #[cfg(feature = "dma")]
                unsafe {
                    self.usb
                        .doep0_dmaaddr()
                        .write(|w| w.bits(core::ptr::addr_of!(EP1_OUT_BUF) as u32));
                }
                #[cfg(feature = "dma")]
                if max_xfer > 0 {
                    let xfer = (max_xfer as usize).min(EP1_OUT_BUF_SIZE) as u32;
                    let pktcnt = ((xfer as usize + mps as usize - 1) / mps as usize) as u16;
                    self.usb
                        .doep0_tsiz()
                        .write(|w| unsafe { w.xfersize().bits(xfer).pktcnt().bits(pktcnt) });
                } else {
                    self.usb
                        .doep0_tsiz()
                        .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                }
                #[cfg(not(feature = "dma"))]
                self.usb
                    .doep0_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep0_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            2 => {
                #[cfg(feature = "dma")]
                unsafe {
                    self.usb
                        .doep1_dmaaddr()
                        .write(|w| w.bits(core::ptr::addr_of!(EP2_OUT_BUF) as u32));
                }
                self.usb
                    .doep1_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep1_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            3 => {
                #[cfg(feature = "dma")]
                unsafe {
                    self.usb
                        .doep2_dmaaddr()
                        .write(|w| w.bits(core::ptr::addr_of!(EP3_OUT_BUF) as u32));
                }
                self.usb
                    .doep2_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep2_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            _ => {}
        }
    }

    /// Flush EP0 IN TX FIFO if a transfer is pending.
    pub fn flush_ep0_tx_if_pending(&self) {
        if self.usb.diep0tsiz().read().pktcnt().bits() != 0 {
            self.usb
                .grstctl()
                .write(|w| w.txfflsh().set_bit().txfnum().f0());
            while self.usb.grstctl().read().txfflsh().bit_is_set() {}
        }
    }

    /// Clear SETUP-phase bits in DOEP0INT (prevents race with data-stage XFERCOMPL).
    pub fn clear_ep0_setup_int(&self) {
        self.usb
            .doep0int()
            .write(|w| w.setup().set_bit().xfercompl().set_bit());
    }
}
