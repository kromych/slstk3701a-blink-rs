#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    route: ROUTE,
    _reserved7: [u8; 0x10],
    cdconf: CDCONF,
    cmd: CMD,
    dattrim1: DATTRIM1,
    _reserved10: [u8; 0x04],
    lemctrl: LEMCTRL,
    _reserved11: [u8; 0x000d_dfc0],
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    grxstsr: GRXSTSR,
    grxstsp: GRXSTSP,
    grxfsiz: GRXFSIZ,
    gnptxfsiz: GNPTXFSIZ,
    gnptxsts: GNPTXSTS,
    _reserved23: [u8; 0x10],
    gsnpsid: GSNPSID,
    _reserved24: [u8; 0x18],
    gdfifocfg: GDFIFOCFG,
    _reserved25: [u8; 0xa0],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    dieptxf5: DIEPTXF5,
    dieptxf6: DIEPTXF6,
    _reserved32: [u8; 0x02e4],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved35: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved38: [u8; 0x24],
    hprt: HPRT,
    _reserved39: [u8; 0xbc],
    hc0_char: HC0_CHAR,
    hc0_splt: HC0_SPLT,
    hc0_int: HC0_INT,
    hc0_intmsk: HC0_INTMSK,
    hc0_tsiz: HC0_TSIZ,
    hc0_dmaaddr: HC0_DMAADDR,
    _reserved45: [u8; 0x08],
    hc1_char: HC1_CHAR,
    hc1_splt: HC1_SPLT,
    hc1_int: HC1_INT,
    hc1_intmsk: HC1_INTMSK,
    hc1_tsiz: HC1_TSIZ,
    hc1_dmaaddr: HC1_DMAADDR,
    _reserved51: [u8; 0x08],
    hc2_char: HC2_CHAR,
    hc2_splt: HC2_SPLT,
    hc2_int: HC2_INT,
    hc2_intmsk: HC2_INTMSK,
    hc2_tsiz: HC2_TSIZ,
    hc2_dmaaddr: HC2_DMAADDR,
    _reserved57: [u8; 0x08],
    hc3_char: HC3_CHAR,
    hc3_splt: HC3_SPLT,
    hc3_int: HC3_INT,
    hc3_intmsk: HC3_INTMSK,
    hc3_tsiz: HC3_TSIZ,
    hc3_dmaaddr: HC3_DMAADDR,
    _reserved63: [u8; 0x08],
    hc4_char: HC4_CHAR,
    hc4_splt: HC4_SPLT,
    hc4_int: HC4_INT,
    hc4_intmsk: HC4_INTMSK,
    hc4_tsiz: HC4_TSIZ,
    hc4_dmaaddr: HC4_DMAADDR,
    _reserved69: [u8; 0x08],
    hc5_char: HC5_CHAR,
    hc5_splt: HC5_SPLT,
    hc5_int: HC5_INT,
    hc5_intmsk: HC5_INTMSK,
    hc5_tsiz: HC5_TSIZ,
    hc5_dmaaddr: HC5_DMAADDR,
    _reserved75: [u8; 0x08],
    hc6_char: HC6_CHAR,
    hc6_splt: HC6_SPLT,
    hc6_int: HC6_INT,
    hc6_intmsk: HC6_INTMSK,
    hc6_tsiz: HC6_TSIZ,
    hc6_dmaaddr: HC6_DMAADDR,
    _reserved81: [u8; 0x08],
    hc7_char: HC7_CHAR,
    hc7_splt: HC7_SPLT,
    hc7_int: HC7_INT,
    hc7_intmsk: HC7_INTMSK,
    hc7_tsiz: HC7_TSIZ,
    hc7_dmaaddr: HC7_DMAADDR,
    _reserved87: [u8; 0x08],
    hc8_char: HC8_CHAR,
    hc8_splt: HC8_SPLT,
    hc8_int: HC8_INT,
    hc8_intmsk: HC8_INTMSK,
    hc8_tsiz: HC8_TSIZ,
    hc8_dmaaddr: HC8_DMAADDR,
    _reserved93: [u8; 0x08],
    hc9_char: HC9_CHAR,
    hc9_splt: HC9_SPLT,
    hc9_int: HC9_INT,
    hc9_intmsk: HC9_INTMSK,
    hc9_tsiz: HC9_TSIZ,
    hc9_dmaaddr: HC9_DMAADDR,
    _reserved99: [u8; 0x08],
    hc10_char: HC10_CHAR,
    hc10_splt: HC10_SPLT,
    hc10_int: HC10_INT,
    hc10_intmsk: HC10_INTMSK,
    hc10_tsiz: HC10_TSIZ,
    hc10_dmaaddr: HC10_DMAADDR,
    _reserved105: [u8; 0x08],
    hc11_char: HC11_CHAR,
    hc11_splt: HC11_SPLT,
    hc11_int: HC11_INT,
    hc11_intmsk: HC11_INTMSK,
    hc11_tsiz: HC11_TSIZ,
    hc11_dmaaddr: HC11_DMAADDR,
    _reserved111: [u8; 0x08],
    hc12_char: HC12_CHAR,
    hc12_splt: HC12_SPLT,
    hc12_int: HC12_INT,
    hc12_intmsk: HC12_INTMSK,
    hc12_tsiz: HC12_TSIZ,
    hc12_dmaaddr: HC12_DMAADDR,
    _reserved117: [u8; 0x08],
    hc13_char: HC13_CHAR,
    hc13_splt: HC13_SPLT,
    hc13_int: HC13_INT,
    hc13_intmsk: HC13_INTMSK,
    hc13_tsiz: HC13_TSIZ,
    hc13_dmaaddr: HC13_DMAADDR,
    _reserved123: [u8; 0x0148],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved126: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved130: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    _reserved134: [u8; 0xc8],
    diep0ctl: DIEP0CTL,
    _reserved135: [u8; 0x04],
    diep0int: DIEP0INT,
    _reserved136: [u8; 0x04],
    diep0tsiz: DIEP0TSIZ,
    diep0dmaaddr: DIEP0DMAADDR,
    diep0txfsts: DIEP0TXFSTS,
    _reserved139: [u8; 0x04],
    diep0_ctl: DIEP0_CTL,
    _reserved140: [u8; 0x04],
    diep0_int: DIEP0_INT,
    _reserved141: [u8; 0x04],
    diep0_tsiz: DIEP0_TSIZ,
    diep0_dmaaddr: DIEP0_DMAADDR,
    diep0_dtxfsts: DIEP0_DTXFSTS,
    _reserved144: [u8; 0x04],
    diep1_ctl: DIEP1_CTL,
    _reserved145: [u8; 0x04],
    diep1_int: DIEP1_INT,
    _reserved146: [u8; 0x04],
    diep1_tsiz: DIEP1_TSIZ,
    diep1_dmaaddr: DIEP1_DMAADDR,
    diep1_dtxfsts: DIEP1_DTXFSTS,
    _reserved149: [u8; 0x04],
    diep2_ctl: DIEP2_CTL,
    _reserved150: [u8; 0x04],
    diep2_int: DIEP2_INT,
    _reserved151: [u8; 0x04],
    diep2_tsiz: DIEP2_TSIZ,
    diep2_dmaaddr: DIEP2_DMAADDR,
    diep2_dtxfsts: DIEP2_DTXFSTS,
    _reserved154: [u8; 0x04],
    diep3_ctl: DIEP3_CTL,
    _reserved155: [u8; 0x04],
    diep3_int: DIEP3_INT,
    _reserved156: [u8; 0x04],
    diep3_tsiz: DIEP3_TSIZ,
    diep3_dmaaddr: DIEP3_DMAADDR,
    diep3_dtxfsts: DIEP3_DTXFSTS,
    _reserved159: [u8; 0x04],
    diep4_ctl: DIEP4_CTL,
    _reserved160: [u8; 0x04],
    diep4_int: DIEP4_INT,
    _reserved161: [u8; 0x04],
    diep4_tsiz: DIEP4_TSIZ,
    diep4_dmaaddr: DIEP4_DMAADDR,
    diep4_dtxfsts: DIEP4_DTXFSTS,
    _reserved164: [u8; 0x04],
    diep5_ctl: DIEP5_CTL,
    _reserved165: [u8; 0x04],
    diep5_int: DIEP5_INT,
    _reserved166: [u8; 0x04],
    diep5_tsiz: DIEP5_TSIZ,
    diep5_dmaaddr: DIEP5_DMAADDR,
    diep5_dtxfsts: DIEP5_DTXFSTS,
    _reserved169: [u8; 0x0124],
    doep0ctl: DOEP0CTL,
    _reserved170: [u8; 0x04],
    doep0int: DOEP0INT,
    _reserved171: [u8; 0x04],
    doep0tsiz: DOEP0TSIZ,
    doep0dmaaddr: DOEP0DMAADDR,
    _reserved173: [u8; 0x08],
    doep0_ctl: DOEP0_CTL,
    _reserved174: [u8; 0x04],
    doep0_int: DOEP0_INT,
    _reserved175: [u8; 0x04],
    doep0_tsiz: DOEP0_TSIZ,
    doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved177: [u8; 0x08],
    doep1_ctl: DOEP1_CTL,
    _reserved178: [u8; 0x04],
    doep1_int: DOEP1_INT,
    _reserved179: [u8; 0x04],
    doep1_tsiz: DOEP1_TSIZ,
    doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved181: [u8; 0x08],
    doep2_ctl: DOEP2_CTL,
    _reserved182: [u8; 0x04],
    doep2_int: DOEP2_INT,
    _reserved183: [u8; 0x04],
    doep2_tsiz: DOEP2_TSIZ,
    doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved185: [u8; 0x08],
    doep3_ctl: DOEP3_CTL,
    _reserved186: [u8; 0x04],
    doep3_int: DOEP3_INT,
    _reserved187: [u8; 0x04],
    doep3_tsiz: DOEP3_TSIZ,
    doep3_dmaaddr: DOEP3_DMAADDR,
    _reserved189: [u8; 0x08],
    doep4_ctl: DOEP4_CTL,
    _reserved190: [u8; 0x04],
    doep4_int: DOEP4_INT,
    _reserved191: [u8; 0x04],
    doep4_tsiz: DOEP4_TSIZ,
    doep4_dmaaddr: DOEP4_DMAADDR,
    _reserved193: [u8; 0x08],
    doep5_ctl: DOEP5_CTL,
    _reserved194: [u8; 0x04],
    doep5_int: DOEP5_INT,
    _reserved195: [u8; 0x04],
    doep5_tsiz: DOEP5_TSIZ,
    doep5_dmaaddr: DOEP5_DMAADDR,
    _reserved197: [u8; 0x0228],
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - System Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x0c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x18 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0x2c - Charger Detect Configuration Register"]
    #[inline(always)]
    pub const fn cdconf(&self) -> &CDCONF {
        &self.cdconf
    }
    #[doc = "0x30 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x34 - Data TRIM 1 Values for USB DP and DM"]
    #[inline(always)]
    pub const fn dattrim1(&self) -> &DATTRIM1 {
        &self.dattrim1
    }
    #[doc = "0x3c - USB LEM Control Register"]
    #[inline(always)]
    pub const fn lemctrl(&self) -> &LEMCTRL {
        &self.lemctrl
    }
    #[doc = "0xde000 - OTG Control and Status Register"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0xde004 - OTG Interrupt Register"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0xde008 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0xde00c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0xde010 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0xde014 - Interrupt Register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0xde018 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0xde01c - Receive Status Debug Read Register"]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &GRXSTSR {
        &self.grxstsr
    }
    #[doc = "0xde020 - Receive Status Read /Pop Register"]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &GRXSTSP {
        &self.grxstsp
    }
    #[doc = "0xde024 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0xde028 - Non-periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &GNPTXFSIZ {
        &self.gnptxfsiz
    }
    #[doc = "0xde02c - Non-periodic Transmit FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &GNPTXSTS {
        &self.gnptxsts
    }
    #[doc = "0xde040 - Synopsys ID Register"]
    #[inline(always)]
    pub const fn gsnpsid(&self) -> &GSNPSID {
        &self.gsnpsid
    }
    #[doc = "0xde05c - Global DFIFO Configuration Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &GDFIFOCFG {
        &self.gdfifocfg
    }
    #[doc = "0xde100 - Host Periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0xde104 - Device IN Endpoint Transmit FIFO Size Register 1"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    #[doc = "0xde108 - Device IN Endpoint Transmit FIFO Size Register 2"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    #[doc = "0xde10c - Device IN Endpoint Transmit FIFO Size Register 3"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    #[doc = "0xde110 - Device IN Endpoint Transmit FIFO Size Register 4"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    #[doc = "0xde114 - Device IN Endpoint Transmit FIFO Size Register 5"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF5 {
        &self.dieptxf5
    }
    #[doc = "0xde118 - Device IN Endpoint Transmit FIFO Size Register 6"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &DIEPTXF6 {
        &self.dieptxf6
    }
    #[doc = "0xde400 - Host Configuration Register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0xde404 - Host Frame Interval Register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0xde408 - Host Frame Number/Frame Time Remaining Register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0xde410 - Host Periodic Transmit FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0xde414 - Host All Channels Interrupt Register"]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0xde418 - Host All Channels Interrupt Mask Register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0xde440 - Host Port Control and Status Register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0xde500 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc0_char(&self) -> &HC0_CHAR {
        &self.hc0_char
    }
    #[doc = "0xde504 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc0_splt(&self) -> &HC0_SPLT {
        &self.hc0_splt
    }
    #[doc = "0xde508 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc0_int(&self) -> &HC0_INT {
        &self.hc0_int
    }
    #[doc = "0xde50c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc0_intmsk(&self) -> &HC0_INTMSK {
        &self.hc0_intmsk
    }
    #[doc = "0xde510 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc0_tsiz(&self) -> &HC0_TSIZ {
        &self.hc0_tsiz
    }
    #[doc = "0xde514 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc0_dmaaddr(&self) -> &HC0_DMAADDR {
        &self.hc0_dmaaddr
    }
    #[doc = "0xde520 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc1_char(&self) -> &HC1_CHAR {
        &self.hc1_char
    }
    #[doc = "0xde524 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc1_splt(&self) -> &HC1_SPLT {
        &self.hc1_splt
    }
    #[doc = "0xde528 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc1_int(&self) -> &HC1_INT {
        &self.hc1_int
    }
    #[doc = "0xde52c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc1_intmsk(&self) -> &HC1_INTMSK {
        &self.hc1_intmsk
    }
    #[doc = "0xde530 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc1_tsiz(&self) -> &HC1_TSIZ {
        &self.hc1_tsiz
    }
    #[doc = "0xde534 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc1_dmaaddr(&self) -> &HC1_DMAADDR {
        &self.hc1_dmaaddr
    }
    #[doc = "0xde540 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc2_char(&self) -> &HC2_CHAR {
        &self.hc2_char
    }
    #[doc = "0xde544 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc2_splt(&self) -> &HC2_SPLT {
        &self.hc2_splt
    }
    #[doc = "0xde548 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc2_int(&self) -> &HC2_INT {
        &self.hc2_int
    }
    #[doc = "0xde54c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc2_intmsk(&self) -> &HC2_INTMSK {
        &self.hc2_intmsk
    }
    #[doc = "0xde550 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc2_tsiz(&self) -> &HC2_TSIZ {
        &self.hc2_tsiz
    }
    #[doc = "0xde554 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc2_dmaaddr(&self) -> &HC2_DMAADDR {
        &self.hc2_dmaaddr
    }
    #[doc = "0xde560 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc3_char(&self) -> &HC3_CHAR {
        &self.hc3_char
    }
    #[doc = "0xde564 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc3_splt(&self) -> &HC3_SPLT {
        &self.hc3_splt
    }
    #[doc = "0xde568 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc3_int(&self) -> &HC3_INT {
        &self.hc3_int
    }
    #[doc = "0xde56c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc3_intmsk(&self) -> &HC3_INTMSK {
        &self.hc3_intmsk
    }
    #[doc = "0xde570 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc3_tsiz(&self) -> &HC3_TSIZ {
        &self.hc3_tsiz
    }
    #[doc = "0xde574 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc3_dmaaddr(&self) -> &HC3_DMAADDR {
        &self.hc3_dmaaddr
    }
    #[doc = "0xde580 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc4_char(&self) -> &HC4_CHAR {
        &self.hc4_char
    }
    #[doc = "0xde584 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc4_splt(&self) -> &HC4_SPLT {
        &self.hc4_splt
    }
    #[doc = "0xde588 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc4_int(&self) -> &HC4_INT {
        &self.hc4_int
    }
    #[doc = "0xde58c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc4_intmsk(&self) -> &HC4_INTMSK {
        &self.hc4_intmsk
    }
    #[doc = "0xde590 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc4_tsiz(&self) -> &HC4_TSIZ {
        &self.hc4_tsiz
    }
    #[doc = "0xde594 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc4_dmaaddr(&self) -> &HC4_DMAADDR {
        &self.hc4_dmaaddr
    }
    #[doc = "0xde5a0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc5_char(&self) -> &HC5_CHAR {
        &self.hc5_char
    }
    #[doc = "0xde5a4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc5_splt(&self) -> &HC5_SPLT {
        &self.hc5_splt
    }
    #[doc = "0xde5a8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc5_int(&self) -> &HC5_INT {
        &self.hc5_int
    }
    #[doc = "0xde5ac - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc5_intmsk(&self) -> &HC5_INTMSK {
        &self.hc5_intmsk
    }
    #[doc = "0xde5b0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc5_tsiz(&self) -> &HC5_TSIZ {
        &self.hc5_tsiz
    }
    #[doc = "0xde5b4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc5_dmaaddr(&self) -> &HC5_DMAADDR {
        &self.hc5_dmaaddr
    }
    #[doc = "0xde5c0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc6_char(&self) -> &HC6_CHAR {
        &self.hc6_char
    }
    #[doc = "0xde5c4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc6_splt(&self) -> &HC6_SPLT {
        &self.hc6_splt
    }
    #[doc = "0xde5c8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc6_int(&self) -> &HC6_INT {
        &self.hc6_int
    }
    #[doc = "0xde5cc - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc6_intmsk(&self) -> &HC6_INTMSK {
        &self.hc6_intmsk
    }
    #[doc = "0xde5d0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc6_tsiz(&self) -> &HC6_TSIZ {
        &self.hc6_tsiz
    }
    #[doc = "0xde5d4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc6_dmaaddr(&self) -> &HC6_DMAADDR {
        &self.hc6_dmaaddr
    }
    #[doc = "0xde5e0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc7_char(&self) -> &HC7_CHAR {
        &self.hc7_char
    }
    #[doc = "0xde5e4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc7_splt(&self) -> &HC7_SPLT {
        &self.hc7_splt
    }
    #[doc = "0xde5e8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc7_int(&self) -> &HC7_INT {
        &self.hc7_int
    }
    #[doc = "0xde5ec - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc7_intmsk(&self) -> &HC7_INTMSK {
        &self.hc7_intmsk
    }
    #[doc = "0xde5f0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc7_tsiz(&self) -> &HC7_TSIZ {
        &self.hc7_tsiz
    }
    #[doc = "0xde5f4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc7_dmaaddr(&self) -> &HC7_DMAADDR {
        &self.hc7_dmaaddr
    }
    #[doc = "0xde600 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc8_char(&self) -> &HC8_CHAR {
        &self.hc8_char
    }
    #[doc = "0xde604 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc8_splt(&self) -> &HC8_SPLT {
        &self.hc8_splt
    }
    #[doc = "0xde608 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc8_int(&self) -> &HC8_INT {
        &self.hc8_int
    }
    #[doc = "0xde60c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc8_intmsk(&self) -> &HC8_INTMSK {
        &self.hc8_intmsk
    }
    #[doc = "0xde610 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc8_tsiz(&self) -> &HC8_TSIZ {
        &self.hc8_tsiz
    }
    #[doc = "0xde614 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc8_dmaaddr(&self) -> &HC8_DMAADDR {
        &self.hc8_dmaaddr
    }
    #[doc = "0xde620 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc9_char(&self) -> &HC9_CHAR {
        &self.hc9_char
    }
    #[doc = "0xde624 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc9_splt(&self) -> &HC9_SPLT {
        &self.hc9_splt
    }
    #[doc = "0xde628 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc9_int(&self) -> &HC9_INT {
        &self.hc9_int
    }
    #[doc = "0xde62c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc9_intmsk(&self) -> &HC9_INTMSK {
        &self.hc9_intmsk
    }
    #[doc = "0xde630 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc9_tsiz(&self) -> &HC9_TSIZ {
        &self.hc9_tsiz
    }
    #[doc = "0xde634 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc9_dmaaddr(&self) -> &HC9_DMAADDR {
        &self.hc9_dmaaddr
    }
    #[doc = "0xde640 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc10_char(&self) -> &HC10_CHAR {
        &self.hc10_char
    }
    #[doc = "0xde644 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc10_splt(&self) -> &HC10_SPLT {
        &self.hc10_splt
    }
    #[doc = "0xde648 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc10_int(&self) -> &HC10_INT {
        &self.hc10_int
    }
    #[doc = "0xde64c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc10_intmsk(&self) -> &HC10_INTMSK {
        &self.hc10_intmsk
    }
    #[doc = "0xde650 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc10_tsiz(&self) -> &HC10_TSIZ {
        &self.hc10_tsiz
    }
    #[doc = "0xde654 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc10_dmaaddr(&self) -> &HC10_DMAADDR {
        &self.hc10_dmaaddr
    }
    #[doc = "0xde660 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc11_char(&self) -> &HC11_CHAR {
        &self.hc11_char
    }
    #[doc = "0xde664 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc11_splt(&self) -> &HC11_SPLT {
        &self.hc11_splt
    }
    #[doc = "0xde668 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc11_int(&self) -> &HC11_INT {
        &self.hc11_int
    }
    #[doc = "0xde66c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc11_intmsk(&self) -> &HC11_INTMSK {
        &self.hc11_intmsk
    }
    #[doc = "0xde670 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc11_tsiz(&self) -> &HC11_TSIZ {
        &self.hc11_tsiz
    }
    #[doc = "0xde674 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc11_dmaaddr(&self) -> &HC11_DMAADDR {
        &self.hc11_dmaaddr
    }
    #[doc = "0xde680 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc12_char(&self) -> &HC12_CHAR {
        &self.hc12_char
    }
    #[doc = "0xde684 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc12_splt(&self) -> &HC12_SPLT {
        &self.hc12_splt
    }
    #[doc = "0xde688 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc12_int(&self) -> &HC12_INT {
        &self.hc12_int
    }
    #[doc = "0xde68c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc12_intmsk(&self) -> &HC12_INTMSK {
        &self.hc12_intmsk
    }
    #[doc = "0xde690 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc12_tsiz(&self) -> &HC12_TSIZ {
        &self.hc12_tsiz
    }
    #[doc = "0xde694 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc12_dmaaddr(&self) -> &HC12_DMAADDR {
        &self.hc12_dmaaddr
    }
    #[doc = "0xde6a0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc13_char(&self) -> &HC13_CHAR {
        &self.hc13_char
    }
    #[doc = "0xde6a4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc13_splt(&self) -> &HC13_SPLT {
        &self.hc13_splt
    }
    #[doc = "0xde6a8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc13_int(&self) -> &HC13_INT {
        &self.hc13_int
    }
    #[doc = "0xde6ac - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc13_intmsk(&self) -> &HC13_INTMSK {
        &self.hc13_intmsk
    }
    #[doc = "0xde6b0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc13_tsiz(&self) -> &HC13_TSIZ {
        &self.hc13_tsiz
    }
    #[doc = "0xde6b4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc13_dmaaddr(&self) -> &HC13_DMAADDR {
        &self.hc13_dmaaddr
    }
    #[doc = "0xde800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0xde804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0xde808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0xde810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0xde814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0xde818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0xde81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0xde828 - Device VBUS Discharge Time Register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0xde82c - Device VBUS Pulsing Time Register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0xde830 - Device Threshold Control Register"]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    #[doc = "0xde834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0xde900 - Device Control IN Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn diep0ctl(&self) -> &DIEP0CTL {
        &self.diep0ctl
    }
    #[doc = "0xde908 - Device IN Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0int(&self) -> &DIEP0INT {
        &self.diep0int
    }
    #[doc = "0xde910 - Device IN Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0tsiz(&self) -> &DIEP0TSIZ {
        &self.diep0tsiz
    }
    #[doc = "0xde914 - Device IN Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0dmaaddr(&self) -> &DIEP0DMAADDR {
        &self.diep0dmaaddr
    }
    #[doc = "0xde918 - Device IN Endpoint Transmit FIFO Status Register 0"]
    #[inline(always)]
    pub const fn diep0txfsts(&self) -> &DIEP0TXFSTS {
        &self.diep0txfsts
    }
    #[doc = "0xde920 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep0_ctl(&self) -> &DIEP0_CTL {
        &self.diep0_ctl
    }
    #[doc = "0xde928 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0_int(&self) -> &DIEP0_INT {
        &self.diep0_int
    }
    #[doc = "0xde930 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0_tsiz(&self) -> &DIEP0_TSIZ {
        &self.diep0_tsiz
    }
    #[doc = "0xde934 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0_dmaaddr(&self) -> &DIEP0_DMAADDR {
        &self.diep0_dmaaddr
    }
    #[doc = "0xde938 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep0_dtxfsts(&self) -> &DIEP0_DTXFSTS {
        &self.diep0_dtxfsts
    }
    #[doc = "0xde940 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep1_ctl(&self) -> &DIEP1_CTL {
        &self.diep1_ctl
    }
    #[doc = "0xde948 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep1_int(&self) -> &DIEP1_INT {
        &self.diep1_int
    }
    #[doc = "0xde950 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep1_tsiz(&self) -> &DIEP1_TSIZ {
        &self.diep1_tsiz
    }
    #[doc = "0xde954 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep1_dmaaddr(&self) -> &DIEP1_DMAADDR {
        &self.diep1_dmaaddr
    }
    #[doc = "0xde958 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep1_dtxfsts(&self) -> &DIEP1_DTXFSTS {
        &self.diep1_dtxfsts
    }
    #[doc = "0xde960 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep2_ctl(&self) -> &DIEP2_CTL {
        &self.diep2_ctl
    }
    #[doc = "0xde968 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep2_int(&self) -> &DIEP2_INT {
        &self.diep2_int
    }
    #[doc = "0xde970 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep2_tsiz(&self) -> &DIEP2_TSIZ {
        &self.diep2_tsiz
    }
    #[doc = "0xde974 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep2_dmaaddr(&self) -> &DIEP2_DMAADDR {
        &self.diep2_dmaaddr
    }
    #[doc = "0xde978 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep2_dtxfsts(&self) -> &DIEP2_DTXFSTS {
        &self.diep2_dtxfsts
    }
    #[doc = "0xde980 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep3_ctl(&self) -> &DIEP3_CTL {
        &self.diep3_ctl
    }
    #[doc = "0xde988 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep3_int(&self) -> &DIEP3_INT {
        &self.diep3_int
    }
    #[doc = "0xde990 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep3_tsiz(&self) -> &DIEP3_TSIZ {
        &self.diep3_tsiz
    }
    #[doc = "0xde994 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep3_dmaaddr(&self) -> &DIEP3_DMAADDR {
        &self.diep3_dmaaddr
    }
    #[doc = "0xde998 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep3_dtxfsts(&self) -> &DIEP3_DTXFSTS {
        &self.diep3_dtxfsts
    }
    #[doc = "0xde9a0 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep4_ctl(&self) -> &DIEP4_CTL {
        &self.diep4_ctl
    }
    #[doc = "0xde9a8 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep4_int(&self) -> &DIEP4_INT {
        &self.diep4_int
    }
    #[doc = "0xde9b0 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep4_tsiz(&self) -> &DIEP4_TSIZ {
        &self.diep4_tsiz
    }
    #[doc = "0xde9b4 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep4_dmaaddr(&self) -> &DIEP4_DMAADDR {
        &self.diep4_dmaaddr
    }
    #[doc = "0xde9b8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep4_dtxfsts(&self) -> &DIEP4_DTXFSTS {
        &self.diep4_dtxfsts
    }
    #[doc = "0xde9c0 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep5_ctl(&self) -> &DIEP5_CTL {
        &self.diep5_ctl
    }
    #[doc = "0xde9c8 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep5_int(&self) -> &DIEP5_INT {
        &self.diep5_int
    }
    #[doc = "0xde9d0 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep5_tsiz(&self) -> &DIEP5_TSIZ {
        &self.diep5_tsiz
    }
    #[doc = "0xde9d4 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep5_dmaaddr(&self) -> &DIEP5_DMAADDR {
        &self.diep5_dmaaddr
    }
    #[doc = "0xde9d8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep5_dtxfsts(&self) -> &DIEP5_DTXFSTS {
        &self.diep5_dtxfsts
    }
    #[doc = "0xdeb00 - Device Control OUT Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn doep0ctl(&self) -> &DOEP0CTL {
        &self.doep0ctl
    }
    #[doc = "0xdeb08 - Device OUT Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0int(&self) -> &DOEP0INT {
        &self.doep0int
    }
    #[doc = "0xdeb10 - Device OUT Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0tsiz(&self) -> &DOEP0TSIZ {
        &self.doep0tsiz
    }
    #[doc = "0xdeb14 - Device OUT Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0dmaaddr(&self) -> &DOEP0DMAADDR {
        &self.doep0dmaaddr
    }
    #[doc = "0xdeb20 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep0_ctl(&self) -> &DOEP0_CTL {
        &self.doep0_ctl
    }
    #[doc = "0xdeb28 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0_int(&self) -> &DOEP0_INT {
        &self.doep0_int
    }
    #[doc = "0xdeb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0_tsiz(&self) -> &DOEP0_TSIZ {
        &self.doep0_tsiz
    }
    #[doc = "0xdeb34 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0_dmaaddr(&self) -> &DOEP0_DMAADDR {
        &self.doep0_dmaaddr
    }
    #[doc = "0xdeb40 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep1_ctl(&self) -> &DOEP1_CTL {
        &self.doep1_ctl
    }
    #[doc = "0xdeb48 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep1_int(&self) -> &DOEP1_INT {
        &self.doep1_int
    }
    #[doc = "0xdeb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep1_tsiz(&self) -> &DOEP1_TSIZ {
        &self.doep1_tsiz
    }
    #[doc = "0xdeb54 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep1_dmaaddr(&self) -> &DOEP1_DMAADDR {
        &self.doep1_dmaaddr
    }
    #[doc = "0xdeb60 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep2_ctl(&self) -> &DOEP2_CTL {
        &self.doep2_ctl
    }
    #[doc = "0xdeb68 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep2_int(&self) -> &DOEP2_INT {
        &self.doep2_int
    }
    #[doc = "0xdeb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep2_tsiz(&self) -> &DOEP2_TSIZ {
        &self.doep2_tsiz
    }
    #[doc = "0xdeb74 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep2_dmaaddr(&self) -> &DOEP2_DMAADDR {
        &self.doep2_dmaaddr
    }
    #[doc = "0xdeb80 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep3_ctl(&self) -> &DOEP3_CTL {
        &self.doep3_ctl
    }
    #[doc = "0xdeb88 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep3_int(&self) -> &DOEP3_INT {
        &self.doep3_int
    }
    #[doc = "0xdeb90 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep3_tsiz(&self) -> &DOEP3_TSIZ {
        &self.doep3_tsiz
    }
    #[doc = "0xdeb94 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep3_dmaaddr(&self) -> &DOEP3_DMAADDR {
        &self.doep3_dmaaddr
    }
    #[doc = "0xdeba0 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep4_ctl(&self) -> &DOEP4_CTL {
        &self.doep4_ctl
    }
    #[doc = "0xdeba8 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep4_int(&self) -> &DOEP4_INT {
        &self.doep4_int
    }
    #[doc = "0xdebb0 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep4_tsiz(&self) -> &DOEP4_TSIZ {
        &self.doep4_tsiz
    }
    #[doc = "0xdebb4 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep4_dmaaddr(&self) -> &DOEP4_DMAADDR {
        &self.doep4_dmaaddr
    }
    #[doc = "0xdebc0 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep5_ctl(&self) -> &DOEP5_CTL {
        &self.doep5_ctl
    }
    #[doc = "0xdebc8 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep5_int(&self) -> &DOEP5_INT {
        &self.doep5_int
    }
    #[doc = "0xdebd0 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep5_tsiz(&self) -> &DOEP5_TSIZ {
        &self.doep5_tsiz
    }
    #[doc = "0xdebd4 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep5_dmaaddr(&self) -> &DOEP5_DMAADDR {
        &self.doep5_dmaaddr
    }
    #[doc = "0xdee00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "CTRL (rw) register accessor: System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CDCONF (rw) register accessor: Charger Detect Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdconf`]
