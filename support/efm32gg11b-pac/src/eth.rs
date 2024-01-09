#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    networkctrl: NETWORKCTRL,
    networkcfg: NETWORKCFG,
    networkstatus: NETWORKSTATUS,
    _reserved3: [u8; 0x04],
    dmacfg: DMACFG,
    txstatus: TXSTATUS,
    rxqptr: RXQPTR,
    txqptr: TXQPTR,
    rxstatus: RXSTATUS,
    ifcr: IFCR,
    iens: IENS,
    ienc: IENC,
    ienro: IENRO,
    phymngmnt: PHYMNGMNT,
    rxpausequant: RXPAUSEQUANT,
    txpausequant: TXPAUSEQUANT,
    pbuftxcutthru: PBUFTXCUTTHRU,
    pbufrxcutthru: PBUFRXCUTTHRU,
    jumbomaxlen: JUMBOMAXLEN,
    _reserved18: [u8; 0x10],
    imod: IMOD,
    syswaketime: SYSWAKETIME,
    _reserved20: [u8; 0x1c],
    hashbottom: HASHBOTTOM,
    hashtop: HASHTOP,
    specaddr1bottom: SPECADDR1BOTTOM,
    specaddr1top: SPECADDR1TOP,
    specaddr2bottom: SPECADDR2BOTTOM,
    specaddr2top: SPECADDR2TOP,
    specaddr3bottom: SPECADDR3BOTTOM,
    specaddr3top: SPECADDR3TOP,
    specaddr4bottom: SPECADDR4BOTTOM,
    specaddr4top: SPECADDR4TOP,
    spectype1: SPECTYPE1,
    spectype2: SPECTYPE2,
    spectype3: SPECTYPE3,
    spectype4: SPECTYPE4,
    wolreg: WOLREG,
    stretchratio: STRETCHRATIO,
    stackedvlan: STACKEDVLAN,
    txpfcpause: TXPFCPAUSE,
    maskadd1bottom: MASKADD1BOTTOM,
    maskadd1top: MASKADD1TOP,
    _reserved40: [u8; 0x04],
    rxptpunicast: RXPTPUNICAST,
    txptpunicast: TXPTPUNICAST,
    tsunseccmp: TSUNSECCMP,
    tsuseccmp: TSUSECCMP,
    tsumsbseccmp: TSUMSBSECCMP,
    tsuptptxmsbsec: TSUPTPTXMSBSEC,
    tsuptprxmsbsec: TSUPTPRXMSBSEC,
    tsupeertxmsbsec: TSUPEERTXMSBSEC,
    tsupeerrxmsbsec: TSUPEERRXMSBSEC,
    _reserved49: [u8; 0x08],
    octetstxedbottom: OCTETSTXEDBOTTOM,
    octetstxedtop: OCTETSTXEDTOP,
    framestxedok: FRAMESTXEDOK,
    broadcasttxed: BROADCASTTXED,
    multicasttxed: MULTICASTTXED,
    pframestxed: PFRAMESTXED,
    framestxed64: FRAMESTXED64,
    framestxed65: FRAMESTXED65,
    framestxed128: FRAMESTXED128,
    framestxed256: FRAMESTXED256,
    framestxed512: FRAMESTXED512,
    framestxed1024: FRAMESTXED1024,
    framestxed1519: FRAMESTXED1519,
    txunderruns: TXUNDERRUNS,
    singlecols: SINGLECOLS,
    multicols: MULTICOLS,
    excesscols: EXCESSCOLS,
    latecols: LATECOLS,
    deferredframes: DEFERREDFRAMES,
    crserrs: CRSERRS,
    octetsrxedbottom: OCTETSRXEDBOTTOM,
    octetsrxedtop: OCTETSRXEDTOP,
    framesrxedok: FRAMESRXEDOK,
    broadcastrxed: BROADCASTRXED,
    multicastrxed: MULTICASTRXED,
    pframesrxed: PFRAMESRXED,
    framesrxed64: FRAMESRXED64,
    framesrxed65: FRAMESRXED65,
    framesrxed128: FRAMESRXED128,
    framesrxed256: FRAMESRXED256,
    framesrxed512: FRAMESRXED512,
    framesrxed1024: FRAMESRXED1024,
    framesrxed1519: FRAMESRXED1519,
    undersizeframes: UNDERSIZEFRAMES,
    excessiverxlen: EXCESSIVERXLEN,
    rxjabbers: RXJABBERS,
    fcserrs: FCSERRS,
    rxlenerrs: RXLENERRS,
    rxsymbolerrs: RXSYMBOLERRS,
    alignerrs: ALIGNERRS,
    rxresourceerrs: RXRESOURCEERRS,
    rxoverruns: RXOVERRUNS,
    rxipckerrs: RXIPCKERRS,
    rxtcpckerrs: RXTCPCKERRS,
    rxudpckerrs: RXUDPCKERRS,
    autoflushedpkts: AUTOFLUSHEDPKTS,
    _reserved95: [u8; 0x04],
    tsutimerincrsubnsec: TSUTIMERINCRSUBNSEC,
    tsutimermsbsec: TSUTIMERMSBSEC,
    _reserved97: [u8; 0x0c],
    tsutimersec: TSUTIMERSEC,
    tsutimernsec: TSUTIMERNSEC,
    tsutimeradjust: TSUTIMERADJUST,
    tsutimerincr: TSUTIMERINCR,
    tsuptptxsec: TSUPTPTXSEC,
    tsuptptxnsec: TSUPTPTXNSEC,
    tsuptprxsec: TSUPTPRXSEC,
    tsuptprxnsec: TSUPTPRXNSEC,
    tsupeertxsec: TSUPEERTXSEC,
    tsupeertxnsec: TSUPEERTXNSEC,
    tsupeerrxsec: TSUPEERRXSEC,
    tsupeerrxnsec: TSUPEERRXNSEC,
    _reserved109: [u8; 0x60],
    txpausequant1: TXPAUSEQUANT1,
    txpausequant2: TXPAUSEQUANT2,
    txpausequant3: TXPAUSEQUANT3,
    _reserved112: [u8; 0x04],
    rxlpi: RXLPI,
    rxlpitime: RXLPITIME,
    txlpi: TXLPI,
    txlpitime: TXLPITIME,
    _reserved116: [u8; 0x024c],
    txbdctrl: TXBDCTRL,
    rxbdctrl: RXBDCTRL,
    _reserved118: [u8; 0x072c],
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    _reserved120: [u8; 0x04],
    routeloc1: ROUTELOC1,
    ctrl: CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Network control register"]
    #[inline(always)]
    pub const fn networkctrl(&self) -> &NETWORKCTRL {
        &self.networkctrl
    }
    #[doc = "0x04 - Network configuration register"]
    #[inline(always)]
    pub const fn networkcfg(&self) -> &NETWORKCFG {
        &self.networkcfg
    }
    #[doc = "0x08 - Network status register"]
    #[inline(always)]
    pub const fn networkstatus(&self) -> &NETWORKSTATUS {
        &self.networkstatus
    }
    #[doc = "0x10 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &DMACFG {
        &self.dmacfg
    }
    #[doc = "0x14 - Transmit status register"]
    #[inline(always)]
    pub const fn txstatus(&self) -> &TXSTATUS {
        &self.txstatus
    }
    #[doc = "0x18 - Start address of the receive buffer queue"]
    #[inline(always)]
    pub const fn rxqptr(&self) -> &RXQPTR {
        &self.rxqptr
    }
    #[doc = "0x1c - Start address of the transmit buffer queue"]
    #[inline(always)]
    pub const fn txqptr(&self) -> &TXQPTR {
        &self.txqptr
    }
    #[doc = "0x20 - Receive status register"]
    #[inline(always)]
    pub const fn rxstatus(&self) -> &RXSTATUS {
        &self.rxstatus
    }
    #[doc = "0x24 - Interrupt status register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn iens(&self) -> &IENS {
        &self.iens
    }
    #[doc = "0x2c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn ienc(&self) -> &IENC {
        &self.ienc
    }
    #[doc = "0x30 - Interrupt mask register"]
    #[inline(always)]
    pub const fn ienro(&self) -> &IENRO {
        &self.ienro
    }
    #[doc = "0x34 - PHY management register"]
    #[inline(always)]
    pub const fn phymngmnt(&self) -> &PHYMNGMNT {
        &self.phymngmnt
    }
    #[doc = "0x38 - Received Pause Quantum Register"]
    #[inline(always)]
    pub const fn rxpausequant(&self) -> &RXPAUSEQUANT {
        &self.rxpausequant
    }
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    #[inline(always)]
    pub const fn txpausequant(&self) -> &TXPAUSEQUANT {
        &self.txpausequant
    }
    #[doc = "0x40 - TX Partial Store and Forward"]
    #[inline(always)]
    pub const fn pbuftxcutthru(&self) -> &PBUFTXCUTTHRU {
        &self.pbuftxcutthru
    }
    #[doc = "0x44 - RX Partial Store and Forward"]
    #[inline(always)]
    pub const fn pbufrxcutthru(&self) -> &PBUFRXCUTTHRU {
        &self.pbufrxcutthru
    }
    #[doc = "0x48 - Maximum Jumbo Frame Size."]
    #[inline(always)]
    pub const fn jumbomaxlen(&self) -> &JUMBOMAXLEN {
        &self.jumbomaxlen
    }
    #[doc = "0x5c - Interrupt moderation register"]
    #[inline(always)]
    pub const fn imod(&self) -> &IMOD {
        &self.imod
    }
    #[doc = "0x60 - System wake time"]
    #[inline(always)]
    pub const fn syswaketime(&self) -> &SYSWAKETIME {
        &self.syswaketime
    }
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    #[inline(always)]
    pub const fn hashbottom(&self) -> &HASHBOTTOM {
        &self.hashbottom
    }
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    #[inline(always)]
    pub const fn hashtop(&self) -> &HASHTOP {
        &self.hashtop
    }
    #[doc = "0x88 - Specific Address 1 Bottom"]
    #[inline(always)]
    pub const fn specaddr1bottom(&self) -> &SPECADDR1BOTTOM {
        &self.specaddr1bottom
    }
    #[doc = "0x8c - Specific Address 1 Top"]
    #[inline(always)]
    pub const fn specaddr1top(&self) -> &SPECADDR1TOP {
        &self.specaddr1top
    }
    #[doc = "0x90 - Specific Address 2 Bottom"]
    #[inline(always)]
    pub const fn specaddr2bottom(&self) -> &SPECADDR2BOTTOM {
        &self.specaddr2bottom
    }
    #[doc = "0x94 - Specific Address 2 Top"]
    #[inline(always)]
    pub const fn specaddr2top(&self) -> &SPECADDR2TOP {
        &self.specaddr2top
    }
    #[doc = "0x98 - Specific Address 3 Bottom"]
    #[inline(always)]
    pub const fn specaddr3bottom(&self) -> &SPECADDR3BOTTOM {
        &self.specaddr3bottom
    }
    #[doc = "0x9c - Specific Address 3 Top"]
    #[inline(always)]
    pub const fn specaddr3top(&self) -> &SPECADDR3TOP {
        &self.specaddr3top
    }
    #[doc = "0xa0 - Specific Address 4 Bottom"]
    #[inline(always)]
    pub const fn specaddr4bottom(&self) -> &SPECADDR4BOTTOM {
        &self.specaddr4bottom
    }
    #[doc = "0xa4 - Specific Address 4 Top"]
    #[inline(always)]
    pub const fn specaddr4top(&self) -> &SPECADDR4TOP {
        &self.specaddr4top
    }
    #[doc = "0xa8 - Type ID Match 1"]
    #[inline(always)]
    pub const fn spectype1(&self) -> &SPECTYPE1 {
        &self.spectype1
    }
    #[doc = "0xac - Type ID Match 2"]
    #[inline(always)]
    pub const fn spectype2(&self) -> &SPECTYPE2 {
        &self.spectype2
    }
    #[doc = "0xb0 - Type ID Match 3"]
    #[inline(always)]
    pub const fn spectype3(&self) -> &SPECTYPE3 {
        &self.spectype3
    }
    #[doc = "0xb4 - Type ID Match 4"]
    #[inline(always)]
    pub const fn spectype4(&self) -> &SPECTYPE4 {
        &self.spectype4
    }
    #[doc = "0xb8 - Wake on LAN Register"]
    #[inline(always)]
    pub const fn wolreg(&self) -> &WOLREG {
        &self.wolreg
    }
    #[doc = "0xbc - IPG stretch register"]
    #[inline(always)]
    pub const fn stretchratio(&self) -> &STRETCHRATIO {
        &self.stretchratio
    }
    #[doc = "0xc0 - Stacked VLAN Register"]
    #[inline(always)]
    pub const fn stackedvlan(&self) -> &STACKEDVLAN {
        &self.stackedvlan
    }
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    #[inline(always)]
    pub const fn txpfcpause(&self) -> &TXPFCPAUSE {
        &self.txpfcpause
    }
    #[doc = "0xc8 - Specific Address Mask 1 Bottom 31:0"]
    #[inline(always)]
    pub const fn maskadd1bottom(&self) -> &MASKADD1BOTTOM {
        &self.maskadd1bottom
    }
    #[doc = "0xcc - Specific Address Mask 1 Top 47:32"]
    #[inline(always)]
    pub const fn maskadd1top(&self) -> &MASKADD1TOP {
        &self.maskadd1top
    }
    #[doc = "0xd4 - PTP RX unicast IP destination address"]
    #[inline(always)]
    pub const fn rxptpunicast(&self) -> &RXPTPUNICAST {
        &self.rxptpunicast
    }
    #[doc = "0xd8 - PTP TX unicast IP destination address"]
    #[inline(always)]
    pub const fn txptpunicast(&self) -> &TXPTPUNICAST {
        &self.txptpunicast
    }
    #[doc = "0xdc - TSU timer comparison value nanoseconds"]
    #[inline(always)]
    pub const fn tsunseccmp(&self) -> &TSUNSECCMP {
        &self.tsunseccmp
    }
    #[doc = "0xe0 - TSU timer comparison value seconds \\[31:0\\]"]
    #[inline(always)]
    pub const fn tsuseccmp(&self) -> &TSUSECCMP {
        &self.tsuseccmp
    }
    #[doc = "0xe4 - TSU timer comparison value seconds \\[47:32\\]"]
    #[inline(always)]
    pub const fn tsumsbseccmp(&self) -> &TSUMSBSECCMP {
        &self.tsumsbseccmp
    }
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsuptptxmsbsec(&self) -> &TSUPTPTXMSBSEC {
        &self.tsuptptxmsbsec
    }
    #[doc = "0xec - PTP Event Frame Received Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsuptprxmsbsec(&self) -> &TSUPTPRXMSBSEC {
        &self.tsuptprxmsbsec
    }
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsupeertxmsbsec(&self) -> &TSUPEERTXMSBSEC {
        &self.tsupeertxmsbsec
    }
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsupeerrxmsbsec(&self) -> &TSUPEERRXMSBSEC {
        &self.tsupeerrxmsbsec
    }
    #[doc = "0x100 - Octets transmitted 31:0"]
    #[inline(always)]
    pub const fn octetstxedbottom(&self) -> &OCTETSTXEDBOTTOM {
        &self.octetstxedbottom
    }
    #[doc = "0x104 - Octets Transmitted 47:32"]
    #[inline(always)]
    pub const fn octetstxedtop(&self) -> &OCTETSTXEDTOP {
        &self.octetstxedtop
    }
    #[doc = "0x108 - Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxedok(&self) -> &FRAMESTXEDOK {
        &self.framestxedok
    }
    #[doc = "0x10c - Broadcast Frames Transmitted"]
    #[inline(always)]
    pub const fn broadcasttxed(&self) -> &BROADCASTTXED {
        &self.broadcasttxed
    }
    #[doc = "0x110 - Multicast Frames Transmitted"]
    #[inline(always)]
    pub const fn multicasttxed(&self) -> &MULTICASTTXED {
        &self.multicasttxed
    }
    #[doc = "0x114 - Pause Frames Transmitted"]
    #[inline(always)]
    pub const fn pframestxed(&self) -> &PFRAMESTXED {
        &self.pframestxed
    }
    #[doc = "0x118 - 64 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed64(&self) -> &FRAMESTXED64 {
        &self.framestxed64
    }
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed65(&self) -> &FRAMESTXED65 {
        &self.framestxed65
    }
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed128(&self) -> &FRAMESTXED128 {
        &self.framestxed128
    }
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed256(&self) -> &FRAMESTXED256 {
        &self.framestxed256
    }
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed512(&self) -> &FRAMESTXED512 {
        &self.framestxed512
    }
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed1024(&self) -> &FRAMESTXED1024 {
        &self.framestxed1024
    }
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted"]
    #[inline(always)]
    pub const fn framestxed1519(&self) -> &FRAMESTXED1519 {
        &self.framestxed1519
    }
    #[doc = "0x134 - Transmit Under Runs"]
    #[inline(always)]
    pub const fn txunderruns(&self) -> &TXUNDERRUNS {
        &self.txunderruns
    }
    #[doc = "0x138 - Single Collision Frames"]
    #[inline(always)]
    pub const fn singlecols(&self) -> &SINGLECOLS {
        &self.singlecols
    }
    #[doc = "0x13c - Multiple Collision Frames"]
    #[inline(always)]
    pub const fn multicols(&self) -> &MULTICOLS {
        &self.multicols
    }
    #[doc = "0x140 - Excessive Collisions"]
    #[inline(always)]
    pub const fn excesscols(&self) -> &EXCESSCOLS {
        &self.excesscols
    }
    #[doc = "0x144 - Late Collisions"]
    #[inline(always)]
    pub const fn latecols(&self) -> &LATECOLS {
        &self.latecols
    }
    #[doc = "0x148 - Deferred Transmission Frames"]
    #[inline(always)]
    pub const fn deferredframes(&self) -> &DEFERREDFRAMES {
        &self.deferredframes
    }
    #[doc = "0x14c - Carrier Sense Errors"]
    #[inline(always)]
    pub const fn crserrs(&self) -> &CRSERRS {
        &self.crserrs
    }
    #[doc = "0x150 - Octets Received 31:0"]
    #[inline(always)]
    pub const fn octetsrxedbottom(&self) -> &OCTETSRXEDBOTTOM {
        &self.octetsrxedbottom
    }
    #[doc = "0x154 - Octets Received 47:32"]
    #[inline(always)]
    pub const fn octetsrxedtop(&self) -> &OCTETSRXEDTOP {
        &self.octetsrxedtop
    }
    #[doc = "0x158 - Frames Received"]
    #[inline(always)]
    pub const fn framesrxedok(&self) -> &FRAMESRXEDOK {
        &self.framesrxedok
    }
    #[doc = "0x15c - Broadcast Frames Received"]
    #[inline(always)]
    pub const fn broadcastrxed(&self) -> &BROADCASTRXED {
        &self.broadcastrxed
    }
    #[doc = "0x160 - Multicast Frames Received"]
    #[inline(always)]
    pub const fn multicastrxed(&self) -> &MULTICASTRXED {
        &self.multicastrxed
    }
    #[doc = "0x164 - Pause Frames Received"]
    #[inline(always)]
    pub const fn pframesrxed(&self) -> &PFRAMESRXED {
        &self.pframesrxed
    }
    #[doc = "0x168 - 64 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed64(&self) -> &FRAMESRXED64 {
        &self.framesrxed64
    }
    #[doc = "0x16c - 65 to 127 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed65(&self) -> &FRAMESRXED65 {
        &self.framesrxed65
    }
    #[doc = "0x170 - 128 to 255 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed128(&self) -> &FRAMESRXED128 {
        &self.framesrxed128
    }
    #[doc = "0x174 - 256 to 511 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed256(&self) -> &FRAMESRXED256 {
        &self.framesrxed256
    }
    #[doc = "0x178 - 512 to 1023 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed512(&self) -> &FRAMESRXED512 {
        &self.framesrxed512
    }
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed1024(&self) -> &FRAMESRXED1024 {
        &self.framesrxed1024
    }
    #[doc = "0x180 - 1519 to maximum Byte Frames Received"]
    #[inline(always)]
    pub const fn framesrxed1519(&self) -> &FRAMESRXED1519 {
        &self.framesrxed1519
    }
    #[doc = "0x184 - Undersized Frames Received"]
    #[inline(always)]
    pub const fn undersizeframes(&self) -> &UNDERSIZEFRAMES {
        &self.undersizeframes
    }
    #[doc = "0x188 - Oversize Frames Received"]
    #[inline(always)]
    pub const fn excessiverxlen(&self) -> &EXCESSIVERXLEN {
        &self.excessiverxlen
    }
    #[doc = "0x18c - Jabbers Received"]
    #[inline(always)]
    pub const fn rxjabbers(&self) -> &RXJABBERS {
        &self.rxjabbers
    }
    #[doc = "0x190 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub const fn fcserrs(&self) -> &FCSERRS {
        &self.fcserrs
    }
    #[doc = "0x194 - Length Field Frame Errors"]
    #[inline(always)]
    pub const fn rxlenerrs(&self) -> &RXLENERRS {
        &self.rxlenerrs
    }
    #[doc = "0x198 - Receive Symbol Errors"]
    #[inline(always)]
    pub const fn rxsymbolerrs(&self) -> &RXSYMBOLERRS {
        &self.rxsymbolerrs
    }
    #[doc = "0x19c - Alignment Errors"]
    #[inline(always)]
    pub const fn alignerrs(&self) -> &ALIGNERRS {
        &self.alignerrs
    }
    #[doc = "0x1a0 - Receive Resource Errors"]
    #[inline(always)]
    pub const fn rxresourceerrs(&self) -> &RXRESOURCEERRS {
        &self.rxresourceerrs
    }
    #[doc = "0x1a4 - Receive Overruns"]
    #[inline(always)]
    pub const fn rxoverruns(&self) -> &RXOVERRUNS {
        &self.rxoverruns
    }
    #[doc = "0x1a8 - IP Header Checksum Errors"]
    #[inline(always)]
    pub const fn rxipckerrs(&self) -> &RXIPCKERRS {
        &self.rxipckerrs
    }
    #[doc = "0x1ac - TCP Checksum Errors"]
    #[inline(always)]
    pub const fn rxtcpckerrs(&self) -> &RXTCPCKERRS {
        &self.rxtcpckerrs
    }
    #[doc = "0x1b0 - UDP Checksum Errors"]
    #[inline(always)]
    pub const fn rxudpckerrs(&self) -> &RXUDPCKERRS {
        &self.rxudpckerrs
    }
    #[doc = "0x1b4 - Receive DMA Flushed Packets"]
    #[inline(always)]
    pub const fn autoflushedpkts(&self) -> &AUTOFLUSHEDPKTS {
        &self.autoflushedpkts
    }
    #[doc = "0x1bc - 1588 Timer Increment Register subscript nsec"]
    #[inline(always)]
    pub const fn tsutimerincrsubnsec(&self) -> &TSUTIMERINCRSUBNSEC {
        &self.tsutimerincrsubnsec
    }
    #[doc = "0x1c0 - 1588 Timer Seconds Register 47:32"]
    #[inline(always)]
    pub const fn tsutimermsbsec(&self) -> &TSUTIMERMSBSEC {
        &self.tsutimermsbsec
    }
    #[doc = "0x1d0 - 1588 Timer Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsutimersec(&self) -> &TSUTIMERSEC {
        &self.tsutimersec
    }
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsutimernsec(&self) -> &TSUTIMERNSEC {
        &self.tsutimernsec
    }
    #[doc = "0x1d8 - This register returns all zeroes when read."]
    #[inline(always)]
    pub const fn tsutimeradjust(&self) -> &TSUTIMERADJUST {
        &self.tsutimeradjust
    }
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    #[inline(always)]
    pub const fn tsutimerincr(&self) -> &TSUTIMERINCR {
        &self.tsutimerincr
    }
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsuptptxsec(&self) -> &TSUPTPTXSEC {
        &self.tsuptptxsec
    }
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsuptptxnsec(&self) -> &TSUPTPTXNSEC {
        &self.tsuptptxnsec
    }
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsuptprxsec(&self) -> &TSUPTPRXSEC {
        &self.tsuptprxsec
    }
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsuptprxnsec(&self) -> &TSUPTPRXNSEC {
        &self.tsuptprxnsec
    }
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsupeertxsec(&self) -> &TSUPEERTXSEC {
        &self.tsupeertxsec
    }
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsupeertxnsec(&self) -> &TSUPEERTXNSEC {
        &self.tsupeertxnsec
    }
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Register 31:0"]
    #[inline(always)]
    pub const fn tsupeerrxsec(&self) -> &TSUPEERRXSEC {
        &self.tsupeerrxsec
    }
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    #[inline(always)]
    pub const fn tsupeerrxnsec(&self) -> &TSUPEERRXNSEC {
        &self.tsupeerrxnsec
    }
    #[doc = "0x260 - Transmit Pause Quantum Register 1"]
    #[inline(always)]
    pub const fn txpausequant1(&self) -> &TXPAUSEQUANT1 {
        &self.txpausequant1
    }
    #[doc = "0x264 - Transmit Pause Quantum Register 2"]
    #[inline(always)]
    pub const fn txpausequant2(&self) -> &TXPAUSEQUANT2 {
        &self.txpausequant2
    }
    #[doc = "0x268 - Transmit Pause Quantum Register 3"]
    #[inline(always)]
    pub const fn txpausequant3(&self) -> &TXPAUSEQUANT3 {
        &self.txpausequant3
    }
    #[doc = "0x270 - Received LPI transitions"]
    #[inline(always)]
    pub const fn rxlpi(&self) -> &RXLPI {
        &self.rxlpi
    }
    #[doc = "0x274 - Received LPI time"]
    #[inline(always)]
    pub const fn rxlpitime(&self) -> &RXLPITIME {
        &self.rxlpitime
    }
    #[doc = "0x278 - Transmit LPI transitions"]
    #[inline(always)]
    pub const fn txlpi(&self) -> &TXLPI {
        &self.txlpi
    }
    #[doc = "0x27c - Transmit LPI time"]
    #[inline(always)]
    pub const fn txlpitime(&self) -> &TXLPITIME {
        &self.txlpitime
    }
    #[doc = "0x4cc - TX BD control register"]
    #[inline(always)]
    pub const fn txbdctrl(&self) -> &TXBDCTRL {
        &self.txbdctrl
    }
    #[doc = "0x4d0 - RX BD control register"]
    #[inline(always)]
    pub const fn rxbdctrl(&self) -> &RXBDCTRL {
        &self.rxbdctrl
    }
    #[doc = "0xc00 - I/O Route Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0xc04 - I/O Route Location Register 0"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0xc0c - I/O Route Location Register 1"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &ROUTELOC1 {
        &self.routeloc1
    }
    #[doc = "0xc10 - Ethernet control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
}
#[doc = "NETWORKCTRL (rw) register accessor: Network control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`networkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`networkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@networkctrl`]
module"]
pub type NETWORKCTRL = crate::Reg<networkctrl::NETWORKCTRL_SPEC>;
#[doc = "Network control register"]
pub mod networkctrl;
#[doc = "NETWORKCFG (rw) register accessor: Network configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`networkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`networkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@networkcfg`]
module"]
pub type NETWORKCFG = crate::Reg<networkcfg::NETWORKCFG_SPEC>;
#[doc = "Network configuration register"]
pub mod networkcfg;
#[doc = "NETWORKSTATUS (r) register accessor: Network status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`networkstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@networkstatus`]
module"]
pub type NETWORKSTATUS = crate::Reg<networkstatus::NETWORKSTATUS_SPEC>;
#[doc = "Network status register"]
pub mod networkstatus;
#[doc = "DMACFG (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "TXSTATUS (rw) register accessor: Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txstatus`]
module"]
pub type TXSTATUS = crate::Reg<txstatus::TXSTATUS_SPEC>;
#[doc = "Transmit status register"]
pub mod txstatus;
#[doc = "RXQPTR (rw) register accessor: Start address of the receive buffer queue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxqptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxqptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxqptr`]
module"]
pub type RXQPTR = crate::Reg<rxqptr::RXQPTR_SPEC>;
#[doc = "Start address of the receive buffer queue"]
pub mod rxqptr;
#[doc = "TXQPTR (rw) register accessor: Start address of the transmit buffer queue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txqptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txqptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txqptr`]
module"]
pub type TXQPTR = crate::Reg<txqptr::TXQPTR_SPEC>;
#[doc = "Start address of the transmit buffer queue"]
pub mod txqptr;
#[doc = "RXSTATUS (rw) register accessor: Receive status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstatus`]
module"]
pub type RXSTATUS = crate::Reg<rxstatus::RXSTATUS_SPEC>;
#[doc = "Receive status register"]
pub mod rxstatus;
#[doc = "IFCR (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Interrupt status register"]
pub mod ifcr;
#[doc = "IENS (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iens::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iens`]
module"]
pub type IENS = crate::Reg<iens::IENS_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod iens;
#[doc = "IENC (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienc`]
module"]
pub type IENC = crate::Reg<ienc::IENC_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod ienc;
#[doc = "IENRO (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ienro::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienro::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ienro`]
module"]
pub type IENRO = crate::Reg<ienro::IENRO_SPEC>;
#[doc = "Interrupt mask register"]
pub mod ienro;
#[doc = "PHYMNGMNT (rw) register accessor: PHY management register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phymngmnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phymngmnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phymngmnt`]
module"]
pub type PHYMNGMNT = crate::Reg<phymngmnt::PHYMNGMNT_SPEC>;
#[doc = "PHY management register"]
pub mod phymngmnt;
#[doc = "RXPAUSEQUANT (r) register accessor: Received Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpausequant::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxpausequant`]
module"]
pub type RXPAUSEQUANT = crate::Reg<rxpausequant::RXPAUSEQUANT_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod rxpausequant;
#[doc = "TXPAUSEQUANT (rw) register accessor: Transmit Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant`]
module"]
pub type TXPAUSEQUANT = crate::Reg<txpausequant::TXPAUSEQUANT_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod txpausequant;
#[doc = "PBUFTXCUTTHRU (rw) register accessor: TX Partial Store and Forward\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbuftxcutthru::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbuftxcutthru::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbuftxcutthru`]
module"]
pub type PBUFTXCUTTHRU = crate::Reg<pbuftxcutthru::PBUFTXCUTTHRU_SPEC>;
#[doc = "TX Partial Store and Forward"]
pub mod pbuftxcutthru;
#[doc = "PBUFRXCUTTHRU (rw) register accessor: RX Partial Store and Forward\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbufrxcutthru::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbufrxcutthru::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbufrxcutthru`]
module"]
pub type PBUFRXCUTTHRU = crate::Reg<pbufrxcutthru::PBUFRXCUTTHRU_SPEC>;
#[doc = "RX Partial Store and Forward"]
pub mod pbufrxcutthru;
#[doc = "JUMBOMAXLEN (rw) register accessor: Maximum Jumbo Frame Size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jumbomaxlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jumbomaxlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jumbomaxlen`]
module"]
pub type JUMBOMAXLEN = crate::Reg<jumbomaxlen::JUMBOMAXLEN_SPEC>;
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbomaxlen;
#[doc = "IMOD (rw) register accessor: Interrupt moderation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imod`]
module"]
pub type IMOD = crate::Reg<imod::IMOD_SPEC>;
#[doc = "Interrupt moderation register"]
pub mod imod;
#[doc = "SYSWAKETIME (rw) register accessor: System wake time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syswaketime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syswaketime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syswaketime`]
module"]
pub type SYSWAKETIME = crate::Reg<syswaketime::SYSWAKETIME_SPEC>;
#[doc = "System wake time"]
pub mod syswaketime;
#[doc = "HASHBOTTOM (rw) register accessor: Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashbottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashbottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashbottom`]
module"]
pub type HASHBOTTOM = crate::Reg<hashbottom::HASHBOTTOM_SPEC>;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hashbottom;
#[doc = "HASHTOP (rw) register accessor: Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashtop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashtop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashtop`]
module"]
pub type HASHTOP = crate::Reg<hashtop::HASHTOP_SPEC>;
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hashtop;
#[doc = "SPECADDR1BOTTOM (rw) register accessor: Specific Address 1 Bottom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr1bottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr1bottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr1bottom`]
module"]
pub type SPECADDR1BOTTOM = crate::Reg<specaddr1bottom::SPECADDR1BOTTOM_SPEC>;
#[doc = "Specific Address 1 Bottom"]
pub mod specaddr1bottom;
#[doc = "SPECADDR1TOP (rw) register accessor: Specific Address 1 Top\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr1top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr1top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr1top`]
module"]
pub type SPECADDR1TOP = crate::Reg<specaddr1top::SPECADDR1TOP_SPEC>;
#[doc = "Specific Address 1 Top"]
pub mod specaddr1top;
#[doc = "SPECADDR2BOTTOM (rw) register accessor: Specific Address 2 Bottom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr2bottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr2bottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr2bottom`]
module"]
pub type SPECADDR2BOTTOM = crate::Reg<specaddr2bottom::SPECADDR2BOTTOM_SPEC>;
#[doc = "Specific Address 2 Bottom"]
pub mod specaddr2bottom;
#[doc = "SPECADDR2TOP (rw) register accessor: Specific Address 2 Top\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr2top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr2top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr2top`]
module"]
pub type SPECADDR2TOP = crate::Reg<specaddr2top::SPECADDR2TOP_SPEC>;
#[doc = "Specific Address 2 Top"]
pub mod specaddr2top;
#[doc = "SPECADDR3BOTTOM (rw) register accessor: Specific Address 3 Bottom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr3bottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr3bottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr3bottom`]
module"]
pub type SPECADDR3BOTTOM = crate::Reg<specaddr3bottom::SPECADDR3BOTTOM_SPEC>;
#[doc = "Specific Address 3 Bottom"]
pub mod specaddr3bottom;
#[doc = "SPECADDR3TOP (rw) register accessor: Specific Address 3 Top\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr3top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr3top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr3top`]
module"]
pub type SPECADDR3TOP = crate::Reg<specaddr3top::SPECADDR3TOP_SPEC>;
#[doc = "Specific Address 3 Top"]
pub mod specaddr3top;
#[doc = "SPECADDR4BOTTOM (rw) register accessor: Specific Address 4 Bottom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr4bottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr4bottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr4bottom`]
module"]
pub type SPECADDR4BOTTOM = crate::Reg<specaddr4bottom::SPECADDR4BOTTOM_SPEC>;
#[doc = "Specific Address 4 Bottom"]
pub mod specaddr4bottom;
#[doc = "SPECADDR4TOP (rw) register accessor: Specific Address 4 Top\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`specaddr4top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`specaddr4top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@specaddr4top`]
module"]
pub type SPECADDR4TOP = crate::Reg<specaddr4top::SPECADDR4TOP_SPEC>;
#[doc = "Specific Address 4 Top"]
pub mod specaddr4top;
#[doc = "SPECTYPE1 (rw) register accessor: Type ID Match 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectype1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectype1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype1`]
module"]
pub type SPECTYPE1 = crate::Reg<spectype1::SPECTYPE1_SPEC>;
#[doc = "Type ID Match 1"]
pub mod spectype1;
#[doc = "SPECTYPE2 (rw) register accessor: Type ID Match 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectype2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectype2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype2`]
module"]
pub type SPECTYPE2 = crate::Reg<spectype2::SPECTYPE2_SPEC>;
#[doc = "Type ID Match 2"]
pub mod spectype2;
#[doc = "SPECTYPE3 (rw) register accessor: Type ID Match 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectype3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectype3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype3`]
module"]
pub type SPECTYPE3 = crate::Reg<spectype3::SPECTYPE3_SPEC>;
#[doc = "Type ID Match 3"]
pub mod spectype3;
#[doc = "SPECTYPE4 (rw) register accessor: Type ID Match 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectype4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectype4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectype4`]
module"]
pub type SPECTYPE4 = crate::Reg<spectype4::SPECTYPE4_SPEC>;
#[doc = "Type ID Match 4"]
pub mod spectype4;
#[doc = "WOLREG (rw) register accessor: Wake on LAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wolreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wolreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wolreg`]
module"]
pub type WOLREG = crate::Reg<wolreg::WOLREG_SPEC>;
#[doc = "Wake on LAN Register"]
pub mod wolreg;
#[doc = "STRETCHRATIO (rw) register accessor: IPG stretch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stretchratio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stretchratio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stretchratio`]
module"]
pub type STRETCHRATIO = crate::Reg<stretchratio::STRETCHRATIO_SPEC>;
#[doc = "IPG stretch register"]
pub mod stretchratio;
#[doc = "STACKEDVLAN (rw) register accessor: Stacked VLAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stackedvlan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stackedvlan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stackedvlan`]
module"]
pub type STACKEDVLAN = crate::Reg<stackedvlan::STACKEDVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod stackedvlan;
#[doc = "TXPFCPAUSE (rw) register accessor: Transmit PFC Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpfcpause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpfcpause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpfcpause`]
module"]
pub type TXPFCPAUSE = crate::Reg<txpfcpause::TXPFCPAUSE_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod txpfcpause;
#[doc = "MASKADD1BOTTOM (rw) register accessor: Specific Address Mask 1 Bottom 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskadd1bottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskadd1bottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskadd1bottom`]
module"]
pub type MASKADD1BOTTOM = crate::Reg<maskadd1bottom::MASKADD1BOTTOM_SPEC>;
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub mod maskadd1bottom;
#[doc = "MASKADD1TOP (rw) register accessor: Specific Address Mask 1 Top 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskadd1top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskadd1top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskadd1top`]
module"]
pub type MASKADD1TOP = crate::Reg<maskadd1top::MASKADD1TOP_SPEC>;
#[doc = "Specific Address Mask 1 Top 47:32"]
pub mod maskadd1top;
#[doc = "RXPTPUNICAST (rw) register accessor: PTP RX unicast IP destination address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxptpunicast::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxptpunicast::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxptpunicast`]
module"]
pub type RXPTPUNICAST = crate::Reg<rxptpunicast::RXPTPUNICAST_SPEC>;
#[doc = "PTP RX unicast IP destination address"]
pub mod rxptpunicast;
#[doc = "TXPTPUNICAST (rw) register accessor: PTP TX unicast IP destination address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txptpunicast::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txptpunicast::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txptpunicast`]
module"]
pub type TXPTPUNICAST = crate::Reg<txptpunicast::TXPTPUNICAST_SPEC>;
#[doc = "PTP TX unicast IP destination address"]
pub mod txptpunicast;
#[doc = "TSUNSECCMP (rw) register accessor: TSU timer comparison value nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsunseccmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsunseccmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsunseccmp`]
module"]
pub type TSUNSECCMP = crate::Reg<tsunseccmp::TSUNSECCMP_SPEC>;
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsunseccmp;
#[doc = "TSUSECCMP (rw) register accessor: TSU timer comparison value seconds \\[31:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuseccmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsuseccmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuseccmp`]
module"]
pub type TSUSECCMP = crate::Reg<tsuseccmp::TSUSECCMP_SPEC>;
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub mod tsuseccmp;
#[doc = "TSUMSBSECCMP (rw) register accessor: TSU timer comparison value seconds \\[47:32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsumsbseccmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsumsbseccmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsumsbseccmp`]
module"]
pub type TSUMSBSECCMP = crate::Reg<tsumsbseccmp::TSUMSBSECCMP_SPEC>;
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub mod tsumsbseccmp;
#[doc = "TSUPTPTXMSBSEC (r) register accessor: PTP Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptptxmsbsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptptxmsbsec`]
module"]
pub type TSUPTPTXMSBSEC = crate::Reg<tsuptptxmsbsec::TSUPTPTXMSBSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub mod tsuptptxmsbsec;
#[doc = "TSUPTPRXMSBSEC (r) register accessor: PTP Event Frame Received Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptprxmsbsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptprxmsbsec`]
module"]
pub type TSUPTPRXMSBSEC = crate::Reg<tsuptprxmsbsec::TSUPTPRXMSBSEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub mod tsuptprxmsbsec;
#[doc = "TSUPEERTXMSBSEC (r) register accessor: PTP Peer Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeertxmsbsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeertxmsbsec`]
module"]
pub type TSUPEERTXMSBSEC = crate::Reg<tsupeertxmsbsec::TSUPEERTXMSBSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub mod tsupeertxmsbsec;
#[doc = "TSUPEERRXMSBSEC (r) register accessor: PTP Peer Event Frame Received Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeerrxmsbsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeerrxmsbsec`]
module"]
pub type TSUPEERRXMSBSEC = crate::Reg<tsupeerrxmsbsec::TSUPEERRXMSBSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub mod tsupeerrxmsbsec;
#[doc = "OCTETSTXEDBOTTOM (rw) register accessor: Octets transmitted 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octetstxedbottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octetstxedbottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetstxedbottom`]
module"]
pub type OCTETSTXEDBOTTOM = crate::Reg<octetstxedbottom::OCTETSTXEDBOTTOM_SPEC>;
#[doc = "Octets transmitted 31:0"]
pub mod octetstxedbottom;
#[doc = "OCTETSTXEDTOP (rw) register accessor: Octets Transmitted 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octetstxedtop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octetstxedtop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetstxedtop`]
module"]
pub type OCTETSTXEDTOP = crate::Reg<octetstxedtop::OCTETSTXEDTOP_SPEC>;
#[doc = "Octets Transmitted 47:32"]
pub mod octetstxedtop;
#[doc = "FRAMESTXEDOK (rw) register accessor: Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxedok::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxedok::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxedok`]
module"]
pub type FRAMESTXEDOK = crate::Reg<framestxedok::FRAMESTXEDOK_SPEC>;
#[doc = "Frames Transmitted"]
pub mod framestxedok;
#[doc = "BROADCASTTXED (rw) register accessor: Broadcast Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`broadcasttxed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`broadcasttxed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@broadcasttxed`]
module"]
pub type BROADCASTTXED = crate::Reg<broadcasttxed::BROADCASTTXED_SPEC>;
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcasttxed;
#[doc = "MULTICASTTXED (rw) register accessor: Multicast Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`multicasttxed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`multicasttxed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@multicasttxed`]
module"]
pub type MULTICASTTXED = crate::Reg<multicasttxed::MULTICASTTXED_SPEC>;
#[doc = "Multicast Frames Transmitted"]
pub mod multicasttxed;
#[doc = "PFRAMESTXED (rw) register accessor: Pause Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pframestxed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pframestxed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pframestxed`]
module"]
pub type PFRAMESTXED = crate::Reg<pframestxed::PFRAMESTXED_SPEC>;
#[doc = "Pause Frames Transmitted"]
pub mod pframestxed;
#[doc = "FRAMESTXED64 (rw) register accessor: 64 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed64`]
module"]
pub type FRAMESTXED64 = crate::Reg<framestxed64::FRAMESTXED64_SPEC>;
#[doc = "64 Byte Frames Transmitted"]
pub mod framestxed64;
#[doc = "FRAMESTXED65 (rw) register accessor: 65 to 127 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed65`]
module"]
pub type FRAMESTXED65 = crate::Reg<framestxed65::FRAMESTXED65_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod framestxed65;
#[doc = "FRAMESTXED128 (rw) register accessor: 128 to 255 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed128`]
module"]
pub type FRAMESTXED128 = crate::Reg<framestxed128::FRAMESTXED128_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod framestxed128;
#[doc = "FRAMESTXED256 (rw) register accessor: 256 to 511 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed256::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed256::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed256`]
module"]
pub type FRAMESTXED256 = crate::Reg<framestxed256::FRAMESTXED256_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod framestxed256;
#[doc = "FRAMESTXED512 (rw) register accessor: 512 to 1023 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed512::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed512::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed512`]
module"]
pub type FRAMESTXED512 = crate::Reg<framestxed512::FRAMESTXED512_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod framestxed512;
#[doc = "FRAMESTXED1024 (rw) register accessor: 1024 to 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed1024::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed1024::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed1024`]
module"]
pub type FRAMESTXED1024 = crate::Reg<framestxed1024::FRAMESTXED1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod framestxed1024;
#[doc = "FRAMESTXED1519 (rw) register accessor: Greater Than 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed1519::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed1519::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framestxed1519`]
module"]
pub type FRAMESTXED1519 = crate::Reg<framestxed1519::FRAMESTXED1519_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod framestxed1519;
#[doc = "TXUNDERRUNS (rw) register accessor: Transmit Under Runs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txunderruns::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txunderruns::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txunderruns`]
module"]
pub type TXUNDERRUNS = crate::Reg<txunderruns::TXUNDERRUNS_SPEC>;
#[doc = "Transmit Under Runs"]
pub mod txunderruns;
#[doc = "SINGLECOLS (rw) register accessor: Single Collision Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlecols::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlecols::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlecols`]
module"]
pub type SINGLECOLS = crate::Reg<singlecols::SINGLECOLS_SPEC>;
#[doc = "Single Collision Frames"]
pub mod singlecols;
#[doc = "MULTICOLS (rw) register accessor: Multiple Collision Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`multicols::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`multicols::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@multicols`]
module"]
pub type MULTICOLS = crate::Reg<multicols::MULTICOLS_SPEC>;
#[doc = "Multiple Collision Frames"]
pub mod multicols;
#[doc = "EXCESSCOLS (rw) register accessor: Excessive Collisions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`excesscols::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`excesscols::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@excesscols`]
module"]
pub type EXCESSCOLS = crate::Reg<excesscols::EXCESSCOLS_SPEC>;
#[doc = "Excessive Collisions"]
pub mod excesscols;
#[doc = "LATECOLS (rw) register accessor: Late Collisions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`latecols::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`latecols::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@latecols`]
module"]
pub type LATECOLS = crate::Reg<latecols::LATECOLS_SPEC>;
#[doc = "Late Collisions"]
pub mod latecols;
#[doc = "DEFERREDFRAMES (rw) register accessor: Deferred Transmission Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deferredframes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deferredframes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deferredframes`]
module"]
pub type DEFERREDFRAMES = crate::Reg<deferredframes::DEFERREDFRAMES_SPEC>;
#[doc = "Deferred Transmission Frames"]
pub mod deferredframes;
#[doc = "CRSERRS (rw) register accessor: Carrier Sense Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crserrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crserrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crserrs`]
module"]
pub type CRSERRS = crate::Reg<crserrs::CRSERRS_SPEC>;
#[doc = "Carrier Sense Errors"]
pub mod crserrs;
#[doc = "OCTETSRXEDBOTTOM (rw) register accessor: Octets Received 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octetsrxedbottom::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octetsrxedbottom::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetsrxedbottom`]
module"]
pub type OCTETSRXEDBOTTOM = crate::Reg<octetsrxedbottom::OCTETSRXEDBOTTOM_SPEC>;
#[doc = "Octets Received 31:0"]
pub mod octetsrxedbottom;
#[doc = "OCTETSRXEDTOP (rw) register accessor: Octets Received 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octetsrxedtop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octetsrxedtop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octetsrxedtop`]
module"]
pub type OCTETSRXEDTOP = crate::Reg<octetsrxedtop::OCTETSRXEDTOP_SPEC>;
#[doc = "Octets Received 47:32"]
pub mod octetsrxedtop;
#[doc = "FRAMESRXEDOK (rw) register accessor: Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxedok::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxedok::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxedok`]
module"]
pub type FRAMESRXEDOK = crate::Reg<framesrxedok::FRAMESRXEDOK_SPEC>;
#[doc = "Frames Received"]
pub mod framesrxedok;
#[doc = "BROADCASTRXED (rw) register accessor: Broadcast Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`broadcastrxed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`broadcastrxed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@broadcastrxed`]
module"]
pub type BROADCASTRXED = crate::Reg<broadcastrxed::BROADCASTRXED_SPEC>;
#[doc = "Broadcast Frames Received"]
pub mod broadcastrxed;
#[doc = "MULTICASTRXED (rw) register accessor: Multicast Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`multicastrxed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`multicastrxed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@multicastrxed`]
module"]
pub type MULTICASTRXED = crate::Reg<multicastrxed::MULTICASTRXED_SPEC>;
#[doc = "Multicast Frames Received"]
pub mod multicastrxed;
#[doc = "PFRAMESRXED (rw) register accessor: Pause Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pframesrxed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pframesrxed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pframesrxed`]
module"]
pub type PFRAMESRXED = crate::Reg<pframesrxed::PFRAMESRXED_SPEC>;
#[doc = "Pause Frames Received"]
pub mod pframesrxed;
#[doc = "FRAMESRXED64 (rw) register accessor: 64 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed64`]
module"]
pub type FRAMESRXED64 = crate::Reg<framesrxed64::FRAMESRXED64_SPEC>;
#[doc = "64 Byte Frames Received"]
pub mod framesrxed64;
#[doc = "FRAMESRXED65 (rw) register accessor: 65 to 127 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed65`]
module"]
pub type FRAMESRXED65 = crate::Reg<framesrxed65::FRAMESRXED65_SPEC>;
#[doc = "65 to 127 Byte Frames Received"]
pub mod framesrxed65;
#[doc = "FRAMESRXED128 (rw) register accessor: 128 to 255 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed128`]
module"]
pub type FRAMESRXED128 = crate::Reg<framesrxed128::FRAMESRXED128_SPEC>;
#[doc = "128 to 255 Byte Frames Received"]
pub mod framesrxed128;
#[doc = "FRAMESRXED256 (rw) register accessor: 256 to 511 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed256::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed256::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed256`]
module"]
pub type FRAMESRXED256 = crate::Reg<framesrxed256::FRAMESRXED256_SPEC>;
#[doc = "256 to 511 Byte Frames Received"]
pub mod framesrxed256;
#[doc = "FRAMESRXED512 (rw) register accessor: 512 to 1023 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed512::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed512::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed512`]
module"]
pub type FRAMESRXED512 = crate::Reg<framesrxed512::FRAMESRXED512_SPEC>;
#[doc = "512 to 1023 Byte Frames Received"]
pub mod framesrxed512;
#[doc = "FRAMESRXED1024 (rw) register accessor: 1024 to 1518 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed1024::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed1024::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed1024`]
module"]
pub type FRAMESRXED1024 = crate::Reg<framesrxed1024::FRAMESRXED1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod framesrxed1024;
#[doc = "FRAMESRXED1519 (rw) register accessor: 1519 to maximum Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed1519::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed1519::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framesrxed1519`]
module"]
pub type FRAMESRXED1519 = crate::Reg<framesrxed1519::FRAMESRXED1519_SPEC>;
#[doc = "1519 to maximum Byte Frames Received"]
pub mod framesrxed1519;
#[doc = "UNDERSIZEFRAMES (rw) register accessor: Undersized Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`undersizeframes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`undersizeframes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@undersizeframes`]
module"]
pub type UNDERSIZEFRAMES = crate::Reg<undersizeframes::UNDERSIZEFRAMES_SPEC>;
#[doc = "Undersized Frames Received"]
pub mod undersizeframes;
#[doc = "EXCESSIVERXLEN (rw) register accessor: Oversize Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`excessiverxlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`excessiverxlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@excessiverxlen`]
module"]
pub type EXCESSIVERXLEN = crate::Reg<excessiverxlen::EXCESSIVERXLEN_SPEC>;
#[doc = "Oversize Frames Received"]
pub mod excessiverxlen;
#[doc = "RXJABBERS (rw) register accessor: Jabbers Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxjabbers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxjabbers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxjabbers`]
module"]
pub type RXJABBERS = crate::Reg<rxjabbers::RXJABBERS_SPEC>;
#[doc = "Jabbers Received"]
pub mod rxjabbers;
#[doc = "FCSERRS (rw) register accessor: Frame Check Sequence Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcserrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcserrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcserrs`]
module"]
pub type FCSERRS = crate::Reg<fcserrs::FCSERRS_SPEC>;
#[doc = "Frame Check Sequence Errors"]
pub mod fcserrs;
#[doc = "RXLENERRS (rw) register accessor: Length Field Frame Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlenerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxlenerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlenerrs`]
module"]
pub type RXLENERRS = crate::Reg<rxlenerrs::RXLENERRS_SPEC>;
#[doc = "Length Field Frame Errors"]
pub mod rxlenerrs;
#[doc = "RXSYMBOLERRS (rw) register accessor: Receive Symbol Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxsymbolerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxsymbolerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxsymbolerrs`]
module"]
pub type RXSYMBOLERRS = crate::Reg<rxsymbolerrs::RXSYMBOLERRS_SPEC>;
#[doc = "Receive Symbol Errors"]
pub mod rxsymbolerrs;
#[doc = "ALIGNERRS (rw) register accessor: Alignment Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alignerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alignerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alignerrs`]
module"]
pub type ALIGNERRS = crate::Reg<alignerrs::ALIGNERRS_SPEC>;
#[doc = "Alignment Errors"]
pub mod alignerrs;
#[doc = "RXRESOURCEERRS (rw) register accessor: Receive Resource Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxresourceerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxresourceerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxresourceerrs`]
module"]
pub type RXRESOURCEERRS = crate::Reg<rxresourceerrs::RXRESOURCEERRS_SPEC>;
#[doc = "Receive Resource Errors"]
pub mod rxresourceerrs;
#[doc = "RXOVERRUNS (rw) register accessor: Receive Overruns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoverruns::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxoverruns::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoverruns`]
module"]
pub type RXOVERRUNS = crate::Reg<rxoverruns::RXOVERRUNS_SPEC>;
#[doc = "Receive Overruns"]
pub mod rxoverruns;
#[doc = "RXIPCKERRS (rw) register accessor: IP Header Checksum Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipckerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxipckerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipckerrs`]
module"]
pub type RXIPCKERRS = crate::Reg<rxipckerrs::RXIPCKERRS_SPEC>;
#[doc = "IP Header Checksum Errors"]
pub mod rxipckerrs;
#[doc = "RXTCPCKERRS (rw) register accessor: TCP Checksum Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcpckerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtcpckerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcpckerrs`]
module"]
pub type RXTCPCKERRS = crate::Reg<rxtcpckerrs::RXTCPCKERRS_SPEC>;
#[doc = "TCP Checksum Errors"]
pub mod rxtcpckerrs;
#[doc = "RXUDPCKERRS (rw) register accessor: UDP Checksum Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudpckerrs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxudpckerrs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudpckerrs`]
module"]
pub type RXUDPCKERRS = crate::Reg<rxudpckerrs::RXUDPCKERRS_SPEC>;
#[doc = "UDP Checksum Errors"]
pub mod rxudpckerrs;
#[doc = "AUTOFLUSHEDPKTS (rw) register accessor: Receive DMA Flushed Packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autoflushedpkts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autoflushedpkts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autoflushedpkts`]
module"]
pub type AUTOFLUSHEDPKTS = crate::Reg<autoflushedpkts::AUTOFLUSHEDPKTS_SPEC>;
#[doc = "Receive DMA Flushed Packets"]
pub mod autoflushedpkts;
#[doc = "TSUTIMERINCRSUBNSEC (rw) register accessor: 1588 Timer Increment Register subscript nsec\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimerincrsubnsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimerincrsubnsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimerincrsubnsec`]
module"]
pub type TSUTIMERINCRSUBNSEC = crate::Reg<tsutimerincrsubnsec::TSUTIMERINCRSUBNSEC_SPEC>;
#[doc = "1588 Timer Increment Register subscript nsec"]
pub mod tsutimerincrsubnsec;
#[doc = "TSUTIMERMSBSEC (rw) register accessor: 1588 Timer Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimermsbsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimermsbsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimermsbsec`]
module"]
pub type TSUTIMERMSBSEC = crate::Reg<tsutimermsbsec::TSUTIMERMSBSEC_SPEC>;
#[doc = "1588 Timer Seconds Register 47:32"]
pub mod tsutimermsbsec;
#[doc = "TSUTIMERSEC (rw) register accessor: 1588 Timer Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimersec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimersec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimersec`]
module"]
pub type TSUTIMERSEC = crate::Reg<tsutimersec::TSUTIMERSEC_SPEC>;
#[doc = "1588 Timer Seconds Register 31:0"]
pub mod tsutimersec;
#[doc = "TSUTIMERNSEC (rw) register accessor: 1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimernsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimernsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimernsec`]
module"]
pub type TSUTIMERNSEC = crate::Reg<tsutimernsec::TSUTIMERNSEC_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsutimernsec;
#[doc = "TSUTIMERADJUST (rw) register accessor: This register returns all zeroes when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimeradjust::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimeradjust::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimeradjust`]
module"]
pub type TSUTIMERADJUST = crate::Reg<tsutimeradjust::TSUTIMERADJUST_SPEC>;
#[doc = "This register returns all zeroes when read."]
pub mod tsutimeradjust;
#[doc = "TSUTIMERINCR (rw) register accessor: 1588 Timer Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimerincr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimerincr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsutimerincr`]
module"]
pub type TSUTIMERINCR = crate::Reg<tsutimerincr::TSUTIMERINCR_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod tsutimerincr;
#[doc = "TSUPTPTXSEC (r) register accessor: PTP Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptptxsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptptxsec`]
module"]
pub type TSUPTPTXSEC = crate::Reg<tsuptptxsec::TSUPTPTXSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub mod tsuptptxsec;
#[doc = "TSUPTPTXNSEC (r) register accessor: PTP Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptptxnsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptptxnsec`]
module"]
pub type TSUPTPTXNSEC = crate::Reg<tsuptptxnsec::TSUPTPTXNSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsuptptxnsec;
#[doc = "TSUPTPRXSEC (r) register accessor: PTP Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptprxsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptprxsec`]
module"]
pub type TSUPTPRXSEC = crate::Reg<tsuptprxsec::TSUPTPRXSEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub mod tsuptprxsec;
#[doc = "TSUPTPRXNSEC (r) register accessor: PTP Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptprxnsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsuptprxnsec`]
module"]
pub type TSUPTPRXNSEC = crate::Reg<tsuptprxnsec::TSUPTPRXNSEC_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsuptprxnsec;
#[doc = "TSUPEERTXSEC (r) register accessor: PTP Peer Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeertxsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeertxsec`]
module"]
pub type TSUPEERTXSEC = crate::Reg<tsupeertxsec::TSUPEERTXSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub mod tsupeertxsec;
#[doc = "TSUPEERTXNSEC (r) register accessor: PTP Peer Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeertxnsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeertxnsec`]
module"]
pub type TSUPEERTXNSEC = crate::Reg<tsupeertxnsec::TSUPEERTXNSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsupeertxnsec;
#[doc = "TSUPEERRXSEC (r) register accessor: PTP Peer Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeerrxsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeerrxsec`]
module"]
pub type TSUPEERRXSEC = crate::Reg<tsupeerrxsec::TSUPEERRXSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub mod tsupeerrxsec;
#[doc = "TSUPEERRXNSEC (r) register accessor: PTP Peer Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeerrxnsec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsupeerrxnsec`]
module"]
pub type TSUPEERRXNSEC = crate::Reg<tsupeerrxnsec::TSUPEERRXNSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsupeerrxnsec;
#[doc = "TXPAUSEQUANT1 (rw) register accessor: Transmit Pause Quantum Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant1`]
module"]
pub type TXPAUSEQUANT1 = crate::Reg<txpausequant1::TXPAUSEQUANT1_SPEC>;
#[doc = "Transmit Pause Quantum Register 1"]
pub mod txpausequant1;
#[doc = "TXPAUSEQUANT2 (rw) register accessor: Transmit Pause Quantum Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant2`]
module"]
pub type TXPAUSEQUANT2 = crate::Reg<txpausequant2::TXPAUSEQUANT2_SPEC>;
#[doc = "Transmit Pause Quantum Register 2"]
pub mod txpausequant2;
#[doc = "TXPAUSEQUANT3 (rw) register accessor: Transmit Pause Quantum Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpausequant3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpausequant3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpausequant3`]
module"]
pub type TXPAUSEQUANT3 = crate::Reg<txpausequant3::TXPAUSEQUANT3_SPEC>;
#[doc = "Transmit Pause Quantum Register 3"]
pub mod txpausequant3;
#[doc = "RXLPI (rw) register accessor: Received LPI transitions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlpi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxlpi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlpi`]
module"]
pub type RXLPI = crate::Reg<rxlpi::RXLPI_SPEC>;
#[doc = "Received LPI transitions"]
pub mod rxlpi;
#[doc = "RXLPITIME (rw) register accessor: Received LPI time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlpitime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxlpitime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlpitime`]
module"]
pub type RXLPITIME = crate::Reg<rxlpitime::RXLPITIME_SPEC>;
#[doc = "Received LPI time"]
pub mod rxlpitime;
#[doc = "TXLPI (rw) register accessor: Transmit LPI transitions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlpi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txlpi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlpi`]
module"]
pub type TXLPI = crate::Reg<txlpi::TXLPI_SPEC>;
#[doc = "Transmit LPI transitions"]
pub mod txlpi;
#[doc = "TXLPITIME (rw) register accessor: Transmit LPI time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlpitime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txlpitime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlpitime`]
module"]
pub type TXLPITIME = crate::Reg<txlpitime::TXLPITIME_SPEC>;
#[doc = "Transmit LPI time"]
pub mod txlpitime;
#[doc = "TXBDCTRL (rw) register accessor: TX BD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbdctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbdctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbdctrl`]
module"]
pub type TXBDCTRL = crate::Reg<txbdctrl::TXBDCTRL_SPEC>;
#[doc = "TX BD control register"]
pub mod txbdctrl;
#[doc = "RXBDCTRL (rw) register accessor: RX BD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbdctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbdctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbdctrl`]
module"]
pub type RXBDCTRL = crate::Reg<rxbdctrl::RXBDCTRL_SPEC>;
#[doc = "RX BD control register"]
pub mod rxbdctrl;
#[doc = "ROUTEPEN (rw) register accessor: I/O Route Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Route Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Route Location Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Route Location Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`]
module"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Route Location Register 1"]
pub mod routeloc1;
#[doc = "CTRL (rw) register accessor: Ethernet control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Ethernet control register"]
pub mod ctrl;
