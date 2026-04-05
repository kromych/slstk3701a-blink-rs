#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    networkctrl: Networkctrl,
    networkcfg: Networkcfg,
    networkstatus: Networkstatus,
    _reserved3: [u8; 0x04],
    dmacfg: Dmacfg,
    txstatus: Txstatus,
    rxqptr: Rxqptr,
    txqptr: Txqptr,
    rxstatus: Rxstatus,
    ifcr: Ifcr,
    iens: Iens,
    ienc: Ienc,
    ienro: Ienro,
    phymngmnt: Phymngmnt,
    rxpausequant: Rxpausequant,
    txpausequant: Txpausequant,
    pbuftxcutthru: Pbuftxcutthru,
    pbufrxcutthru: Pbufrxcutthru,
    jumbomaxlen: Jumbomaxlen,
    _reserved18: [u8; 0x10],
    imod: Imod,
    syswaketime: Syswaketime,
    _reserved20: [u8; 0x1c],
    hashbottom: Hashbottom,
    hashtop: Hashtop,
    specaddr1bottom: Specaddr1bottom,
    specaddr1top: Specaddr1top,
    specaddr2bottom: Specaddr2bottom,
    specaddr2top: Specaddr2top,
    specaddr3bottom: Specaddr3bottom,
    specaddr3top: Specaddr3top,
    specaddr4bottom: Specaddr4bottom,
    specaddr4top: Specaddr4top,
    spectype1: Spectype1,
    spectype2: Spectype2,
    spectype3: Spectype3,
    spectype4: Spectype4,
    wolreg: Wolreg,
    stretchratio: Stretchratio,
    stackedvlan: Stackedvlan,
    txpfcpause: Txpfcpause,
    maskadd1bottom: Maskadd1bottom,
    maskadd1top: Maskadd1top,
    _reserved40: [u8; 0x04],
    rxptpunicast: Rxptpunicast,
    txptpunicast: Txptpunicast,
    tsunseccmp: Tsunseccmp,
    tsuseccmp: Tsuseccmp,
    tsumsbseccmp: Tsumsbseccmp,
    tsuptptxmsbsec: Tsuptptxmsbsec,
    tsuptprxmsbsec: Tsuptprxmsbsec,
    tsupeertxmsbsec: Tsupeertxmsbsec,
    tsupeerrxmsbsec: Tsupeerrxmsbsec,
    _reserved49: [u8; 0x08],
    octetstxedbottom: Octetstxedbottom,
    octetstxedtop: Octetstxedtop,
    framestxedok: Framestxedok,
    broadcasttxed: Broadcasttxed,
    multicasttxed: Multicasttxed,
    pframestxed: Pframestxed,
    framestxed64: Framestxed64,
    framestxed65: Framestxed65,
    framestxed128: Framestxed128,
    framestxed256: Framestxed256,
    framestxed512: Framestxed512,
    framestxed1024: Framestxed1024,
    framestxed1519: Framestxed1519,
    txunderruns: Txunderruns,
    singlecols: Singlecols,
    multicols: Multicols,
    excesscols: Excesscols,
    latecols: Latecols,
    deferredframes: Deferredframes,
    crserrs: Crserrs,
    octetsrxedbottom: Octetsrxedbottom,
    octetsrxedtop: Octetsrxedtop,
    framesrxedok: Framesrxedok,
    broadcastrxed: Broadcastrxed,
    multicastrxed: Multicastrxed,
    pframesrxed: Pframesrxed,
    framesrxed64: Framesrxed64,
    framesrxed65: Framesrxed65,
    framesrxed128: Framesrxed128,
    framesrxed256: Framesrxed256,
    framesrxed512: Framesrxed512,
    framesrxed1024: Framesrxed1024,
    framesrxed1519: Framesrxed1519,
    undersizeframes: Undersizeframes,
    excessiverxlen: Excessiverxlen,
    rxjabbers: Rxjabbers,
    fcserrs: Fcserrs,
    rxlenerrs: Rxlenerrs,
    rxsymbolerrs: Rxsymbolerrs,
    alignerrs: Alignerrs,
    rxresourceerrs: Rxresourceerrs,
    rxoverruns: Rxoverruns,
    rxipckerrs: Rxipckerrs,
    rxtcpckerrs: Rxtcpckerrs,
    rxudpckerrs: Rxudpckerrs,
    autoflushedpkts: Autoflushedpkts,
    _reserved95: [u8; 0x04],
    tsutimerincrsubnsec: Tsutimerincrsubnsec,
    tsutimermsbsec: Tsutimermsbsec,
    _reserved97: [u8; 0x0c],
    tsutimersec: Tsutimersec,
    tsutimernsec: Tsutimernsec,
    tsutimeradjust: Tsutimeradjust,
    tsutimerincr: Tsutimerincr,
    tsuptptxsec: Tsuptptxsec,
    tsuptptxnsec: Tsuptptxnsec,
    tsuptprxsec: Tsuptprxsec,
    tsuptprxnsec: Tsuptprxnsec,
    tsupeertxsec: Tsupeertxsec,
    tsupeertxnsec: Tsupeertxnsec,
    tsupeerrxsec: Tsupeerrxsec,
    tsupeerrxnsec: Tsupeerrxnsec,
    _reserved109: [u8; 0x60],
    txpausequant1: Txpausequant1,
    txpausequant2: Txpausequant2,
    txpausequant3: Txpausequant3,
    _reserved112: [u8; 0x04],
    rxlpi: Rxlpi,
    rxlpitime: Rxlpitime,
    txlpi: Txlpi,
    txlpitime: Txlpitime,
    _reserved116: [u8; 0x024c],
    txbdctrl: Txbdctrl,
    rxbdctrl: Rxbdctrl,
    _reserved118: [u8; 0x072c],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved120: [u8; 0x04],
    routeloc1: Routeloc1,
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Network control register"]
    #[inline(always)]
    pub const fn networkctrl(&self) -> &Networkctrl {
        &self.networkctrl
    }
    #[doc = "0x04 - Network configuration register"]
    #[inline(always)]
    pub const fn networkcfg(&self) -> &Networkcfg {
        &self.networkcfg
    }
    #[doc = "0x08 - Network status register"]
    #[inline(always)]
    pub const fn networkstatus(&self) -> &Networkstatus {
        &self.networkstatus
    }
    #[doc = "0x10 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x14 - Transmit status register"]
    #[inline(always)]
    pub const fn txstatus(&self) -> &Txstatus {
        &self.txstatus
    }
    #[doc = "0x18 - Start address of the receive buffer queue"]
    #[inline(always)]
    pub const fn rxqptr(&self) -> &Rxqptr {
        &self.rxqptr
    }
    #[doc = "0x1c - Start address of the transmit buffer queue"]
    #[inline(always)]
    pub const fn txqptr(&self) -> &Txqptr {
        &self.txqptr
    }
    #[doc = "0x20 - Receive status register"]
    #[inline(always)]
    pub const fn rxstatus(&self) -> &Rxstatus {
        &self.rxstatus
    }
    #[doc = "0x24 - Interrupt status register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn iens(&self) -> &Iens {
        &self.iens
    }
    #[doc = "0x2c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn ienc(&self) -> &Ienc {
        &self.ienc
    }
    #[doc = "0x30 - Interrupt mask register"]
    #[inline(always)]
    pub const fn ienro(&self) -> &Ienro {
        &self.ienro
    }
    #[doc = "0x34 - PHY management register"]
    #[inline(always)]
    pub const fn phymngmnt(&self) -> &Phymngmnt {
        &self.phymngmnt
    }
    #[doc = "0x38 - Received Pause Quantum Register"]
    #[inline(always)]
    pub const fn rxpausequant(&self) -> &Rxpausequant {
        &self.rxpausequant
    }
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    #[inline(always)]
    pub const fn txpausequant(&self) -> &Txpausequant {
        &self.txpausequant
    }
    #[doc = "0x40 - TX Partial Store and Forward"]
    #[inline(always)]
    pub const fn pbuftxcutthru(&self) -> &Pbuftxcutthru {
        &self.pbuftxcutthru
    }
    #[doc = "0x44 - RX Partial Store and Forward"]
    #[inline(always)]
    pub const fn pbufrxcutthru(&self) -> &Pbufrxcutthru {
        &self.pbufrxcutthru
    }
    #[doc = "0x48 - Maximum Jumbo Frame Size."]
    #[inline(always)]
    pub const fn jumbomaxlen(&self) -> &Jumbomaxlen {
        &self.jumbomaxlen
    }
    #[doc = "0x5c - Interrupt moderation register"]
    #[inline(always)]
    pub const fn imod(&self) -> &Imod {
        &self.imod
    }
    #[doc = "0x60 - System wake time"]
    #[inline(always)]
    pub const fn syswaketime(&self) -> &Syswaketime {
        &self.syswaketime
    }
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    #[inline(always)]
    pub const fn hashbottom(&self) -> &Hashbottom {
        &self.hashbottom
    }
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    #[inline(always)]
    pub const fn hashtop(&self) -> &Hashtop {
        &self.hashtop
    }
    #[doc = "0x88 - Specific Address 1 Bottom"]
    #[inline(always)]
    pub const fn specaddr1bottom(&self) -> &Specaddr1bottom {
        &self.specaddr1bottom
    }
    #[doc = "0x8c - Specific Address 1 Top"]
    #[inline(always)]
    pub const fn specaddr1top(&self) -> &Specaddr1top {
        &self.specaddr1top
    }
    #[doc = "0x90 - Specific Address 2 Bottom"]
    #[inline(always)]
    pub const fn specaddr2bottom(&self) -> &Specaddr2bottom {
        &self.specaddr2bottom
    }
    #[doc = "0x94 - Specific Address 2 Top"]
    #[inline(always)]
    pub const fn specaddr2top(&self) -> &Specaddr2top {
        &self.specaddr2top
    }
    #[doc = "0x98 - Specific Address 3 Bottom"]
    #[inline(always)]
    pub const fn specaddr3bottom(&self) -> &Specaddr3bottom {
        &self.specaddr3bottom
    }
    #[doc = "0x9c - Specific Address 3 Top"]
    #[inline(always)]
    pub const fn specaddr3top(&self) -> &Specaddr3top {
        &self.specaddr3top
    }
    #[doc = "0xa0 - Specific Address 4 Bottom"]
    #[inline(always)]
    pub const fn specaddr4bottom(&self) -> &Specaddr4bottom {
        &self.specaddr4bottom
    }
    #[doc = "0xa4 - Specific Address 4 Top"]
    #[inline(always)]
    pub const fn specaddr4top(&self) -> &Specaddr4top {
        &self.specaddr4top
    }
    #[doc = "0xa8 - Type ID Match 1"]
    #[inline(always)]
    pub const fn spectype1(&self) -> &Spectype1 {
        &self.spectype1
    }
    #[doc = "0xac - Type ID Match 2"]
    #[inline(always)]
    pub const fn spectype2(&self) -> &Spectype2 {
        &self.spectype2
    }
    #[doc = "0xb0 - Type ID Match 3"]
    #[inline(always)]
    pub const fn spectype3(&self) -> &Spectype3 {
        &self.spectype3
    }
    #[doc = "0xb4 - Type ID Match 4"]
    #[inline(always)]
    pub const fn spectype4(&self) -> &Spectype4 {
        &self.spectype4
    }
    #[doc = "0xb8 - Wake on LAN Register"]
    #[inline(always)]
    pub const fn wolreg(&self) -> &Wolreg {
        &self.wolreg
    }
    #[doc = "0xbc - IPG stretch register"]
    #[inline(always)]
    pub const fn stretchratio(&self) -> &Stretchratio {
        &self.stretchratio
    }
    #[doc = "0xc0 - Stacked VLAN Register"]
    #[inline(always)]
    pub const fn stackedvlan(&self) -> &Stackedvlan {
        &self.stackedvlan
    }
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    #[inline(always)]
    pub const fn txpfcpause(&self) -> &Txpfcpause {
        &self.txpfcpause
    }
    #[doc = "0xc8 - Specific Address Mask 1 Bottom 31:0"]
    #[inline(always)]
    pub const fn maskadd1bottom(&self) -> &Maskadd1bottom {
        &self.maskadd1bottom
    }
    #[doc = "0xcc - Specific Address Mask 1 Top 47:32"]
    #[inline(always)]
    pub const fn maskadd1top(&self) -> &Maskadd1top {
        &self.maskadd1top
    }
    #[doc = "0xd4 - PTP RX unicast IP destination address"]
    #[inline(always)]
    pub const fn rxptpunicast(&self) -> &Rxptpunicast {
        &self.rxptpunicast
    }
    #[doc = "0xd8 - PTP TX unicast IP destination address"]
    #[inline(always)]
    pub const fn txptpunicast(&self) -> &Txptpunicast {
        &self.txptpunicast
    }
    #[doc = "0xdc - TSU timer comparison value nanoseconds"]
    #[inline(always)]
    pub const fn tsunseccmp(&self) -> &Tsunseccmp {
        &self.tsunseccmp
    }
    #[doc = "0xe0 - TSU timer comparison value seconds \\[31:0\\]"]
    #[inline(always)]
    pub const fn tsuseccmp(&self) -> &Tsuseccmp {
        &self.tsuseccmp
    }
    #[doc = "0xe4 - TSU timer comparison value seconds \\[47:32\\]"]
    #[inline(always)]
    pub const fn tsumsbseccmp(&self) -> &Tsumsbseccmp {
        &self.tsumsbseccmp
    }
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsuptptxmsbsec(&self) -> &Tsuptptxmsbsec {
        &self.tsuptptxmsbsec
    }
    #[doc = "0xec - PTP Event Frame Received Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsuptprxmsbsec(&self) -> &Tsuptprxmsbsec {
        &self.tsuptprxmsbsec
    }
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsupeertxmsbsec(&self) -> &Tsupeertxmsbsec {
        &self.tsupeertxmsbsec
    }
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsupeerrxmsbsec(&self) -> &Tsupeerrxmsbsec {
        &self.tsupeerrxmsbsec
    }
    #[doc = "0x100 - Octets transmitted 31:0"]
    #[inline(always)]
    pub const fn octetstxedbottom(&self) -> &Octetstxedbottom {
        &self.octetstxedbottom
    }
    #[doc = "0x104 - Octets Transmitted 47:32"]
    #[inline(always)]
    pub const fn octetstxedtop(&self) -> &Octetstxedtop {
        &self.octetstxedtop
    }
    #[doc = "0x108 - Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxedok(&self) -> &Framestxedok {
        &self.framestxedok
    }
    #[doc = "0x10c - Broadcast Frames Transmitted"]
    #[inline(always)]
    pub const fn broadcasttxed(&self) -> &Broadcasttxed {
        &self.broadcasttxed
    }
    #[doc = "0x110 - Multicast Frames Transmitted"]
    #[inline(always)]
    pub const fn multicasttxed(&self) -> &Multicasttxed {
        &self.multicasttxed
    }
    #[doc = "0x114 - Pause Frames Transmitted"]
    #[inline(always)]
    pub const fn pframestxed(&self) -> &Pframestxed {
        &self.pframestxed
    }
    #[doc = "0x118 - 64 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed64(&self) -> &Framestxed64 {
        &self.framestxed64
    }
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed65(&self) -> &Framestxed65 {
        &self.framestxed65
    }
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed128(&self) -> &Framestxed128 {
        &self.framestxed128
    }
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed256(&self) -> &Framestxed256 {
        &self.framestxed256
    }
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed512(&self) -> &Framestxed512 {
        &self.framestxed512
    }
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed1024(&self) -> &Framestxed1024 {
        &self.framestxed1024
    }
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed1519(&self) -> &Framestxed1519 {
        &self.framestxed1519
    }
    #[doc = "0x134 - Transmit Under Runs"]
    #[inline(always)]
    pub const fn txunderruns(&self) -> &Txunderruns {
        &self.txunderruns
    }
    #[doc = "0x138 - Single Collision Frames"]
    #[inline(always)]
    pub const fn singlecols(&self) -> &Singlecols {
        &self.singlecols
    }
    #[doc = "0x13c - Multiple Collision Frames"]
    #[inline(always)]
    pub const fn multicols(&self) -> &Multicols {
        &self.multicols
    }
    #[doc = "0x140 - Excessive Collisions"]
    #[inline(always)]
    pub const fn excesscols(&self) -> &Excesscols {
        &self.excesscols
    }
    #[doc = "0x144 - Late Collisions"]
    #[inline(always)]
    pub const fn latecols(&self) -> &Latecols {
        &self.latecols
    }
    #[doc = "0x148 - Deferred Transmission Frames"]
    #[inline(always)]
    pub const fn deferredframes(&self) -> &Deferredframes {
        &self.deferredframes
    }
    #[doc = "0x14c - Carrier Sense Errors"]
    #[inline(always)]
    pub const fn crserrs(&self) -> &Crserrs {
        &self.crserrs
    }
    #[doc = "0x150 - Octets Received 31:0"]
    #[inline(always)]
    pub const fn octetsrxedbottom(&self) -> &Octetsrxedbottom {
        &self.octetsrxedbottom
    }
    #[doc = "0x154 - Octets Received 47:32"]
    #[inline(always)]
    pub const fn octetsrxedtop(&self) -> &Octetsrxedtop {
        &self.octetsrxedtop
    }
    #[doc = "0x158 - Frames Received"]
    #[inline(always)]
    pub const fn framesrxedok(&self) -> &Framesrxedok {
        &self.framesrxedok
    }
    #[doc = "0x15c - Broadcast Frames Received"]
    #[inline(always)]
    pub const fn broadcastrxed(&self) -> &Broadcastrxed {
        &self.broadcastrxed
    }
    #[doc = "0x160 - Multicast Frames Received"]
    #[inline(always)]
    pub const fn multicastrxed(&self) -> &Multicastrxed {
        &self.multicastrxed
    }
    #[doc = "0x164 - Pause Frames Received"]
    #[inline(always)]
    pub const fn pframesrxed(&self) -> &Pframesrxed {
        &self.pframesrxed
    }
    #[doc = "0x168 - 64 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed64(&self) -> &Framesrxed64 {
        &self.framesrxed64
    }
    #[doc = "0x16c - 65 to 127 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed65(&self) -> &Framesrxed65 {
        &self.framesrxed65
    }
    #[doc = "0x170 - 128 to 255 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed128(&self) -> &Framesrxed128 {
        &self.framesrxed128
    }
    #[doc = "0x174 - 256 to 511 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed256(&self) -> &Framesrxed256 {
        &self.framesrxed256
    }
    #[doc = "0x178 - 512 to 1023 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed512(&self) -> &Framesrxed512 {
        &self.framesrxed512
    }
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed1024(&self) -> &Framesrxed1024 {
        &self.framesrxed1024
    }
    #[doc = "0x180 - 1519 to maximum Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed1519(&self) -> &Framesrxed1519 {
        &self.framesrxed1519
    }
    #[doc = "0x184 - Undersized Frames Received"]
    #[inline(always)]
    pub const fn undersizeframes(&self) -> &Undersizeframes {
        &self.undersizeframes
    }
    #[doc = "0x188 - Oversize Frames Received"]
    #[inline(always)]
    pub const fn excessiverxlen(&self) -> &Excessiverxlen {
        &self.excessiverxlen
    }
    #[doc = "0x18c - Jabbers Received"]
    #[inline(always)]
    pub const fn rxjabbers(&self) -> &Rxjabbers {
        &self.rxjabbers
    }
    #[doc = "0x190 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub const fn fcserrs(&self) -> &Fcserrs {
        &self.fcserrs
    }
    #[doc = "0x194 - Length Field Frame Errors"]
    #[inline(always)]
    pub const fn rxlenerrs(&self) -> &Rxlenerrs {
        &self.rxlenerrs
    }
    #[doc = "0x198 - Receive Symbol Errors"]
    #[inline(always)]
    pub const fn rxsymbolerrs(&self) -> &Rxsymbolerrs {
        &self.rxsymbolerrs
    }
    #[doc = "0x19c - Alignment Errors"]
    #[inline(always)]
    pub const fn alignerrs(&self) -> &Alignerrs {
        &self.alignerrs
    }
    #[doc = "0x1a0 - Receive Resource Errors"]
    #[inline(always)]
    pub const fn rxresourceerrs(&self) -> &Rxresourceerrs {
        &self.rxresourceerrs
    }
    #[doc = "0x1a4 - Receive Overruns"]
    #[inline(always)]
    pub const fn rxoverruns(&self) -> &Rxoverruns {
        &self.rxoverruns
    }
    #[doc = "0x1a8 - IP Header Checksum Errors"]
    #[inline(always)]
    pub const fn rxipckerrs(&self) -> &Rxipckerrs {
        &self.rxipckerrs
    }
    #[doc = "0x1ac - TCP Checksum Errors"]
    #[inline(always)]
    pub const fn rxtcpckerrs(&self) -> &Rxtcpckerrs {
        &self.rxtcpckerrs
    }
    #[doc = "0x1b0 - UDP Checksum Errors"]
    #[inline(always)]
    pub const fn rxudpckerrs(&self) -> &Rxudpckerrs {
        &self.rxudpckerrs
    }
    #[doc = "0x1b4 - Receive DMA Flushed Packets"]
    #[inline(always)]
    pub const fn autoflushedpkts(&self) -> &Autoflushedpkts {
        &self.autoflushedpkts
    }
    #[doc = "0x1bc - 1588 Timer Increment Register subscript nsec"]
    #[inline(always)]
    pub const fn tsutimerincrsubnsec(&self) -> &Tsutimerincrsubnsec {
        &self.tsutimerincrsubnsec
    }
    #[doc = "0x1c0 - 1588 Timer Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsutimermsbsec(&self) -> &Tsutimermsbsec {
        &self.tsutimermsbsec
    }
    #[doc = "0x1d0 - 1588 Timer Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsutimersec(&self) -> &Tsutimersec {
        &self.tsutimersec
    }
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsutimernsec(&self) -> &Tsutimernsec {
        &self.tsutimernsec
    }
    #[doc = "0x1d8 - This register returns all zeroes when read."]
    #[inline(always)]
    pub const fn tsutimeradjust(&self) -> &Tsutimeradjust {
        &self.tsutimeradjust
    }
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    #[inline(always)]
    pub const fn tsutimerincr(&self) -> &Tsutimerincr {
        &self.tsutimerincr
    }
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsuptptxsec(&self) -> &Tsuptptxsec {
        &self.tsuptptxsec
    }
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsuptptxnsec(&self) -> &Tsuptptxnsec {
        &self.tsuptptxnsec
    }
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsuptprxsec(&self) -> &Tsuptprxsec {
        &self.tsuptprxsec
    }
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsuptprxnsec(&self) -> &Tsuptprxnsec {
        &self.tsuptprxnsec
    }
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsupeertxsec(&self) -> &Tsupeertxsec {
        &self.tsupeertxsec
    }
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsupeertxnsec(&self) -> &Tsupeertxnsec {
        &self.tsupeertxnsec
    }
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsupeerrxsec(&self) -> &Tsupeerrxsec {
        &self.tsupeerrxsec
    }
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsupeerrxnsec(&self) -> &Tsupeerrxnsec {
        &self.tsupeerrxnsec
    }
    #[doc = "0x260 - Transmit Pause Quantum Register 1"]
    #[inline(always)]
    pub const fn txpausequant1(&self) -> &Txpausequant1 {
        &self.txpausequant1
    }
    #[doc = "0x264 - Transmit Pause Quantum Register 2"]
    #[inline(always)]
    pub const fn txpausequant2(&self) -> &Txpausequant2 {
        &self.txpausequant2
    }
    #[doc = "0x268 - Transmit Pause Quantum Register 3"]
    #[inline(always)]
    pub const fn txpausequant3(&self) -> &Txpausequant3 {
        &self.txpausequant3
    }
    #[doc = "0x270 - Received LPI transitions"]
    #[inline(always)]
    pub const fn rxlpi(&self) -> &Rxlpi {
        &self.rxlpi
    }
    #[doc = "0x274 - Received LPI time"]
    #[inline(always)]
    pub const fn rxlpitime(&self) -> &Rxlpitime {
        &self.rxlpitime
    }
    #[doc = "0x278 - Transmit LPI transitions"]
    #[inline(always)]
    pub const fn txlpi(&self) -> &Txlpi {
        &self.txlpi
    }
    #[doc = "0x27c - Transmit LPI time"]
    #[inline(always)]
    pub const fn txlpitime(&self) -> &Txlpitime {
        &self.txlpitime
    }
    #[doc = "0x4cc - TX BD control register"]
    #[inline(always)]
    pub const fn txbdctrl(&self) -> &Txbdctrl {
        &self.txbdctrl
    }
    #[doc = "0x4d0 - RX BD control register"]
    #[inline(always)]
    pub const fn rxbdctrl(&self) -> &Rxbdctrl {
        &self.rxbdctrl
    }
    #[doc = "0xc00 - I/O Route Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0xc04 - I/O Route Location Register 0"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0xc0c - I/O Route Location Register 1"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
    }
    #[doc = "0xc10 - Ethernet control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "NETWORKCTRL (rw) register accessor: Network control register\n\nYou can [`read`](crate::Reg::read) this register and get [`networkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`networkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@networkctrl`] module"]
