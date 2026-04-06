#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    route: Route,
    _reserved7: [u8; 0x10],
    cdconf: Cdconf,
    cmd: Cmd,
    dattrim1: Dattrim1,
    _reserved10: [u8; 0x04],
    lemctrl: Lemctrl,
    _reserved11: [u8; 0x000d_dfc0],
    gotgctl: Gotgctl,
    gotgint: Gotgint,
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    gintsts: Gintsts,
    gintmsk: Gintmsk,
    grxstsr: Grxstsr,
    grxstsp: Grxstsp,
    grxfsiz: Grxfsiz,
    gnptxfsiz: Gnptxfsiz,
    gnptxsts: Gnptxsts,
    _reserved23: [u8; 0x10],
    gsnpsid: Gsnpsid,
    _reserved24: [u8; 0x18],
    gdfifocfg: Gdfifocfg,
    _reserved25: [u8; 0xa0],
    hptxfsiz: Hptxfsiz,
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    dieptxf3: Dieptxf3,
    dieptxf4: Dieptxf4,
    dieptxf5: Dieptxf5,
    dieptxf6: Dieptxf6,
    _reserved32: [u8; 0x02e4],
    hcfg: Hcfg,
    hfir: Hfir,
    hfnum: Hfnum,
    _reserved35: [u8; 0x04],
    hptxsts: Hptxsts,
    haint: Haint,
    haintmsk: Haintmsk,
    _reserved38: [u8; 0x24],
    hprt: Hprt,
    _reserved39: [u8; 0xbc],
    hc0_char: Hc0Char,
    hc0_splt: Hc0Splt,
    hc0_int: Hc0Int,
    hc0_intmsk: Hc0Intmsk,
    hc0_tsiz: Hc0Tsiz,
    hc0_dmaaddr: Hc0Dmaaddr,
    _reserved45: [u8; 0x08],
    hc1_char: Hc1Char,
    hc1_splt: Hc1Splt,
    hc1_int: Hc1Int,
    hc1_intmsk: Hc1Intmsk,
    hc1_tsiz: Hc1Tsiz,
    hc1_dmaaddr: Hc1Dmaaddr,
    _reserved51: [u8; 0x08],
    hc2_char: Hc2Char,
    hc2_splt: Hc2Splt,
    hc2_int: Hc2Int,
    hc2_intmsk: Hc2Intmsk,
    hc2_tsiz: Hc2Tsiz,
    hc2_dmaaddr: Hc2Dmaaddr,
    _reserved57: [u8; 0x08],
    hc3_char: Hc3Char,
    hc3_splt: Hc3Splt,
    hc3_int: Hc3Int,
    hc3_intmsk: Hc3Intmsk,
    hc3_tsiz: Hc3Tsiz,
    hc3_dmaaddr: Hc3Dmaaddr,
    _reserved63: [u8; 0x08],
    hc4_char: Hc4Char,
    hc4_splt: Hc4Splt,
    hc4_int: Hc4Int,
    hc4_intmsk: Hc4Intmsk,
    hc4_tsiz: Hc4Tsiz,
    hc4_dmaaddr: Hc4Dmaaddr,
    _reserved69: [u8; 0x08],
    hc5_char: Hc5Char,
    hc5_splt: Hc5Splt,
    hc5_int: Hc5Int,
    hc5_intmsk: Hc5Intmsk,
    hc5_tsiz: Hc5Tsiz,
    hc5_dmaaddr: Hc5Dmaaddr,
    _reserved75: [u8; 0x08],
    hc6_char: Hc6Char,
    hc6_splt: Hc6Splt,
    hc6_int: Hc6Int,
    hc6_intmsk: Hc6Intmsk,
    hc6_tsiz: Hc6Tsiz,
    hc6_dmaaddr: Hc6Dmaaddr,
    _reserved81: [u8; 0x08],
    hc7_char: Hc7Char,
    hc7_splt: Hc7Splt,
    hc7_int: Hc7Int,
    hc7_intmsk: Hc7Intmsk,
    hc7_tsiz: Hc7Tsiz,
    hc7_dmaaddr: Hc7Dmaaddr,
    _reserved87: [u8; 0x08],
    hc8_char: Hc8Char,
    hc8_splt: Hc8Splt,
    hc8_int: Hc8Int,
    hc8_intmsk: Hc8Intmsk,
    hc8_tsiz: Hc8Tsiz,
    hc8_dmaaddr: Hc8Dmaaddr,
    _reserved93: [u8; 0x08],
    hc9_char: Hc9Char,
    hc9_splt: Hc9Splt,
    hc9_int: Hc9Int,
    hc9_intmsk: Hc9Intmsk,
    hc9_tsiz: Hc9Tsiz,
    hc9_dmaaddr: Hc9Dmaaddr,
    _reserved99: [u8; 0x08],
    hc10_char: Hc10Char,
    hc10_splt: Hc10Splt,
    hc10_int: Hc10Int,
    hc10_intmsk: Hc10Intmsk,
    hc10_tsiz: Hc10Tsiz,
    hc10_dmaaddr: Hc10Dmaaddr,
    _reserved105: [u8; 0x08],
    hc11_char: Hc11Char,
    hc11_splt: Hc11Splt,
    hc11_int: Hc11Int,
    hc11_intmsk: Hc11Intmsk,
    hc11_tsiz: Hc11Tsiz,
    hc11_dmaaddr: Hc11Dmaaddr,
    _reserved111: [u8; 0x08],
    hc12_char: Hc12Char,
    hc12_splt: Hc12Splt,
    hc12_int: Hc12Int,
    hc12_intmsk: Hc12Intmsk,
    hc12_tsiz: Hc12Tsiz,
    hc12_dmaaddr: Hc12Dmaaddr,
    _reserved117: [u8; 0x08],
    hc13_char: Hc13Char,
    hc13_splt: Hc13Splt,
    hc13_int: Hc13Int,
    hc13_intmsk: Hc13Intmsk,
    hc13_tsiz: Hc13Tsiz,
    hc13_dmaaddr: Hc13Dmaaddr,
    _reserved123: [u8; 0x0148],
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved126: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved130: [u8; 0x08],
    dvbusdis: Dvbusdis,
    dvbuspulse: Dvbuspulse,
    dthrctl: Dthrctl,
    diepempmsk: Diepempmsk,
    _reserved134: [u8; 0xc8],
    diep0ctl: Diep0ctl,
    _reserved135: [u8; 0x04],
    diep0int: Diep0int,
    _reserved136: [u8; 0x04],
    diep0tsiz: Diep0tsiz,
    diep0dmaaddr: Diep0dmaaddr,
    diep0txfsts: Diep0txfsts,
    _reserved139: [u8; 0x04],
    diep0_ctl: Diep0Ctl,
    _reserved140: [u8; 0x04],
    diep0_int: Diep0Int,
    _reserved141: [u8; 0x04],
    diep0_tsiz: Diep0Tsiz,
    diep0_dmaaddr: Diep0Dmaaddr,
    diep0_dtxfsts: Diep0Dtxfsts,
    _reserved144: [u8; 0x04],
    diep1_ctl: Diep1Ctl,
    _reserved145: [u8; 0x04],
    diep1_int: Diep1Int,
    _reserved146: [u8; 0x04],
    diep1_tsiz: Diep1Tsiz,
    diep1_dmaaddr: Diep1Dmaaddr,
    diep1_dtxfsts: Diep1Dtxfsts,
    _reserved149: [u8; 0x04],
    diep2_ctl: Diep2Ctl,
    _reserved150: [u8; 0x04],
    diep2_int: Diep2Int,
    _reserved151: [u8; 0x04],
    diep2_tsiz: Diep2Tsiz,
    diep2_dmaaddr: Diep2Dmaaddr,
    diep2_dtxfsts: Diep2Dtxfsts,
    _reserved154: [u8; 0x04],
    diep3_ctl: Diep3Ctl,
    _reserved155: [u8; 0x04],
    diep3_int: Diep3Int,
    _reserved156: [u8; 0x04],
    diep3_tsiz: Diep3Tsiz,
    diep3_dmaaddr: Diep3Dmaaddr,
    diep3_dtxfsts: Diep3Dtxfsts,
    _reserved159: [u8; 0x04],
    diep4_ctl: Diep4Ctl,
    _reserved160: [u8; 0x04],
    diep4_int: Diep4Int,
    _reserved161: [u8; 0x04],
    diep4_tsiz: Diep4Tsiz,
    diep4_dmaaddr: Diep4Dmaaddr,
    diep4_dtxfsts: Diep4Dtxfsts,
    _reserved164: [u8; 0x04],
    diep5_ctl: Diep5Ctl,
    _reserved165: [u8; 0x04],
    diep5_int: Diep5Int,
    _reserved166: [u8; 0x04],
    diep5_tsiz: Diep5Tsiz,
    diep5_dmaaddr: Diep5Dmaaddr,
    diep5_dtxfsts: Diep5Dtxfsts,
    _reserved169: [u8; 0x0124],
    doep0ctl: Doep0ctl,
    _reserved170: [u8; 0x04],
    doep0int: Doep0int,
    _reserved171: [u8; 0x04],
    doep0tsiz: Doep0tsiz,
    doep0dmaaddr: Doep0dmaaddr,
    _reserved173: [u8; 0x08],
    doep0_ctl: Doep0Ctl,
    _reserved174: [u8; 0x04],
    doep0_int: Doep0Int,
    _reserved175: [u8; 0x04],
    doep0_tsiz: Doep0Tsiz,
    doep0_dmaaddr: Doep0Dmaaddr,
    _reserved177: [u8; 0x08],
    doep1_ctl: Doep1Ctl,
    _reserved178: [u8; 0x04],
    doep1_int: Doep1Int,
    _reserved179: [u8; 0x04],
    doep1_tsiz: Doep1Tsiz,
    doep1_dmaaddr: Doep1Dmaaddr,
    _reserved181: [u8; 0x08],
    doep2_ctl: Doep2Ctl,
    _reserved182: [u8; 0x04],
    doep2_int: Doep2Int,
    _reserved183: [u8; 0x04],
    doep2_tsiz: Doep2Tsiz,
    doep2_dmaaddr: Doep2Dmaaddr,
    _reserved185: [u8; 0x08],
    doep3_ctl: Doep3Ctl,
    _reserved186: [u8; 0x04],
    doep3_int: Doep3Int,
    _reserved187: [u8; 0x04],
    doep3_tsiz: Doep3Tsiz,
    doep3_dmaaddr: Doep3Dmaaddr,
    _reserved189: [u8; 0x08],
    doep4_ctl: Doep4Ctl,
    _reserved190: [u8; 0x04],
    doep4_int: Doep4Int,
    _reserved191: [u8; 0x04],
    doep4_tsiz: Doep4Tsiz,
    doep4_dmaaddr: Doep4Dmaaddr,
    _reserved193: [u8; 0x08],
    doep5_ctl: Doep5Ctl,
    _reserved194: [u8; 0x04],
    doep5_int: Doep5Int,
    _reserved195: [u8; 0x04],
    doep5_tsiz: Doep5Tsiz,
    doep5_dmaaddr: Doep5Dmaaddr,
    _reserved197: [u8; 0x0228],
    pcgcctl: Pcgcctl,
}
impl RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - System Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x0c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x18 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x2c - Charger Detect Configuration Register"]
    #[inline(always)]
    pub const fn cdconf(&self) -> &Cdconf {
        &self.cdconf
    }
    #[doc = "0x30 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x34 - Data TRIM 1 Values for USB DP and DM"]
    #[inline(always)]
    pub const fn dattrim1(&self) -> &Dattrim1 {
        &self.dattrim1
    }
    #[doc = "0x3c - USB LEM Control Register"]
    #[inline(always)]
    pub const fn lemctrl(&self) -> &Lemctrl {
        &self.lemctrl
    }
    #[doc = "0xde000 - OTG Control and Status Register"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &Gotgctl {
        &self.gotgctl
    }
    #[doc = "0xde004 - OTG Interrupt Register"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &Gotgint {
        &self.gotgint
    }
    #[doc = "0xde008 - AHB Configuration Register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0xde00c - USB Configuration Register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0xde010 - Reset Register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0xde014 - Interrupt Register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &Gintsts {
        &self.gintsts
    }
    #[doc = "0xde018 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &Gintmsk {
        &self.gintmsk
    }
    #[doc = "0xde01c - Receive Status Debug Read Register"]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &Grxstsr {
        &self.grxstsr
    }
    #[doc = "0xde020 - Receive Status Read /Pop Register"]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &Grxstsp {
        &self.grxstsp
    }
    #[doc = "0xde024 - Receive FIFO Size Register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0xde028 - Non-periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &Gnptxfsiz {
        &self.gnptxfsiz
    }
    #[doc = "0xde02c - Non-periodic Transmit FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &Gnptxsts {
        &self.gnptxsts
    }
    #[doc = "0xde040 - Synopsys ID Register"]
    #[inline(always)]
    pub const fn gsnpsid(&self) -> &Gsnpsid {
        &self.gsnpsid
    }
    #[doc = "0xde05c - Global DFIFO Configuration Register"]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &Gdfifocfg {
        &self.gdfifocfg
    }
    #[doc = "0xde100 - Host Periodic Transmit FIFO Size Register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &Hptxfsiz {
        &self.hptxfsiz
    }
    #[doc = "0xde104 - Device IN Endpoint Transmit FIFO Size Register 1"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0xde108 - Device IN Endpoint Transmit FIFO Size Register 2"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0xde10c - Device IN Endpoint Transmit FIFO Size Register 3"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0xde110 - Device IN Endpoint Transmit FIFO Size Register 4"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &Dieptxf4 {
        &self.dieptxf4
    }
    #[doc = "0xde114 - Device IN Endpoint Transmit FIFO Size Register 5"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &Dieptxf5 {
        &self.dieptxf5
    }
    #[doc = "0xde118 - Device IN Endpoint Transmit FIFO Size Register 6"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &Dieptxf6 {
        &self.dieptxf6
    }
    #[doc = "0xde400 - Host Configuration Register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &Hcfg {
        &self.hcfg
    }
    #[doc = "0xde404 - Host Frame Interval Register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &Hfir {
        &self.hfir
    }
    #[doc = "0xde408 - Host Frame Number/Frame Time Remaining Register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &Hfnum {
        &self.hfnum
    }
    #[doc = "0xde410 - Host Periodic Transmit FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &Hptxsts {
        &self.hptxsts
    }
    #[doc = "0xde414 - Host All Channels Interrupt Register"]
    #[inline(always)]
    pub const fn haint(&self) -> &Haint {
        &self.haint
    }
    #[doc = "0xde418 - Host All Channels Interrupt Mask Register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &Haintmsk {
        &self.haintmsk
    }
    #[doc = "0xde440 - Host Port Control and Status Register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &Hprt {
        &self.hprt
    }
    #[doc = "0xde500 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc0_char(&self) -> &Hc0Char {
        &self.hc0_char
    }
    #[doc = "0xde504 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc0_splt(&self) -> &Hc0Splt {
        &self.hc0_splt
    }
    #[doc = "0xde508 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc0_int(&self) -> &Hc0Int {
        &self.hc0_int
    }
    #[doc = "0xde50c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc0_intmsk(&self) -> &Hc0Intmsk {
        &self.hc0_intmsk
    }
    #[doc = "0xde510 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc0_tsiz(&self) -> &Hc0Tsiz {
        &self.hc0_tsiz
    }
    #[doc = "0xde514 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc0_dmaaddr(&self) -> &Hc0Dmaaddr {
        &self.hc0_dmaaddr
    }
    #[doc = "0xde520 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc1_char(&self) -> &Hc1Char {
        &self.hc1_char
    }
    #[doc = "0xde524 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc1_splt(&self) -> &Hc1Splt {
        &self.hc1_splt
    }
    #[doc = "0xde528 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc1_int(&self) -> &Hc1Int {
        &self.hc1_int
    }
    #[doc = "0xde52c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc1_intmsk(&self) -> &Hc1Intmsk {
        &self.hc1_intmsk
    }
    #[doc = "0xde530 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc1_tsiz(&self) -> &Hc1Tsiz {
        &self.hc1_tsiz
    }
    #[doc = "0xde534 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc1_dmaaddr(&self) -> &Hc1Dmaaddr {
        &self.hc1_dmaaddr
    }
    #[doc = "0xde540 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc2_char(&self) -> &Hc2Char {
        &self.hc2_char
    }
    #[doc = "0xde544 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc2_splt(&self) -> &Hc2Splt {
        &self.hc2_splt
    }
    #[doc = "0xde548 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc2_int(&self) -> &Hc2Int {
        &self.hc2_int
    }
    #[doc = "0xde54c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc2_intmsk(&self) -> &Hc2Intmsk {
        &self.hc2_intmsk
    }
    #[doc = "0xde550 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc2_tsiz(&self) -> &Hc2Tsiz {
        &self.hc2_tsiz
    }
    #[doc = "0xde554 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc2_dmaaddr(&self) -> &Hc2Dmaaddr {
        &self.hc2_dmaaddr
    }
    #[doc = "0xde560 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc3_char(&self) -> &Hc3Char {
        &self.hc3_char
    }
    #[doc = "0xde564 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc3_splt(&self) -> &Hc3Splt {
        &self.hc3_splt
    }
    #[doc = "0xde568 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc3_int(&self) -> &Hc3Int {
        &self.hc3_int
    }
    #[doc = "0xde56c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc3_intmsk(&self) -> &Hc3Intmsk {
        &self.hc3_intmsk
    }
    #[doc = "0xde570 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc3_tsiz(&self) -> &Hc3Tsiz {
        &self.hc3_tsiz
    }
    #[doc = "0xde574 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc3_dmaaddr(&self) -> &Hc3Dmaaddr {
        &self.hc3_dmaaddr
    }
    #[doc = "0xde580 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc4_char(&self) -> &Hc4Char {
        &self.hc4_char
    }
    #[doc = "0xde584 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc4_splt(&self) -> &Hc4Splt {
        &self.hc4_splt
    }
    #[doc = "0xde588 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc4_int(&self) -> &Hc4Int {
        &self.hc4_int
    }
    #[doc = "0xde58c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc4_intmsk(&self) -> &Hc4Intmsk {
        &self.hc4_intmsk
    }
    #[doc = "0xde590 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc4_tsiz(&self) -> &Hc4Tsiz {
        &self.hc4_tsiz
    }
    #[doc = "0xde594 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc4_dmaaddr(&self) -> &Hc4Dmaaddr {
        &self.hc4_dmaaddr
    }
    #[doc = "0xde5a0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc5_char(&self) -> &Hc5Char {
        &self.hc5_char
    }
    #[doc = "0xde5a4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc5_splt(&self) -> &Hc5Splt {
        &self.hc5_splt
    }
    #[doc = "0xde5a8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc5_int(&self) -> &Hc5Int {
        &self.hc5_int
    }
    #[doc = "0xde5ac - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc5_intmsk(&self) -> &Hc5Intmsk {
        &self.hc5_intmsk
    }
    #[doc = "0xde5b0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc5_tsiz(&self) -> &Hc5Tsiz {
        &self.hc5_tsiz
    }
    #[doc = "0xde5b4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc5_dmaaddr(&self) -> &Hc5Dmaaddr {
        &self.hc5_dmaaddr
    }
    #[doc = "0xde5c0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc6_char(&self) -> &Hc6Char {
        &self.hc6_char
    }
    #[doc = "0xde5c4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc6_splt(&self) -> &Hc6Splt {
        &self.hc6_splt
    }
    #[doc = "0xde5c8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc6_int(&self) -> &Hc6Int {
        &self.hc6_int
    }
    #[doc = "0xde5cc - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc6_intmsk(&self) -> &Hc6Intmsk {
        &self.hc6_intmsk
    }
    #[doc = "0xde5d0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc6_tsiz(&self) -> &Hc6Tsiz {
        &self.hc6_tsiz
    }
    #[doc = "0xde5d4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc6_dmaaddr(&self) -> &Hc6Dmaaddr {
        &self.hc6_dmaaddr
    }
    #[doc = "0xde5e0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc7_char(&self) -> &Hc7Char {
        &self.hc7_char
    }
    #[doc = "0xde5e4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc7_splt(&self) -> &Hc7Splt {
        &self.hc7_splt
    }
    #[doc = "0xde5e8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc7_int(&self) -> &Hc7Int {
        &self.hc7_int
    }
    #[doc = "0xde5ec - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc7_intmsk(&self) -> &Hc7Intmsk {
        &self.hc7_intmsk
    }
    #[doc = "0xde5f0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc7_tsiz(&self) -> &Hc7Tsiz {
        &self.hc7_tsiz
    }
    #[doc = "0xde5f4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc7_dmaaddr(&self) -> &Hc7Dmaaddr {
        &self.hc7_dmaaddr
    }
    #[doc = "0xde600 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc8_char(&self) -> &Hc8Char {
        &self.hc8_char
    }
    #[doc = "0xde604 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc8_splt(&self) -> &Hc8Splt {
        &self.hc8_splt
    }
    #[doc = "0xde608 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc8_int(&self) -> &Hc8Int {
        &self.hc8_int
    }
    #[doc = "0xde60c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc8_intmsk(&self) -> &Hc8Intmsk {
        &self.hc8_intmsk
    }
    #[doc = "0xde610 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc8_tsiz(&self) -> &Hc8Tsiz {
        &self.hc8_tsiz
    }
    #[doc = "0xde614 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc8_dmaaddr(&self) -> &Hc8Dmaaddr {
        &self.hc8_dmaaddr
    }
    #[doc = "0xde620 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc9_char(&self) -> &Hc9Char {
        &self.hc9_char
    }
    #[doc = "0xde624 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc9_splt(&self) -> &Hc9Splt {
        &self.hc9_splt
    }
    #[doc = "0xde628 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc9_int(&self) -> &Hc9Int {
        &self.hc9_int
    }
    #[doc = "0xde62c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc9_intmsk(&self) -> &Hc9Intmsk {
        &self.hc9_intmsk
    }
    #[doc = "0xde630 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc9_tsiz(&self) -> &Hc9Tsiz {
        &self.hc9_tsiz
    }
    #[doc = "0xde634 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc9_dmaaddr(&self) -> &Hc9Dmaaddr {
        &self.hc9_dmaaddr
    }
    #[doc = "0xde640 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc10_char(&self) -> &Hc10Char {
        &self.hc10_char
    }
    #[doc = "0xde644 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc10_splt(&self) -> &Hc10Splt {
        &self.hc10_splt
    }
    #[doc = "0xde648 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc10_int(&self) -> &Hc10Int {
        &self.hc10_int
    }
    #[doc = "0xde64c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc10_intmsk(&self) -> &Hc10Intmsk {
        &self.hc10_intmsk
    }
    #[doc = "0xde650 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc10_tsiz(&self) -> &Hc10Tsiz {
        &self.hc10_tsiz
    }
    #[doc = "0xde654 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc10_dmaaddr(&self) -> &Hc10Dmaaddr {
        &self.hc10_dmaaddr
    }
    #[doc = "0xde660 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc11_char(&self) -> &Hc11Char {
        &self.hc11_char
    }
    #[doc = "0xde664 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc11_splt(&self) -> &Hc11Splt {
        &self.hc11_splt
    }
    #[doc = "0xde668 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc11_int(&self) -> &Hc11Int {
        &self.hc11_int
    }
    #[doc = "0xde66c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc11_intmsk(&self) -> &Hc11Intmsk {
        &self.hc11_intmsk
    }
    #[doc = "0xde670 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc11_tsiz(&self) -> &Hc11Tsiz {
        &self.hc11_tsiz
    }
    #[doc = "0xde674 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc11_dmaaddr(&self) -> &Hc11Dmaaddr {
        &self.hc11_dmaaddr
    }
    #[doc = "0xde680 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc12_char(&self) -> &Hc12Char {
        &self.hc12_char
    }
    #[doc = "0xde684 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc12_splt(&self) -> &Hc12Splt {
        &self.hc12_splt
    }
    #[doc = "0xde688 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc12_int(&self) -> &Hc12Int {
        &self.hc12_int
    }
    #[doc = "0xde68c - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc12_intmsk(&self) -> &Hc12Intmsk {
        &self.hc12_intmsk
    }
    #[doc = "0xde690 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc12_tsiz(&self) -> &Hc12Tsiz {
        &self.hc12_tsiz
    }
    #[doc = "0xde694 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc12_dmaaddr(&self) -> &Hc12Dmaaddr {
        &self.hc12_dmaaddr
    }
    #[doc = "0xde6a0 - Host Channel x Characteristics Register"]
    #[inline(always)]
    pub const fn hc13_char(&self) -> &Hc13Char {
        &self.hc13_char
    }
    #[doc = "0xde6a4 - Host Channel x Split Control Register"]
    #[inline(always)]
    pub const fn hc13_splt(&self) -> &Hc13Splt {
        &self.hc13_splt
    }
    #[doc = "0xde6a8 - Host Channel x Interrupt Register"]
    #[inline(always)]
    pub const fn hc13_int(&self) -> &Hc13Int {
        &self.hc13_int
    }
    #[doc = "0xde6ac - Host Channel x Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hc13_intmsk(&self) -> &Hc13Intmsk {
        &self.hc13_intmsk
    }
    #[doc = "0xde6b0 - Host Channel x Transfer Size Register"]
    #[inline(always)]
    pub const fn hc13_tsiz(&self) -> &Hc13Tsiz {
        &self.hc13_tsiz
    }
    #[doc = "0xde6b4 - Host Channel x DMA Address Register"]
    #[inline(always)]
    pub const fn hc13_dmaaddr(&self) -> &Hc13Dmaaddr {
        &self.hc13_dmaaddr
    }
    #[doc = "0xde800 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0xde804 - Device Control Register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0xde808 - Device Status Register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0xde810 - Device IN Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0xde814 - Device OUT Endpoint Common Interrupt Mask Register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0xde818 - Device All Endpoints Interrupt Register"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0xde81c - Device All Endpoints Interrupt Mask Register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0xde828 - Device VBUS Discharge Time Register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &Dvbusdis {
        &self.dvbusdis
    }
    #[doc = "0xde82c - Device VBUS Pulsing Time Register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &Dvbuspulse {
        &self.dvbuspulse
    }
    #[doc = "0xde830 - Device Threshold Control Register"]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &Dthrctl {
        &self.dthrctl
    }
    #[doc = "0xde834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0xde900 - Device Control IN Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn diep0ctl(&self) -> &Diep0ctl {
        &self.diep0ctl
    }
    #[doc = "0xde908 - Device IN Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0int(&self) -> &Diep0int {
        &self.diep0int
    }
    #[doc = "0xde910 - Device IN Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0tsiz(&self) -> &Diep0tsiz {
        &self.diep0tsiz
    }
    #[doc = "0xde914 - Device IN Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0dmaaddr(&self) -> &Diep0dmaaddr {
        &self.diep0dmaaddr
    }
    #[doc = "0xde918 - Device IN Endpoint Transmit FIFO Status Register 0"]
    #[inline(always)]
    pub const fn diep0txfsts(&self) -> &Diep0txfsts {
        &self.diep0txfsts
    }
    #[doc = "0xde920 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep0_ctl(&self) -> &Diep0Ctl {
        &self.diep0_ctl
    }
    #[doc = "0xde928 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep0_int(&self) -> &Diep0Int {
        &self.diep0_int
    }
    #[doc = "0xde930 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep0_tsiz(&self) -> &Diep0Tsiz {
        &self.diep0_tsiz
    }
    #[doc = "0xde934 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep0_dmaaddr(&self) -> &Diep0Dmaaddr {
        &self.diep0_dmaaddr
    }
    #[doc = "0xde938 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep0_dtxfsts(&self) -> &Diep0Dtxfsts {
        &self.diep0_dtxfsts
    }
    #[doc = "0xde940 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep1_ctl(&self) -> &Diep1Ctl {
        &self.diep1_ctl
    }
    #[doc = "0xde948 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep1_int(&self) -> &Diep1Int {
        &self.diep1_int
    }
    #[doc = "0xde950 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep1_tsiz(&self) -> &Diep1Tsiz {
        &self.diep1_tsiz
    }
    #[doc = "0xde954 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep1_dmaaddr(&self) -> &Diep1Dmaaddr {
        &self.diep1_dmaaddr
    }
    #[doc = "0xde958 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep1_dtxfsts(&self) -> &Diep1Dtxfsts {
        &self.diep1_dtxfsts
    }
    #[doc = "0xde960 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep2_ctl(&self) -> &Diep2Ctl {
        &self.diep2_ctl
    }
    #[doc = "0xde968 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep2_int(&self) -> &Diep2Int {
        &self.diep2_int
    }
    #[doc = "0xde970 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep2_tsiz(&self) -> &Diep2Tsiz {
        &self.diep2_tsiz
    }
    #[doc = "0xde974 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep2_dmaaddr(&self) -> &Diep2Dmaaddr {
        &self.diep2_dmaaddr
    }
    #[doc = "0xde978 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep2_dtxfsts(&self) -> &Diep2Dtxfsts {
        &self.diep2_dtxfsts
    }
    #[doc = "0xde980 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep3_ctl(&self) -> &Diep3Ctl {
        &self.diep3_ctl
    }
    #[doc = "0xde988 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep3_int(&self) -> &Diep3Int {
        &self.diep3_int
    }
    #[doc = "0xde990 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep3_tsiz(&self) -> &Diep3Tsiz {
        &self.diep3_tsiz
    }
    #[doc = "0xde994 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep3_dmaaddr(&self) -> &Diep3Dmaaddr {
        &self.diep3_dmaaddr
    }
    #[doc = "0xde998 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep3_dtxfsts(&self) -> &Diep3Dtxfsts {
        &self.diep3_dtxfsts
    }
    #[doc = "0xde9a0 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep4_ctl(&self) -> &Diep4Ctl {
        &self.diep4_ctl
    }
    #[doc = "0xde9a8 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep4_int(&self) -> &Diep4Int {
        &self.diep4_int
    }
    #[doc = "0xde9b0 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep4_tsiz(&self) -> &Diep4Tsiz {
        &self.diep4_tsiz
    }
    #[doc = "0xde9b4 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep4_dmaaddr(&self) -> &Diep4Dmaaddr {
        &self.diep4_dmaaddr
    }
    #[doc = "0xde9b8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep4_dtxfsts(&self) -> &Diep4Dtxfsts {
        &self.diep4_dtxfsts
    }
    #[doc = "0xde9c0 - Device Control IN Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn diep5_ctl(&self) -> &Diep5Ctl {
        &self.diep5_ctl
    }
    #[doc = "0xde9c8 - Device IN Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn diep5_int(&self) -> &Diep5Int {
        &self.diep5_int
    }
    #[doc = "0xde9d0 - Device IN Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn diep5_tsiz(&self) -> &Diep5Tsiz {
        &self.diep5_tsiz
    }
    #[doc = "0xde9d4 - Device IN Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn diep5_dmaaddr(&self) -> &Diep5Dmaaddr {
        &self.diep5_dmaaddr
    }
    #[doc = "0xde9d8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    #[inline(always)]
    pub const fn diep5_dtxfsts(&self) -> &Diep5Dtxfsts {
        &self.diep5_dtxfsts
    }
    #[doc = "0xdeb00 - Device Control OUT Endpoint 0 Control Register"]
    #[inline(always)]
    pub const fn doep0ctl(&self) -> &Doep0ctl {
        &self.doep0ctl
    }
    #[doc = "0xdeb08 - Device OUT Endpoint 0 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0int(&self) -> &Doep0int {
        &self.doep0int
    }
    #[doc = "0xdeb10 - Device OUT Endpoint 0 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0tsiz(&self) -> &Doep0tsiz {
        &self.doep0tsiz
    }
    #[doc = "0xdeb14 - Device OUT Endpoint 0 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0dmaaddr(&self) -> &Doep0dmaaddr {
        &self.doep0dmaaddr
    }
    #[doc = "0xdeb20 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep0_ctl(&self) -> &Doep0Ctl {
        &self.doep0_ctl
    }
    #[doc = "0xdeb28 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep0_int(&self) -> &Doep0Int {
        &self.doep0_int
    }
    #[doc = "0xdeb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep0_tsiz(&self) -> &Doep0Tsiz {
        &self.doep0_tsiz
    }
    #[doc = "0xdeb34 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep0_dmaaddr(&self) -> &Doep0Dmaaddr {
        &self.doep0_dmaaddr
    }
    #[doc = "0xdeb40 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep1_ctl(&self) -> &Doep1Ctl {
        &self.doep1_ctl
    }
    #[doc = "0xdeb48 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep1_int(&self) -> &Doep1Int {
        &self.doep1_int
    }
    #[doc = "0xdeb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep1_tsiz(&self) -> &Doep1Tsiz {
        &self.doep1_tsiz
    }
    #[doc = "0xdeb54 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep1_dmaaddr(&self) -> &Doep1Dmaaddr {
        &self.doep1_dmaaddr
    }
    #[doc = "0xdeb60 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep2_ctl(&self) -> &Doep2Ctl {
        &self.doep2_ctl
    }
    #[doc = "0xdeb68 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep2_int(&self) -> &Doep2Int {
        &self.doep2_int
    }
    #[doc = "0xdeb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep2_tsiz(&self) -> &Doep2Tsiz {
        &self.doep2_tsiz
    }
    #[doc = "0xdeb74 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep2_dmaaddr(&self) -> &Doep2Dmaaddr {
        &self.doep2_dmaaddr
    }
    #[doc = "0xdeb80 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep3_ctl(&self) -> &Doep3Ctl {
        &self.doep3_ctl
    }
    #[doc = "0xdeb88 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep3_int(&self) -> &Doep3Int {
        &self.doep3_int
    }
    #[doc = "0xdeb90 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep3_tsiz(&self) -> &Doep3Tsiz {
        &self.doep3_tsiz
    }
    #[doc = "0xdeb94 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep3_dmaaddr(&self) -> &Doep3Dmaaddr {
        &self.doep3_dmaaddr
    }
    #[doc = "0xdeba0 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep4_ctl(&self) -> &Doep4Ctl {
        &self.doep4_ctl
    }
    #[doc = "0xdeba8 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep4_int(&self) -> &Doep4Int {
        &self.doep4_int
    }
    #[doc = "0xdebb0 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep4_tsiz(&self) -> &Doep4Tsiz {
        &self.doep4_tsiz
    }
    #[doc = "0xdebb4 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep4_dmaaddr(&self) -> &Doep4Dmaaddr {
        &self.doep4_dmaaddr
    }
    #[doc = "0xdebc0 - Device Control OUT Endpoint x+1 Control Register"]
    #[inline(always)]
    pub const fn doep5_ctl(&self) -> &Doep5Ctl {
        &self.doep5_ctl
    }
    #[doc = "0xdebc8 - Device OUT Endpoint x+1 Interrupt Register"]
    #[inline(always)]
    pub const fn doep5_int(&self) -> &Doep5Int {
        &self.doep5_int
    }
    #[doc = "0xdebd0 - Device OUT Endpoint x+1 Transfer Size Register"]
    #[inline(always)]
    pub const fn doep5_tsiz(&self) -> &Doep5Tsiz {
        &self.doep5_tsiz
    }
    #[doc = "0xdebd4 - Device OUT Endpoint x+1 DMA Address Register"]
    #[inline(always)]
    pub const fn doep5_dmaaddr(&self) -> &Doep5Dmaaddr {
        &self.doep5_dmaaddr
    }
    #[doc = "0xdee00 - Power and Clock Gating Control Register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &Pcgcctl {
        &self.pcgcctl
    }
}
#[doc = "CTRL (rw) register accessor: System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "System Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`] module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IfsSpec>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`] module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IfcSpec>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CDCONF (rw) register accessor: Charger Detect Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdconf`] module"]
#[doc(alias = "CDCONF")]
pub type Cdconf = crate::Reg<cdconf::CdconfSpec>;
#[doc = "Charger Detect Configuration Register"]
pub mod cdconf;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "DATTRIM1 (rw) register accessor: Data TRIM 1 Values for USB DP and DM\n\nYou can [`read`](crate::Reg::read) this register and get [`dattrim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dattrim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dattrim1`] module"]
#[doc(alias = "DATTRIM1")]
pub type Dattrim1 = crate::Reg<dattrim1::Dattrim1Spec>;
#[doc = "Data TRIM 1 Values for USB DP and DM"]
pub mod dattrim1;
#[doc = "LEMCTRL (rw) register accessor: USB LEM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lemctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lemctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lemctrl`] module"]
#[doc(alias = "LEMCTRL")]
pub type Lemctrl = crate::Reg<lemctrl::LemctrlSpec>;
#[doc = "USB LEM Control Register"]
pub mod lemctrl;
#[doc = "GOTGCTL (rw) register accessor: OTG Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`] module"]
#[doc(alias = "GOTGCTL")]
pub type Gotgctl = crate::Reg<gotgctl::GotgctlSpec>;
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`] module"]
#[doc(alias = "GOTGINT")]
pub type Gotgint = crate::Reg<gotgint::GotgintSpec>;
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: AHB Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`] module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`] module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`] module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`] module"]
#[doc(alias = "GINTSTS")]
pub type Gintsts = crate::Reg<gintsts::GintstsSpec>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`] module"]
#[doc(alias = "GINTMSK")]
pub type Gintmsk = crate::Reg<gintmsk::GintmskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: Receive Status Debug Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`] module"]
#[doc(alias = "GRXSTSR")]
pub type Grxstsr = crate::Reg<grxstsr::GrxstsrSpec>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: Receive Status Read /Pop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`] module"]
#[doc(alias = "GRXSTSP")]
pub type Grxstsp = crate::Reg<grxstsp::GrxstspSpec>;
#[doc = "Receive Status Read /Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: Receive FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`] module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`] module"]
#[doc(alias = "GNPTXFSIZ")]
pub type Gnptxfsiz = crate::Reg<gnptxfsiz::GnptxfsizSpec>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: Non-periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`] module"]
#[doc(alias = "GNPTXSTS")]
pub type Gnptxsts = crate::Reg<gnptxsts::GnptxstsSpec>;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "GSNPSID (r) register accessor: Synopsys ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gsnpsid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsnpsid`] module"]
#[doc(alias = "GSNPSID")]
pub type Gsnpsid = crate::Reg<gsnpsid::GsnpsidSpec>;
#[doc = "Synopsys ID Register"]
pub mod gsnpsid;
#[doc = "GDFIFOCFG (rw) register accessor: Global DFIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdfifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`] module"]
#[doc(alias = "GDFIFOCFG")]
pub type Gdfifocfg = crate::Reg<gdfifocfg::GdfifocfgSpec>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "HPTXFSIZ (rw) register accessor: Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`] module"]
#[doc(alias = "HPTXFSIZ")]
pub type Hptxfsiz = crate::Reg<hptxfsiz::HptxfsizSpec>;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`] module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`] module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`] module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`] module"]
#[doc(alias = "DIEPTXF4")]
pub type Dieptxf4 = crate::Reg<dieptxf4::Dieptxf4Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`] module"]
#[doc(alias = "DIEPTXF5")]
pub type Dieptxf5 = crate::Reg<dieptxf5::Dieptxf5Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: Device IN Endpoint Transmit FIFO Size Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`] module"]
#[doc(alias = "DIEPTXF6")]
pub type Dieptxf6 = crate::Reg<dieptxf6::Dieptxf6Spec>;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6"]
pub mod dieptxf6;
#[doc = "HCFG (rw) register accessor: Host Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`] module"]
#[doc(alias = "HCFG")]
pub type Hcfg = crate::Reg<hcfg::HcfgSpec>;
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: Host Frame Interval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`] module"]
#[doc(alias = "HFIR")]
pub type Hfir = crate::Reg<hfir::HfirSpec>;
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`] module"]
#[doc(alias = "HFNUM")]
pub type Hfnum = crate::Reg<hfnum::HfnumSpec>;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: Host Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`] module"]
#[doc(alias = "HPTXSTS")]
pub type Hptxsts = crate::Reg<hptxsts::HptxstsSpec>;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: Host All Channels Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`] module"]
#[doc(alias = "HAINT")]
pub type Haint = crate::Reg<haint::HaintSpec>;
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`] module"]
#[doc(alias = "HAINTMSK")]
pub type Haintmsk = crate::Reg<haintmsk::HaintmskSpec>;
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: Host Port Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`] module"]
#[doc(alias = "HPRT")]
pub type Hprt = crate::Reg<hprt::HprtSpec>;
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "HC0_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc0_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc0_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_char`] module"]
#[doc(alias = "HC0_CHAR")]
pub type Hc0Char = crate::Reg<hc0_char::Hc0CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc0_char;
#[doc = "HC0_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc0_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc0_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_splt`] module"]
#[doc(alias = "HC0_SPLT")]
pub type Hc0Splt = crate::Reg<hc0_splt::Hc0SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc0_splt;
#[doc = "HC0_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc0_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc0_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_int`] module"]
#[doc(alias = "HC0_INT")]
pub type Hc0Int = crate::Reg<hc0_int::Hc0IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc0_int;
#[doc = "HC0_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc0_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc0_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_intmsk`] module"]
#[doc(alias = "HC0_INTMSK")]
pub type Hc0Intmsk = crate::Reg<hc0_intmsk::Hc0IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "HC0_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc0_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc0_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_tsiz`] module"]
#[doc(alias = "HC0_TSIZ")]
pub type Hc0Tsiz = crate::Reg<hc0_tsiz::Hc0TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "HC0_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc0_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc0_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc0_dmaaddr`] module"]
#[doc(alias = "HC0_DMAADDR")]
pub type Hc0Dmaaddr = crate::Reg<hc0_dmaaddr::Hc0DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "HC1_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_char`] module"]
#[doc(alias = "HC1_CHAR")]
pub type Hc1Char = crate::Reg<hc1_char::Hc1CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc1_char;
#[doc = "HC1_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_splt`] module"]
#[doc(alias = "HC1_SPLT")]
pub type Hc1Splt = crate::Reg<hc1_splt::Hc1SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc1_splt;
#[doc = "HC1_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_int`] module"]
#[doc(alias = "HC1_INT")]
pub type Hc1Int = crate::Reg<hc1_int::Hc1IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc1_int;
#[doc = "HC1_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_intmsk`] module"]
#[doc(alias = "HC1_INTMSK")]
pub type Hc1Intmsk = crate::Reg<hc1_intmsk::Hc1IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "HC1_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_tsiz`] module"]
#[doc(alias = "HC1_TSIZ")]
pub type Hc1Tsiz = crate::Reg<hc1_tsiz::Hc1TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "HC1_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1_dmaaddr`] module"]
#[doc(alias = "HC1_DMAADDR")]
pub type Hc1Dmaaddr = crate::Reg<hc1_dmaaddr::Hc1DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "HC2_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_char`] module"]
#[doc(alias = "HC2_CHAR")]
pub type Hc2Char = crate::Reg<hc2_char::Hc2CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc2_char;
#[doc = "HC2_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_splt`] module"]
#[doc(alias = "HC2_SPLT")]
pub type Hc2Splt = crate::Reg<hc2_splt::Hc2SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc2_splt;
#[doc = "HC2_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_int`] module"]
#[doc(alias = "HC2_INT")]
pub type Hc2Int = crate::Reg<hc2_int::Hc2IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc2_int;
#[doc = "HC2_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_intmsk`] module"]
#[doc(alias = "HC2_INTMSK")]
pub type Hc2Intmsk = crate::Reg<hc2_intmsk::Hc2IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "HC2_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_tsiz`] module"]
#[doc(alias = "HC2_TSIZ")]
pub type Hc2Tsiz = crate::Reg<hc2_tsiz::Hc2TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "HC2_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2_dmaaddr`] module"]
#[doc(alias = "HC2_DMAADDR")]
pub type Hc2Dmaaddr = crate::Reg<hc2_dmaaddr::Hc2DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "HC3_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc3_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc3_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_char`] module"]
#[doc(alias = "HC3_CHAR")]
pub type Hc3Char = crate::Reg<hc3_char::Hc3CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc3_char;
#[doc = "HC3_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc3_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc3_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_splt`] module"]
#[doc(alias = "HC3_SPLT")]
pub type Hc3Splt = crate::Reg<hc3_splt::Hc3SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc3_splt;
#[doc = "HC3_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc3_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc3_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_int`] module"]
#[doc(alias = "HC3_INT")]
pub type Hc3Int = crate::Reg<hc3_int::Hc3IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc3_int;
#[doc = "HC3_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc3_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc3_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_intmsk`] module"]
#[doc(alias = "HC3_INTMSK")]
pub type Hc3Intmsk = crate::Reg<hc3_intmsk::Hc3IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "HC3_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc3_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc3_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_tsiz`] module"]
#[doc(alias = "HC3_TSIZ")]
pub type Hc3Tsiz = crate::Reg<hc3_tsiz::Hc3TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "HC3_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc3_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc3_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc3_dmaaddr`] module"]
#[doc(alias = "HC3_DMAADDR")]
pub type Hc3Dmaaddr = crate::Reg<hc3_dmaaddr::Hc3DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "HC4_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_char`] module"]
#[doc(alias = "HC4_CHAR")]
pub type Hc4Char = crate::Reg<hc4_char::Hc4CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc4_char;
#[doc = "HC4_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_splt`] module"]
#[doc(alias = "HC4_SPLT")]
pub type Hc4Splt = crate::Reg<hc4_splt::Hc4SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc4_splt;
#[doc = "HC4_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_int`] module"]
#[doc(alias = "HC4_INT")]
pub type Hc4Int = crate::Reg<hc4_int::Hc4IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc4_int;
#[doc = "HC4_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_intmsk`] module"]
#[doc(alias = "HC4_INTMSK")]
pub type Hc4Intmsk = crate::Reg<hc4_intmsk::Hc4IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "HC4_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_tsiz`] module"]
#[doc(alias = "HC4_TSIZ")]
pub type Hc4Tsiz = crate::Reg<hc4_tsiz::Hc4TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "HC4_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc4_dmaaddr`] module"]
#[doc(alias = "HC4_DMAADDR")]
pub type Hc4Dmaaddr = crate::Reg<hc4_dmaaddr::Hc4DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "HC5_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_char`] module"]
#[doc(alias = "HC5_CHAR")]
pub type Hc5Char = crate::Reg<hc5_char::Hc5CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc5_char;
#[doc = "HC5_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_splt`] module"]
#[doc(alias = "HC5_SPLT")]
pub type Hc5Splt = crate::Reg<hc5_splt::Hc5SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc5_splt;
#[doc = "HC5_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_int`] module"]
#[doc(alias = "HC5_INT")]
pub type Hc5Int = crate::Reg<hc5_int::Hc5IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc5_int;
#[doc = "HC5_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_intmsk`] module"]
#[doc(alias = "HC5_INTMSK")]
pub type Hc5Intmsk = crate::Reg<hc5_intmsk::Hc5IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "HC5_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_tsiz`] module"]
#[doc(alias = "HC5_TSIZ")]
pub type Hc5Tsiz = crate::Reg<hc5_tsiz::Hc5TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "HC5_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc5_dmaaddr`] module"]
#[doc(alias = "HC5_DMAADDR")]
pub type Hc5Dmaaddr = crate::Reg<hc5_dmaaddr::Hc5DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "HC6_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_char`] module"]
#[doc(alias = "HC6_CHAR")]
pub type Hc6Char = crate::Reg<hc6_char::Hc6CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc6_char;
#[doc = "HC6_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_splt`] module"]
#[doc(alias = "HC6_SPLT")]
pub type Hc6Splt = crate::Reg<hc6_splt::Hc6SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc6_splt;
#[doc = "HC6_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_int`] module"]
#[doc(alias = "HC6_INT")]
pub type Hc6Int = crate::Reg<hc6_int::Hc6IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc6_int;
#[doc = "HC6_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_intmsk`] module"]
#[doc(alias = "HC6_INTMSK")]
pub type Hc6Intmsk = crate::Reg<hc6_intmsk::Hc6IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "HC6_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_tsiz`] module"]
#[doc(alias = "HC6_TSIZ")]
pub type Hc6Tsiz = crate::Reg<hc6_tsiz::Hc6TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "HC6_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc6_dmaaddr`] module"]
#[doc(alias = "HC6_DMAADDR")]
pub type Hc6Dmaaddr = crate::Reg<hc6_dmaaddr::Hc6DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "HC7_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc7_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc7_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_char`] module"]
#[doc(alias = "HC7_CHAR")]
pub type Hc7Char = crate::Reg<hc7_char::Hc7CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc7_char;
#[doc = "HC7_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc7_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc7_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_splt`] module"]
#[doc(alias = "HC7_SPLT")]
pub type Hc7Splt = crate::Reg<hc7_splt::Hc7SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc7_splt;
#[doc = "HC7_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc7_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc7_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_int`] module"]
#[doc(alias = "HC7_INT")]
pub type Hc7Int = crate::Reg<hc7_int::Hc7IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc7_int;
#[doc = "HC7_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc7_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc7_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_intmsk`] module"]
#[doc(alias = "HC7_INTMSK")]
pub type Hc7Intmsk = crate::Reg<hc7_intmsk::Hc7IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "HC7_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc7_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc7_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_tsiz`] module"]
#[doc(alias = "HC7_TSIZ")]
pub type Hc7Tsiz = crate::Reg<hc7_tsiz::Hc7TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "HC7_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc7_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc7_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc7_dmaaddr`] module"]
#[doc(alias = "HC7_DMAADDR")]
pub type Hc7Dmaaddr = crate::Reg<hc7_dmaaddr::Hc7DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "HC8_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_char`] module"]
#[doc(alias = "HC8_CHAR")]
pub type Hc8Char = crate::Reg<hc8_char::Hc8CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc8_char;
#[doc = "HC8_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_splt`] module"]
#[doc(alias = "HC8_SPLT")]
pub type Hc8Splt = crate::Reg<hc8_splt::Hc8SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc8_splt;
#[doc = "HC8_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_int`] module"]
#[doc(alias = "HC8_INT")]
pub type Hc8Int = crate::Reg<hc8_int::Hc8IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc8_int;
#[doc = "HC8_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_intmsk`] module"]
#[doc(alias = "HC8_INTMSK")]
pub type Hc8Intmsk = crate::Reg<hc8_intmsk::Hc8IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "HC8_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_tsiz`] module"]
#[doc(alias = "HC8_TSIZ")]
pub type Hc8Tsiz = crate::Reg<hc8_tsiz::Hc8TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "HC8_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc8_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc8_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc8_dmaaddr`] module"]
#[doc(alias = "HC8_DMAADDR")]
pub type Hc8Dmaaddr = crate::Reg<hc8_dmaaddr::Hc8DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "HC9_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_char`] module"]
#[doc(alias = "HC9_CHAR")]
pub type Hc9Char = crate::Reg<hc9_char::Hc9CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc9_char;
#[doc = "HC9_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_splt`] module"]
#[doc(alias = "HC9_SPLT")]
pub type Hc9Splt = crate::Reg<hc9_splt::Hc9SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc9_splt;
#[doc = "HC9_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_int`] module"]
#[doc(alias = "HC9_INT")]
pub type Hc9Int = crate::Reg<hc9_int::Hc9IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc9_int;
#[doc = "HC9_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_intmsk`] module"]
#[doc(alias = "HC9_INTMSK")]
pub type Hc9Intmsk = crate::Reg<hc9_intmsk::Hc9IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "HC9_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_tsiz`] module"]
#[doc(alias = "HC9_TSIZ")]
pub type Hc9Tsiz = crate::Reg<hc9_tsiz::Hc9TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "HC9_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc9_dmaaddr`] module"]
#[doc(alias = "HC9_DMAADDR")]
pub type Hc9Dmaaddr = crate::Reg<hc9_dmaaddr::Hc9DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "HC10_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc10_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc10_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_char`] module"]
#[doc(alias = "HC10_CHAR")]
pub type Hc10Char = crate::Reg<hc10_char::Hc10CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc10_char;
#[doc = "HC10_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc10_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc10_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_splt`] module"]
#[doc(alias = "HC10_SPLT")]
pub type Hc10Splt = crate::Reg<hc10_splt::Hc10SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc10_splt;
#[doc = "HC10_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc10_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc10_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_int`] module"]
#[doc(alias = "HC10_INT")]
pub type Hc10Int = crate::Reg<hc10_int::Hc10IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc10_int;
#[doc = "HC10_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc10_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc10_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_intmsk`] module"]
#[doc(alias = "HC10_INTMSK")]
pub type Hc10Intmsk = crate::Reg<hc10_intmsk::Hc10IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "HC10_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc10_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc10_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_tsiz`] module"]
#[doc(alias = "HC10_TSIZ")]
pub type Hc10Tsiz = crate::Reg<hc10_tsiz::Hc10TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "HC10_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc10_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc10_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc10_dmaaddr`] module"]
#[doc(alias = "HC10_DMAADDR")]
pub type Hc10Dmaaddr = crate::Reg<hc10_dmaaddr::Hc10DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "HC11_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc11_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc11_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_char`] module"]
#[doc(alias = "HC11_CHAR")]
pub type Hc11Char = crate::Reg<hc11_char::Hc11CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc11_char;
#[doc = "HC11_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc11_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc11_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_splt`] module"]
#[doc(alias = "HC11_SPLT")]
pub type Hc11Splt = crate::Reg<hc11_splt::Hc11SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc11_splt;
#[doc = "HC11_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc11_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc11_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_int`] module"]
#[doc(alias = "HC11_INT")]
pub type Hc11Int = crate::Reg<hc11_int::Hc11IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc11_int;
#[doc = "HC11_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc11_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc11_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_intmsk`] module"]
#[doc(alias = "HC11_INTMSK")]
pub type Hc11Intmsk = crate::Reg<hc11_intmsk::Hc11IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "HC11_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc11_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc11_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_tsiz`] module"]
#[doc(alias = "HC11_TSIZ")]
pub type Hc11Tsiz = crate::Reg<hc11_tsiz::Hc11TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "HC11_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc11_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc11_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc11_dmaaddr`] module"]
#[doc(alias = "HC11_DMAADDR")]
pub type Hc11Dmaaddr = crate::Reg<hc11_dmaaddr::Hc11DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "HC12_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_char`] module"]
#[doc(alias = "HC12_CHAR")]
pub type Hc12Char = crate::Reg<hc12_char::Hc12CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc12_char;
#[doc = "HC12_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_splt`] module"]
#[doc(alias = "HC12_SPLT")]
pub type Hc12Splt = crate::Reg<hc12_splt::Hc12SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc12_splt;
#[doc = "HC12_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_int`] module"]
#[doc(alias = "HC12_INT")]
pub type Hc12Int = crate::Reg<hc12_int::Hc12IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc12_int;
#[doc = "HC12_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_intmsk`] module"]
#[doc(alias = "HC12_INTMSK")]
pub type Hc12Intmsk = crate::Reg<hc12_intmsk::Hc12IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "HC12_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_tsiz`] module"]
#[doc(alias = "HC12_TSIZ")]
pub type Hc12Tsiz = crate::Reg<hc12_tsiz::Hc12TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "HC12_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc12_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc12_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc12_dmaaddr`] module"]
#[doc(alias = "HC12_DMAADDR")]
pub type Hc12Dmaaddr = crate::Reg<hc12_dmaaddr::Hc12DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "HC13_CHAR (rw) register accessor: Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc13_char::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc13_char::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_char`] module"]
#[doc(alias = "HC13_CHAR")]
pub type Hc13Char = crate::Reg<hc13_char::Hc13CharSpec>;
#[doc = "Host Channel x Characteristics Register"]
pub mod hc13_char;
#[doc = "HC13_SPLT (rw) register accessor: Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc13_splt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc13_splt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_splt`] module"]
#[doc(alias = "HC13_SPLT")]
pub type Hc13Splt = crate::Reg<hc13_splt::Hc13SpltSpec>;
#[doc = "Host Channel x Split Control Register"]
pub mod hc13_splt;
#[doc = "HC13_INT (rw) register accessor: Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc13_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc13_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_int`] module"]
#[doc(alias = "HC13_INT")]
pub type Hc13Int = crate::Reg<hc13_int::Hc13IntSpec>;
#[doc = "Host Channel x Interrupt Register"]
pub mod hc13_int;
#[doc = "HC13_INTMSK (rw) register accessor: Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc13_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc13_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_intmsk`] module"]
#[doc(alias = "HC13_INTMSK")]
pub type Hc13Intmsk = crate::Reg<hc13_intmsk::Hc13IntmskSpec>;
#[doc = "Host Channel x Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "HC13_TSIZ (rw) register accessor: Host Channel x Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc13_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc13_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_tsiz`] module"]
#[doc(alias = "HC13_TSIZ")]
pub type Hc13Tsiz = crate::Reg<hc13_tsiz::Hc13TsizSpec>;
#[doc = "Host Channel x Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "HC13_DMAADDR (rw) register accessor: Host Channel x DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc13_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc13_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc13_dmaaddr`] module"]
#[doc(alias = "HC13_DMAADDR")]
pub type Hc13Dmaaddr = crate::Reg<hc13_dmaaddr::Hc13DmaaddrSpec>;
#[doc = "Host Channel x DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "DCFG (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`] module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`] module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: Device Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`] module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: Device IN Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`] module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: Device OUT Endpoint Common Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`] module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: Device All Endpoints Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`] module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`] module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: Device VBUS Discharge Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`] module"]
#[doc(alias = "DVBUSDIS")]
pub type Dvbusdis = crate::Reg<dvbusdis::DvbusdisSpec>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`] module"]
#[doc(alias = "DVBUSPULSE")]
pub type Dvbuspulse = crate::Reg<dvbuspulse::DvbuspulseSpec>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: Device Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`] module"]
#[doc(alias = "DTHRCTL")]
pub type Dthrctl = crate::Reg<dthrctl::DthrctlSpec>;
#[doc = "Device Threshold Control Register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`] module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: Device Control IN Endpoint 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0ctl`] module"]
#[doc(alias = "DIEP0CTL")]
pub type Diep0ctl = crate::Reg<diep0ctl::Diep0ctlSpec>;
#[doc = "Device Control IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: Device IN Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0int`] module"]
#[doc(alias = "DIEP0INT")]
pub type Diep0int = crate::Reg<diep0int::Diep0intSpec>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: Device IN Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tsiz`] module"]
#[doc(alias = "DIEP0TSIZ")]
pub type Diep0tsiz = crate::Reg<diep0tsiz::Diep0tsizSpec>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0dmaaddr`] module"]
#[doc(alias = "DIEP0DMAADDR")]
pub type Diep0dmaaddr = crate::Reg<diep0dmaaddr::Diep0dmaaddrSpec>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0txfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0txfsts`] module"]
#[doc(alias = "DIEP0TXFSTS")]
pub type Diep0txfsts = crate::Reg<diep0txfsts::Diep0txfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_ctl`] module"]
#[doc(alias = "DIEP0_CTL")]
pub type Diep0Ctl = crate::Reg<diep0_ctl::Diep0CtlSpec>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_int`] module"]
#[doc(alias = "DIEP0_INT")]
pub type Diep0Int = crate::Reg<diep0_int::Diep0IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_tsiz`] module"]
#[doc(alias = "DIEP0_TSIZ")]
pub type Diep0Tsiz = crate::Reg<diep0_tsiz::Diep0TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_dmaaddr`] module"]
#[doc(alias = "DIEP0_DMAADDR")]
pub type Diep0Dmaaddr = crate::Reg<diep0_dmaaddr::Diep0DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0_dtxfsts`] module"]
#[doc(alias = "DIEP0_DTXFSTS")]
pub type Diep0Dtxfsts = crate::Reg<diep0_dtxfsts::Diep0DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep0_dtxfsts;
#[doc = "DIEP1_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_ctl`] module"]
#[doc(alias = "DIEP1_CTL")]
pub type Diep1Ctl = crate::Reg<diep1_ctl::Diep1CtlSpec>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_int`] module"]
#[doc(alias = "DIEP1_INT")]
pub type Diep1Int = crate::Reg<diep1_int::Diep1IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_tsiz`] module"]
#[doc(alias = "DIEP1_TSIZ")]
pub type Diep1Tsiz = crate::Reg<diep1_tsiz::Diep1TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_dmaaddr`] module"]
#[doc(alias = "DIEP1_DMAADDR")]
pub type Diep1Dmaaddr = crate::Reg<diep1_dmaaddr::Diep1DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1_dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1_dtxfsts`] module"]
#[doc(alias = "DIEP1_DTXFSTS")]
pub type Diep1Dtxfsts = crate::Reg<diep1_dtxfsts::Diep1DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep1_dtxfsts;
#[doc = "DIEP2_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_ctl`] module"]
#[doc(alias = "DIEP2_CTL")]
pub type Diep2Ctl = crate::Reg<diep2_ctl::Diep2CtlSpec>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_int`] module"]
#[doc(alias = "DIEP2_INT")]
pub type Diep2Int = crate::Reg<diep2_int::Diep2IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_tsiz`] module"]
#[doc(alias = "DIEP2_TSIZ")]
pub type Diep2Tsiz = crate::Reg<diep2_tsiz::Diep2TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_dmaaddr`] module"]
#[doc(alias = "DIEP2_DMAADDR")]
pub type Diep2Dmaaddr = crate::Reg<diep2_dmaaddr::Diep2DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2_dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2_dtxfsts`] module"]
#[doc(alias = "DIEP2_DTXFSTS")]
pub type Diep2Dtxfsts = crate::Reg<diep2_dtxfsts::Diep2DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep2_dtxfsts;
#[doc = "DIEP3_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_ctl`] module"]
#[doc(alias = "DIEP3_CTL")]
pub type Diep3Ctl = crate::Reg<diep3_ctl::Diep3CtlSpec>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep3_ctl;
#[doc = "DIEP3_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_int`] module"]
#[doc(alias = "DIEP3_INT")]
pub type Diep3Int = crate::Reg<diep3_int::Diep3IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep3_int;
#[doc = "DIEP3_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_tsiz`] module"]
#[doc(alias = "DIEP3_TSIZ")]
pub type Diep3Tsiz = crate::Reg<diep3_tsiz::Diep3TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "DIEP3_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_dmaaddr`] module"]
#[doc(alias = "DIEP3_DMAADDR")]
pub type Diep3Dmaaddr = crate::Reg<diep3_dmaaddr::Diep3DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "DIEP3_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3_dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3_dtxfsts`] module"]
#[doc(alias = "DIEP3_DTXFSTS")]
pub type Diep3Dtxfsts = crate::Reg<diep3_dtxfsts::Diep3DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep3_dtxfsts;
#[doc = "DIEP4_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep4_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep4_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_ctl`] module"]
#[doc(alias = "DIEP4_CTL")]
pub type Diep4Ctl = crate::Reg<diep4_ctl::Diep4CtlSpec>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep4_ctl;
#[doc = "DIEP4_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep4_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep4_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_int`] module"]
#[doc(alias = "DIEP4_INT")]
pub type Diep4Int = crate::Reg<diep4_int::Diep4IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep4_int;
#[doc = "DIEP4_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep4_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep4_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_tsiz`] module"]
#[doc(alias = "DIEP4_TSIZ")]
pub type Diep4Tsiz = crate::Reg<diep4_tsiz::Diep4TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "DIEP4_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep4_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep4_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_dmaaddr`] module"]
#[doc(alias = "DIEP4_DMAADDR")]
pub type Diep4Dmaaddr = crate::Reg<diep4_dmaaddr::Diep4DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "DIEP4_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep4_dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep4_dtxfsts`] module"]
#[doc(alias = "DIEP4_DTXFSTS")]
pub type Diep4Dtxfsts = crate::Reg<diep4_dtxfsts::Diep4DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep4_dtxfsts;
#[doc = "DIEP5_CTL (rw) register accessor: Device Control IN Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep5_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep5_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_ctl`] module"]
#[doc(alias = "DIEP5_CTL")]
pub type Diep5Ctl = crate::Reg<diep5_ctl::Diep5CtlSpec>;
#[doc = "Device Control IN Endpoint x+1 Control Register"]
pub mod diep5_ctl;
#[doc = "DIEP5_INT (rw) register accessor: Device IN Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep5_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep5_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_int`] module"]
#[doc(alias = "DIEP5_INT")]
pub type Diep5Int = crate::Reg<diep5_int::Diep5IntSpec>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep5_int;
#[doc = "DIEP5_TSIZ (rw) register accessor: Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep5_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep5_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_tsiz`] module"]
#[doc(alias = "DIEP5_TSIZ")]
pub type Diep5Tsiz = crate::Reg<diep5_tsiz::Diep5TsizSpec>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "DIEP5_DMAADDR (rw) register accessor: Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep5_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep5_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_dmaaddr`] module"]
#[doc(alias = "DIEP5_DMAADDR")]
pub type Diep5Dmaaddr = crate::Reg<diep5_dmaaddr::Diep5DmaaddrSpec>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "DIEP5_DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep5_dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep5_dtxfsts`] module"]
#[doc(alias = "DIEP5_DTXFSTS")]
pub type Diep5Dtxfsts = crate::Reg<diep5_dtxfsts::Diep5DtxfstsSpec>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep5_dtxfsts;
#[doc = "DOEP0CTL (rw) register accessor: Device Control OUT Endpoint 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0ctl`] module"]
#[doc(alias = "DOEP0CTL")]
pub type Doep0ctl = crate::Reg<doep0ctl::Doep0ctlSpec>;
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: Device OUT Endpoint 0 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0int`] module"]
#[doc(alias = "DOEP0INT")]
pub type Doep0int = crate::Reg<doep0int::Doep0intSpec>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: Device OUT Endpoint 0 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0tsiz`] module"]
#[doc(alias = "DOEP0TSIZ")]
pub type Doep0tsiz = crate::Reg<doep0tsiz::Doep0tsizSpec>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: Device OUT Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0dmaaddr`] module"]
#[doc(alias = "DOEP0DMAADDR")]
pub type Doep0dmaaddr = crate::Reg<doep0dmaaddr::Doep0dmaaddrSpec>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_ctl`] module"]
#[doc(alias = "DOEP0_CTL")]
pub type Doep0Ctl = crate::Reg<doep0_ctl::Doep0CtlSpec>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_int`] module"]
#[doc(alias = "DOEP0_INT")]
pub type Doep0Int = crate::Reg<doep0_int::Doep0IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_tsiz`] module"]
#[doc(alias = "DOEP0_TSIZ")]
pub type Doep0Tsiz = crate::Reg<doep0_tsiz::Doep0TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0_dmaaddr`] module"]
#[doc(alias = "DOEP0_DMAADDR")]
pub type Doep0Dmaaddr = crate::Reg<doep0_dmaaddr::Doep0DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_ctl`] module"]
#[doc(alias = "DOEP1_CTL")]
pub type Doep1Ctl = crate::Reg<doep1_ctl::Doep1CtlSpec>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_int`] module"]
#[doc(alias = "DOEP1_INT")]
pub type Doep1Int = crate::Reg<doep1_int::Doep1IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_tsiz`] module"]
#[doc(alias = "DOEP1_TSIZ")]
pub type Doep1Tsiz = crate::Reg<doep1_tsiz::Doep1TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1_dmaaddr`] module"]
#[doc(alias = "DOEP1_DMAADDR")]
pub type Doep1Dmaaddr = crate::Reg<doep1_dmaaddr::Doep1DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_ctl`] module"]
#[doc(alias = "DOEP2_CTL")]
pub type Doep2Ctl = crate::Reg<doep2_ctl::Doep2CtlSpec>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_int`] module"]
#[doc(alias = "DOEP2_INT")]
pub type Doep2Int = crate::Reg<doep2_int::Doep2IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_tsiz`] module"]
#[doc(alias = "DOEP2_TSIZ")]
pub type Doep2Tsiz = crate::Reg<doep2_tsiz::Doep2TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2_dmaaddr`] module"]
#[doc(alias = "DOEP2_DMAADDR")]
pub type Doep2Dmaaddr = crate::Reg<doep2_dmaaddr::Doep2DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "DOEP3_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_ctl`] module"]
#[doc(alias = "DOEP3_CTL")]
pub type Doep3Ctl = crate::Reg<doep3_ctl::Doep3CtlSpec>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep3_ctl;
#[doc = "DOEP3_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_int`] module"]
#[doc(alias = "DOEP3_INT")]
pub type Doep3Int = crate::Reg<doep3_int::Doep3IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep3_int;
#[doc = "DOEP3_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_tsiz`] module"]
#[doc(alias = "DOEP3_TSIZ")]
pub type Doep3Tsiz = crate::Reg<doep3_tsiz::Doep3TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "DOEP3_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3_dmaaddr`] module"]
#[doc(alias = "DOEP3_DMAADDR")]
pub type Doep3Dmaaddr = crate::Reg<doep3_dmaaddr::Doep3DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "DOEP4_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep4_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep4_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_ctl`] module"]
#[doc(alias = "DOEP4_CTL")]
pub type Doep4Ctl = crate::Reg<doep4_ctl::Doep4CtlSpec>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep4_ctl;
#[doc = "DOEP4_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep4_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep4_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_int`] module"]
#[doc(alias = "DOEP4_INT")]
pub type Doep4Int = crate::Reg<doep4_int::Doep4IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep4_int;
#[doc = "DOEP4_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep4_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep4_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_tsiz`] module"]
#[doc(alias = "DOEP4_TSIZ")]
pub type Doep4Tsiz = crate::Reg<doep4_tsiz::Doep4TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "DOEP4_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep4_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep4_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep4_dmaaddr`] module"]
#[doc(alias = "DOEP4_DMAADDR")]
pub type Doep4Dmaaddr = crate::Reg<doep4_dmaaddr::Doep4DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "DOEP5_CTL (rw) register accessor: Device Control OUT Endpoint x+1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep5_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep5_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_ctl`] module"]
#[doc(alias = "DOEP5_CTL")]
pub type Doep5Ctl = crate::Reg<doep5_ctl::Doep5CtlSpec>;
#[doc = "Device Control OUT Endpoint x+1 Control Register"]
pub mod doep5_ctl;
#[doc = "DOEP5_INT (rw) register accessor: Device OUT Endpoint x+1 Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep5_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep5_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_int`] module"]
#[doc(alias = "DOEP5_INT")]
pub type Doep5Int = crate::Reg<doep5_int::Doep5IntSpec>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep5_int;
#[doc = "DOEP5_TSIZ (rw) register accessor: Device OUT Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep5_tsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep5_tsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_tsiz`] module"]
#[doc(alias = "DOEP5_TSIZ")]
pub type Doep5Tsiz = crate::Reg<doep5_tsiz::Doep5TsizSpec>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "DOEP5_DMAADDR (rw) register accessor: Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep5_dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep5_dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep5_dmaaddr`] module"]
#[doc(alias = "DOEP5_DMAADDR")]
pub type Doep5Dmaaddr = crate::Reg<doep5_dmaaddr::Doep5DmaaddrSpec>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: Power and Clock Gating Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`] module"]
#[doc(alias = "PCGCCTL")]
pub type Pcgcctl = crate::Reg<pcgcctl::PcgcctlSpec>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
