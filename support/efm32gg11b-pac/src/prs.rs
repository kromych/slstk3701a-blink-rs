#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    swpulse: Swpulse,
    swlevel: Swlevel,
    routepen: Routepen,
    _reserved3: [u8; 0x04],
    routeloc0: Routeloc0,
    routeloc1: Routeloc1,
    routeloc2: Routeloc2,
    routeloc3: Routeloc3,
    routeloc4: Routeloc4,
    routeloc5: Routeloc5,
    _reserved9: [u8; 0x08],
    ctrl: Ctrl,
    dmareq0: Dmareq0,
    dmareq1: Dmareq1,
    _reserved12: [u8; 0x04],
    peek: Peek,
    _reserved13: [u8; 0x0c],
    ch0_ctrl: Ch0Ctrl,
    ch1_ctrl: Ch1Ctrl,
    ch2_ctrl: Ch2Ctrl,
    ch3_ctrl: Ch3Ctrl,
    ch4_ctrl: Ch4Ctrl,
    ch5_ctrl: Ch5Ctrl,
    ch6_ctrl: Ch6Ctrl,
    ch7_ctrl: Ch7Ctrl,
    ch8_ctrl: Ch8Ctrl,
    ch9_ctrl: Ch9Ctrl,
    ch10_ctrl: Ch10Ctrl,
    ch11_ctrl: Ch11Ctrl,
    ch12_ctrl: Ch12Ctrl,
    ch13_ctrl: Ch13Ctrl,
    ch14_ctrl: Ch14Ctrl,
    ch15_ctrl: Ch15Ctrl,
    ch16_ctrl: Ch16Ctrl,
    ch17_ctrl: Ch17Ctrl,
    ch18_ctrl: Ch18Ctrl,
    ch19_ctrl: Ch19Ctrl,
    ch20_ctrl: Ch20Ctrl,
    ch21_ctrl: Ch21Ctrl,
    ch22_ctrl: Ch22Ctrl,
    ch23_ctrl: Ch23Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    #[inline(always)]
    pub const fn swpulse(&self) -> &Swpulse {
        &self.swpulse
    }
    #[doc = "0x04 - Software Level Register"]
    #[inline(always)]
    pub const fn swlevel(&self) -> &Swlevel {
        &self.swlevel
    }
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x10 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x14 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
    }
    #[doc = "0x18 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc2(&self) -> &Routeloc2 {
        &self.routeloc2
    }
    #[doc = "0x1c - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc3(&self) -> &Routeloc3 {
        &self.routeloc3
    }
    #[doc = "0x20 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc4(&self) -> &Routeloc4 {
        &self.routeloc4
    }
    #[doc = "0x24 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc5(&self) -> &Routeloc5 {
        &self.routeloc5
    }
    #[doc = "0x30 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x34 - DMA Request 0 Register"]
    #[inline(always)]
    pub const fn dmareq0(&self) -> &Dmareq0 {
        &self.dmareq0
    }
    #[doc = "0x38 - DMA Request 1 Register"]
    #[inline(always)]
    pub const fn dmareq1(&self) -> &Dmareq1 {
        &self.dmareq1
    }
    #[doc = "0x40 - PRS Channel Values"]
    #[inline(always)]
    pub const fn peek(&self) -> &Peek {
        &self.peek
    }
    #[doc = "0x50 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    #[doc = "0x54 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    #[doc = "0x58 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    #[doc = "0x5c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    #[doc = "0x60 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    #[doc = "0x64 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
    #[doc = "0x68 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &Ch6Ctrl {
        &self.ch6_ctrl
    }
    #[doc = "0x6c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &Ch7Ctrl {
        &self.ch7_ctrl
    }
    #[doc = "0x70 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch8_ctrl(&self) -> &Ch8Ctrl {
        &self.ch8_ctrl
    }
    #[doc = "0x74 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch9_ctrl(&self) -> &Ch9Ctrl {
        &self.ch9_ctrl
    }
    #[doc = "0x78 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch10_ctrl(&self) -> &Ch10Ctrl {
        &self.ch10_ctrl
    }
    #[doc = "0x7c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch11_ctrl(&self) -> &Ch11Ctrl {
        &self.ch11_ctrl
    }
    #[doc = "0x80 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch12_ctrl(&self) -> &Ch12Ctrl {
        &self.ch12_ctrl
    }
    #[doc = "0x84 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch13_ctrl(&self) -> &Ch13Ctrl {
        &self.ch13_ctrl
    }
    #[doc = "0x88 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch14_ctrl(&self) -> &Ch14Ctrl {
        &self.ch14_ctrl
    }
    #[doc = "0x8c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch15_ctrl(&self) -> &Ch15Ctrl {
        &self.ch15_ctrl
    }
    #[doc = "0x90 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch16_ctrl(&self) -> &Ch16Ctrl {
        &self.ch16_ctrl
    }
    #[doc = "0x94 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch17_ctrl(&self) -> &Ch17Ctrl {
        &self.ch17_ctrl
    }
    #[doc = "0x98 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch18_ctrl(&self) -> &Ch18Ctrl {
        &self.ch18_ctrl
    }
    #[doc = "0x9c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch19_ctrl(&self) -> &Ch19Ctrl {
        &self.ch19_ctrl
    }
    #[doc = "0xa0 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch20_ctrl(&self) -> &Ch20Ctrl {
        &self.ch20_ctrl
    }
    #[doc = "0xa4 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch21_ctrl(&self) -> &Ch21Ctrl {
        &self.ch21_ctrl
    }
    #[doc = "0xa8 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch22_ctrl(&self) -> &Ch22Ctrl {
        &self.ch22_ctrl
    }
    #[doc = "0xac - Channel Control Register"]
    #[inline(always)]
    pub const fn ch23_ctrl(&self) -> &Ch23Ctrl {
        &self.ch23_ctrl
    }
}
#[doc = "SWPULSE (w) register accessor: Software Pulse Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpulse`] module"]
#[doc(alias = "SWPULSE")]
pub type Swpulse = crate::Reg<swpulse::SwpulseSpec>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL (rw) register accessor: Software Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swlevel`] module"]
#[doc(alias = "SWLEVEL")]
pub type Swlevel = crate::Reg<swlevel::SwlevelSpec>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`] module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::Routeloc0Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`] module"]
#[doc(alias = "ROUTELOC1")]
pub type Routeloc1 = crate::Reg<routeloc1::Routeloc1Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "ROUTELOC2 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc2`] module"]
#[doc(alias = "ROUTELOC2")]
pub type Routeloc2 = crate::Reg<routeloc2::Routeloc2Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "ROUTELOC3 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc3`] module"]
#[doc(alias = "ROUTELOC3")]
pub type Routeloc3 = crate::Reg<routeloc3::Routeloc3Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc3;
#[doc = "ROUTELOC4 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc4`] module"]
#[doc(alias = "ROUTELOC4")]
pub type Routeloc4 = crate::Reg<routeloc4::Routeloc4Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc4;
#[doc = "ROUTELOC5 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc5`] module"]
#[doc(alias = "ROUTELOC5")]
pub type Routeloc5 = crate::Reg<routeloc5::Routeloc5Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc5;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMAREQ0 (rw) register accessor: DMA Request 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq0`] module"]
#[doc(alias = "DMAREQ0")]
pub type Dmareq0 = crate::Reg<dmareq0::Dmareq0Spec>;
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMAREQ1 (rw) register accessor: DMA Request 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq1`] module"]
#[doc(alias = "DMAREQ1")]
pub type Dmareq1 = crate::Reg<dmareq1::Dmareq1Spec>;
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PEEK (r) register accessor: PRS Channel Values\n\nYou can [`read`](crate::Reg::read) this register and get [`peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek`] module"]
#[doc(alias = "PEEK")]
pub type Peek = crate::Reg<peek::PeekSpec>;
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "CH0_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`] module"]
#[doc(alias = "CH0_CTRL")]
pub type Ch0Ctrl = crate::Reg<ch0_ctrl::Ch0CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`] module"]
#[doc(alias = "CH1_CTRL")]
pub type Ch1Ctrl = crate::Reg<ch1_ctrl::Ch1CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`] module"]
#[doc(alias = "CH2_CTRL")]
pub type Ch2Ctrl = crate::Reg<ch2_ctrl::Ch2CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`] module"]
#[doc(alias = "CH3_CTRL")]
pub type Ch3Ctrl = crate::Reg<ch3_ctrl::Ch3CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`] module"]
#[doc(alias = "CH4_CTRL")]
pub type Ch4Ctrl = crate::Reg<ch4_ctrl::Ch4CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`] module"]
#[doc(alias = "CH5_CTRL")]
pub type Ch5Ctrl = crate::Reg<ch5_ctrl::Ch5CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`] module"]
#[doc(alias = "CH6_CTRL")]
pub type Ch6Ctrl = crate::Reg<ch6_ctrl::Ch6CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`] module"]
#[doc(alias = "CH7_CTRL")]
pub type Ch7Ctrl = crate::Reg<ch7_ctrl::Ch7CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "CH8_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ctrl`] module"]
#[doc(alias = "CH8_CTRL")]
pub type Ch8Ctrl = crate::Reg<ch8_ctrl::Ch8CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "CH9_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ctrl`] module"]
#[doc(alias = "CH9_CTRL")]
pub type Ch9Ctrl = crate::Reg<ch9_ctrl::Ch9CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "CH10_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ctrl`] module"]
#[doc(alias = "CH10_CTRL")]
pub type Ch10Ctrl = crate::Reg<ch10_ctrl::Ch10CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "CH11_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ctrl`] module"]
#[doc(alias = "CH11_CTRL")]
pub type Ch11Ctrl = crate::Reg<ch11_ctrl::Ch11CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
#[doc = "CH12_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_ctrl`] module"]
#[doc(alias = "CH12_CTRL")]
pub type Ch12Ctrl = crate::Reg<ch12_ctrl::Ch12CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch12_ctrl;
#[doc = "CH13_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_ctrl`] module"]
#[doc(alias = "CH13_CTRL")]
pub type Ch13Ctrl = crate::Reg<ch13_ctrl::Ch13CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch13_ctrl;
#[doc = "CH14_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_ctrl`] module"]
#[doc(alias = "CH14_CTRL")]
pub type Ch14Ctrl = crate::Reg<ch14_ctrl::Ch14CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch14_ctrl;
#[doc = "CH15_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_ctrl`] module"]
#[doc(alias = "CH15_CTRL")]
pub type Ch15Ctrl = crate::Reg<ch15_ctrl::Ch15CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch15_ctrl;
#[doc = "CH16_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch16_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch16_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_ctrl`] module"]
#[doc(alias = "CH16_CTRL")]
pub type Ch16Ctrl = crate::Reg<ch16_ctrl::Ch16CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch16_ctrl;
#[doc = "CH17_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_ctrl`] module"]
#[doc(alias = "CH17_CTRL")]
pub type Ch17Ctrl = crate::Reg<ch17_ctrl::Ch17CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch17_ctrl;
#[doc = "CH18_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch18_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch18_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_ctrl`] module"]
#[doc(alias = "CH18_CTRL")]
pub type Ch18Ctrl = crate::Reg<ch18_ctrl::Ch18CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch18_ctrl;
#[doc = "CH19_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_ctrl`] module"]
#[doc(alias = "CH19_CTRL")]
pub type Ch19Ctrl = crate::Reg<ch19_ctrl::Ch19CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch19_ctrl;
#[doc = "CH20_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch20_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch20_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_ctrl`] module"]
#[doc(alias = "CH20_CTRL")]
pub type Ch20Ctrl = crate::Reg<ch20_ctrl::Ch20CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch20_ctrl;
#[doc = "CH21_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch21_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch21_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_ctrl`] module"]
#[doc(alias = "CH21_CTRL")]
pub type Ch21Ctrl = crate::Reg<ch21_ctrl::Ch21CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch21_ctrl;
#[doc = "CH22_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_ctrl`] module"]
#[doc(alias = "CH22_CTRL")]
pub type Ch22Ctrl = crate::Reg<ch22_ctrl::Ch22CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch22_ctrl;
#[doc = "CH23_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch23_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch23_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_ctrl`] module"]
#[doc(alias = "CH23_CTRL")]
pub type Ch23Ctrl = crate::Reg<ch23_ctrl::Ch23CtrlSpec>;
#[doc = "Channel Control Register"]
pub mod ch23_ctrl;