#[doc(alias = "NETWORKCTRL")]
pub type Networkctrl = crate::Reg<networkctrl::NetworkctrlSpec>;
#[doc = "Network control register"]
pub mod networkctrl;
#[doc = "NETWORKCFG (rw) register accessor: Network configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`networkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`networkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@networkcfg`] module"]
#[doc(alias = "NETWORKCFG")]
pub type Networkcfg = crate::Reg<networkcfg::NetworkcfgSpec>;
#[doc = "Network configuration register"]
pub mod networkcfg;
#[doc = "NETWORKSTATUS (r) register accessor: Network status register\n\nYou can [`read`](crate::Reg::read) this register and get [`networkstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@networkstatus`] module"]
#[doc(alias = "NETWORKSTATUS")]
pub type Networkstatus = crate::Reg<networkstatus::NetworkstatusSpec>;
#[doc = "Network status register"]
pub mod networkstatus;
#[doc = "DMACFG (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`] module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "TXSTATUS (rw) register accessor: Transmit status register\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txstatus`] module"]
#[doc(alias = "TXSTATUS")]
pub type Txstatus = crate::Reg<txstatus::TxstatusSpec>;
#[doc = "Transmit status register"]
pub mod txstatus;
#[doc = "RXQPTR (rw) register accessor: Start address of the receive buffer queue\n\nYou can [`read`](crate::Reg::read) this register and get [`rxqptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxqptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxqptr`] module"]
#[doc(alias = "RXQPTR")]
pub type Rxqptr = crate::Reg<rxqptr::RxqptrSpec>;
#[doc = "Start address of the receive buffer queue"]
pub mod rxqptr;
#[doc = "TXQPTR (rw) register accessor: Start address of the transmit buffer queue\n\nYou can [`read`](crate::Reg::read) this register and get [`txqptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txqptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txqptr`] module"]
#[doc(alias = "TXQPTR")]
pub type Txqptr = crate::Reg<txqptr::TxqptrSpec>;
#[doc = "Start address of the transmit buffer queue"]
pub mod txqptr;
#[doc = "RXSTATUS (rw) register accessor: Receive status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstatus`] module"]
#[doc(alias = "RXSTATUS")]
pub type Rxstatus = crate::Reg<rxstatus::RxstatusSpec>;
#[doc = "Receive status register"]
pub mod rxstatus;
#[doc = "IFCR (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "Interrupt status register"]
pub mod ifcr;
#[doc = "IENS (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iens::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iens`] module"]
#[doc(alias = "IENS")]
pub type Iens = crate::Reg<iens::IensSpec>;
#[doc = "Interrupt Enable Register"]
pub mod iens;
#[doc = "IENC (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienc`] module"]
#[doc(alias = "IENC")]
pub type Ienc = crate::Reg<ienc::IencSpec>;
#[doc = "Interrupt Disable Register"]
pub mod ienc;
#[doc = "IENRO (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienro`] module"]
#[doc(alias = "IENRO")]
pub type Ienro = crate::Reg<ienro::IenroSpec>;
#[doc = "Interrupt mask register"]
pub mod ienro;
#[doc = "PHYMNGMNT (rw) register accessor: PHY management register\n\nYou can [`read`](crate::Reg::read) this register and get [`phymngmnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phymngmnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phymngmnt`] module"]
#[doc(alias = "PHYMNGMNT")]
pub type Phymngmnt = crate::Reg<phymngmnt::PhymngmntSpec>;
#[doc = "PHY management register"]
pub mod phymngmnt;
#[doc = "RXPAUSEQUANT (r) register accessor: Received Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxpausequant::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxpausequant`] module"]
#[doc(alias = "RXPAUSEQUANT")]
pub type Rxpausequant = crate::Reg<rxpausequant::RxpausequantSpec>;
#[doc = "Received Pause Quantum Register"]
pub mod rxpausequant;
#[doc = "TXPAUSEQUANT (rw) register accessor: Transmit Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant`] module"]
#[doc(alias = "TXPAUSEQUANT")]
pub type Txpausequant = crate::Reg<txpausequant::TxpausequantSpec>;
#[doc = "Transmit Pause Quantum Register"]
pub mod txpausequant;
#[doc = "PBUFTXCUTTHRU (rw) register accessor: TX Partial Store and Forward\n\nYou can [`read`](crate::Reg::read) this register and get [`pbuftxcutthru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbuftxcutthru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbuftxcutthru`] module"]
#[doc(alias = "PBUFTXCUTTHRU")]
pub type Pbuftxcutthru = crate::Reg<pbuftxcutthru::PbuftxcutthruSpec>;
#[doc = "TX Partial Store and Forward"]
pub mod pbuftxcutthru;
#[doc = "PBUFRXCUTTHRU (rw) register accessor: RX Partial Store and Forward\n\nYou can [`read`](crate::Reg::read) this register and get [`pbufrxcutthru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbufrxcutthru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbufrxcutthru`] module"]
#[doc(alias = "PBUFRXCUTTHRU")]
pub type Pbufrxcutthru = crate::Reg<pbufrxcutthru::PbufrxcutthruSpec>;
#[doc = "RX Partial Store and Forward"]
pub mod pbufrxcutthru;
#[doc = "JUMBOMAXLEN (rw) register accessor: Maximum Jumbo Frame Size.\n\nYou can [`read`](crate::Reg::read) this register and get [`jumbomaxlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jumbomaxlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jumbomaxlen`] module"]
#[doc(alias = "JUMBOMAXLEN")]
pub type Jumbomaxlen = crate::Reg<jumbomaxlen::JumbomaxlenSpec>;
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbomaxlen;
#[doc = "IMOD (rw) register accessor: Interrupt moderation register\n\nYou can [`read`](crate::Reg::read) this register and get [`imod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imod`] module"]
#[doc(alias = "IMOD")]
pub type Imod = crate::Reg<imod::ImodSpec>;
#[doc = "Interrupt moderation register"]
pub mod imod;
#[doc = "SYSWAKETIME (rw) register accessor: System wake time\n\nYou can [`read`](crate::Reg::read) this register and get [`syswaketime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syswaketime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syswaketime`] module"]
#[doc(alias = "SYSWAKETIME")]
pub type Syswaketime = crate::Reg<syswaketime::SyswaketimeSpec>;
#[doc = "System wake time"]
pub mod syswaketime;
#[doc = "HASHBOTTOM (rw) register accessor: Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hashbottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashbottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashbottom`] module"]
#[doc(alias = "HASHBOTTOM")]
pub type Hashbottom = crate::Reg<hashbottom::HashbottomSpec>;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hashbottom;
#[doc = "HASHTOP (rw) register accessor: Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hashtop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashtop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashtop`] module"]
#[doc(alias = "HASHTOP")]
pub type Hashtop = crate::Reg<hashtop::HashtopSpec>;
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hashtop;
#[doc = "SPECADDR1BOTTOM (rw) register accessor: Specific Address 1 Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr1bottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr1bottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr1bottom`] module"]
#[doc(alias = "SPECADDR1BOTTOM")]
pub type Specaddr1bottom = crate::Reg<specaddr1bottom::Specaddr1bottomSpec>;
#[doc = "Specific Address 1 Bottom"]
pub mod specaddr1bottom;
#[doc = "SPECADDR1TOP (rw) register accessor: Specific Address 1 Top\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr1top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr1top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr1top`] module"]
#[doc(alias = "SPECADDR1TOP")]
pub type Specaddr1top = crate::Reg<specaddr1top::Specaddr1topSpec>;
#[doc = "Specific Address 1 Top"]
pub mod specaddr1top;
#[doc = "SPECADDR2BOTTOM (rw) register accessor: Specific Address 2 Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr2bottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr2bottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr2bottom`] module"]
#[doc(alias = "SPECADDR2BOTTOM")]
pub type Specaddr2bottom = crate::Reg<specaddr2bottom::Specaddr2bottomSpec>;
#[doc = "Specific Address 2 Bottom"]
pub mod specaddr2bottom;
#[doc = "SPECADDR2TOP (rw) register accessor: Specific Address 2 Top\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr2top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr2top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr2top`] module"]
#[doc(alias = "SPECADDR2TOP")]
pub type Specaddr2top = crate::Reg<specaddr2top::Specaddr2topSpec>;
#[doc = "Specific Address 2 Top"]
pub mod specaddr2top;
#[doc = "SPECADDR3BOTTOM (rw) register accessor: Specific Address 3 Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr3bottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr3bottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr3bottom`] module"]
#[doc(alias = "SPECADDR3BOTTOM")]
pub type Specaddr3bottom = crate::Reg<specaddr3bottom::Specaddr3bottomSpec>;
#[doc = "Specific Address 3 Bottom"]
pub mod specaddr3bottom;
#[doc = "SPECADDR3TOP (rw) register accessor: Specific Address 3 Top\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr3top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr3top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr3top`] module"]
#[doc(alias = "SPECADDR3TOP")]
pub type Specaddr3top = crate::Reg<specaddr3top::Specaddr3topSpec>;
#[doc = "Specific Address 3 Top"]
pub mod specaddr3top;
#[doc = "SPECADDR4BOTTOM (rw) register accessor: Specific Address 4 Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr4bottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr4bottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr4bottom`] module"]
#[doc(alias = "SPECADDR4BOTTOM")]
pub type Specaddr4bottom = crate::Reg<specaddr4bottom::Specaddr4bottomSpec>;
#[doc = "Specific Address 4 Bottom"]
pub mod specaddr4bottom;
#[doc = "SPECADDR4TOP (rw) register accessor: Specific Address 4 Top\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr4top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr4top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr4top`] module"]
#[doc(alias = "SPECADDR4TOP")]
pub type Specaddr4top = crate::Reg<specaddr4top::Specaddr4topSpec>;
#[doc = "Specific Address 4 Top"]
pub mod specaddr4top;
#[doc = "SPECTYPE1 (rw) register accessor: Type ID Match 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spectype1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spectype1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype1`] module"]
#[doc(alias = "SPECTYPE1")]
pub type Spectype1 = crate::Reg<spectype1::Spectype1Spec>;
#[doc = "Type ID Match 1"]
pub mod spectype1;
#[doc = "SPECTYPE2 (rw) register accessor: Type ID Match 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spectype2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spectype2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype2`] module"]
#[doc(alias = "SPECTYPE2")]
pub type Spectype2 = crate::Reg<spectype2::Spectype2Spec>;
#[doc = "Type ID Match 2"]
pub mod spectype2;
#[doc = "SPECTYPE3 (rw) register accessor: Type ID Match 3\n\nYou can [`read`](crate::Reg::read) this register and get [`spectype3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spectype3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype3`] module"]
#[doc(alias = "SPECTYPE3")]
pub type Spectype3 = crate::Reg<spectype3::Spectype3Spec>;
#[doc = "Type ID Match 3"]
pub mod spectype3;
#[doc = "SPECTYPE4 (rw) register accessor: Type ID Match 4\n\nYou can [`read`](crate::Reg::read) this register and get [`spectype4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spectype4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype4`] module"]
#[doc(alias = "SPECTYPE4")]
pub type Spectype4 = crate::Reg<spectype4::Spectype4Spec>;
#[doc = "Type ID Match 4"]
pub mod spectype4;
#[doc = "WOLREG (rw) register accessor: Wake on LAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wolreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wolreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wolreg`] module"]
#[doc(alias = "WOLREG")]
pub type Wolreg = crate::Reg<wolreg::WolregSpec>;
#[doc = "Wake on LAN Register"]
pub mod wolreg;
#[doc = "STRETCHRATIO (rw) register accessor: IPG stretch register\n\nYou can [`read`](crate::Reg::read) this register and get [`stretchratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stretchratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stretchratio`] module"]
#[doc(alias = "STRETCHRATIO")]
pub type Stretchratio = crate::Reg<stretchratio::StretchratioSpec>;
#[doc = "IPG stretch register"]
pub mod stretchratio;
#[doc = "STACKEDVLAN (rw) register accessor: Stacked VLAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stackedvlan::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stackedvlan::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stackedvlan`] module"]
#[doc(alias = "STACKEDVLAN")]
pub type Stackedvlan = crate::Reg<stackedvlan::StackedvlanSpec>;
#[doc = "Stacked VLAN Register"]
pub mod stackedvlan;
#[doc = "TXPFCPAUSE (rw) register accessor: Transmit PFC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txpfcpause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpfcpause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpfcpause`] module"]
#[doc(alias = "TXPFCPAUSE")]
pub type Txpfcpause = crate::Reg<txpfcpause::TxpfcpauseSpec>;
#[doc = "Transmit PFC Pause Register"]
pub mod txpfcpause;
#[doc = "MASKADD1BOTTOM (rw) register accessor: Specific Address Mask 1 Bottom 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`maskadd1bottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskadd1bottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskadd1bottom`] module"]
#[doc(alias = "MASKADD1BOTTOM")]
pub type Maskadd1bottom = crate::Reg<maskadd1bottom::Maskadd1bottomSpec>;
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub mod maskadd1bottom;
#[doc = "MASKADD1TOP (rw) register accessor: Specific Address Mask 1 Top 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`maskadd1top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskadd1top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskadd1top`] module"]
#[doc(alias = "MASKADD1TOP")]
pub type Maskadd1top = crate::Reg<maskadd1top::Maskadd1topSpec>;
#[doc = "Specific Address Mask 1 Top 47:32"]
pub mod maskadd1top;
#[doc = "RXPTPUNICAST (rw) register accessor: PTP RX unicast IP destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxptpunicast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxptpunicast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxptpunicast`] module"]
#[doc(alias = "RXPTPUNICAST")]
pub type Rxptpunicast = crate::Reg<rxptpunicast::RxptpunicastSpec>;
#[doc = "PTP RX unicast IP destination address"]
pub mod rxptpunicast;
#[doc = "TXPTPUNICAST (rw) register accessor: PTP TX unicast IP destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`txptpunicast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txptpunicast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txptpunicast`] module"]
#[doc(alias = "TXPTPUNICAST")]
pub type Txptpunicast = crate::Reg<txptpunicast::TxptpunicastSpec>;
#[doc = "PTP TX unicast IP destination address"]
pub mod txptpunicast;
#[doc = "TSUNSECCMP (rw) register accessor: TSU timer comparison value nanoseconds\n\nYou can [`read`](crate::Reg::read) this register and get [`tsunseccmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsunseccmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsunseccmp`] module"]
#[doc(alias = "TSUNSECCMP")]
pub type Tsunseccmp = crate::Reg<tsunseccmp::TsunseccmpSpec>;
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsunseccmp;
#[doc = "TSUSECCMP (rw) register accessor: TSU timer comparison value seconds \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuseccmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsuseccmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuseccmp`] module"]
#[doc(alias = "TSUSECCMP")]
pub type Tsuseccmp = crate::Reg<tsuseccmp::TsuseccmpSpec>;
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub mod tsuseccmp;
#[doc = "TSUMSBSECCMP (rw) register accessor: TSU timer comparison value seconds \\[47:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`tsumsbseccmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsumsbseccmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsumsbseccmp`] module"]
#[doc(alias = "TSUMSBSECCMP")]
pub type Tsumsbseccmp = crate::Reg<tsumsbseccmp::TsumsbseccmpSpec>;
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub mod tsumsbseccmp;
#[doc = "TSUPTPTXMSBSEC (r) register accessor: PTP Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptptxmsbsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptptxmsbsec`] module"]
#[doc(alias = "TSUPTPTXMSBSEC")]
pub type Tsuptptxmsbsec = crate::Reg<tsuptptxmsbsec::TsuptptxmsbsecSpec>;
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub mod tsuptptxmsbsec;
#[doc = "TSUPTPRXMSBSEC (r) register accessor: PTP Event Frame Received Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptprxmsbsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptprxmsbsec`] module"]
#[doc(alias = "TSUPTPRXMSBSEC")]
pub type Tsuptprxmsbsec = crate::Reg<tsuptprxmsbsec::TsuptprxmsbsecSpec>;
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub mod tsuptprxmsbsec;
#[doc = "TSUPEERTXMSBSEC (r) register accessor: PTP Peer Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeertxmsbsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeertxmsbsec`] module"]
#[doc(alias = "TSUPEERTXMSBSEC")]
pub type Tsupeertxmsbsec = crate::Reg<tsupeertxmsbsec::TsupeertxmsbsecSpec>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub mod tsupeertxmsbsec;
#[doc = "TSUPEERRXMSBSEC (r) register accessor: PTP Peer Event Frame Received Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeerrxmsbsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeerrxmsbsec`] module"]
#[doc(alias = "TSUPEERRXMSBSEC")]
pub type Tsupeerrxmsbsec = crate::Reg<tsupeerrxmsbsec::TsupeerrxmsbsecSpec>;
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub mod tsupeerrxmsbsec;
#[doc = "OCTETSTXEDBOTTOM (rw) register accessor: Octets transmitted 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`octetstxedbottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetstxedbottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetstxedbottom`] module"]
#[doc(alias = "OCTETSTXEDBOTTOM")]
pub type Octetstxedbottom = crate::Reg<octetstxedbottom::OctetstxedbottomSpec>;
#[doc = "Octets transmitted 31:0"]
pub mod octetstxedbottom;
#[doc = "OCTETSTXEDTOP (rw) register accessor: Octets Transmitted 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`octetstxedtop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetstxedtop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetstxedtop`] module"]
#[doc(alias = "OCTETSTXEDTOP")]
pub type Octetstxedtop = crate::Reg<octetstxedtop::OctetstxedtopSpec>;
#[doc = "Octets Transmitted 47:32"]
pub mod octetstxedtop;
#[doc = "FRAMESTXEDOK (rw) register accessor: Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxedok::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxedok::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxedok`] module"]
#[doc(alias = "FRAMESTXEDOK")]
pub type Framestxedok = crate::Reg<framestxedok::FramestxedokSpec>;
#[doc = "Frames Transmitted"]
pub mod framestxedok;
#[doc = "BROADCASTTXED (rw) register accessor: Broadcast Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`broadcasttxed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`broadcasttxed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@broadcasttxed`] module"]
#[doc(alias = "BROADCASTTXED")]
pub type Broadcasttxed = crate::Reg<broadcasttxed::BroadcasttxedSpec>;
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcasttxed;
#[doc = "MULTICASTTXED (rw) register accessor: Multicast Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`multicasttxed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multicasttxed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@multicasttxed`] module"]
#[doc(alias = "MULTICASTTXED")]
pub type Multicasttxed = crate::Reg<multicasttxed::MulticasttxedSpec>;
#[doc = "Multicast Frames Transmitted"]
pub mod multicasttxed;
#[doc = "PFRAMESTXED (rw) register accessor: Pause Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`pframestxed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pframestxed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pframestxed`] module"]
#[doc(alias = "PFRAMESTXED")]
pub type Pframestxed = crate::Reg<pframestxed::PframestxedSpec>;
#[doc = "Pause Frames Transmitted"]
pub mod pframestxed;
#[doc = "FRAMESTXED64 (rw) register accessor: 64 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed64`] module"]
#[doc(alias = "FRAMESTXED64")]
pub type Framestxed64 = crate::Reg<framestxed64::Framestxed64Spec>;
#[doc = "64 Byte Frames Transmitted"]
pub mod framestxed64;
#[doc = "FRAMESTXED65 (rw) register accessor: 65 to 127 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed65`] module"]
#[doc(alias = "FRAMESTXED65")]
pub type Framestxed65 = crate::Reg<framestxed65::Framestxed65Spec>;
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod framestxed65;
#[doc = "FRAMESTXED128 (rw) register accessor: 128 to 255 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed128`] module"]
#[doc(alias = "FRAMESTXED128")]
pub type Framestxed128 = crate::Reg<framestxed128::Framestxed128Spec>;
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod framestxed128;
#[doc = "FRAMESTXED256 (rw) register accessor: 256 to 511 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed256::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed256::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed256`] module"]
#[doc(alias = "FRAMESTXED256")]
pub type Framestxed256 = crate::Reg<framestxed256::Framestxed256Spec>;
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod framestxed256;
#[doc = "FRAMESTXED512 (rw) register accessor: 512 to 1023 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed512`] module"]
#[doc(alias = "FRAMESTXED512")]
pub type Framestxed512 = crate::Reg<framestxed512::Framestxed512Spec>;
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod framestxed512;
#[doc = "FRAMESTXED1024 (rw) register accessor: 1024 to 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed1024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed1024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed1024`] module"]
#[doc(alias = "FRAMESTXED1024")]
pub type Framestxed1024 = crate::Reg<framestxed1024::Framestxed1024Spec>;
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod framestxed1024;
#[doc = "FRAMESTXED1519 (rw) register accessor: Greater Than 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed1519::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed1519::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed1519`] module"]
#[doc(alias = "FRAMESTXED1519")]
pub type Framestxed1519 = crate::Reg<framestxed1519::Framestxed1519Spec>;
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod framestxed1519;
#[doc = "TXUNDERRUNS (rw) register accessor: Transmit Under Runs\n\nYou can [`read`](crate::Reg::read) this register and get [`txunderruns::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txunderruns::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txunderruns`] module"]
#[doc(alias = "TXUNDERRUNS")]
pub type Txunderruns = crate::Reg<txunderruns::TxunderrunsSpec>;
#[doc = "Transmit Under Runs"]
pub mod txunderruns;
#[doc = "SINGLECOLS (rw) register accessor: Single Collision Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`singlecols::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlecols::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlecols`] module"]
#[doc(alias = "SINGLECOLS")]
pub type Singlecols = crate::Reg<singlecols::SinglecolsSpec>;
#[doc = "Single Collision Frames"]
pub mod singlecols;
#[doc = "MULTICOLS (rw) register accessor: Multiple Collision Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`multicols::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multicols::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@multicols`] module"]
#[doc(alias = "MULTICOLS")]
pub type Multicols = crate::Reg<multicols::MulticolsSpec>;
#[doc = "Multiple Collision Frames"]
pub mod multicols;
#[doc = "EXCESSCOLS (rw) register accessor: Excessive Collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`excesscols::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`excesscols::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@excesscols`] module"]
#[doc(alias = "EXCESSCOLS")]
pub type Excesscols = crate::Reg<excesscols::ExcesscolsSpec>;
#[doc = "Excessive Collisions"]
pub mod excesscols;
#[doc = "LATECOLS (rw) register accessor: Late Collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`latecols::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`latecols::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@latecols`] module"]
#[doc(alias = "LATECOLS")]
pub type Latecols = crate::Reg<latecols::LatecolsSpec>;
#[doc = "Late Collisions"]
pub mod latecols;
#[doc = "DEFERREDFRAMES (rw) register accessor: Deferred Transmission Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`deferredframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deferredframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deferredframes`] module"]
#[doc(alias = "DEFERREDFRAMES")]
pub type Deferredframes = crate::Reg<deferredframes::DeferredframesSpec>;
#[doc = "Deferred Transmission Frames"]
pub mod deferredframes;
#[doc = "CRSERRS (rw) register accessor: Carrier Sense Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`crserrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crserrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crserrs`] module"]
#[doc(alias = "CRSERRS")]
pub type Crserrs = crate::Reg<crserrs::CrserrsSpec>;
#[doc = "Carrier Sense Errors"]
pub mod crserrs;
#[doc = "OCTETSRXEDBOTTOM (rw) register accessor: Octets Received 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`octetsrxedbottom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetsrxedbottom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetsrxedbottom`] module"]
#[doc(alias = "OCTETSRXEDBOTTOM")]
pub type Octetsrxedbottom = crate::Reg<octetsrxedbottom::OctetsrxedbottomSpec>;
#[doc = "Octets Received 31:0"]
pub mod octetsrxedbottom;
#[doc = "OCTETSRXEDTOP (rw) register accessor: Octets Received 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`octetsrxedtop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetsrxedtop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetsrxedtop`] module"]
#[doc(alias = "OCTETSRXEDTOP")]
pub type Octetsrxedtop = crate::Reg<octetsrxedtop::OctetsrxedtopSpec>;
#[doc = "Octets Received 47:32"]
pub mod octetsrxedtop;
#[doc = "FRAMESRXEDOK (rw) register accessor: Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxedok::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxedok::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxedok`] module"]
#[doc(alias = "FRAMESRXEDOK")]
pub type Framesrxedok = crate::Reg<framesrxedok::FramesrxedokSpec>;
#[doc = "Frames Received"]
pub mod framesrxedok;
#[doc = "BROADCASTRXED (rw) register accessor: Broadcast Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`broadcastrxed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`broadcastrxed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@broadcastrxed`] module"]
#[doc(alias = "BROADCASTRXED")]
pub type Broadcastrxed = crate::Reg<broadcastrxed::BroadcastrxedSpec>;
#[doc = "Broadcast Frames Received"]
pub mod broadcastrxed;
#[doc = "MULTICASTRXED (rw) register accessor: Multicast Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`multicastrxed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multicastrxed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@multicastrxed`] module"]
#[doc(alias = "MULTICASTRXED")]
pub type Multicastrxed = crate::Reg<multicastrxed::MulticastrxedSpec>;
#[doc = "Multicast Frames Received"]
pub mod multicastrxed;
#[doc = "PFRAMESRXED (rw) register accessor: Pause Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`pframesrxed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pframesrxed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pframesrxed`] module"]
#[doc(alias = "PFRAMESRXED")]
pub type Pframesrxed = crate::Reg<pframesrxed::PframesrxedSpec>;
#[doc = "Pause Frames Received"]
pub mod pframesrxed;
#[doc = "FRAMESRXED64 (rw) register accessor: 64 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed64`] module"]
#[doc(alias = "FRAMESRXED64")]
pub type Framesrxed64 = crate::Reg<framesrxed64::Framesrxed64Spec>;
#[doc = "64 Byte Frames Received"]
pub mod framesrxed64;
#[doc = "FRAMESRXED65 (rw) register accessor: 65 to 127 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed65`] module"]
#[doc(alias = "FRAMESRXED65")]
pub type Framesrxed65 = crate::Reg<framesrxed65::Framesrxed65Spec>;
#[doc = "65 to 127 Byte Frames Received"]
pub mod framesrxed65;
#[doc = "FRAMESRXED128 (rw) register accessor: 128 to 255 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed128`] module"]
#[doc(alias = "FRAMESRXED128")]
pub type Framesrxed128 = crate::Reg<framesrxed128::Framesrxed128Spec>;
#[doc = "128 to 255 Byte Frames Received"]
pub mod framesrxed128;
#[doc = "FRAMESRXED256 (rw) register accessor: 256 to 511 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed256::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed256::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed256`] module"]
#[doc(alias = "FRAMESRXED256")]
pub type Framesrxed256 = crate::Reg<framesrxed256::Framesrxed256Spec>;
#[doc = "256 to 511 Byte Frames Received"]
pub mod framesrxed256;
#[doc = "FRAMESRXED512 (rw) register accessor: 512 to 1023 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed512`] module"]
#[doc(alias = "FRAMESRXED512")]
pub type Framesrxed512 = crate::Reg<framesrxed512::Framesrxed512Spec>;
#[doc = "512 to 1023 Byte Frames Received"]
pub mod framesrxed512;
#[doc = "FRAMESRXED1024 (rw) register accessor: 1024 to 1518 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed1024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed1024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed1024`] module"]
#[doc(alias = "FRAMESRXED1024")]
pub type Framesrxed1024 = crate::Reg<framesrxed1024::Framesrxed1024Spec>;
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod framesrxed1024;
#[doc = "FRAMESRXED1519 (rw) register accessor: 1519 to maximum Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed1519::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed1519::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed1519`] module"]
#[doc(alias = "FRAMESRXED1519")]
pub type Framesrxed1519 = crate::Reg<framesrxed1519::Framesrxed1519Spec>;
#[doc = "1519 to maximum Byte Frames Received"]
pub mod framesrxed1519;
#[doc = "UNDERSIZEFRAMES (rw) register accessor: Undersized Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`undersizeframes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`undersizeframes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@undersizeframes`] module"]
#[doc(alias = "UNDERSIZEFRAMES")]
pub type Undersizeframes = crate::Reg<undersizeframes::UndersizeframesSpec>;
#[doc = "Undersized Frames Received"]
pub mod undersizeframes;
#[doc = "EXCESSIVERXLEN (rw) register accessor: Oversize Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`excessiverxlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`excessiverxlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@excessiverxlen`] module"]
#[doc(alias = "EXCESSIVERXLEN")]
pub type Excessiverxlen = crate::Reg<excessiverxlen::ExcessiverxlenSpec>;
#[doc = "Oversize Frames Received"]
pub mod excessiverxlen;
#[doc = "RXJABBERS (rw) register accessor: Jabbers Received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxjabbers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxjabbers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxjabbers`] module"]
#[doc(alias = "RXJABBERS")]
pub type Rxjabbers = crate::Reg<rxjabbers::RxjabbersSpec>;
#[doc = "Jabbers Received"]
pub mod rxjabbers;
#[doc = "FCSERRS (rw) register accessor: Frame Check Sequence Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`fcserrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcserrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcserrs`] module"]
#[doc(alias = "FCSERRS")]
pub type Fcserrs = crate::Reg<fcserrs::FcserrsSpec>;
#[doc = "Frame Check Sequence Errors"]
pub mod fcserrs;
#[doc = "RXLENERRS (rw) register accessor: Length Field Frame Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlenerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxlenerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlenerrs`] module"]
#[doc(alias = "RXLENERRS")]
pub type Rxlenerrs = crate::Reg<rxlenerrs::RxlenerrsSpec>;
#[doc = "Length Field Frame Errors"]
pub mod rxlenerrs;
#[doc = "RXSYMBOLERRS (rw) register accessor: Receive Symbol Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxsymbolerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxsymbolerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxsymbolerrs`] module"]
#[doc(alias = "RXSYMBOLERRS")]
pub type Rxsymbolerrs = crate::Reg<rxsymbolerrs::RxsymbolerrsSpec>;
#[doc = "Receive Symbol Errors"]
pub mod rxsymbolerrs;
#[doc = "ALIGNERRS (rw) register accessor: Alignment Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`alignerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alignerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alignerrs`] module"]
#[doc(alias = "ALIGNERRS")]
pub type Alignerrs = crate::Reg<alignerrs::AlignerrsSpec>;
#[doc = "Alignment Errors"]
pub mod alignerrs;
#[doc = "RXRESOURCEERRS (rw) register accessor: Receive Resource Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxresourceerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxresourceerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxresourceerrs`] module"]
#[doc(alias = "RXRESOURCEERRS")]
pub type Rxresourceerrs = crate::Reg<rxresourceerrs::RxresourceerrsSpec>;
#[doc = "Receive Resource Errors"]
pub mod rxresourceerrs;
#[doc = "RXOVERRUNS (rw) register accessor: Receive Overruns\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoverruns::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxoverruns::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoverruns`] module"]
#[doc(alias = "RXOVERRUNS")]
pub type Rxoverruns = crate::Reg<rxoverruns::RxoverrunsSpec>;
#[doc = "Receive Overruns"]
pub mod rxoverruns;
#[doc = "RXIPCKERRS (rw) register accessor: IP Header Checksum Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxipckerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxipckerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipckerrs`] module"]
#[doc(alias = "RXIPCKERRS")]
pub type Rxipckerrs = crate::Reg<rxipckerrs::RxipckerrsSpec>;
#[doc = "IP Header Checksum Errors"]
pub mod rxipckerrs;
#[doc = "RXTCPCKERRS (rw) register accessor: TCP Checksum Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtcpckerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtcpckerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcpckerrs`] module"]
#[doc(alias = "RXTCPCKERRS")]
pub type Rxtcpckerrs = crate::Reg<rxtcpckerrs::RxtcpckerrsSpec>;
#[doc = "TCP Checksum Errors"]
pub mod rxtcpckerrs;
#[doc = "RXUDPCKERRS (rw) register accessor: UDP Checksum Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxudpckerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxudpckerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudpckerrs`] module"]
#[doc(alias = "RXUDPCKERRS")]
pub type Rxudpckerrs = crate::Reg<rxudpckerrs::RxudpckerrsSpec>;
#[doc = "UDP Checksum Errors"]
pub mod rxudpckerrs;
#[doc = "AUTOFLUSHEDPKTS (rw) register accessor: Receive DMA Flushed Packets\n\nYou can [`read`](crate::Reg::read) this register and get [`autoflushedpkts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoflushedpkts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autoflushedpkts`] module"]
#[doc(alias = "AUTOFLUSHEDPKTS")]
pub type Autoflushedpkts = crate::Reg<autoflushedpkts::AutoflushedpktsSpec>;
#[doc = "Receive DMA Flushed Packets"]
pub mod autoflushedpkts;
#[doc = "TSUTIMERINCRSUBNSEC (rw) register accessor: 1588 Timer Increment Register subscript nsec\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimerincrsubnsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimerincrsubnsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimerincrsubnsec`] module"]
#[doc(alias = "TSUTIMERINCRSUBNSEC")]
pub type Tsutimerincrsubnsec = crate::Reg<tsutimerincrsubnsec::TsutimerincrsubnsecSpec>;
#[doc = "1588 Timer Increment Register subscript nsec"]
pub mod tsutimerincrsubnsec;
#[doc = "TSUTIMERMSBSEC (rw) register accessor: 1588 Timer Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimermsbsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimermsbsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimermsbsec`] module"]
#[doc(alias = "TSUTIMERMSBSEC")]
pub type Tsutimermsbsec = crate::Reg<tsutimermsbsec::TsutimermsbsecSpec>;
#[doc = "1588 Timer Seconds Register 47:32"]
pub mod tsutimermsbsec;
#[doc = "TSUTIMERSEC (rw) register accessor: 1588 Timer Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimersec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimersec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimersec`] module"]
#[doc(alias = "TSUTIMERSEC")]
pub type Tsutimersec = crate::Reg<tsutimersec::TsutimersecSpec>;
#[doc = "1588 Timer Seconds Register 31:0"]
pub mod tsutimersec;
#[doc = "TSUTIMERNSEC (rw) register accessor: 1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimernsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimernsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimernsec`] module"]
#[doc(alias = "TSUTIMERNSEC")]
pub type Tsutimernsec = crate::Reg<tsutimernsec::TsutimernsecSpec>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsutimernsec;
#[doc = "TSUTIMERADJUST (rw) register accessor: This register returns all zeroes when read.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimeradjust::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimeradjust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimeradjust`] module"]
#[doc(alias = "TSUTIMERADJUST")]
pub type Tsutimeradjust = crate::Reg<tsutimeradjust::TsutimeradjustSpec>;
#[doc = "This register returns all zeroes when read."]
pub mod tsutimeradjust;
#[doc = "TSUTIMERINCR (rw) register accessor: 1588 Timer Increment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimerincr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimerincr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimerincr`] module"]
#[doc(alias = "TSUTIMERINCR")]
pub type Tsutimerincr = crate::Reg<tsutimerincr::TsutimerincrSpec>;
#[doc = "1588 Timer Increment Register"]
pub mod tsutimerincr;
#[doc = "TSUPTPTXSEC (r) register accessor: PTP Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptptxsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptptxsec`] module"]
#[doc(alias = "TSUPTPTXSEC")]
pub type Tsuptptxsec = crate::Reg<tsuptptxsec::TsuptptxsecSpec>;
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub mod tsuptptxsec;
#[doc = "TSUPTPTXNSEC (r) register accessor: PTP Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptptxnsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptptxnsec`] module"]
#[doc(alias = "TSUPTPTXNSEC")]
pub type Tsuptptxnsec = crate::Reg<tsuptptxnsec::TsuptptxnsecSpec>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsuptptxnsec;
#[doc = "TSUPTPRXSEC (r) register accessor: PTP Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptprxsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptprxsec`] module"]
#[doc(alias = "TSUPTPRXSEC")]
pub type Tsuptprxsec = crate::Reg<tsuptprxsec::TsuptprxsecSpec>;
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub mod tsuptprxsec;
#[doc = "TSUPTPRXNSEC (r) register accessor: PTP Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptprxnsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptprxnsec`] module"]
#[doc(alias = "TSUPTPRXNSEC")]
pub type Tsuptprxnsec = crate::Reg<tsuptprxnsec::TsuptprxnsecSpec>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsuptprxnsec;
#[doc = "TSUPEERTXSEC (r) register accessor: PTP Peer Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeertxsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeertxsec`] module"]
#[doc(alias = "TSUPEERTXSEC")]
pub type Tsupeertxsec = crate::Reg<tsupeertxsec::TsupeertxsecSpec>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub mod tsupeertxsec;
#[doc = "TSUPEERTXNSEC (r) register accessor: PTP Peer Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeertxnsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeertxnsec`] module"]
#[doc(alias = "TSUPEERTXNSEC")]
pub type Tsupeertxnsec = crate::Reg<tsupeertxnsec::TsupeertxnsecSpec>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsupeertxnsec;
#[doc = "TSUPEERRXSEC (r) register accessor: PTP Peer Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeerrxsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeerrxsec`] module"]
#[doc(alias = "TSUPEERRXSEC")]
pub type Tsupeerrxsec = crate::Reg<tsupeerrxsec::TsupeerrxsecSpec>;
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub mod tsupeerrxsec;
#[doc = "TSUPEERRXNSEC (r) register accessor: PTP Peer Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeerrxnsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeerrxnsec`] module"]
#[doc(alias = "TSUPEERRXNSEC")]
pub type Tsupeerrxnsec = crate::Reg<tsupeerrxnsec::TsupeerrxnsecSpec>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsupeerrxnsec;
#[doc = "TXPAUSEQUANT1 (rw) register accessor: Transmit Pause Quantum Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant1`] module"]
#[doc(alias = "TXPAUSEQUANT1")]
pub type Txpausequant1 = crate::Reg<txpausequant1::Txpausequant1Spec>;
#[doc = "Transmit Pause Quantum Register 1"]
pub mod txpausequant1;
#[doc = "TXPAUSEQUANT2 (rw) register accessor: Transmit Pause Quantum Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant2`] module"]
#[doc(alias = "TXPAUSEQUANT2")]
pub type Txpausequant2 = crate::Reg<txpausequant2::Txpausequant2Spec>;
#[doc = "Transmit Pause Quantum Register 2"]
pub mod txpausequant2;
#[doc = "TXPAUSEQUANT3 (rw) register accessor: Transmit Pause Quantum Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`txpausequant3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpausequant3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant3`] module"]
#[doc(alias = "TXPAUSEQUANT3")]
pub type Txpausequant3 = crate::Reg<txpausequant3::Txpausequant3Spec>;
#[doc = "Transmit Pause Quantum Register 3"]
pub mod txpausequant3;
#[doc = "RXLPI (rw) register accessor: Received LPI transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxlpi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlpi`] module"]
#[doc(alias = "RXLPI")]
pub type Rxlpi = crate::Reg<rxlpi::RxlpiSpec>;
#[doc = "Received LPI transitions"]
pub mod rxlpi;
#[doc = "RXLPITIME (rw) register accessor: Received LPI time\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpitime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxlpitime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlpitime`] module"]
#[doc(alias = "RXLPITIME")]
pub type Rxlpitime = crate::Reg<rxlpitime::RxlpitimeSpec>;
#[doc = "Received LPI time"]
pub mod rxlpitime;
#[doc = "TXLPI (rw) register accessor: Transmit LPI transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txlpi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlpi`] module"]
#[doc(alias = "TXLPI")]
pub type Txlpi = crate::Reg<txlpi::TxlpiSpec>;
#[doc = "Transmit LPI transitions"]
pub mod txlpi;
#[doc = "TXLPITIME (rw) register accessor: Transmit LPI time\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpitime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txlpitime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlpitime`] module"]
#[doc(alias = "TXLPITIME")]
pub type Txlpitime = crate::Reg<txlpitime::TxlpitimeSpec>;
#[doc = "Transmit LPI time"]
pub mod txlpitime;
#[doc = "TXBDCTRL (rw) register accessor: TX BD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbdctrl`] module"]
#[doc(alias = "TXBDCTRL")]
pub type Txbdctrl = crate::Reg<txbdctrl::TxbdctrlSpec>;
#[doc = "TX BD control register"]
pub mod txbdctrl;
#[doc = "RXBDCTRL (rw) register accessor: RX BD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbdctrl`] module"]
#[doc(alias = "RXBDCTRL")]
pub type Rxbdctrl = crate::Reg<rxbdctrl::RxbdctrlSpec>;
#[doc = "RX BD control register"]
pub mod rxbdctrl;
#[doc = "ROUTEPEN (rw) register accessor: I/O Route Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O Route Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Route Location Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`] module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::Routeloc0Spec>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Route Location Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`] module"]
#[doc(alias = "ROUTELOC1")]
pub type Routeloc1 = crate::Reg<routeloc1::Routeloc1Spec>;
#[doc = "I/O Route Location Register 1"]
pub mod routeloc1;
#[doc = "CTRL (rw) register accessor: Ethernet control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Ethernet control register"]
pub mod ctrl;