module"]
pub type CDCONF = crate::Reg<cdconf::CDCONF_SPEC>;
#[doc = "Charger Detect Configuration Register"]
pub mod cdconf;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DATTRIM1 (rw) register accessor: Data TRIM 1 Values for USB DP and DM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dattrim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dattrim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dattrim1`]
module"]
pub type DATTRIM1 = crate::Reg<dattrim1::DATTRIM1_SPEC>;
#[doc = "Data TRIM 1 Values for USB DP and DM"]
pub mod dattrim1;
#[doc = "LEMCTRL (rw) register accessor: USB LEM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lemctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lemctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lemctrl`]
module"]
pub type LEMCTRL = crate::Reg<lemctrl::LEMCTRL_SPEC>;
#[doc = "USB LEM Control Register"]
pub mod lemctrl;
#[doc = "GOTGCTL (rw) register accessor: OTG Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: Receive Status Debug Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`]
module"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: Receive Status Read /Pop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`]
module"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read /Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`]
module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: Non-periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`]
module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GSNPSID (r) register accessor: Synopsys ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsnpsid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsnpsid`]
module"]
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
#[doc = "Synopsys ID Register"]
pub mod gsnpsid;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`]
module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6"]
pub mod dieptxf6;
#[doc = "HCFG (rw) register accessor: Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: Host Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: Host Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: Host All Channels Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "HC0_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_char`]
module"]
pub type HC0_CHAR = crate::Reg<hc0_char::HC0_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc0_char;
#[doc = "HC0_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_splt`]
module"]
pub type HC0_SPLT = crate::Reg<hc0_splt::HC0_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc0_splt;
#[doc = "HC0_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_int`]
module"]
pub type HC0_INT = crate::Reg<hc0_int::HC0_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc0_int;
#[doc = "HC0_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_intmsk`]
module"]
pub type HC0_INTMSK = crate::Reg<hc0_intmsk::HC0_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "HC0_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_tsiz`]
module"]
pub type HC0_TSIZ = crate::Reg<hc0_tsiz::HC0_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "HC0_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_dmaaddr`]
module"]
pub type HC0_DMAADDR = crate::Reg<hc0_dmaaddr::HC0_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "HC1_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_char`]
module"]
pub type HC1_CHAR = crate::Reg<hc1_char::HC1_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc1_char;
#[doc = "HC1_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_splt`]
module"]
pub type HC1_SPLT = crate::Reg<hc1_splt::HC1_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc1_splt;
#[doc = "HC1_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_int`]
module"]
pub type HC1_INT = crate::Reg<hc1_int::HC1_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc1_int;
#[doc = "HC1_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_intmsk`]
module"]
pub type HC1_INTMSK = crate::Reg<hc1_intmsk::HC1_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "HC1_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_tsiz`]
module"]
pub type HC1_TSIZ = crate::Reg<hc1_tsiz::HC1_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "HC1_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_dmaaddr`]
module"]
pub type HC1_DMAADDR = crate::Reg<hc1_dmaaddr::HC1_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "HC2_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_char`]
module"]
pub type HC2_CHAR = crate::Reg<hc2_char::HC2_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc2_char;
#[doc = "HC2_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_splt`]
module"]
pub type HC2_SPLT = crate::Reg<hc2_splt::HC2_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc2_splt;
#[doc = "HC2_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_int`]
module"]
pub type HC2_INT = crate::Reg<hc2_int::HC2_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc2_int;
#[doc = "HC2_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_intmsk`]
module"]
pub type HC2_INTMSK = crate::Reg<hc2_intmsk::HC2_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "HC2_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_tsiz`]
module"]
pub type HC2_TSIZ = crate::Reg<hc2_tsiz::HC2_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "HC2_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_dmaaddr`]
module"]
pub type HC2_DMAADDR = crate::Reg<hc2_dmaaddr::HC2_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "HC3_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_char`]
module"]
pub type HC3_CHAR = crate::Reg<hc3_char::HC3_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc3_char;
#[doc = "HC3_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_splt`]
module"]
pub type HC3_SPLT = crate::Reg<hc3_splt::HC3_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc3_splt;
#[doc = "HC3_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_int`]
module"]
pub type HC3_INT = crate::Reg<hc3_int::HC3_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc3_int;
#[doc = "HC3_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_intmsk`]
module"]
pub type HC3_INTMSK = crate::Reg<hc3_intmsk::HC3_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "HC3_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_tsiz`]
module"]
pub type HC3_TSIZ = crate::Reg<hc3_tsiz::HC3_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "HC3_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_dmaaddr`]
module"]
pub type HC3_DMAADDR = crate::Reg<hc3_dmaaddr::HC3_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "HC4_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_char`]
module"]
pub type HC4_CHAR = crate::Reg<hc4_char::HC4_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc4_char;
#[doc = "HC4_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_splt`]
module"]
pub type HC4_SPLT = crate::Reg<hc4_splt::HC4_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc4_splt;
#[doc = "HC4_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_int`]
module"]
pub type HC4_INT = crate::Reg<hc4_int::HC4_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc4_int;
#[doc = "HC4_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_intmsk`]
module"]
pub type HC4_INTMSK = crate::Reg<hc4_intmsk::HC4_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "HC4_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_tsiz`]
module"]
pub type HC4_TSIZ = crate::Reg<hc4_tsiz::HC4_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "HC4_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc4_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc4_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_dmaaddr`]
module"]
pub type HC4_DMAADDR = crate::Reg<hc4_dmaaddr::HC4_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "HC5_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_char`]
module"]
pub type HC5_CHAR = crate::Reg<hc5_char::HC5_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc5_char;
#[doc = "HC5_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_splt`]
module"]
pub type HC5_SPLT = crate::Reg<hc5_splt::HC5_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc5_splt;
#[doc = "HC5_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_int`]
module"]
pub type HC5_INT = crate::Reg<hc5_int::HC5_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc5_int;
#[doc = "HC5_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_intmsk`]
module"]
pub type HC5_INTMSK = crate::Reg<hc5_intmsk::HC5_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "HC5_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_tsiz`]
module"]
pub type HC5_TSIZ = crate::Reg<hc5_tsiz::HC5_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "HC5_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc5_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc5_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_dmaaddr`]
module"]
pub type HC5_DMAADDR = crate::Reg<hc5_dmaaddr::HC5_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "HC6_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_char`]
module"]
pub type HC6_CHAR = crate::Reg<hc6_char::HC6_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc6_char;
#[doc = "HC6_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_splt`]
module"]
pub type HC6_SPLT = crate::Reg<hc6_splt::HC6_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc6_splt;
#[doc = "HC6_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_int`]
module"]
pub type HC6_INT = crate::Reg<hc6_int::HC6_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc6_int;
#[doc = "HC6_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_intmsk`]
module"]
pub type HC6_INTMSK = crate::Reg<hc6_intmsk::HC6_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "HC6_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_tsiz`]
module"]
pub type HC6_TSIZ = crate::Reg<hc6_tsiz::HC6_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "HC6_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc6_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc6_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_dmaaddr`]
module"]
pub type HC6_DMAADDR = crate::Reg<hc6_dmaaddr::HC6_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "HC7_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_char`]
module"]
pub type HC7_CHAR = crate::Reg<hc7_char::HC7_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc7_char;
#[doc = "HC7_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_splt`]
module"]
pub type HC7_SPLT = crate::Reg<hc7_splt::HC7_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc7_splt;
#[doc = "HC7_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_int`]
module"]
pub type HC7_INT = crate::Reg<hc7_int::HC7_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc7_int;
#[doc = "HC7_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_intmsk`]
module"]
pub type HC7_INTMSK = crate::Reg<hc7_intmsk::HC7_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "HC7_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_tsiz`]
module"]
pub type HC7_TSIZ = crate::Reg<hc7_tsiz::HC7_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "HC7_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc7_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc7_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_dmaaddr`]
module"]
pub type HC7_DMAADDR = crate::Reg<hc7_dmaaddr::HC7_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "HC8_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_char`]
module"]
pub type HC8_CHAR = crate::Reg<hc8_char::HC8_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc8_char;
#[doc = "HC8_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_splt`]
module"]
pub type HC8_SPLT = crate::Reg<hc8_splt::HC8_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc8_splt;
#[doc = "HC8_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_int`]
module"]
pub type HC8_INT = crate::Reg<hc8_int::HC8_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc8_int;
#[doc = "HC8_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_intmsk`]
module"]
pub type HC8_INTMSK = crate::Reg<hc8_intmsk::HC8_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "HC8_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_tsiz`]
module"]
pub type HC8_TSIZ = crate::Reg<hc8_tsiz::HC8_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "HC8_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc8_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc8_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_dmaaddr`]
module"]
pub type HC8_DMAADDR = crate::Reg<hc8_dmaaddr::HC8_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "HC9_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_char`]
module"]
pub type HC9_CHAR = crate::Reg<hc9_char::HC9_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc9_char;
#[doc = "HC9_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_splt`]
module"]
pub type HC9_SPLT = crate::Reg<hc9_splt::HC9_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc9_splt;
#[doc = "HC9_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_int`]
module"]
pub type HC9_INT = crate::Reg<hc9_int::HC9_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc9_int;
#[doc = "HC9_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_intmsk`]
module"]
pub type HC9_INTMSK = crate::Reg<hc9_intmsk::HC9_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "HC9_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_tsiz`]
module"]
pub type HC9_TSIZ = crate::Reg<hc9_tsiz::HC9_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "HC9_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc9_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc9_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_dmaaddr`]
module"]
pub type HC9_DMAADDR = crate::Reg<hc9_dmaaddr::HC9_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "HC10_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_char`]
module"]
pub type HC10_CHAR = crate::Reg<hc10_char::HC10_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc10_char;
#[doc = "HC10_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_splt`]
module"]
pub type HC10_SPLT = crate::Reg<hc10_splt::HC10_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc10_splt;
#[doc = "HC10_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_int`]
module"]
pub type HC10_INT = crate::Reg<hc10_int::HC10_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc10_int;
#[doc = "HC10_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_intmsk`]
module"]
pub type HC10_INTMSK = crate::Reg<hc10_intmsk::HC10_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "HC10_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_tsiz`]
module"]
pub type HC10_TSIZ = crate::Reg<hc10_tsiz::HC10_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "HC10_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc10_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc10_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_dmaaddr`]
module"]
pub type HC10_DMAADDR = crate::Reg<hc10_dmaaddr::HC10_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "HC11_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_char`]
module"]
pub type HC11_CHAR = crate::Reg<hc11_char::HC11_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc11_char;
#[doc = "HC11_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_splt`]
module"]
pub type HC11_SPLT = crate::Reg<hc11_splt::HC11_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc11_splt;
#[doc = "HC11_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_int`]
module"]
pub type HC11_INT = crate::Reg<hc11_int::HC11_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc11_int;
#[doc = "HC11_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_intmsk`]
module"]
pub type HC11_INTMSK = crate::Reg<hc11_intmsk::HC11_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "HC11_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_tsiz`]
module"]
pub type HC11_TSIZ = crate::Reg<hc11_tsiz::HC11_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "HC11_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc11_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc11_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_dmaaddr`]
module"]
pub type HC11_DMAADDR = crate::Reg<hc11_dmaaddr::HC11_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "HC12_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_char`]
module"]
pub type HC12_CHAR = crate::Reg<hc12_char::HC12_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc12_char;
#[doc = "HC12_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_splt`]
module"]
pub type HC12_SPLT = crate::Reg<hc12_splt::HC12_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc12_splt;
#[doc = "HC12_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_int`]
module"]
pub type HC12_INT = crate::Reg<hc12_int::HC12_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc12_int;
#[doc = "HC12_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_intmsk`]
module"]
pub type HC12_INTMSK = crate::Reg<hc12_intmsk::HC12_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "HC12_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_tsiz`]
module"]
pub type HC12_TSIZ = crate::Reg<hc12_tsiz::HC12_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "HC12_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc12_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc12_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_dmaaddr`]
module"]
pub type HC12_DMAADDR = crate::Reg<hc12_dmaaddr::HC12_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "HC13_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_char`]
module"]
pub type HC13_CHAR = crate::Reg<hc13_char::HC13_CHAR_SPEC>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc13_char;
#[doc = "HC13_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_splt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_splt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_splt`]
module"]
pub type HC13_SPLT = crate::Reg<hc13_splt::HC13_SPLT_SPEC>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc13_splt;
#[doc = "HC13_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_int`]
module"]
pub type HC13_INT = crate::Reg<hc13_int::HC13_INT_SPEC>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc13_int;
#[doc = "HC13_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_intmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_intmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_intmsk`]
module"]
pub type HC13_INTMSK = crate::Reg<hc13_intmsk::HC13_INTMSK_SPEC>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "HC13_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_tsiz`]
module"]
pub type HC13_TSIZ = crate::Reg<hc13_tsiz::HC13_TSIZ_SPEC>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "HC13_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc13_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc13_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_dmaaddr`]
module"]
pub type HC13_DMAADDR = crate::Reg<hc13_dmaaddr::HC13_DMAADDR_SPEC>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: Device VBUS Discharge Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: Device Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`]
module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "Device Threshold Control Register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: Device Control IN Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0ctl`]
module"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "Device Control IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0int`]
module"]
pub type DIEP0INT = crate::Reg<diep0int::DIEP0INT_SPEC>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tsiz`]
module"]
pub type DIEP0TSIZ = crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0dmaaddr`]
module"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0txfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0txfsts`]
module"]
pub type DIEP0TXFSTS = crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_ctl`]
module"]
pub type DIEP0_CTL = crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_int`]
module"]
pub type DIEP0_INT = crate::Reg<diep0_int::DIEP0_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_tsiz`]
module"]
pub type DIEP0_TSIZ = crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_dmaaddr`]
module"]
pub type DIEP0_DMAADDR = crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_dtxfsts`]
module"]
pub type DIEP0_DTXFSTS = crate::Reg<diep0_dtxfsts::DIEP0_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep0_dtxfsts;
#[doc = "DIEP1_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_ctl`]
module"]
pub type DIEP1_CTL = crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_int`]
module"]
pub type DIEP1_INT = crate::Reg<diep1_int::DIEP1_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_tsiz`]
module"]
pub type DIEP1_TSIZ = crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_dmaaddr`]
module"]
pub type DIEP1_DMAADDR = crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_dtxfsts`]
module"]
pub type DIEP1_DTXFSTS = crate::Reg<diep1_dtxfsts::DIEP1_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep1_dtxfsts;
#[doc = "DIEP2_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_ctl`]
module"]
pub type DIEP2_CTL = crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_int`]
module"]
pub type DIEP2_INT = crate::Reg<diep2_int::DIEP2_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_tsiz`]
module"]
pub type DIEP2_TSIZ = crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_dmaaddr`]
module"]
pub type DIEP2_DMAADDR = crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_dtxfsts`]
module"]
pub type DIEP2_DTXFSTS = crate::Reg<diep2_dtxfsts::DIEP2_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep2_dtxfsts;
#[doc = "DIEP3_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_ctl`]
module"]
pub type DIEP3_CTL = crate::Reg<diep3_ctl::DIEP3_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep3_ctl;
#[doc = "DIEP3_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_int`]
module"]
pub type DIEP3_INT = crate::Reg<diep3_int::DIEP3_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep3_int;
#[doc = "DIEP3_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_tsiz`]
module"]
pub type DIEP3_TSIZ = crate::Reg<diep3_tsiz::DIEP3_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "DIEP3_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_dmaaddr`]
module"]
pub type DIEP3_DMAADDR = crate::Reg<diep3_dmaaddr::DIEP3_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "DIEP3_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_dtxfsts`]
module"]
pub type DIEP3_DTXFSTS = crate::Reg<diep3_dtxfsts::DIEP3_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep3_dtxfsts;
#[doc = "DIEP4_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_ctl`]
module"]
pub type DIEP4_CTL = crate::Reg<diep4_ctl::DIEP4_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep4_ctl;
#[doc = "DIEP4_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_int`]
module"]
pub type DIEP4_INT = crate::Reg<diep4_int::DIEP4_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep4_int;
#[doc = "DIEP4_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_tsiz`]
module"]
pub type DIEP4_TSIZ = crate::Reg<diep4_tsiz::DIEP4_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "DIEP4_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep4_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_dmaaddr`]
module"]
pub type DIEP4_DMAADDR = crate::Reg<diep4_dmaaddr::DIEP4_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "DIEP4_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep4_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_dtxfsts`]
module"]
pub type DIEP4_DTXFSTS = crate::Reg<diep4_dtxfsts::DIEP4_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep4_dtxfsts;
#[doc = "DIEP5_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_ctl`]
module"]
pub type DIEP5_CTL = crate::Reg<diep5_ctl::DIEP5_CTL_SPEC>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep5_ctl;
#[doc = "DIEP5_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_int`]
module"]
pub type DIEP5_INT = crate::Reg<diep5_int::DIEP5_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep5_int;
#[doc = "DIEP5_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_tsiz`]
module"]
pub type DIEP5_TSIZ = crate::Reg<diep5_tsiz::DIEP5_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "DIEP5_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_dmaaddr`]
module"]
pub type DIEP5_DMAADDR = crate::Reg<diep5_dmaaddr::DIEP5_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "DIEP5_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_dtxfsts`]
module"]
pub type DIEP5_DTXFSTS = crate::Reg<diep5_dtxfsts::DIEP5_DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep5_dtxfsts;
#[doc = "DOEP0CTL (rw) register accessor: Device Control OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0ctl`]
module"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: Device OUT Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0int`]
module"]
pub type DOEP0INT = crate::Reg<doep0int::DOEP0INT_SPEC>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0tsiz`]
module"]
pub type DOEP0TSIZ = crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: Device OUT Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0dmaaddr`]
module"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_ctl`]
module"]
pub type DOEP0_CTL = crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_int`]
module"]
pub type DOEP0_INT = crate::Reg<doep0_int::DOEP0_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_tsiz`]
module"]
pub type DOEP0_TSIZ = crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_dmaaddr`]
module"]
pub type DOEP0_DMAADDR = crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_ctl`]
module"]
pub type DOEP1_CTL = crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_int`]
module"]
pub type DOEP1_INT = crate::Reg<doep1_int::DOEP1_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_tsiz`]
module"]
pub type DOEP1_TSIZ = crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_dmaaddr`]
module"]
pub type DOEP1_DMAADDR = crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_ctl`]
module"]
pub type DOEP2_CTL = crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_int`]
module"]
pub type DOEP2_INT = crate::Reg<doep2_int::DOEP2_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_tsiz`]
module"]
pub type DOEP2_TSIZ = crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_dmaaddr`]
module"]
pub type DOEP2_DMAADDR = crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "DOEP3_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_ctl`]
module"]
pub type DOEP3_CTL = crate::Reg<doep3_ctl::DOEP3_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep3_ctl;
#[doc = "DOEP3_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_int`]
module"]
pub type DOEP3_INT = crate::Reg<doep3_int::DOEP3_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep3_int;
#[doc = "DOEP3_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_tsiz`]
module"]
pub type DOEP3_TSIZ = crate::Reg<doep3_tsiz::DOEP3_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "DOEP3_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_dmaaddr`]
module"]
pub type DOEP3_DMAADDR = crate::Reg<doep3_dmaaddr::DOEP3_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "DOEP4_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_ctl`]
module"]
pub type DOEP4_CTL = crate::Reg<doep4_ctl::DOEP4_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep4_ctl;
#[doc = "DOEP4_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_int`]
module"]
pub type DOEP4_INT = crate::Reg<doep4_int::DOEP4_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep4_int;
#[doc = "DOEP4_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_tsiz`]
module"]
pub type DOEP4_TSIZ = crate::Reg<doep4_tsiz::DOEP4_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "DOEP4_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep4_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep4_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_dmaaddr`]
module"]
pub type DOEP4_DMAADDR = crate::Reg<doep4_dmaaddr::DOEP4_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "DOEP5_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_ctl`]
module"]
pub type DOEP5_CTL = crate::Reg<doep5_ctl::DOEP5_CTL_SPEC>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep5_ctl;
#[doc = "DOEP5_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_int`]
module"]
pub type DOEP5_INT = crate::Reg<doep5_int::DOEP5_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep5_int;
#[doc = "DOEP5_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_tsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_tsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_tsiz`]
module"]
pub type DOEP5_TSIZ = crate::Reg<doep5_tsiz::DOEP5_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "DOEP5_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep5_dmaaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep5_dmaaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_dmaaddr`]
module"]
pub type DOEP5_DMAADDR = crate::Reg<doep5_dmaaddr::DOEP5_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
