#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    sync: Sync,
    _reserved3: [u8; 0x14],
    chen: Chen,
    chbusy: Chbusy,
    chdone: Chdone,
    dbghalt: Dbghalt,
    swreq: Swreq,
    reqdis: Reqdis,
    reqpend: Reqpend,
    linkload: Linkload,
    reqclear: Reqclear,
    _reserved12: [u8; 0x1c],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved16: [u8; 0x10],
    ch0_reqsel: Ch0Reqsel,
    ch0_cfg: Ch0Cfg,
    ch0_loop: Ch0Loop,
    ch0_ctrl: Ch0Ctrl,
    ch0_src: Ch0Src,
    ch0_dst: Ch0Dst,
    ch0_link: Ch0Link,
    _reserved23: [u8; 0x14],
    ch1_reqsel: Ch1Reqsel,
    ch1_cfg: Ch1Cfg,
    ch1_loop: Ch1Loop,
    ch1_ctrl: Ch1Ctrl,
    ch1_src: Ch1Src,
    ch1_dst: Ch1Dst,
    ch1_link: Ch1Link,
    _reserved30: [u8; 0x14],
    ch2_reqsel: Ch2Reqsel,
    ch2_cfg: Ch2Cfg,
    ch2_loop: Ch2Loop,
    ch2_ctrl: Ch2Ctrl,
    ch2_src: Ch2Src,
    ch2_dst: Ch2Dst,
    ch2_link: Ch2Link,
    _reserved37: [u8; 0x14],
    ch3_reqsel: Ch3Reqsel,
    ch3_cfg: Ch3Cfg,
    ch3_loop: Ch3Loop,
    ch3_ctrl: Ch3Ctrl,
    ch3_src: Ch3Src,
    ch3_dst: Ch3Dst,
    ch3_link: Ch3Link,
    _reserved44: [u8; 0x14],
    ch4_reqsel: Ch4Reqsel,
    ch4_cfg: Ch4Cfg,
    ch4_loop: Ch4Loop,
    ch4_ctrl: Ch4Ctrl,
    ch4_src: Ch4Src,
    ch4_dst: Ch4Dst,
    ch4_link: Ch4Link,
    _reserved51: [u8; 0x14],
    ch5_reqsel: Ch5Reqsel,
    ch5_cfg: Ch5Cfg,
    ch5_loop: Ch5Loop,
    ch5_ctrl: Ch5Ctrl,
    ch5_src: Ch5Src,
    ch5_dst: Ch5Dst,
    ch5_link: Ch5Link,
    _reserved58: [u8; 0x14],
    ch6_reqsel: Ch6Reqsel,
    ch6_cfg: Ch6Cfg,
    ch6_loop: Ch6Loop,
    ch6_ctrl: Ch6Ctrl,
    ch6_src: Ch6Src,
    ch6_dst: Ch6Dst,
    ch6_link: Ch6Link,
    _reserved65: [u8; 0x14],
    ch7_reqsel: Ch7Reqsel,
    ch7_cfg: Ch7Cfg,
    ch7_loop: Ch7Loop,
    ch7_ctrl: Ch7Ctrl,
    ch7_src: Ch7Src,
    ch7_dst: Ch7Dst,
    ch7_link: Ch7Link,
    _reserved72: [u8; 0x14],
    ch8_reqsel: Ch8Reqsel,
    ch8_cfg: Ch8Cfg,
    ch8_loop: Ch8Loop,
    ch8_ctrl: Ch8Ctrl,
    ch8_src: Ch8Src,
    ch8_dst: Ch8Dst,
    ch8_link: Ch8Link,
    _reserved79: [u8; 0x14],
    ch9_reqsel: Ch9Reqsel,
    ch9_cfg: Ch9Cfg,
    ch9_loop: Ch9Loop,
    ch9_ctrl: Ch9Ctrl,
    ch9_src: Ch9Src,
    ch9_dst: Ch9Dst,
    ch9_link: Ch9Link,
    _reserved86: [u8; 0x14],
    ch10_reqsel: Ch10Reqsel,
    ch10_cfg: Ch10Cfg,
    ch10_loop: Ch10Loop,
    ch10_ctrl: Ch10Ctrl,
    ch10_src: Ch10Src,
    ch10_dst: Ch10Dst,
    ch10_link: Ch10Link,
    _reserved93: [u8; 0x14],
    ch11_reqsel: Ch11Reqsel,
    ch11_cfg: Ch11Cfg,
    ch11_loop: Ch11Loop,
    ch11_ctrl: Ch11Ctrl,
    ch11_src: Ch11Src,
    ch11_dst: Ch11Dst,
    ch11_link: Ch11Link,
    _reserved100: [u8; 0x14],
    ch12_reqsel: Ch12Reqsel,
    ch12_cfg: Ch12Cfg,
    ch12_loop: Ch12Loop,
    ch12_ctrl: Ch12Ctrl,
    ch12_src: Ch12Src,
    ch12_dst: Ch12Dst,
    ch12_link: Ch12Link,
    _reserved107: [u8; 0x14],
    ch13_reqsel: Ch13Reqsel,
    ch13_cfg: Ch13Cfg,
    ch13_loop: Ch13Loop,
    ch13_ctrl: Ch13Ctrl,
    ch13_src: Ch13Src,
    ch13_dst: Ch13Dst,
    ch13_link: Ch13Link,
    _reserved114: [u8; 0x14],
    ch14_reqsel: Ch14Reqsel,
    ch14_cfg: Ch14Cfg,
    ch14_loop: Ch14Loop,
    ch14_ctrl: Ch14Ctrl,
    ch14_src: Ch14Src,
    ch14_dst: Ch14Dst,
    ch14_link: Ch14Link,
    _reserved121: [u8; 0x14],
    ch15_reqsel: Ch15Reqsel,
    ch15_cfg: Ch15Cfg,
    ch15_loop: Ch15Loop,
    ch15_ctrl: Ch15Ctrl,
    ch15_src: Ch15Src,
    ch15_dst: Ch15Dst,
    ch15_link: Ch15Link,
    _reserved128: [u8; 0x14],
    ch16_reqsel: Ch16Reqsel,
    ch16_cfg: Ch16Cfg,
    ch16_loop: Ch16Loop,
    ch16_ctrl: Ch16Ctrl,
    ch16_src: Ch16Src,
    ch16_dst: Ch16Dst,
    ch16_link: Ch16Link,
    _reserved135: [u8; 0x14],
    ch17_reqsel: Ch17Reqsel,
    ch17_cfg: Ch17Cfg,
    ch17_loop: Ch17Loop,
    ch17_ctrl: Ch17Ctrl,
    ch17_src: Ch17Src,
    ch17_dst: Ch17Dst,
    ch17_link: Ch17Link,
    _reserved142: [u8; 0x14],
    ch18_reqsel: Ch18Reqsel,
    ch18_cfg: Ch18Cfg,
    ch18_loop: Ch18Loop,
    ch18_ctrl: Ch18Ctrl,
    ch18_src: Ch18Src,
    ch18_dst: Ch18Dst,
    ch18_link: Ch18Link,
    _reserved149: [u8; 0x14],
    ch19_reqsel: Ch19Reqsel,
    ch19_cfg: Ch19Cfg,
    ch19_loop: Ch19Loop,
    ch19_ctrl: Ch19Ctrl,
    ch19_src: Ch19Src,
    ch19_dst: Ch19Dst,
    ch19_link: Ch19Link,
    _reserved156: [u8; 0x14],
    ch20_reqsel: Ch20Reqsel,
    ch20_cfg: Ch20Cfg,
    ch20_loop: Ch20Loop,
    ch20_ctrl: Ch20Ctrl,
    ch20_src: Ch20Src,
    ch20_dst: Ch20Dst,
    ch20_link: Ch20Link,
    _reserved163: [u8; 0x14],
    ch21_reqsel: Ch21Reqsel,
    ch21_cfg: Ch21Cfg,
    ch21_loop: Ch21Loop,
    ch21_ctrl: Ch21Ctrl,
    ch21_src: Ch21Src,
    ch21_dst: Ch21Dst,
    ch21_link: Ch21Link,
    _reserved170: [u8; 0x14],
    ch22_reqsel: Ch22Reqsel,
    ch22_cfg: Ch22Cfg,
    ch22_loop: Ch22Loop,
    ch22_ctrl: Ch22Ctrl,
    ch22_src: Ch22Src,
    ch22_dst: Ch22Dst,
    ch22_link: Ch22Link,
    _reserved177: [u8; 0x14],
    ch23_reqsel: Ch23Reqsel,
    ch23_cfg: Ch23Cfg,
    ch23_loop: Ch23Loop,
    ch23_ctrl: Ch23Ctrl,
    ch23_src: Ch23Src,
    ch23_dst: Ch23Dst,
    ch23_link: Ch23Link,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - DMA Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x24 - DMA Channel Busy Register"]
    #[inline(always)]
    pub const fn chbusy(&self) -> &Chbusy {
        &self.chbusy
    }
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    #[inline(always)]
    pub const fn chdone(&self) -> &Chdone {
        &self.chdone
    }
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    #[inline(always)]
    pub const fn dbghalt(&self) -> &Dbghalt {
        &self.dbghalt
    }
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    #[inline(always)]
    pub const fn swreq(&self) -> &Swreq {
        &self.swreq
    }
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    #[inline(always)]
    pub const fn reqdis(&self) -> &Reqdis {
        &self.reqdis
    }
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    #[inline(always)]
    pub const fn reqpend(&self) -> &Reqpend {
        &self.reqpend
    }
    #[doc = "0x3c - DMA Channel Link Load Register"]
    #[inline(always)]
    pub const fn linkload(&self) -> &Linkload {
        &self.linkload
    }
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    #[inline(always)]
    pub const fn reqclear(&self) -> &Reqclear {
        &self.reqclear
    }
    #[doc = "0x60 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x64 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x6c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch0_reqsel(&self) -> &Ch0Reqsel {
        &self.ch0_reqsel
    }
    #[doc = "0x84 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch0_cfg(&self) -> &Ch0Cfg {
        &self.ch0_cfg
    }
    #[doc = "0x88 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch0_loop(&self) -> &Ch0Loop {
        &self.ch0_loop
    }
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch0_src(&self) -> &Ch0Src {
        &self.ch0_src
    }
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch0_dst(&self) -> &Ch0Dst {
        &self.ch0_dst
    }
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch0_link(&self) -> &Ch0Link {
        &self.ch0_link
    }
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch1_reqsel(&self) -> &Ch1Reqsel {
        &self.ch1_reqsel
    }
    #[doc = "0xb4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch1_cfg(&self) -> &Ch1Cfg {
        &self.ch1_cfg
    }
    #[doc = "0xb8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch1_loop(&self) -> &Ch1Loop {
        &self.ch1_loop
    }
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch1_src(&self) -> &Ch1Src {
        &self.ch1_src
    }
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch1_dst(&self) -> &Ch1Dst {
        &self.ch1_dst
    }
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch1_link(&self) -> &Ch1Link {
        &self.ch1_link
    }
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch2_reqsel(&self) -> &Ch2Reqsel {
        &self.ch2_reqsel
    }
    #[doc = "0xe4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch2_cfg(&self) -> &Ch2Cfg {
        &self.ch2_cfg
    }
    #[doc = "0xe8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch2_loop(&self) -> &Ch2Loop {
        &self.ch2_loop
    }
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch2_src(&self) -> &Ch2Src {
        &self.ch2_src
    }
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch2_dst(&self) -> &Ch2Dst {
        &self.ch2_dst
    }
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch2_link(&self) -> &Ch2Link {
        &self.ch2_link
    }
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch3_reqsel(&self) -> &Ch3Reqsel {
        &self.ch3_reqsel
    }
    #[doc = "0x114 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch3_cfg(&self) -> &Ch3Cfg {
        &self.ch3_cfg
    }
    #[doc = "0x118 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch3_loop(&self) -> &Ch3Loop {
        &self.ch3_loop
    }
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch3_src(&self) -> &Ch3Src {
        &self.ch3_src
    }
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch3_dst(&self) -> &Ch3Dst {
        &self.ch3_dst
    }
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch3_link(&self) -> &Ch3Link {
        &self.ch3_link
    }
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch4_reqsel(&self) -> &Ch4Reqsel {
        &self.ch4_reqsel
    }
    #[doc = "0x144 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch4_cfg(&self) -> &Ch4Cfg {
        &self.ch4_cfg
    }
    #[doc = "0x148 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch4_loop(&self) -> &Ch4Loop {
        &self.ch4_loop
    }
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch4_src(&self) -> &Ch4Src {
        &self.ch4_src
    }
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch4_dst(&self) -> &Ch4Dst {
        &self.ch4_dst
    }
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch4_link(&self) -> &Ch4Link {
        &self.ch4_link
    }
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch5_reqsel(&self) -> &Ch5Reqsel {
        &self.ch5_reqsel
    }
    #[doc = "0x174 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch5_cfg(&self) -> &Ch5Cfg {
        &self.ch5_cfg
    }
    #[doc = "0x178 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch5_loop(&self) -> &Ch5Loop {
        &self.ch5_loop
    }
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch5_src(&self) -> &Ch5Src {
        &self.ch5_src
    }
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch5_dst(&self) -> &Ch5Dst {
        &self.ch5_dst
    }
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch5_link(&self) -> &Ch5Link {
        &self.ch5_link
    }
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch6_reqsel(&self) -> &Ch6Reqsel {
        &self.ch6_reqsel
    }
    #[doc = "0x1a4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch6_cfg(&self) -> &Ch6Cfg {
        &self.ch6_cfg
    }
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch6_loop(&self) -> &Ch6Loop {
        &self.ch6_loop
    }
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &Ch6Ctrl {
        &self.ch6_ctrl
    }
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch6_src(&self) -> &Ch6Src {
        &self.ch6_src
    }
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch6_dst(&self) -> &Ch6Dst {
        &self.ch6_dst
    }
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch6_link(&self) -> &Ch6Link {
        &self.ch6_link
    }
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch7_reqsel(&self) -> &Ch7Reqsel {
        &self.ch7_reqsel
    }
    #[doc = "0x1d4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch7_cfg(&self) -> &Ch7Cfg {
        &self.ch7_cfg
    }
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch7_loop(&self) -> &Ch7Loop {
        &self.ch7_loop
    }
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &Ch7Ctrl {
        &self.ch7_ctrl
    }
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch7_src(&self) -> &Ch7Src {
        &self.ch7_src
    }
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch7_dst(&self) -> &Ch7Dst {
        &self.ch7_dst
    }
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch7_link(&self) -> &Ch7Link {
        &self.ch7_link
    }
    #[doc = "0x200 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch8_reqsel(&self) -> &Ch8Reqsel {
        &self.ch8_reqsel
    }
    #[doc = "0x204 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch8_cfg(&self) -> &Ch8Cfg {
        &self.ch8_cfg
    }
    #[doc = "0x208 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch8_loop(&self) -> &Ch8Loop {
        &self.ch8_loop
    }
    #[doc = "0x20c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch8_ctrl(&self) -> &Ch8Ctrl {
        &self.ch8_ctrl
    }
    #[doc = "0x210 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch8_src(&self) -> &Ch8Src {
        &self.ch8_src
    }
    #[doc = "0x214 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch8_dst(&self) -> &Ch8Dst {
        &self.ch8_dst
    }
    #[doc = "0x218 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch8_link(&self) -> &Ch8Link {
        &self.ch8_link
    }
    #[doc = "0x230 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch9_reqsel(&self) -> &Ch9Reqsel {
        &self.ch9_reqsel
    }
    #[doc = "0x234 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch9_cfg(&self) -> &Ch9Cfg {
        &self.ch9_cfg
    }
    #[doc = "0x238 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch9_loop(&self) -> &Ch9Loop {
        &self.ch9_loop
    }
    #[doc = "0x23c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch9_ctrl(&self) -> &Ch9Ctrl {
        &self.ch9_ctrl
    }
    #[doc = "0x240 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch9_src(&self) -> &Ch9Src {
        &self.ch9_src
    }
    #[doc = "0x244 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch9_dst(&self) -> &Ch9Dst {
        &self.ch9_dst
    }
    #[doc = "0x248 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch9_link(&self) -> &Ch9Link {
        &self.ch9_link
    }
    #[doc = "0x260 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch10_reqsel(&self) -> &Ch10Reqsel {
        &self.ch10_reqsel
    }
    #[doc = "0x264 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch10_cfg(&self) -> &Ch10Cfg {
        &self.ch10_cfg
    }
    #[doc = "0x268 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch10_loop(&self) -> &Ch10Loop {
        &self.ch10_loop
    }
    #[doc = "0x26c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch10_ctrl(&self) -> &Ch10Ctrl {
        &self.ch10_ctrl
    }
    #[doc = "0x270 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch10_src(&self) -> &Ch10Src {
        &self.ch10_src
    }
    #[doc = "0x274 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch10_dst(&self) -> &Ch10Dst {
        &self.ch10_dst
    }
    #[doc = "0x278 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch10_link(&self) -> &Ch10Link {
        &self.ch10_link
    }
    #[doc = "0x290 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch11_reqsel(&self) -> &Ch11Reqsel {
        &self.ch11_reqsel
    }
    #[doc = "0x294 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch11_cfg(&self) -> &Ch11Cfg {
        &self.ch11_cfg
    }
    #[doc = "0x298 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch11_loop(&self) -> &Ch11Loop {
        &self.ch11_loop
    }
    #[doc = "0x29c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch11_ctrl(&self) -> &Ch11Ctrl {
        &self.ch11_ctrl
    }
    #[doc = "0x2a0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch11_src(&self) -> &Ch11Src {
        &self.ch11_src
    }
    #[doc = "0x2a4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch11_dst(&self) -> &Ch11Dst {
        &self.ch11_dst
    }
    #[doc = "0x2a8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch11_link(&self) -> &Ch11Link {
        &self.ch11_link
    }
    #[doc = "0x2c0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch12_reqsel(&self) -> &Ch12Reqsel {
        &self.ch12_reqsel
    }
    #[doc = "0x2c4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch12_cfg(&self) -> &Ch12Cfg {
        &self.ch12_cfg
    }
    #[doc = "0x2c8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch12_loop(&self) -> &Ch12Loop {
        &self.ch12_loop
    }
    #[doc = "0x2cc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch12_ctrl(&self) -> &Ch12Ctrl {
        &self.ch12_ctrl
    }
    #[doc = "0x2d0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch12_src(&self) -> &Ch12Src {
        &self.ch12_src
    }
    #[doc = "0x2d4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch12_dst(&self) -> &Ch12Dst {
        &self.ch12_dst
    }
    #[doc = "0x2d8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch12_link(&self) -> &Ch12Link {
        &self.ch12_link
    }
    #[doc = "0x2f0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch13_reqsel(&self) -> &Ch13Reqsel {
        &self.ch13_reqsel
    }
    #[doc = "0x2f4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch13_cfg(&self) -> &Ch13Cfg {
        &self.ch13_cfg
    }
    #[doc = "0x2f8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch13_loop(&self) -> &Ch13Loop {
        &self.ch13_loop
    }
    #[doc = "0x2fc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch13_ctrl(&self) -> &Ch13Ctrl {
        &self.ch13_ctrl
    }
    #[doc = "0x300 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch13_src(&self) -> &Ch13Src {
        &self.ch13_src
    }
    #[doc = "0x304 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch13_dst(&self) -> &Ch13Dst {
        &self.ch13_dst
    }
    #[doc = "0x308 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch13_link(&self) -> &Ch13Link {
        &self.ch13_link
    }
    #[doc = "0x320 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch14_reqsel(&self) -> &Ch14Reqsel {
        &self.ch14_reqsel
    }
    #[doc = "0x324 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch14_cfg(&self) -> &Ch14Cfg {
        &self.ch14_cfg
    }
    #[doc = "0x328 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch14_loop(&self) -> &Ch14Loop {
        &self.ch14_loop
    }
    #[doc = "0x32c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch14_ctrl(&self) -> &Ch14Ctrl {
        &self.ch14_ctrl
    }
    #[doc = "0x330 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch14_src(&self) -> &Ch14Src {
        &self.ch14_src
    }
    #[doc = "0x334 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch14_dst(&self) -> &Ch14Dst {
        &self.ch14_dst
    }
    #[doc = "0x338 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch14_link(&self) -> &Ch14Link {
        &self.ch14_link
    }
    #[doc = "0x350 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch15_reqsel(&self) -> &Ch15Reqsel {
        &self.ch15_reqsel
    }
    #[doc = "0x354 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch15_cfg(&self) -> &Ch15Cfg {
        &self.ch15_cfg
    }
    #[doc = "0x358 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch15_loop(&self) -> &Ch15Loop {
        &self.ch15_loop
    }
    #[doc = "0x35c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch15_ctrl(&self) -> &Ch15Ctrl {
        &self.ch15_ctrl
    }
    #[doc = "0x360 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch15_src(&self) -> &Ch15Src {
        &self.ch15_src
    }
    #[doc = "0x364 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch15_dst(&self) -> &Ch15Dst {
        &self.ch15_dst
    }
    #[doc = "0x368 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch15_link(&self) -> &Ch15Link {
        &self.ch15_link
    }
    #[doc = "0x380 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch16_reqsel(&self) -> &Ch16Reqsel {
        &self.ch16_reqsel
    }
    #[doc = "0x384 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch16_cfg(&self) -> &Ch16Cfg {
        &self.ch16_cfg
    }
    #[doc = "0x388 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch16_loop(&self) -> &Ch16Loop {
        &self.ch16_loop
    }
    #[doc = "0x38c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch16_ctrl(&self) -> &Ch16Ctrl {
        &self.ch16_ctrl
    }
    #[doc = "0x390 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch16_src(&self) -> &Ch16Src {
        &self.ch16_src
    }
    #[doc = "0x394 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch16_dst(&self) -> &Ch16Dst {
        &self.ch16_dst
    }
    #[doc = "0x398 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch16_link(&self) -> &Ch16Link {
        &self.ch16_link
    }
    #[doc = "0x3b0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch17_reqsel(&self) -> &Ch17Reqsel {
        &self.ch17_reqsel
    }
    #[doc = "0x3b4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch17_cfg(&self) -> &Ch17Cfg {
        &self.ch17_cfg
    }
    #[doc = "0x3b8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch17_loop(&self) -> &Ch17Loop {
        &self.ch17_loop
    }
    #[doc = "0x3bc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch17_ctrl(&self) -> &Ch17Ctrl {
        &self.ch17_ctrl
    }
    #[doc = "0x3c0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch17_src(&self) -> &Ch17Src {
        &self.ch17_src
    }
    #[doc = "0x3c4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch17_dst(&self) -> &Ch17Dst {
        &self.ch17_dst
    }
    #[doc = "0x3c8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch17_link(&self) -> &Ch17Link {
        &self.ch17_link
    }
    #[doc = "0x3e0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch18_reqsel(&self) -> &Ch18Reqsel {
        &self.ch18_reqsel
    }
    #[doc = "0x3e4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch18_cfg(&self) -> &Ch18Cfg {
        &self.ch18_cfg
    }
    #[doc = "0x3e8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch18_loop(&self) -> &Ch18Loop {
        &self.ch18_loop
    }
    #[doc = "0x3ec - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch18_ctrl(&self) -> &Ch18Ctrl {
        &self.ch18_ctrl
    }
    #[doc = "0x3f0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch18_src(&self) -> &Ch18Src {
        &self.ch18_src
    }
    #[doc = "0x3f4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch18_dst(&self) -> &Ch18Dst {
        &self.ch18_dst
    }
    #[doc = "0x3f8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch18_link(&self) -> &Ch18Link {
        &self.ch18_link
    }
    #[doc = "0x410 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch19_reqsel(&self) -> &Ch19Reqsel {
        &self.ch19_reqsel
    }
    #[doc = "0x414 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch19_cfg(&self) -> &Ch19Cfg {
        &self.ch19_cfg
    }
    #[doc = "0x418 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch19_loop(&self) -> &Ch19Loop {
        &self.ch19_loop
    }
    #[doc = "0x41c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch19_ctrl(&self) -> &Ch19Ctrl {
        &self.ch19_ctrl
    }
    #[doc = "0x420 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch19_src(&self) -> &Ch19Src {
        &self.ch19_src
    }
    #[doc = "0x424 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch19_dst(&self) -> &Ch19Dst {
        &self.ch19_dst
    }
    #[doc = "0x428 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch19_link(&self) -> &Ch19Link {
        &self.ch19_link
    }
    #[doc = "0x440 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch20_reqsel(&self) -> &Ch20Reqsel {
        &self.ch20_reqsel
    }
    #[doc = "0x444 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch20_cfg(&self) -> &Ch20Cfg {
        &self.ch20_cfg
    }
    #[doc = "0x448 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch20_loop(&self) -> &Ch20Loop {
        &self.ch20_loop
    }
    #[doc = "0x44c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch20_ctrl(&self) -> &Ch20Ctrl {
        &self.ch20_ctrl
    }
    #[doc = "0x450 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch20_src(&self) -> &Ch20Src {
        &self.ch20_src
    }
    #[doc = "0x454 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch20_dst(&self) -> &Ch20Dst {
        &self.ch20_dst
    }
    #[doc = "0x458 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch20_link(&self) -> &Ch20Link {
        &self.ch20_link
    }
    #[doc = "0x470 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch21_reqsel(&self) -> &Ch21Reqsel {
        &self.ch21_reqsel
    }
    #[doc = "0x474 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch21_cfg(&self) -> &Ch21Cfg {
        &self.ch21_cfg
    }
    #[doc = "0x478 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch21_loop(&self) -> &Ch21Loop {
        &self.ch21_loop
    }
    #[doc = "0x47c - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch21_ctrl(&self) -> &Ch21Ctrl {
        &self.ch21_ctrl
    }
    #[doc = "0x480 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch21_src(&self) -> &Ch21Src {
        &self.ch21_src
    }
    #[doc = "0x484 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch21_dst(&self) -> &Ch21Dst {
        &self.ch21_dst
    }
    #[doc = "0x488 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch21_link(&self) -> &Ch21Link {
        &self.ch21_link
    }
    #[doc = "0x4a0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch22_reqsel(&self) -> &Ch22Reqsel {
        &self.ch22_reqsel
    }
    #[doc = "0x4a4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch22_cfg(&self) -> &Ch22Cfg {
        &self.ch22_cfg
    }
    #[doc = "0x4a8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch22_loop(&self) -> &Ch22Loop {
        &self.ch22_loop
    }
    #[doc = "0x4ac - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch22_ctrl(&self) -> &Ch22Ctrl {
        &self.ch22_ctrl
    }
    #[doc = "0x4b0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch22_src(&self) -> &Ch22Src {
        &self.ch22_src
    }
    #[doc = "0x4b4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch22_dst(&self) -> &Ch22Dst {
        &self.ch22_dst
    }
    #[doc = "0x4b8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch22_link(&self) -> &Ch22Link {
        &self.ch22_link
    }
    #[doc = "0x4d0 - Channel Peripheral Request Select Register"]
    #[inline(always)]
    pub const fn ch23_reqsel(&self) -> &Ch23Reqsel {
        &self.ch23_reqsel
    }
    #[doc = "0x4d4 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch23_cfg(&self) -> &Ch23Cfg {
        &self.ch23_cfg
    }
    #[doc = "0x4d8 - Channel Loop Counter Register"]
    #[inline(always)]
    pub const fn ch23_loop(&self) -> &Ch23Loop {
        &self.ch23_loop
    }
    #[doc = "0x4dc - Channel Descriptor Control Word Register"]
    #[inline(always)]
    pub const fn ch23_ctrl(&self) -> &Ch23Ctrl {
        &self.ch23_ctrl
    }
    #[doc = "0x4e0 - Channel Descriptor Source Data Address Register"]
    #[inline(always)]
    pub const fn ch23_src(&self) -> &Ch23Src {
        &self.ch23_src
    }
    #[doc = "0x4e4 - Channel Descriptor Destination Data Address Register"]
    #[inline(always)]
    pub const fn ch23_dst(&self) -> &Ch23Dst {
        &self.ch23_dst
    }
    #[doc = "0x4e8 - Channel Descriptor Link Structure Address Register"]
    #[inline(always)]
    pub const fn ch23_link(&self) -> &Ch23Link {
        &self.ch23_link
    }
}
#[doc = "CTRL (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: DMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "SYNC (rw) register accessor: DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`] module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "CHEN (rw) register accessor: DMA Channel Enable Register (Single-Cycle RMW)\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`] module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "CHBUSY (r) register accessor: DMA Channel Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbusy`] module"]
#[doc(alias = "CHBUSY")]
pub type Chbusy = crate::Reg<chbusy::ChbusySpec>;
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE (rw) register accessor: DMA Channel Linking Done Register (Single-Cycle RMW)\n\nYou can [`read`](crate::Reg::read) this register and get [`chdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdone`] module"]
#[doc(alias = "CHDONE")]
pub type Chdone = crate::Reg<chdone::ChdoneSpec>;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DBGHALT (rw) register accessor: DMA Channel Debug Halt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbghalt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbghalt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbghalt`] module"]
#[doc(alias = "DBGHALT")]
pub type Dbghalt = crate::Reg<dbghalt::DbghaltSpec>;
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ (w) register accessor: DMA Channel Software Transfer Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`] module"]
#[doc(alias = "SWREQ")]
pub type Swreq = crate::Reg<swreq::SwreqSpec>;
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "REQDIS (rw) register accessor: DMA Channel Request Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reqdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdis`] module"]
#[doc(alias = "REQDIS")]
pub type Reqdis = crate::Reg<reqdis::ReqdisSpec>;
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "REQPEND (r) register accessor: DMA Channel Requests Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reqpend::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqpend`] module"]
#[doc(alias = "REQPEND")]
pub type Reqpend = crate::Reg<reqpend::ReqpendSpec>;
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD (w) register accessor: DMA Channel Link Load Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkload`] module"]
#[doc(alias = "LINKLOAD")]
pub type Linkload = crate::Reg<linkload::LinkloadSpec>;
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "REQCLEAR (w) register accessor: DMA Channel Request Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqclear`] module"]
#[doc(alias = "REQCLEAR")]
pub type Reqclear = crate::Reg<reqclear::ReqclearSpec>;
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "CH0_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_reqsel`] module"]
#[doc(alias = "CH0_REQSEL")]
pub type Ch0Reqsel = crate::Reg<ch0_reqsel::Ch0ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "CH0_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_cfg`] module"]
#[doc(alias = "CH0_CFG")]
pub type Ch0Cfg = crate::Reg<ch0_cfg::Ch0CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_loop`] module"]
#[doc(alias = "CH0_LOOP")]
pub type Ch0Loop = crate::Reg<ch0_loop::Ch0LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "CH0_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`] module"]
#[doc(alias = "CH0_CTRL")]
pub type Ch0Ctrl = crate::Reg<ch0_ctrl::Ch0CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_src`] module"]
#[doc(alias = "CH0_SRC")]
pub type Ch0Src = crate::Reg<ch0_src::Ch0SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "CH0_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dst`] module"]
#[doc(alias = "CH0_DST")]
pub type Ch0Dst = crate::Reg<ch0_dst::Ch0DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "CH0_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_link`] module"]
#[doc(alias = "CH0_LINK")]
pub type Ch0Link = crate::Reg<ch0_link::Ch0LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "CH1_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_reqsel`] module"]
#[doc(alias = "CH1_REQSEL")]
pub type Ch1Reqsel = crate::Reg<ch1_reqsel::Ch1ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "CH1_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg`] module"]
#[doc(alias = "CH1_CFG")]
pub type Ch1Cfg = crate::Reg<ch1_cfg::Ch1CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_loop`] module"]
#[doc(alias = "CH1_LOOP")]
pub type Ch1Loop = crate::Reg<ch1_loop::Ch1LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "CH1_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`] module"]
#[doc(alias = "CH1_CTRL")]
pub type Ch1Ctrl = crate::Reg<ch1_ctrl::Ch1CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_src`] module"]
#[doc(alias = "CH1_SRC")]
pub type Ch1Src = crate::Reg<ch1_src::Ch1SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "CH1_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dst`] module"]
#[doc(alias = "CH1_DST")]
pub type Ch1Dst = crate::Reg<ch1_dst::Ch1DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "CH1_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_link`] module"]
#[doc(alias = "CH1_LINK")]
pub type Ch1Link = crate::Reg<ch1_link::Ch1LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "CH2_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_reqsel`] module"]
#[doc(alias = "CH2_REQSEL")]
pub type Ch2Reqsel = crate::Reg<ch2_reqsel::Ch2ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "CH2_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg`] module"]
#[doc(alias = "CH2_CFG")]
pub type Ch2Cfg = crate::Reg<ch2_cfg::Ch2CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_loop`] module"]
#[doc(alias = "CH2_LOOP")]
pub type Ch2Loop = crate::Reg<ch2_loop::Ch2LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "CH2_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`] module"]
#[doc(alias = "CH2_CTRL")]
pub type Ch2Ctrl = crate::Reg<ch2_ctrl::Ch2CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_src`] module"]
#[doc(alias = "CH2_SRC")]
pub type Ch2Src = crate::Reg<ch2_src::Ch2SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "CH2_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dst`] module"]
#[doc(alias = "CH2_DST")]
pub type Ch2Dst = crate::Reg<ch2_dst::Ch2DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "CH2_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_link`] module"]
#[doc(alias = "CH2_LINK")]
pub type Ch2Link = crate::Reg<ch2_link::Ch2LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "CH3_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_reqsel`] module"]
#[doc(alias = "CH3_REQSEL")]
pub type Ch3Reqsel = crate::Reg<ch3_reqsel::Ch3ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "CH3_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg`] module"]
#[doc(alias = "CH3_CFG")]
pub type Ch3Cfg = crate::Reg<ch3_cfg::Ch3CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_loop`] module"]
#[doc(alias = "CH3_LOOP")]
pub type Ch3Loop = crate::Reg<ch3_loop::Ch3LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "CH3_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`] module"]
#[doc(alias = "CH3_CTRL")]
pub type Ch3Ctrl = crate::Reg<ch3_ctrl::Ch3CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_src`] module"]
#[doc(alias = "CH3_SRC")]
pub type Ch3Src = crate::Reg<ch3_src::Ch3SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "CH3_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dst`] module"]
#[doc(alias = "CH3_DST")]
pub type Ch3Dst = crate::Reg<ch3_dst::Ch3DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "CH3_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_link`] module"]
#[doc(alias = "CH3_LINK")]
pub type Ch3Link = crate::Reg<ch3_link::Ch3LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "CH4_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_reqsel`] module"]
#[doc(alias = "CH4_REQSEL")]
pub type Ch4Reqsel = crate::Reg<ch4_reqsel::Ch4ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "CH4_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg`] module"]
#[doc(alias = "CH4_CFG")]
pub type Ch4Cfg = crate::Reg<ch4_cfg::Ch4CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_loop`] module"]
#[doc(alias = "CH4_LOOP")]
pub type Ch4Loop = crate::Reg<ch4_loop::Ch4LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "CH4_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`] module"]
#[doc(alias = "CH4_CTRL")]
pub type Ch4Ctrl = crate::Reg<ch4_ctrl::Ch4CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_src`] module"]
#[doc(alias = "CH4_SRC")]
pub type Ch4Src = crate::Reg<ch4_src::Ch4SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "CH4_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dst`] module"]
#[doc(alias = "CH4_DST")]
pub type Ch4Dst = crate::Reg<ch4_dst::Ch4DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "CH4_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_link`] module"]
#[doc(alias = "CH4_LINK")]
pub type Ch4Link = crate::Reg<ch4_link::Ch4LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "CH5_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_reqsel`] module"]
#[doc(alias = "CH5_REQSEL")]
pub type Ch5Reqsel = crate::Reg<ch5_reqsel::Ch5ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "CH5_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_cfg`] module"]
#[doc(alias = "CH5_CFG")]
pub type Ch5Cfg = crate::Reg<ch5_cfg::Ch5CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_loop`] module"]
#[doc(alias = "CH5_LOOP")]
pub type Ch5Loop = crate::Reg<ch5_loop::Ch5LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "CH5_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`] module"]
#[doc(alias = "CH5_CTRL")]
pub type Ch5Ctrl = crate::Reg<ch5_ctrl::Ch5CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_src`] module"]
#[doc(alias = "CH5_SRC")]
pub type Ch5Src = crate::Reg<ch5_src::Ch5SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "CH5_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_dst`] module"]
#[doc(alias = "CH5_DST")]
pub type Ch5Dst = crate::Reg<ch5_dst::Ch5DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "CH5_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_link`] module"]
#[doc(alias = "CH5_LINK")]
pub type Ch5Link = crate::Reg<ch5_link::Ch5LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "CH6_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_reqsel`] module"]
#[doc(alias = "CH6_REQSEL")]
pub type Ch6Reqsel = crate::Reg<ch6_reqsel::Ch6ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "CH6_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_cfg`] module"]
#[doc(alias = "CH6_CFG")]
pub type Ch6Cfg = crate::Reg<ch6_cfg::Ch6CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_loop`] module"]
#[doc(alias = "CH6_LOOP")]
pub type Ch6Loop = crate::Reg<ch6_loop::Ch6LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "CH6_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`] module"]
#[doc(alias = "CH6_CTRL")]
pub type Ch6Ctrl = crate::Reg<ch6_ctrl::Ch6CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_src`] module"]
#[doc(alias = "CH6_SRC")]
pub type Ch6Src = crate::Reg<ch6_src::Ch6SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "CH6_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_dst`] module"]
#[doc(alias = "CH6_DST")]
pub type Ch6Dst = crate::Reg<ch6_dst::Ch6DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "CH6_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_link`] module"]
#[doc(alias = "CH6_LINK")]
pub type Ch6Link = crate::Reg<ch6_link::Ch6LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "CH7_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_reqsel`] module"]
#[doc(alias = "CH7_REQSEL")]
pub type Ch7Reqsel = crate::Reg<ch7_reqsel::Ch7ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "CH7_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_cfg`] module"]
#[doc(alias = "CH7_CFG")]
pub type Ch7Cfg = crate::Reg<ch7_cfg::Ch7CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_loop`] module"]
#[doc(alias = "CH7_LOOP")]
pub type Ch7Loop = crate::Reg<ch7_loop::Ch7LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "CH7_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`] module"]
#[doc(alias = "CH7_CTRL")]
pub type Ch7Ctrl = crate::Reg<ch7_ctrl::Ch7CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_src`] module"]
#[doc(alias = "CH7_SRC")]
pub type Ch7Src = crate::Reg<ch7_src::Ch7SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "CH7_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_dst`] module"]
#[doc(alias = "CH7_DST")]
pub type Ch7Dst = crate::Reg<ch7_dst::Ch7DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "CH7_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_link`] module"]
#[doc(alias = "CH7_LINK")]
pub type Ch7Link = crate::Reg<ch7_link::Ch7LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
#[doc = "CH8_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_reqsel`] module"]
#[doc(alias = "CH8_REQSEL")]
pub type Ch8Reqsel = crate::Reg<ch8_reqsel::Ch8ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch8_reqsel;
#[doc = "CH8_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_cfg`] module"]
#[doc(alias = "CH8_CFG")]
pub type Ch8Cfg = crate::Reg<ch8_cfg::Ch8CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch8_cfg;
#[doc = "CH8_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_loop`] module"]
#[doc(alias = "CH8_LOOP")]
pub type Ch8Loop = crate::Reg<ch8_loop::Ch8LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch8_loop;
#[doc = "CH8_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ctrl`] module"]
#[doc(alias = "CH8_CTRL")]
pub type Ch8Ctrl = crate::Reg<ch8_ctrl::Ch8CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch8_ctrl;
#[doc = "CH8_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_src`] module"]
#[doc(alias = "CH8_SRC")]
pub type Ch8Src = crate::Reg<ch8_src::Ch8SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch8_src;
#[doc = "CH8_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_dst`] module"]
#[doc(alias = "CH8_DST")]
pub type Ch8Dst = crate::Reg<ch8_dst::Ch8DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch8_dst;
#[doc = "CH8_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_link`] module"]
#[doc(alias = "CH8_LINK")]
pub type Ch8Link = crate::Reg<ch8_link::Ch8LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch8_link;
#[doc = "CH9_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_reqsel`] module"]
#[doc(alias = "CH9_REQSEL")]
pub type Ch9Reqsel = crate::Reg<ch9_reqsel::Ch9ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch9_reqsel;
#[doc = "CH9_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_cfg`] module"]
#[doc(alias = "CH9_CFG")]
pub type Ch9Cfg = crate::Reg<ch9_cfg::Ch9CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch9_cfg;
#[doc = "CH9_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_loop`] module"]
#[doc(alias = "CH9_LOOP")]
pub type Ch9Loop = crate::Reg<ch9_loop::Ch9LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch9_loop;
#[doc = "CH9_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ctrl`] module"]
#[doc(alias = "CH9_CTRL")]
pub type Ch9Ctrl = crate::Reg<ch9_ctrl::Ch9CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch9_ctrl;
#[doc = "CH9_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_src`] module"]
#[doc(alias = "CH9_SRC")]
pub type Ch9Src = crate::Reg<ch9_src::Ch9SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch9_src;
#[doc = "CH9_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_dst`] module"]
#[doc(alias = "CH9_DST")]
pub type Ch9Dst = crate::Reg<ch9_dst::Ch9DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch9_dst;
#[doc = "CH9_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_link`] module"]
#[doc(alias = "CH9_LINK")]
pub type Ch9Link = crate::Reg<ch9_link::Ch9LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch9_link;
#[doc = "CH10_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_reqsel`] module"]
#[doc(alias = "CH10_REQSEL")]
pub type Ch10Reqsel = crate::Reg<ch10_reqsel::Ch10ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch10_reqsel;
#[doc = "CH10_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_cfg`] module"]
#[doc(alias = "CH10_CFG")]
pub type Ch10Cfg = crate::Reg<ch10_cfg::Ch10CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch10_cfg;
#[doc = "CH10_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_loop`] module"]
#[doc(alias = "CH10_LOOP")]
pub type Ch10Loop = crate::Reg<ch10_loop::Ch10LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch10_loop;
#[doc = "CH10_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ctrl`] module"]
#[doc(alias = "CH10_CTRL")]
pub type Ch10Ctrl = crate::Reg<ch10_ctrl::Ch10CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch10_ctrl;
#[doc = "CH10_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_src`] module"]
#[doc(alias = "CH10_SRC")]
pub type Ch10Src = crate::Reg<ch10_src::Ch10SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch10_src;
#[doc = "CH10_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_dst`] module"]
#[doc(alias = "CH10_DST")]
pub type Ch10Dst = crate::Reg<ch10_dst::Ch10DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch10_dst;
#[doc = "CH10_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_link`] module"]
#[doc(alias = "CH10_LINK")]
pub type Ch10Link = crate::Reg<ch10_link::Ch10LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch10_link;
#[doc = "CH11_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_reqsel`] module"]
#[doc(alias = "CH11_REQSEL")]
pub type Ch11Reqsel = crate::Reg<ch11_reqsel::Ch11ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch11_reqsel;
#[doc = "CH11_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_cfg`] module"]
#[doc(alias = "CH11_CFG")]
pub type Ch11Cfg = crate::Reg<ch11_cfg::Ch11CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch11_cfg;
#[doc = "CH11_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_loop`] module"]
#[doc(alias = "CH11_LOOP")]
pub type Ch11Loop = crate::Reg<ch11_loop::Ch11LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch11_loop;
#[doc = "CH11_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ctrl`] module"]
#[doc(alias = "CH11_CTRL")]
pub type Ch11Ctrl = crate::Reg<ch11_ctrl::Ch11CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch11_ctrl;
#[doc = "CH11_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_src`] module"]
#[doc(alias = "CH11_SRC")]
pub type Ch11Src = crate::Reg<ch11_src::Ch11SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch11_src;
#[doc = "CH11_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_dst`] module"]
#[doc(alias = "CH11_DST")]
pub type Ch11Dst = crate::Reg<ch11_dst::Ch11DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch11_dst;
#[doc = "CH11_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_link`] module"]
#[doc(alias = "CH11_LINK")]
pub type Ch11Link = crate::Reg<ch11_link::Ch11LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch11_link;
#[doc = "CH12_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_reqsel`] module"]
#[doc(alias = "CH12_REQSEL")]
pub type Ch12Reqsel = crate::Reg<ch12_reqsel::Ch12ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch12_reqsel;
#[doc = "CH12_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_cfg`] module"]
#[doc(alias = "CH12_CFG")]
pub type Ch12Cfg = crate::Reg<ch12_cfg::Ch12CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch12_cfg;
#[doc = "CH12_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_loop`] module"]
#[doc(alias = "CH12_LOOP")]
pub type Ch12Loop = crate::Reg<ch12_loop::Ch12LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch12_loop;
#[doc = "CH12_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_ctrl`] module"]
#[doc(alias = "CH12_CTRL")]
pub type Ch12Ctrl = crate::Reg<ch12_ctrl::Ch12CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch12_ctrl;
#[doc = "CH12_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_src`] module"]
#[doc(alias = "CH12_SRC")]
pub type Ch12Src = crate::Reg<ch12_src::Ch12SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch12_src;
#[doc = "CH12_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_dst`] module"]
#[doc(alias = "CH12_DST")]
pub type Ch12Dst = crate::Reg<ch12_dst::Ch12DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch12_dst;
#[doc = "CH12_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_link`] module"]
#[doc(alias = "CH12_LINK")]
pub type Ch12Link = crate::Reg<ch12_link::Ch12LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch12_link;
#[doc = "CH13_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_reqsel`] module"]
#[doc(alias = "CH13_REQSEL")]
pub type Ch13Reqsel = crate::Reg<ch13_reqsel::Ch13ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch13_reqsel;
#[doc = "CH13_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_cfg`] module"]
#[doc(alias = "CH13_CFG")]
pub type Ch13Cfg = crate::Reg<ch13_cfg::Ch13CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch13_cfg;
#[doc = "CH13_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_loop`] module"]
#[doc(alias = "CH13_LOOP")]
pub type Ch13Loop = crate::Reg<ch13_loop::Ch13LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch13_loop;
#[doc = "CH13_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_ctrl`] module"]
#[doc(alias = "CH13_CTRL")]
pub type Ch13Ctrl = crate::Reg<ch13_ctrl::Ch13CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch13_ctrl;
#[doc = "CH13_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_src`] module"]
#[doc(alias = "CH13_SRC")]
pub type Ch13Src = crate::Reg<ch13_src::Ch13SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch13_src;
#[doc = "CH13_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_dst`] module"]
#[doc(alias = "CH13_DST")]
pub type Ch13Dst = crate::Reg<ch13_dst::Ch13DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch13_dst;
#[doc = "CH13_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_link`] module"]
#[doc(alias = "CH13_LINK")]
pub type Ch13Link = crate::Reg<ch13_link::Ch13LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch13_link;
#[doc = "CH14_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_reqsel`] module"]
#[doc(alias = "CH14_REQSEL")]
pub type Ch14Reqsel = crate::Reg<ch14_reqsel::Ch14ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch14_reqsel;
#[doc = "CH14_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_cfg`] module"]
#[doc(alias = "CH14_CFG")]
pub type Ch14Cfg = crate::Reg<ch14_cfg::Ch14CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch14_cfg;
#[doc = "CH14_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_loop`] module"]
#[doc(alias = "CH14_LOOP")]
pub type Ch14Loop = crate::Reg<ch14_loop::Ch14LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch14_loop;
#[doc = "CH14_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_ctrl`] module"]
#[doc(alias = "CH14_CTRL")]
pub type Ch14Ctrl = crate::Reg<ch14_ctrl::Ch14CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch14_ctrl;
#[doc = "CH14_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_src`] module"]
#[doc(alias = "CH14_SRC")]
pub type Ch14Src = crate::Reg<ch14_src::Ch14SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch14_src;
#[doc = "CH14_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_dst`] module"]
#[doc(alias = "CH14_DST")]
pub type Ch14Dst = crate::Reg<ch14_dst::Ch14DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch14_dst;
#[doc = "CH14_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_link`] module"]
#[doc(alias = "CH14_LINK")]
pub type Ch14Link = crate::Reg<ch14_link::Ch14LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch14_link;
#[doc = "CH15_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_reqsel`] module"]
#[doc(alias = "CH15_REQSEL")]
pub type Ch15Reqsel = crate::Reg<ch15_reqsel::Ch15ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch15_reqsel;
#[doc = "CH15_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_cfg`] module"]
#[doc(alias = "CH15_CFG")]
pub type Ch15Cfg = crate::Reg<ch15_cfg::Ch15CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch15_cfg;
#[doc = "CH15_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_loop`] module"]
#[doc(alias = "CH15_LOOP")]
pub type Ch15Loop = crate::Reg<ch15_loop::Ch15LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch15_loop;
#[doc = "CH15_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_ctrl`] module"]
#[doc(alias = "CH15_CTRL")]
pub type Ch15Ctrl = crate::Reg<ch15_ctrl::Ch15CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch15_ctrl;
#[doc = "CH15_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_src`] module"]
#[doc(alias = "CH15_SRC")]
pub type Ch15Src = crate::Reg<ch15_src::Ch15SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch15_src;
#[doc = "CH15_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_dst`] module"]
#[doc(alias = "CH15_DST")]
pub type Ch15Dst = crate::Reg<ch15_dst::Ch15DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch15_dst;
#[doc = "CH15_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_link`] module"]
#[doc(alias = "CH15_LINK")]
pub type Ch15Link = crate::Reg<ch15_link::Ch15LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch15_link;
#[doc = "CH16_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_reqsel`] module"]
#[doc(alias = "CH16_REQSEL")]
pub type Ch16Reqsel = crate::Reg<ch16_reqsel::Ch16ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch16_reqsel;
#[doc = "CH16_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_cfg`] module"]
#[doc(alias = "CH16_CFG")]
pub type Ch16Cfg = crate::Reg<ch16_cfg::Ch16CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch16_cfg;
#[doc = "CH16_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_loop`] module"]
#[doc(alias = "CH16_LOOP")]
pub type Ch16Loop = crate::Reg<ch16_loop::Ch16LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch16_loop;
#[doc = "CH16_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_ctrl`] module"]
#[doc(alias = "CH16_CTRL")]
pub type Ch16Ctrl = crate::Reg<ch16_ctrl::Ch16CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch16_ctrl;
#[doc = "CH16_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_src`] module"]
#[doc(alias = "CH16_SRC")]
pub type Ch16Src = crate::Reg<ch16_src::Ch16SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch16_src;
#[doc = "CH16_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_dst`] module"]
#[doc(alias = "CH16_DST")]
pub type Ch16Dst = crate::Reg<ch16_dst::Ch16DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch16_dst;
#[doc = "CH16_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_link`] module"]
#[doc(alias = "CH16_LINK")]
pub type Ch16Link = crate::Reg<ch16_link::Ch16LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch16_link;
#[doc = "CH17_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_reqsel`] module"]
#[doc(alias = "CH17_REQSEL")]
pub type Ch17Reqsel = crate::Reg<ch17_reqsel::Ch17ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch17_reqsel;
#[doc = "CH17_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_cfg`] module"]
#[doc(alias = "CH17_CFG")]
pub type Ch17Cfg = crate::Reg<ch17_cfg::Ch17CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch17_cfg;
#[doc = "CH17_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_loop`] module"]
#[doc(alias = "CH17_LOOP")]
pub type Ch17Loop = crate::Reg<ch17_loop::Ch17LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch17_loop;
#[doc = "CH17_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_ctrl`] module"]
#[doc(alias = "CH17_CTRL")]
pub type Ch17Ctrl = crate::Reg<ch17_ctrl::Ch17CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch17_ctrl;
#[doc = "CH17_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_src`] module"]
#[doc(alias = "CH17_SRC")]
pub type Ch17Src = crate::Reg<ch17_src::Ch17SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch17_src;
#[doc = "CH17_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_dst`] module"]
#[doc(alias = "CH17_DST")]
pub type Ch17Dst = crate::Reg<ch17_dst::Ch17DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch17_dst;
#[doc = "CH17_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_link`] module"]
#[doc(alias = "CH17_LINK")]
pub type Ch17Link = crate::Reg<ch17_link::Ch17LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch17_link;
#[doc = "CH18_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_reqsel`] module"]
#[doc(alias = "CH18_REQSEL")]
pub type Ch18Reqsel = crate::Reg<ch18_reqsel::Ch18ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch18_reqsel;
#[doc = "CH18_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_cfg`] module"]
#[doc(alias = "CH18_CFG")]
pub type Ch18Cfg = crate::Reg<ch18_cfg::Ch18CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch18_cfg;
#[doc = "CH18_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_loop`] module"]
#[doc(alias = "CH18_LOOP")]
pub type Ch18Loop = crate::Reg<ch18_loop::Ch18LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch18_loop;
#[doc = "CH18_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_ctrl`] module"]
#[doc(alias = "CH18_CTRL")]
pub type Ch18Ctrl = crate::Reg<ch18_ctrl::Ch18CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch18_ctrl;
#[doc = "CH18_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_src`] module"]
#[doc(alias = "CH18_SRC")]
pub type Ch18Src = crate::Reg<ch18_src::Ch18SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch18_src;
#[doc = "CH18_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_dst`] module"]
#[doc(alias = "CH18_DST")]
pub type Ch18Dst = crate::Reg<ch18_dst::Ch18DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch18_dst;
#[doc = "CH18_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_link`] module"]
#[doc(alias = "CH18_LINK")]
pub type Ch18Link = crate::Reg<ch18_link::Ch18LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch18_link;
#[doc = "CH19_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_reqsel`] module"]
#[doc(alias = "CH19_REQSEL")]
pub type Ch19Reqsel = crate::Reg<ch19_reqsel::Ch19ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch19_reqsel;
#[doc = "CH19_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_cfg`] module"]
#[doc(alias = "CH19_CFG")]
pub type Ch19Cfg = crate::Reg<ch19_cfg::Ch19CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch19_cfg;
#[doc = "CH19_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_loop`] module"]
#[doc(alias = "CH19_LOOP")]
pub type Ch19Loop = crate::Reg<ch19_loop::Ch19LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch19_loop;
#[doc = "CH19_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_ctrl`] module"]
#[doc(alias = "CH19_CTRL")]
pub type Ch19Ctrl = crate::Reg<ch19_ctrl::Ch19CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch19_ctrl;
#[doc = "CH19_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_src`] module"]
#[doc(alias = "CH19_SRC")]
pub type Ch19Src = crate::Reg<ch19_src::Ch19SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch19_src;
#[doc = "CH19_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_dst`] module"]
#[doc(alias = "CH19_DST")]
pub type Ch19Dst = crate::Reg<ch19_dst::Ch19DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch19_dst;
#[doc = "CH19_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_link`] module"]
#[doc(alias = "CH19_LINK")]
pub type Ch19Link = crate::Reg<ch19_link::Ch19LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch19_link;
#[doc = "CH20_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_reqsel`] module"]
#[doc(alias = "CH20_REQSEL")]
pub type Ch20Reqsel = crate::Reg<ch20_reqsel::Ch20ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch20_reqsel;
#[doc = "CH20_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_cfg`] module"]
#[doc(alias = "CH20_CFG")]
pub type Ch20Cfg = crate::Reg<ch20_cfg::Ch20CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch20_cfg;
#[doc = "CH20_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_loop`] module"]
#[doc(alias = "CH20_LOOP")]
pub type Ch20Loop = crate::Reg<ch20_loop::Ch20LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch20_loop;
#[doc = "CH20_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_ctrl`] module"]
#[doc(alias = "CH20_CTRL")]
pub type Ch20Ctrl = crate::Reg<ch20_ctrl::Ch20CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch20_ctrl;
#[doc = "CH20_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_src`] module"]
#[doc(alias = "CH20_SRC")]
pub type Ch20Src = crate::Reg<ch20_src::Ch20SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch20_src;
#[doc = "CH20_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_dst`] module"]
#[doc(alias = "CH20_DST")]
pub type Ch20Dst = crate::Reg<ch20_dst::Ch20DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch20_dst;
#[doc = "CH20_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_link`] module"]
#[doc(alias = "CH20_LINK")]
pub type Ch20Link = crate::Reg<ch20_link::Ch20LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch20_link;
#[doc = "CH21_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_reqsel`] module"]
#[doc(alias = "CH21_REQSEL")]
pub type Ch21Reqsel = crate::Reg<ch21_reqsel::Ch21ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch21_reqsel;
#[doc = "CH21_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_cfg`] module"]
#[doc(alias = "CH21_CFG")]
pub type Ch21Cfg = crate::Reg<ch21_cfg::Ch21CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch21_cfg;
#[doc = "CH21_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_loop`] module"]
#[doc(alias = "CH21_LOOP")]
pub type Ch21Loop = crate::Reg<ch21_loop::Ch21LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch21_loop;
#[doc = "CH21_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_ctrl`] module"]
#[doc(alias = "CH21_CTRL")]
pub type Ch21Ctrl = crate::Reg<ch21_ctrl::Ch21CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch21_ctrl;
#[doc = "CH21_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_src`] module"]
#[doc(alias = "CH21_SRC")]
pub type Ch21Src = crate::Reg<ch21_src::Ch21SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch21_src;
#[doc = "CH21_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_dst`] module"]
#[doc(alias = "CH21_DST")]
pub type Ch21Dst = crate::Reg<ch21_dst::Ch21DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch21_dst;
#[doc = "CH21_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_link`] module"]
#[doc(alias = "CH21_LINK")]
pub type Ch21Link = crate::Reg<ch21_link::Ch21LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch21_link;
#[doc = "CH22_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_reqsel`] module"]
#[doc(alias = "CH22_REQSEL")]
pub type Ch22Reqsel = crate::Reg<ch22_reqsel::Ch22ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch22_reqsel;
#[doc = "CH22_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_cfg`] module"]
#[doc(alias = "CH22_CFG")]
pub type Ch22Cfg = crate::Reg<ch22_cfg::Ch22CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch22_cfg;
#[doc = "CH22_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_loop`] module"]
#[doc(alias = "CH22_LOOP")]
pub type Ch22Loop = crate::Reg<ch22_loop::Ch22LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch22_loop;
#[doc = "CH22_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_ctrl`] module"]
#[doc(alias = "CH22_CTRL")]
pub type Ch22Ctrl = crate::Reg<ch22_ctrl::Ch22CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch22_ctrl;
#[doc = "CH22_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_src`] module"]
#[doc(alias = "CH22_SRC")]
pub type Ch22Src = crate::Reg<ch22_src::Ch22SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch22_src;
#[doc = "CH22_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_dst`] module"]
#[doc(alias = "CH22_DST")]
pub type Ch22Dst = crate::Reg<ch22_dst::Ch22DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch22_dst;
#[doc = "CH22_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_link`] module"]
#[doc(alias = "CH22_LINK")]
pub type Ch22Link = crate::Reg<ch22_link::Ch22LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch22_link;
#[doc = "CH23_REQSEL (rw) register accessor: Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_reqsel`] module"]
#[doc(alias = "CH23_REQSEL")]
pub type Ch23Reqsel = crate::Reg<ch23_reqsel::Ch23ReqselSpec>;
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch23_reqsel;
#[doc = "CH23_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_cfg`] module"]
#[doc(alias = "CH23_CFG")]
pub type Ch23Cfg = crate::Reg<ch23_cfg::Ch23CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch23_cfg;
#[doc = "CH23_LOOP (rw) register accessor: Channel Loop Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_loop`] module"]
#[doc(alias = "CH23_LOOP")]
pub type Ch23Loop = crate::Reg<ch23_loop::Ch23LoopSpec>;
#[doc = "Channel Loop Counter Register"]
pub mod ch23_loop;
#[doc = "CH23_CTRL (rw) register accessor: Channel Descriptor Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_ctrl`] module"]
#[doc(alias = "CH23_CTRL")]
pub type Ch23Ctrl = crate::Reg<ch23_ctrl::Ch23CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch23_ctrl;
#[doc = "CH23_SRC (rw) register accessor: Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_src`] module"]
#[doc(alias = "CH23_SRC")]
pub type Ch23Src = crate::Reg<ch23_src::Ch23SrcSpec>;
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch23_src;
#[doc = "CH23_DST (rw) register accessor: Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_dst`] module"]
#[doc(alias = "CH23_DST")]
pub type Ch23Dst = crate::Reg<ch23_dst::Ch23DstSpec>;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch23_dst;
#[doc = "CH23_LINK (rw) register accessor: Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_link`] module"]
#[doc(alias = "CH23_LINK")]
pub type Ch23Link = crate::Reg<ch23_link::Ch23LinkSpec>;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch23_link;
