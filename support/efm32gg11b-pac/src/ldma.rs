#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    sync: SYNC,
    _reserved3: [u8; 0x14],
    chen: CHEN,
    chbusy: CHBUSY,
    chdone: CHDONE,
    dbghalt: DBGHALT,
    swreq: SWREQ,
    reqdis: REQDIS,
    reqpend: REQPEND,
    linkload: LINKLOAD,
    reqclear: REQCLEAR,
    _reserved12: [u8; 0x1c],
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    _reserved16: [u8; 0x10],
    ch0_reqsel: CH0_REQSEL,
    ch0_cfg: CH0_CFG,
    ch0_loop: CH0_LOOP,
    ch0_ctrl: CH0_CTRL,
    ch0_src: CH0_SRC,
    ch0_dst: CH0_DST,
    ch0_link: CH0_LINK,
    _reserved23: [u8; 0x14],
    ch1_reqsel: CH1_REQSEL,
    ch1_cfg: CH1_CFG,
    ch1_loop: CH1_LOOP,
    ch1_ctrl: CH1_CTRL,
    ch1_src: CH1_SRC,
    ch1_dst: CH1_DST,
    ch1_link: CH1_LINK,
    _reserved30: [u8; 0x14],
    ch2_reqsel: CH2_REQSEL,
    ch2_cfg: CH2_CFG,
    ch2_loop: CH2_LOOP,
    ch2_ctrl: CH2_CTRL,
    ch2_src: CH2_SRC,
    ch2_dst: CH2_DST,
    ch2_link: CH2_LINK,
    _reserved37: [u8; 0x14],
    ch3_reqsel: CH3_REQSEL,
    ch3_cfg: CH3_CFG,
    ch3_loop: CH3_LOOP,
    ch3_ctrl: CH3_CTRL,
    ch3_src: CH3_SRC,
    ch3_dst: CH3_DST,
    ch3_link: CH3_LINK,
    _reserved44: [u8; 0x14],
    ch4_reqsel: CH4_REQSEL,
    ch4_cfg: CH4_CFG,
    ch4_loop: CH4_LOOP,
    ch4_ctrl: CH4_CTRL,
    ch4_src: CH4_SRC,
    ch4_dst: CH4_DST,
    ch4_link: CH4_LINK,
    _reserved51: [u8; 0x14],
    ch5_reqsel: CH5_REQSEL,
    ch5_cfg: CH5_CFG,
    ch5_loop: CH5_LOOP,
    ch5_ctrl: CH5_CTRL,
    ch5_src: CH5_SRC,
    ch5_dst: CH5_DST,
    ch5_link: CH5_LINK,
    _reserved58: [u8; 0x14],
    ch6_reqsel: CH6_REQSEL,
    ch6_cfg: CH6_CFG,
    ch6_loop: CH6_LOOP,
    ch6_ctrl: CH6_CTRL,
    ch6_src: CH6_SRC,
    ch6_dst: CH6_DST,
    ch6_link: CH6_LINK,
    _reserved65: [u8; 0x14],
    ch7_reqsel: CH7_REQSEL,
    ch7_cfg: CH7_CFG,
    ch7_loop: CH7_LOOP,
    ch7_ctrl: CH7_CTRL,
    ch7_src: CH7_SRC,
    ch7_dst: CH7_DST,
    ch7_link: CH7_LINK,
    _reserved72: [u8; 0x14],
    ch8_reqsel: CH8_REQSEL,
    ch8_cfg: CH8_CFG,
    ch8_loop: CH8_LOOP,
    ch8_ctrl: CH8_CTRL,
    ch8_src: CH8_SRC,
    ch8_dst: CH8_DST,
    ch8_link: CH8_LINK,
    _reserved79: [u8; 0x14],
    ch9_reqsel: CH9_REQSEL,
    ch9_cfg: CH9_CFG,
    ch9_loop: CH9_LOOP,
    ch9_ctrl: CH9_CTRL,
    ch9_src: CH9_SRC,
    ch9_dst: CH9_DST,
    ch9_link: CH9_LINK,
    _reserved86: [u8; 0x14],
    ch10_reqsel: CH10_REQSEL,
    ch10_cfg: CH10_CFG,
    ch10_loop: CH10_LOOP,
    ch10_ctrl: CH10_CTRL,
    ch10_src: CH10_SRC,
    ch10_dst: CH10_DST,
    ch10_link: CH10_LINK,
    _reserved93: [u8; 0x14],
    ch11_reqsel: CH11_REQSEL,
    ch11_cfg: CH11_CFG,
    ch11_loop: CH11_LOOP,
    ch11_ctrl: CH11_CTRL,
    ch11_src: CH11_SRC,
    ch11_dst: CH11_DST,
    ch11_link: CH11_LINK,
    _reserved100: [u8; 0x14],
    ch12_reqsel: CH12_REQSEL,
    ch12_cfg: CH12_CFG,
    ch12_loop: CH12_LOOP,
    ch12_ctrl: CH12_CTRL,
    ch12_src: CH12_SRC,
    ch12_dst: CH12_DST,
    ch12_link: CH12_LINK,
    _reserved107: [u8; 0x14],
    ch13_reqsel: CH13_REQSEL,
    ch13_cfg: CH13_CFG,
    ch13_loop: CH13_LOOP,
    ch13_ctrl: CH13_CTRL,
    ch13_src: CH13_SRC,
    ch13_dst: CH13_DST,
    ch13_link: CH13_LINK,
    _reserved114: [u8; 0x14],
    ch14_reqsel: CH14_REQSEL,
    ch14_cfg: CH14_CFG,
    ch14_loop: CH14_LOOP,
    ch14_ctrl: CH14_CTRL,
    ch14_src: CH14_SRC,
    ch14_dst: CH14_DST,
    ch14_link: CH14_LINK,
    _reserved121: [u8; 0x14],
    ch15_reqsel: CH15_REQSEL,
    ch15_cfg: CH15_CFG,
    ch15_loop: CH15_LOOP,
    ch15_ctrl: CH15_CTRL,
    ch15_src: CH15_SRC,
    ch15_dst: CH15_DST,
    ch15_link: CH15_LINK,
    _reserved128: [u8; 0x14],
    ch16_reqsel: CH16_REQSEL,
    ch16_cfg: CH16_CFG,
    ch16_loop: CH16_LOOP,
    ch16_ctrl: CH16_CTRL,
    ch16_src: CH16_SRC,
    ch16_dst: CH16_DST,
    ch16_link: CH16_LINK,
    _reserved135: [u8; 0x14],
    ch17_reqsel: CH17_REQSEL,
    ch17_cfg: CH17_CFG,
    ch17_loop: CH17_LOOP,
    ch17_ctrl: CH17_CTRL,
    ch17_src: CH17_SRC,
    ch17_dst: CH17_DST,
    ch17_link: CH17_LINK,
    _reserved142: [u8; 0x14],
    ch18_reqsel: CH18_REQSEL,
    ch18_cfg: CH18_CFG,
    ch18_loop: CH18_LOOP,
    ch18_ctrl: CH18_CTRL,
    ch18_src: CH18_SRC,
    ch18_dst: CH18_DST,
    ch18_link: CH18_LINK,
    _reserved149: [u8; 0x14],
    ch19_reqsel: CH19_REQSEL,
    ch19_cfg: CH19_CFG,
    ch19_loop: CH19_LOOP,
    ch19_ctrl: CH19_CTRL,
    ch19_src: CH19_SRC,
    ch19_dst: CH19_DST,
    ch19_link: CH19_LINK,
    _reserved156: [u8; 0x14],
    ch20_reqsel: CH20_REQSEL,
    ch20_cfg: CH20_CFG,
    ch20_loop: CH20_LOOP,
    ch20_ctrl: CH20_CTRL,
    ch20_src: CH20_SRC,
    ch20_dst: CH20_DST,
    ch20_link: CH20_LINK,
    _reserved163: [u8; 0x14],
    ch21_reqsel: CH21_REQSEL,
    ch21_cfg: CH21_CFG,
    ch21_loop: CH21_LOOP,
    ch21_ctrl: CH21_CTRL,
    ch21_src: CH21_SRC,
    ch21_dst: CH21_DST,
    ch21_link: CH21_LINK,
    _reserved170: [u8; 0x14],
    ch22_reqsel: CH22_REQSEL,
    ch22_cfg: CH22_CFG,
    ch22_loop: CH22_LOOP,
    ch22_ctrl: CH22_CTRL,
    ch22_src: CH22_SRC,
    ch22_dst: CH22_DST,
    ch22_link: CH22_LINK,
    _reserved177: [u8; 0x14],
    ch23_reqsel: CH23_REQSEL,
    ch23_cfg: CH23_CFG,
    ch23_loop: CH23_LOOP,
    ch23_ctrl: CH23_CTRL,
    ch23_src: CH23_SRC,
    ch23_dst: CH23_DST,
    ch23_link: CH23_LINK,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - DMA Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn sync(&self) -> &SYNC {
        &self.sync
    }
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chen(&self) -> &CHEN {
        &self.chen
    }
    #[doc = "0x24 - DMA Channel Busy Register"]
    #[inline(always)]
    pub const fn chbusy(&self) -> &CHBUSY {
        &self.chbusy
    }
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chdone(&self) -> &CHDONE {
        &self.chdone
    }
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    #[inline(always)]
    pub const fn dbghalt(&self) -> &DBGHALT {
        &self.dbghalt
    }
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    #[inline(always)]
    pub const fn swreq(&self) -> &SWREQ {
        &self.swreq
    }
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    #[inline(always)]
    pub const fn reqdis(&self) -> &REQDIS {
        &self.reqdis
    }
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    #[inline(always)]
    pub const fn reqpend(&self) -> &REQPEND {
        &self.reqpend
    }
    #[doc = "0x3c - DMA Channel Link Load Register"]
    #[inline(always)]
    pub const fn linkload(&self) -> &LINKLOAD {
        &self.linkload
    }
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    #[inline(always)]
    pub const fn reqclear(&self) -> &REQCLEAR {
        &self.reqclear
    }
    #[doc = "0x60 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x64 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x6c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch0_reqsel(&self) -> &CH0_REQSEL {
        &self.ch0_reqsel
    }
    #[doc = "0x84 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch0_cfg(&self) -> &CH0_CFG {
        &self.ch0_cfg
    }
    #[doc = "0x88 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch0_loop(&self) -> &CH0_LOOP {
        &self.ch0_loop
    }
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &CH0_CTRL {
        &self.ch0_ctrl
    }
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch0_src(&self) -> &CH0_SRC {
        &self.ch0_src
    }
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch0_dst(&self) -> &CH0_DST {
        &self.ch0_dst
    }
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch0_link(&self) -> &CH0_LINK {
        &self.ch0_link
    }
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch1_reqsel(&self) -> &CH1_REQSEL {
        &self.ch1_reqsel
    }
    #[doc = "0xb4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch1_cfg(&self) -> &CH1_CFG {
        &self.ch1_cfg
    }
    #[doc = "0xb8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch1_loop(&self) -> &CH1_LOOP {
        &self.ch1_loop
    }
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &CH1_CTRL {
        &self.ch1_ctrl
    }
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch1_src(&self) -> &CH1_SRC {
        &self.ch1_src
    }
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch1_dst(&self) -> &CH1_DST {
        &self.ch1_dst
    }
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch1_link(&self) -> &CH1_LINK {
        &self.ch1_link
    }
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch2_reqsel(&self) -> &CH2_REQSEL {
        &self.ch2_reqsel
    }
    #[doc = "0xe4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch2_cfg(&self) -> &CH2_CFG {
        &self.ch2_cfg
    }
    #[doc = "0xe8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch2_loop(&self) -> &CH2_LOOP {
        &self.ch2_loop
    }
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &CH2_CTRL {
        &self.ch2_ctrl
    }
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch2_src(&self) -> &CH2_SRC {
        &self.ch2_src
    }
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch2_dst(&self) -> &CH2_DST {
        &self.ch2_dst
    }
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch2_link(&self) -> &CH2_LINK {
        &self.ch2_link
    }
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch3_reqsel(&self) -> &CH3_REQSEL {
        &self.ch3_reqsel
    }
    #[doc = "0x114 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch3_cfg(&self) -> &CH3_CFG {
        &self.ch3_cfg
    }
    #[doc = "0x118 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch3_loop(&self) -> &CH3_LOOP {
        &self.ch3_loop
    }
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &CH3_CTRL {
        &self.ch3_ctrl
    }
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch3_src(&self) -> &CH3_SRC {
        &self.ch3_src
    }
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch3_dst(&self) -> &CH3_DST {
        &self.ch3_dst
    }
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch3_link(&self) -> &CH3_LINK {
        &self.ch3_link
    }
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch4_reqsel(&self) -> &CH4_REQSEL {
        &self.ch4_reqsel
    }
    #[doc = "0x144 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch4_cfg(&self) -> &CH4_CFG {
        &self.ch4_cfg
    }
    #[doc = "0x148 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch4_loop(&self) -> &CH4_LOOP {
        &self.ch4_loop
    }
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &CH4_CTRL {
        &self.ch4_ctrl
    }
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch4_src(&self) -> &CH4_SRC {
        &self.ch4_src
    }
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch4_dst(&self) -> &CH4_DST {
        &self.ch4_dst
    }
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch4_link(&self) -> &CH4_LINK {
        &self.ch4_link
    }
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch5_reqsel(&self) -> &CH5_REQSEL {
        &self.ch5_reqsel
    }
    #[doc = "0x174 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch5_cfg(&self) -> &CH5_CFG {
        &self.ch5_cfg
    }
    #[doc = "0x178 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch5_loop(&self) -> &CH5_LOOP {
        &self.ch5_loop
    }
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &CH5_CTRL {
        &self.ch5_ctrl
    }
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch5_src(&self) -> &CH5_SRC {
        &self.ch5_src
    }
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch5_dst(&self) -> &CH5_DST {
        &self.ch5_dst
    }
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch5_link(&self) -> &CH5_LINK {
        &self.ch5_link
    }
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch6_reqsel(&self) -> &CH6_REQSEL {
        &self.ch6_reqsel
    }
    #[doc = "0x1a4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch6_cfg(&self) -> &CH6_CFG {
        &self.ch6_cfg
    }
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch6_loop(&self) -> &CH6_LOOP {
        &self.ch6_loop
    }
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &CH6_CTRL {
        &self.ch6_ctrl
    }
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch6_src(&self) -> &CH6_SRC {
        &self.ch6_src
    }
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch6_dst(&self) -> &CH6_DST {
        &self.ch6_dst
    }
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch6_link(&self) -> &CH6_LINK {
        &self.ch6_link
    }
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch7_reqsel(&self) -> &CH7_REQSEL {
        &self.ch7_reqsel
    }
    #[doc = "0x1d4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch7_cfg(&self) -> &CH7_CFG {
        &self.ch7_cfg
    }
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch7_loop(&self) -> &CH7_LOOP {
        &self.ch7_loop
    }
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &CH7_CTRL {
        &self.ch7_ctrl
    }
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch7_src(&self) -> &CH7_SRC {
        &self.ch7_src
    }
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch7_dst(&self) -> &CH7_DST {
        &self.ch7_dst
    }
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch7_link(&self) -> &CH7_LINK {
        &self.ch7_link
    }
    #[doc = "0x200 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch8_reqsel(&self) -> &CH8_REQSEL {
        &self.ch8_reqsel
    }
    #[doc = "0x204 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch8_cfg(&self) -> &CH8_CFG {
        &self.ch8_cfg
    }
    #[doc = "0x208 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch8_loop(&self) -> &CH8_LOOP {
        &self.ch8_loop
    }
    #[doc = "0x20c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch8_ctrl(&self) -> &CH8_CTRL {
        &self.ch8_ctrl
    }
    #[doc = "0x210 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch8_src(&self) -> &CH8_SRC {
        &self.ch8_src
    }
    #[doc = "0x214 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch8_dst(&self) -> &CH8_DST {
        &self.ch8_dst
    }
    #[doc = "0x218 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch8_link(&self) -> &CH8_LINK {
        &self.ch8_link
    }
    #[doc = "0x230 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch9_reqsel(&self) -> &CH9_REQSEL {
        &self.ch9_reqsel
    }
    #[doc = "0x234 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch9_cfg(&self) -> &CH9_CFG {
        &self.ch9_cfg
    }
    #[doc = "0x238 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch9_loop(&self) -> &CH9_LOOP {
        &self.ch9_loop
    }
    #[doc = "0x23c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch9_ctrl(&self) -> &CH9_CTRL {
        &self.ch9_ctrl
    }
    #[doc = "0x240 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch9_src(&self) -> &CH9_SRC {
        &self.ch9_src
    }
    #[doc = "0x244 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch9_dst(&self) -> &CH9_DST {
        &self.ch9_dst
    }
    #[doc = "0x248 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch9_link(&self) -> &CH9_LINK {
        &self.ch9_link
    }
    #[doc = "0x260 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch10_reqsel(&self) -> &CH10_REQSEL {
        &self.ch10_reqsel
    }
    #[doc = "0x264 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch10_cfg(&self) -> &CH10_CFG {
        &self.ch10_cfg
    }
    #[doc = "0x268 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch10_loop(&self) -> &CH10_LOOP {
        &self.ch10_loop
    }
    #[doc = "0x26c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch10_ctrl(&self) -> &CH10_CTRL {
        &self.ch10_ctrl
    }
    #[doc = "0x270 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch10_src(&self) -> &CH10_SRC {
        &self.ch10_src
    }
    #[doc = "0x274 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch10_dst(&self) -> &CH10_DST {
        &self.ch10_dst
    }
    #[doc = "0x278 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch10_link(&self) -> &CH10_LINK {
        &self.ch10_link
    }
    #[doc = "0x290 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch11_reqsel(&self) -> &CH11_REQSEL {
        &self.ch11_reqsel
    }
    #[doc = "0x294 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch11_cfg(&self) -> &CH11_CFG {
        &self.ch11_cfg
    }
    #[doc = "0x298 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch11_loop(&self) -> &CH11_LOOP {
        &self.ch11_loop
    }
    #[doc = "0x29c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch11_ctrl(&self) -> &CH11_CTRL {
        &self.ch11_ctrl
    }
    #[doc = "0x2a0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch11_src(&self) -> &CH11_SRC {
        &self.ch11_src
    }
    #[doc = "0x2a4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch11_dst(&self) -> &CH11_DST {
        &self.ch11_dst
    }
    #[doc = "0x2a8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch11_link(&self) -> &CH11_LINK {
        &self.ch11_link
    }
    #[doc = "0x2c0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch12_reqsel(&self) -> &CH12_REQSEL {
        &self.ch12_reqsel
    }
    #[doc = "0x2c4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch12_cfg(&self) -> &CH12_CFG {
        &self.ch12_cfg
    }
    #[doc = "0x2c8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch12_loop(&self) -> &CH12_LOOP {
        &self.ch12_loop
    }
    #[doc = "0x2cc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch12_ctrl(&self) -> &CH12_CTRL {
        &self.ch12_ctrl
    }
    #[doc = "0x2d0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch12_src(&self) -> &CH12_SRC {
        &self.ch12_src
    }
    #[doc = "0x2d4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch12_dst(&self) -> &CH12_DST {
        &self.ch12_dst
    }
    #[doc = "0x2d8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch12_link(&self) -> &CH12_LINK {
        &self.ch12_link
    }
    #[doc = "0x2f0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch13_reqsel(&self) -> &CH13_REQSEL {
        &self.ch13_reqsel
    }
    #[doc = "0x2f4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch13_cfg(&self) -> &CH13_CFG {
        &self.ch13_cfg
    }
    #[doc = "0x2f8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch13_loop(&self) -> &CH13_LOOP {
        &self.ch13_loop
    }
    #[doc = "0x2fc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch13_ctrl(&self) -> &CH13_CTRL {
        &self.ch13_ctrl
    }
    #[doc = "0x300 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch13_src(&self) -> &CH13_SRC {
        &self.ch13_src
    }
    #[doc = "0x304 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch13_dst(&self) -> &CH13_DST {
        &self.ch13_dst
    }
    #[doc = "0x308 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch13_link(&self) -> &CH13_LINK {
        &self.ch13_link
    }
    #[doc = "0x320 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch14_reqsel(&self) -> &CH14_REQSEL {
        &self.ch14_reqsel
    }
    #[doc = "0x324 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch14_cfg(&self) -> &CH14_CFG {
        &self.ch14_cfg
    }
    #[doc = "0x328 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch14_loop(&self) -> &CH14_LOOP {
        &self.ch14_loop
    }
    #[doc = "0x32c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch14_ctrl(&self) -> &CH14_CTRL {
        &self.ch14_ctrl
    }
    #[doc = "0x330 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch14_src(&self) -> &CH14_SRC {
        &self.ch14_src
    }
    #[doc = "0x334 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch14_dst(&self) -> &CH14_DST {
        &self.ch14_dst
    }
    #[doc = "0x338 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch14_link(&self) -> &CH14_LINK {
        &self.ch14_link
    }
    #[doc = "0x350 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch15_reqsel(&self) -> &CH15_REQSEL {
        &self.ch15_reqsel
    }
    #[doc = "0x354 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch15_cfg(&self) -> &CH15_CFG {
        &self.ch15_cfg
    }
    #[doc = "0x358 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch15_loop(&self) -> &CH15_LOOP {
        &self.ch15_loop
    }
    #[doc = "0x35c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch15_ctrl(&self) -> &CH15_CTRL {
        &self.ch15_ctrl
    }
    #[doc = "0x360 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch15_src(&self) -> &CH15_SRC {
        &self.ch15_src
    }
    #[doc = "0x364 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch15_dst(&self) -> &CH15_DST {
        &self.ch15_dst
    }
    #[doc = "0x368 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch15_link(&self) -> &CH15_LINK {
        &self.ch15_link
    }
    #[doc = "0x380 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch16_reqsel(&self) -> &CH16_REQSEL {
        &self.ch16_reqsel
    }
    #[doc = "0x384 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch16_cfg(&self) -> &CH16_CFG {
        &self.ch16_cfg
    }
    #[doc = "0x388 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch16_loop(&self) -> &CH16_LOOP {
        &self.ch16_loop
    }
    #[doc = "0x38c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch16_ctrl(&self) -> &CH16_CTRL {
        &self.ch16_ctrl
    }
    #[doc = "0x390 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch16_src(&self) -> &CH16_SRC {
        &self.ch16_src
    }
    #[doc = "0x394 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch16_dst(&self) -> &CH16_DST {
        &self.ch16_dst
    }
    #[doc = "0x398 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch16_link(&self) -> &CH16_LINK {
        &self.ch16_link
    }
    #[doc = "0x3b0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch17_reqsel(&self) -> &CH17_REQSEL {
        &self.ch17_reqsel
    }
    #[doc = "0x3b4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch17_cfg(&self) -> &CH17_CFG {
        &self.ch17_cfg
    }
    #[doc = "0x3b8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch17_loop(&self) -> &CH17_LOOP {
        &self.ch17_loop
    }
    #[doc = "0x3bc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch17_ctrl(&self) -> &CH17_CTRL {
        &self.ch17_ctrl
    }
    #[doc = "0x3c0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch17_src(&self) -> &CH17_SRC {
        &self.ch17_src
    }
    #[doc = "0x3c4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch17_dst(&self) -> &CH17_DST {
        &self.ch17_dst
    }
    #[doc = "0x3c8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch17_link(&self) -> &CH17_LINK {
        &self.ch17_link
    }
    #[doc = "0x3e0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch18_reqsel(&self) -> &CH18_REQSEL {
        &self.ch18_reqsel
    }
    #[doc = "0x3e4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch18_cfg(&self) -> &CH18_CFG {
        &self.ch18_cfg
    }
    #[doc = "0x3e8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch18_loop(&self) -> &CH18_LOOP {
        &self.ch18_loop
    }
    #[doc = "0x3ec - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch18_ctrl(&self) -> &CH18_CTRL {
        &self.ch18_ctrl
    }
    #[doc = "0x3f0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch18_src(&self) -> &CH18_SRC {
        &self.ch18_src
    }
    #[doc = "0x3f4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch18_dst(&self) -> &CH18_DST {
        &self.ch18_dst
    }
    #[doc = "0x3f8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch18_link(&self) -> &CH18_LINK {
        &self.ch18_link
    }
    #[doc = "0x410 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch19_reqsel(&self) -> &CH19_REQSEL {
        &self.ch19_reqsel
    }
    #[doc = "0x414 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch19_cfg(&self) -> &CH19_CFG {
        &self.ch19_cfg
    }
    #[doc = "0x418 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch19_loop(&self) -> &CH19_LOOP {
        &self.ch19_loop
    }
    #[doc = "0x41c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch19_ctrl(&self) -> &CH19_CTRL {
        &self.ch19_ctrl
    }
    #[doc = "0x420 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch19_src(&self) -> &CH19_SRC {
        &self.ch19_src
    }
    #[doc = "0x424 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch19_dst(&self) -> &CH19_DST {
        &self.ch19_dst
    }
    #[doc = "0x428 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch19_link(&self) -> &CH19_LINK {
        &self.ch19_link
    }
    #[doc = "0x440 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch20_reqsel(&self) -> &CH20_REQSEL {
        &self.ch20_reqsel
    }
    #[doc = "0x444 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch20_cfg(&self) -> &CH20_CFG {
        &self.ch20_cfg
    }
    #[doc = "0x448 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch20_loop(&self) -> &CH20_LOOP {
        &self.ch20_loop
    }
    #[doc = "0x44c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch20_ctrl(&self) -> &CH20_CTRL {
        &self.ch20_ctrl
    }
    #[doc = "0x450 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch20_src(&self) -> &CH20_SRC {
        &self.ch20_src
    }
    #[doc = "0x454 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch20_dst(&self) -> &CH20_DST {
        &self.ch20_dst
    }
    #[doc = "0x458 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch20_link(&self) -> &CH20_LINK {
        &self.ch20_link
    }
    #[doc = "0x470 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch21_reqsel(&self) -> &CH21_REQSEL {
        &self.ch21_reqsel
    }
    #[doc = "0x474 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch21_cfg(&self) -> &CH21_CFG {
        &self.ch21_cfg
    }
    #[doc = "0x478 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch21_loop(&self) -> &CH21_LOOP {
        &self.ch21_loop
    }
    #[doc = "0x47c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch21_ctrl(&self) -> &CH21_CTRL {
        &self.ch21_ctrl
    }
    #[doc = "0x480 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch21_src(&self) -> &CH21_SRC {
        &self.ch21_src
    }
    #[doc = "0x484 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch21_dst(&self) -> &CH21_DST {
        &self.ch21_dst
    }
    #[doc = "0x488 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch21_link(&self) -> &CH21_LINK {
        &self.ch21_link
    }
    #[doc = "0x4a0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch22_reqsel(&self) -> &CH22_REQSEL {
        &self.ch22_reqsel
    }
    #[doc = "0x4a4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch22_cfg(&self) -> &CH22_CFG {
        &self.ch22_cfg
    }
    #[doc = "0x4a8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch22_loop(&self) -> &CH22_LOOP {
        &self.ch22_loop
    }
    #[doc = "0x4ac - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch22_ctrl(&self) -> &CH22_CTRL {
        &self.ch22_ctrl
    }
    #[doc = "0x4b0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch22_src(&self) -> &CH22_SRC {
        &self.ch22_src
    }
    #[doc = "0x4b4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch22_dst(&self) -> &CH22_DST {
        &self.ch22_dst
    }
    #[doc = "0x4b8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch22_link(&self) -> &CH22_LINK {
        &self.ch22_link
    }
    #[doc = "0x4d0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch23_reqsel(&self) -> &CH23_REQSEL {
        &self.ch23_reqsel
    }
    #[doc = "0x4d4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch23_cfg(&self) -> &CH23_CFG {
        &self.ch23_cfg
    }
    #[doc = "0x4d8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch23_loop(&self) -> &CH23_LOOP {
        &self.ch23_loop
    }
    #[doc = "0x4dc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch23_ctrl(&self) -> &CH23_CTRL {
        &self.ch23_ctrl
    }
    #[doc = "0x4e0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch23_src(&self) -> &CH23_SRC {
        &self.ch23_src
    }
    #[doc = "0x4e4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch23_dst(&self) -> &CH23_DST {
        &self.ch23_dst
    }
    #[doc = "0x4e8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch23_link(&self) -> &CH23_LINK {
        &self.ch23_link
    }
}
#[doc = "CTRL (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: DMA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "SYNC (rw) register accessor: DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "CHEN (rw) register accessor: DMA Channel Enable Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "CHBUSY (r) register accessor: DMA Channel Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbusy`]
module"]
pub type CHBUSY = crate::Reg<chbusy::CHBUSY_SPEC>;
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE (rw) register accessor: DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chdone::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdone::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdone`]
module"]
pub type CHDONE = crate::Reg<chdone::CHDONE_SPEC>;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DBGHALT (rw) register accessor: DMA Channel Debug Halt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbghalt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbghalt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbghalt`]
module"]
pub type DBGHALT = crate::Reg<dbghalt::DBGHALT_SPEC>;
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ (w) register accessor: DMA Channel Software Transfer Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`]
module"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "REQDIS (rw) register accessor: DMA Channel Request Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdis`]
module"]
pub type REQDIS = crate::Reg<reqdis::REQDIS_SPEC>;
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "REQPEND (r) register accessor: DMA Channel Requests Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqpend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqpend`]
module"]
pub type REQPEND = crate::Reg<reqpend::REQPEND_SPEC>;
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD (w) register accessor: DMA Channel Link Load Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linkload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkload`]
module"]
pub type LINKLOAD = crate::Reg<linkload::LINKLOAD_SPEC>;
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "REQCLEAR (w) register accessor: DMA Channel Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqclear`]
module"]
pub type REQCLEAR = crate::Reg<reqclear::REQCLEAR_SPEC>;
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "CH0_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_reqsel`]
module"]
pub type CH0_REQSEL = crate::Reg<ch0_reqsel::CH0_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "CH0_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_cfg`]
module"]
pub type CH0_CFG = crate::Reg<ch0_cfg::CH0_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_loop`]
module"]
pub type CH0_LOOP = crate::Reg<ch0_loop::CH0_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "CH0_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`]
module"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_src`]
module"]
pub type CH0_SRC = crate::Reg<ch0_src::CH0_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "CH0_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dst`]
module"]
pub type CH0_DST = crate::Reg<ch0_dst::CH0_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "CH0_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_link`]
module"]
pub type CH0_LINK = crate::Reg<ch0_link::CH0_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "CH1_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_reqsel`]
module"]
pub type CH1_REQSEL = crate::Reg<ch1_reqsel::CH1_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "CH1_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg`]
module"]
pub type CH1_CFG = crate::Reg<ch1_cfg::CH1_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_loop`]
module"]
pub type CH1_LOOP = crate::Reg<ch1_loop::CH1_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "CH1_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`]
module"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_src`]
module"]
pub type CH1_SRC = crate::Reg<ch1_src::CH1_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "CH1_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dst`]
module"]
pub type CH1_DST = crate::Reg<ch1_dst::CH1_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "CH1_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_link`]
module"]
pub type CH1_LINK = crate::Reg<ch1_link::CH1_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "CH2_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_reqsel`]
module"]
pub type CH2_REQSEL = crate::Reg<ch2_reqsel::CH2_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "CH2_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg`]
module"]
pub type CH2_CFG = crate::Reg<ch2_cfg::CH2_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_loop`]
module"]
pub type CH2_LOOP = crate::Reg<ch2_loop::CH2_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "CH2_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`]
module"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_src`]
module"]
pub type CH2_SRC = crate::Reg<ch2_src::CH2_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "CH2_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dst`]
module"]
pub type CH2_DST = crate::Reg<ch2_dst::CH2_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "CH2_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_link`]
module"]
pub type CH2_LINK = crate::Reg<ch2_link::CH2_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "CH3_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_reqsel`]
module"]
pub type CH3_REQSEL = crate::Reg<ch3_reqsel::CH3_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "CH3_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg`]
module"]
pub type CH3_CFG = crate::Reg<ch3_cfg::CH3_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_loop`]
module"]
pub type CH3_LOOP = crate::Reg<ch3_loop::CH3_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "CH3_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`]
module"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_src`]
module"]
pub type CH3_SRC = crate::Reg<ch3_src::CH3_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "CH3_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dst`]
module"]
pub type CH3_DST = crate::Reg<ch3_dst::CH3_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "CH3_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_link`]
module"]
pub type CH3_LINK = crate::Reg<ch3_link::CH3_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "CH4_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_reqsel`]
module"]
pub type CH4_REQSEL = crate::Reg<ch4_reqsel::CH4_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "CH4_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg`]
module"]
pub type CH4_CFG = crate::Reg<ch4_cfg::CH4_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_loop`]
module"]
pub type CH4_LOOP = crate::Reg<ch4_loop::CH4_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "CH4_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`]
module"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_src`]
module"]
pub type CH4_SRC = crate::Reg<ch4_src::CH4_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "CH4_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dst`]
module"]
pub type CH4_DST = crate::Reg<ch4_dst::CH4_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "CH4_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_link`]
module"]
pub type CH4_LINK = crate::Reg<ch4_link::CH4_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "CH5_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_reqsel`]
module"]
pub type CH5_REQSEL = crate::Reg<ch5_reqsel::CH5_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "CH5_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_cfg`]
module"]
pub type CH5_CFG = crate::Reg<ch5_cfg::CH5_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_loop`]
module"]
pub type CH5_LOOP = crate::Reg<ch5_loop::CH5_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "CH5_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`]
module"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_src`]
module"]
pub type CH5_SRC = crate::Reg<ch5_src::CH5_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "CH5_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_dst`]
module"]
pub type CH5_DST = crate::Reg<ch5_dst::CH5_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "CH5_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_link`]
module"]
pub type CH5_LINK = crate::Reg<ch5_link::CH5_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "CH6_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_reqsel`]
module"]
pub type CH6_REQSEL = crate::Reg<ch6_reqsel::CH6_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "CH6_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_cfg`]
module"]
pub type CH6_CFG = crate::Reg<ch6_cfg::CH6_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_loop`]
module"]
pub type CH6_LOOP = crate::Reg<ch6_loop::CH6_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "CH6_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`]
module"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_src`]
module"]
pub type CH6_SRC = crate::Reg<ch6_src::CH6_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "CH6_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_dst`]
module"]
pub type CH6_DST = crate::Reg<ch6_dst::CH6_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "CH6_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_link`]
module"]
pub type CH6_LINK = crate::Reg<ch6_link::CH6_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "CH7_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_reqsel`]
module"]
pub type CH7_REQSEL = crate::Reg<ch7_reqsel::CH7_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "CH7_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_cfg`]
module"]
pub type CH7_CFG = crate::Reg<ch7_cfg::CH7_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_loop`]
module"]
pub type CH7_LOOP = crate::Reg<ch7_loop::CH7_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "CH7_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`]
module"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_src`]
module"]
pub type CH7_SRC = crate::Reg<ch7_src::CH7_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "CH7_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_dst`]
module"]
pub type CH7_DST = crate::Reg<ch7_dst::CH7_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "CH7_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_link`]
module"]
pub type CH7_LINK = crate::Reg<ch7_link::CH7_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
#[doc = "CH8_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_reqsel`]
module"]
pub type CH8_REQSEL = crate::Reg<ch8_reqsel::CH8_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch8_reqsel;
#[doc = "CH8_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_cfg`]
module"]
pub type CH8_CFG = crate::Reg<ch8_cfg::CH8_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch8_cfg;
#[doc = "CH8_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_loop`]
module"]
pub type CH8_LOOP = crate::Reg<ch8_loop::CH8_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch8_loop;
#[doc = "CH8_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ctrl`]
module"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch8_ctrl;
#[doc = "CH8_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_src`]
module"]
pub type CH8_SRC = crate::Reg<ch8_src::CH8_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch8_src;
#[doc = "CH8_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_dst`]
module"]
pub type CH8_DST = crate::Reg<ch8_dst::CH8_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch8_dst;
#[doc = "CH8_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_link`]
module"]
pub type CH8_LINK = crate::Reg<ch8_link::CH8_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch8_link;
#[doc = "CH9_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_reqsel`]
module"]
pub type CH9_REQSEL = crate::Reg<ch9_reqsel::CH9_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch9_reqsel;
#[doc = "CH9_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_cfg`]
module"]
pub type CH9_CFG = crate::Reg<ch9_cfg::CH9_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch9_cfg;
#[doc = "CH9_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_loop`]
module"]
pub type CH9_LOOP = crate::Reg<ch9_loop::CH9_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch9_loop;
#[doc = "CH9_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ctrl`]
module"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch9_ctrl;
#[doc = "CH9_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_src`]
module"]
pub type CH9_SRC = crate::Reg<ch9_src::CH9_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch9_src;
#[doc = "CH9_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_dst`]
module"]
pub type CH9_DST = crate::Reg<ch9_dst::CH9_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch9_dst;
#[doc = "CH9_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_link`]
module"]
pub type CH9_LINK = crate::Reg<ch9_link::CH9_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch9_link;
#[doc = "CH10_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_reqsel`]
module"]
pub type CH10_REQSEL = crate::Reg<ch10_reqsel::CH10_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch10_reqsel;
#[doc = "CH10_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_cfg`]
module"]
pub type CH10_CFG = crate::Reg<ch10_cfg::CH10_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch10_cfg;
#[doc = "CH10_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_loop`]
module"]
pub type CH10_LOOP = crate::Reg<ch10_loop::CH10_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch10_loop;
#[doc = "CH10_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ctrl`]
module"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch10_ctrl;
#[doc = "CH10_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_src`]
module"]
pub type CH10_SRC = crate::Reg<ch10_src::CH10_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch10_src;
#[doc = "CH10_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_dst`]
module"]
pub type CH10_DST = crate::Reg<ch10_dst::CH10_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch10_dst;
#[doc = "CH10_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_link`]
module"]
pub type CH10_LINK = crate::Reg<ch10_link::CH10_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch10_link;
#[doc = "CH11_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_reqsel`]
module"]
pub type CH11_REQSEL = crate::Reg<ch11_reqsel::CH11_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch11_reqsel;
#[doc = "CH11_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_cfg`]
module"]
pub type CH11_CFG = crate::Reg<ch11_cfg::CH11_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch11_cfg;
#[doc = "CH11_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_loop`]
module"]
pub type CH11_LOOP = crate::Reg<ch11_loop::CH11_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch11_loop;
#[doc = "CH11_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ctrl`]
module"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch11_ctrl;
#[doc = "CH11_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_src`]
module"]
pub type CH11_SRC = crate::Reg<ch11_src::CH11_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch11_src;
#[doc = "CH11_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_dst`]
module"]
pub type CH11_DST = crate::Reg<ch11_dst::CH11_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch11_dst;
#[doc = "CH11_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_link`]
module"]
pub type CH11_LINK = crate::Reg<ch11_link::CH11_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch11_link;
#[doc = "CH12_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_reqsel`]
module"]
pub type CH12_REQSEL = crate::Reg<ch12_reqsel::CH12_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch12_reqsel;
#[doc = "CH12_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_cfg`]
module"]
pub type CH12_CFG = crate::Reg<ch12_cfg::CH12_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch12_cfg;
#[doc = "CH12_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_loop`]
module"]
pub type CH12_LOOP = crate::Reg<ch12_loop::CH12_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch12_loop;
#[doc = "CH12_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_ctrl`]
module"]
pub type CH12_CTRL = crate::Reg<ch12_ctrl::CH12_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch12_ctrl;
#[doc = "CH12_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_src`]
module"]
pub type CH12_SRC = crate::Reg<ch12_src::CH12_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch12_src;
#[doc = "CH12_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_dst`]
module"]
pub type CH12_DST = crate::Reg<ch12_dst::CH12_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch12_dst;
#[doc = "CH12_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_link`]
module"]
pub type CH12_LINK = crate::Reg<ch12_link::CH12_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch12_link;
#[doc = "CH13_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_reqsel`]
module"]
pub type CH13_REQSEL = crate::Reg<ch13_reqsel::CH13_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch13_reqsel;
#[doc = "CH13_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_cfg`]
module"]
pub type CH13_CFG = crate::Reg<ch13_cfg::CH13_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch13_cfg;
#[doc = "CH13_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_loop`]
module"]
pub type CH13_LOOP = crate::Reg<ch13_loop::CH13_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch13_loop;
#[doc = "CH13_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_ctrl`]
module"]
pub type CH13_CTRL = crate::Reg<ch13_ctrl::CH13_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch13_ctrl;
#[doc = "CH13_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_src`]
module"]
pub type CH13_SRC = crate::Reg<ch13_src::CH13_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch13_src;
#[doc = "CH13_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_dst`]
module"]
pub type CH13_DST = crate::Reg<ch13_dst::CH13_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch13_dst;
#[doc = "CH13_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_link`]
module"]
pub type CH13_LINK = crate::Reg<ch13_link::CH13_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch13_link;
#[doc = "CH14_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_reqsel`]
module"]
pub type CH14_REQSEL = crate::Reg<ch14_reqsel::CH14_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch14_reqsel;
#[doc = "CH14_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_cfg`]
module"]
pub type CH14_CFG = crate::Reg<ch14_cfg::CH14_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch14_cfg;
#[doc = "CH14_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_loop`]
module"]
pub type CH14_LOOP = crate::Reg<ch14_loop::CH14_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch14_loop;
#[doc = "CH14_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_ctrl`]
module"]
pub type CH14_CTRL = crate::Reg<ch14_ctrl::CH14_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch14_ctrl;
#[doc = "CH14_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_src`]
module"]
pub type CH14_SRC = crate::Reg<ch14_src::CH14_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch14_src;
#[doc = "CH14_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_dst`]
module"]
pub type CH14_DST = crate::Reg<ch14_dst::CH14_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch14_dst;
#[doc = "CH14_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_link`]
module"]
pub type CH14_LINK = crate::Reg<ch14_link::CH14_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch14_link;
#[doc = "CH15_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_reqsel`]
module"]
pub type CH15_REQSEL = crate::Reg<ch15_reqsel::CH15_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch15_reqsel;
#[doc = "CH15_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_cfg`]
module"]
pub type CH15_CFG = crate::Reg<ch15_cfg::CH15_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch15_cfg;
#[doc = "CH15_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_loop`]
module"]
pub type CH15_LOOP = crate::Reg<ch15_loop::CH15_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch15_loop;
#[doc = "CH15_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_ctrl`]
module"]
pub type CH15_CTRL = crate::Reg<ch15_ctrl::CH15_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch15_ctrl;
#[doc = "CH15_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_src`]
module"]
pub type CH15_SRC = crate::Reg<ch15_src::CH15_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch15_src;
#[doc = "CH15_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_dst`]
module"]
pub type CH15_DST = crate::Reg<ch15_dst::CH15_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch15_dst;
#[doc = "CH15_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_link`]
module"]
pub type CH15_LINK = crate::Reg<ch15_link::CH15_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch15_link;
#[doc = "CH16_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_reqsel`]
module"]
pub type CH16_REQSEL = crate::Reg<ch16_reqsel::CH16_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch16_reqsel;
#[doc = "CH16_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_cfg`]
module"]
pub type CH16_CFG = crate::Reg<ch16_cfg::CH16_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch16_cfg;
#[doc = "CH16_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_loop`]
module"]
pub type CH16_LOOP = crate::Reg<ch16_loop::CH16_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch16_loop;
#[doc = "CH16_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_ctrl`]
module"]
pub type CH16_CTRL = crate::Reg<ch16_ctrl::CH16_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch16_ctrl;
#[doc = "CH16_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_src`]
module"]
pub type CH16_SRC = crate::Reg<ch16_src::CH16_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch16_src;
#[doc = "CH16_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_dst`]
module"]
pub type CH16_DST = crate::Reg<ch16_dst::CH16_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch16_dst;
#[doc = "CH16_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_link`]
module"]
pub type CH16_LINK = crate::Reg<ch16_link::CH16_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch16_link;
#[doc = "CH17_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_reqsel`]
module"]
pub type CH17_REQSEL = crate::Reg<ch17_reqsel::CH17_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch17_reqsel;
#[doc = "CH17_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_cfg`]
module"]
pub type CH17_CFG = crate::Reg<ch17_cfg::CH17_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch17_cfg;
#[doc = "CH17_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_loop`]
module"]
pub type CH17_LOOP = crate::Reg<ch17_loop::CH17_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch17_loop;
#[doc = "CH17_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_ctrl`]
module"]
pub type CH17_CTRL = crate::Reg<ch17_ctrl::CH17_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch17_ctrl;
#[doc = "CH17_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_src`]
module"]
pub type CH17_SRC = crate::Reg<ch17_src::CH17_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch17_src;
#[doc = "CH17_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_dst`]
module"]
pub type CH17_DST = crate::Reg<ch17_dst::CH17_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch17_dst;
#[doc = "CH17_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_link`]
module"]
pub type CH17_LINK = crate::Reg<ch17_link::CH17_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch17_link;
#[doc = "CH18_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_reqsel`]
module"]
pub type CH18_REQSEL = crate::Reg<ch18_reqsel::CH18_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch18_reqsel;
#[doc = "CH18_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_cfg`]
module"]
pub type CH18_CFG = crate::Reg<ch18_cfg::CH18_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch18_cfg;
#[doc = "CH18_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_loop`]
module"]
pub type CH18_LOOP = crate::Reg<ch18_loop::CH18_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch18_loop;
#[doc = "CH18_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_ctrl`]
module"]
pub type CH18_CTRL = crate::Reg<ch18_ctrl::CH18_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch18_ctrl;
#[doc = "CH18_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_src`]
module"]
pub type CH18_SRC = crate::Reg<ch18_src::CH18_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch18_src;
#[doc = "CH18_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_dst`]
module"]
pub type CH18_DST = crate::Reg<ch18_dst::CH18_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch18_dst;
#[doc = "CH18_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_link`]
module"]
pub type CH18_LINK = crate::Reg<ch18_link::CH18_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch18_link;
#[doc = "CH19_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_reqsel`]
module"]
pub type CH19_REQSEL = crate::Reg<ch19_reqsel::CH19_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch19_reqsel;
#[doc = "CH19_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_cfg`]
module"]
pub type CH19_CFG = crate::Reg<ch19_cfg::CH19_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch19_cfg;
#[doc = "CH19_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_loop`]
module"]
pub type CH19_LOOP = crate::Reg<ch19_loop::CH19_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch19_loop;
#[doc = "CH19_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_ctrl`]
module"]
pub type CH19_CTRL = crate::Reg<ch19_ctrl::CH19_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch19_ctrl;
#[doc = "CH19_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_src`]
module"]
pub type CH19_SRC = crate::Reg<ch19_src::CH19_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch19_src;
#[doc = "CH19_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_dst`]
module"]
pub type CH19_DST = crate::Reg<ch19_dst::CH19_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch19_dst;
#[doc = "CH19_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_link`]
module"]
pub type CH19_LINK = crate::Reg<ch19_link::CH19_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch19_link;
#[doc = "CH20_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_reqsel`]
module"]
pub type CH20_REQSEL = crate::Reg<ch20_reqsel::CH20_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch20_reqsel;
#[doc = "CH20_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_cfg`]
module"]
pub type CH20_CFG = crate::Reg<ch20_cfg::CH20_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch20_cfg;
#[doc = "CH20_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_loop`]
module"]
pub type CH20_LOOP = crate::Reg<ch20_loop::CH20_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch20_loop;
#[doc = "CH20_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_ctrl`]
module"]
pub type CH20_CTRL = crate::Reg<ch20_ctrl::CH20_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch20_ctrl;
#[doc = "CH20_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_src`]
module"]
pub type CH20_SRC = crate::Reg<ch20_src::CH20_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch20_src;
#[doc = "CH20_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_dst`]
module"]
pub type CH20_DST = crate::Reg<ch20_dst::CH20_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch20_dst;
#[doc = "CH20_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_link`]
module"]
pub type CH20_LINK = crate::Reg<ch20_link::CH20_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch20_link;
#[doc = "CH21_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_reqsel`]
module"]
pub type CH21_REQSEL = crate::Reg<ch21_reqsel::CH21_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch21_reqsel;
#[doc = "CH21_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_cfg`]
module"]
pub type CH21_CFG = crate::Reg<ch21_cfg::CH21_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch21_cfg;
#[doc = "CH21_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_loop`]
module"]
pub type CH21_LOOP = crate::Reg<ch21_loop::CH21_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch21_loop;
#[doc = "CH21_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_ctrl`]
module"]
pub type CH21_CTRL = crate::Reg<ch21_ctrl::CH21_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch21_ctrl;
#[doc = "CH21_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_src`]
module"]
pub type CH21_SRC = crate::Reg<ch21_src::CH21_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch21_src;
#[doc = "CH21_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_dst`]
module"]
pub type CH21_DST = crate::Reg<ch21_dst::CH21_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch21_dst;
#[doc = "CH21_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_link`]
module"]
pub type CH21_LINK = crate::Reg<ch21_link::CH21_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch21_link;
#[doc = "CH22_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_reqsel`]
module"]
pub type CH22_REQSEL = crate::Reg<ch22_reqsel::CH22_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch22_reqsel;
#[doc = "CH22_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_cfg`]
module"]
pub type CH22_CFG = crate::Reg<ch22_cfg::CH22_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch22_cfg;
#[doc = "CH22_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_loop`]
module"]
pub type CH22_LOOP = crate::Reg<ch22_loop::CH22_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch22_loop;
#[doc = "CH22_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_ctrl`]
module"]
pub type CH22_CTRL = crate::Reg<ch22_ctrl::CH22_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch22_ctrl;
#[doc = "CH22_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_src`]
module"]
pub type CH22_SRC = crate::Reg<ch22_src::CH22_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch22_src;
#[doc = "CH22_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_dst`]
module"]
pub type CH22_DST = crate::Reg<ch22_dst::CH22_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch22_dst;
#[doc = "CH22_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_link`]
module"]
pub type CH22_LINK = crate::Reg<ch22_link::CH22_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch22_link;
#[doc = "CH23_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_reqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_reqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_reqsel`]
module"]
pub type CH23_REQSEL = crate::Reg<ch23_reqsel::CH23_REQSEL_SPEC>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch23_reqsel;
#[doc = "CH23_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_cfg`]
module"]
pub type CH23_CFG = crate::Reg<ch23_cfg::CH23_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch23_cfg;
#[doc = "CH23_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_loop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_loop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_loop`]
module"]
pub type CH23_LOOP = crate::Reg<ch23_loop::CH23_LOOP_SPEC>;
#[doc = "Channel Loop Counter Register"]
pub mod ch23_loop;
#[doc = "CH23_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_ctrl`]
module"]
pub type CH23_CTRL = crate::Reg<ch23_ctrl::CH23_CTRL_SPEC>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch23_ctrl;
#[doc = "CH23_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_src`]
module"]
pub type CH23_SRC = crate::Reg<ch23_src::CH23_SRC_SPEC>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch23_src;
#[doc = "CH23_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_dst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_dst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_dst`]
module"]
pub type CH23_DST = crate::Reg<ch23_dst::CH23_DST_SPEC>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch23_dst;
#[doc = "CH23_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_link`]
module"]
pub type CH23_LINK = crate::Reg<ch23_link::CH23_LINK_SPEC>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch23_link;
