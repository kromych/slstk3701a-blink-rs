#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    swpulse: SWPULSE,
    swlevel: SWLEVEL,
    routepen: ROUTEPEN,
    _reserved3: [u8; 0x04],
    routeloc0: ROUTELOC0,
    routeloc1: ROUTELOC1,
    routeloc2: ROUTELOC2,
    routeloc3: ROUTELOC3,
    routeloc4: ROUTELOC4,
    routeloc5: ROUTELOC5,
    _reserved9: [u8; 0x08],
    ctrl: CTRL,
    dmareq0: DMAREQ0,
    dmareq1: DMAREQ1,
    _reserved12: [u8; 0x04],
    peek: PEEK,
    _reserved13: [u8; 0x0c],
    ch0_ctrl: CH0_CTRL,
    ch1_ctrl: CH1_CTRL,
    ch2_ctrl: CH2_CTRL,
    ch3_ctrl: CH3_CTRL,
    ch4_ctrl: CH4_CTRL,
    ch5_ctrl: CH5_CTRL,
    ch6_ctrl: CH6_CTRL,
    ch7_ctrl: CH7_CTRL,
    ch8_ctrl: CH8_CTRL,
    ch9_ctrl: CH9_CTRL,
    ch10_ctrl: CH10_CTRL,
    ch11_ctrl: CH11_CTRL,
    ch12_ctrl: CH12_CTRL,
    ch13_ctrl: CH13_CTRL,
    ch14_ctrl: CH14_CTRL,
    ch15_ctrl: CH15_CTRL,
    ch16_ctrl: CH16_CTRL,
    ch17_ctrl: CH17_CTRL,
    ch18_ctrl: CH18_CTRL,
    ch19_ctrl: CH19_CTRL,
    ch20_ctrl: CH20_CTRL,
    ch21_ctrl: CH21_CTRL,
    ch22_ctrl: CH22_CTRL,
    ch23_ctrl: CH23_CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    #[inline(always)]
    pub const fn swpulse(&self) -> &SWPULSE {
        &self.swpulse
    }
    #[doc = "0x04 - Software Level Register"]
    #[inline(always)]
    pub const fn swlevel(&self) -> &SWLEVEL {
        &self.swlevel
    }
    #[doc = "0x08 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x10 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x14 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &ROUTELOC1 {
        &self.routeloc1
    }
    #[doc = "0x18 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc2(&self) -> &ROUTELOC2 {
        &self.routeloc2
    }
    #[doc = "0x1c - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc3(&self) -> &ROUTELOC3 {
        &self.routeloc3
    }
    #[doc = "0x20 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc4(&self) -> &ROUTELOC4 {
        &self.routeloc4
    }
    #[doc = "0x24 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc5(&self) -> &ROUTELOC5 {
        &self.routeloc5
    }
    #[doc = "0x30 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x34 - DMA Request 0 Register"]
    #[inline(always)]
    pub const fn dmareq0(&self) -> &DMAREQ0 {
        &self.dmareq0
    }
    #[doc = "0x38 - DMA Request 1 Register"]
    #[inline(always)]
    pub const fn dmareq1(&self) -> &DMAREQ1 {
        &self.dmareq1
    }
    #[doc = "0x40 - PRS Channel Values"]
    #[inline(always)]
    pub const fn peek(&self) -> &PEEK {
        &self.peek
    }
    #[doc = "0x50 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &CH0_CTRL {
        &self.ch0_ctrl
    }
    #[doc = "0x54 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &CH1_CTRL {
        &self.ch1_ctrl
    }
    #[doc = "0x58 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &CH2_CTRL {
        &self.ch2_ctrl
    }
    #[doc = "0x5c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &CH3_CTRL {
        &self.ch3_ctrl
    }
    #[doc = "0x60 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &CH4_CTRL {
        &self.ch4_ctrl
    }
    #[doc = "0x64 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &CH5_CTRL {
        &self.ch5_ctrl
    }
    #[doc = "0x68 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &CH6_CTRL {
        &self.ch6_ctrl
    }
    #[doc = "0x6c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &CH7_CTRL {
        &self.ch7_ctrl
    }
    #[doc = "0x70 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch8_ctrl(&self) -> &CH8_CTRL {
        &self.ch8_ctrl
    }
    #[doc = "0x74 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch9_ctrl(&self) -> &CH9_CTRL {
        &self.ch9_ctrl
    }
    #[doc = "0x78 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch10_ctrl(&self) -> &CH10_CTRL {
        &self.ch10_ctrl
    }
    #[doc = "0x7c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch11_ctrl(&self) -> &CH11_CTRL {
        &self.ch11_ctrl
    }
    #[doc = "0x80 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch12_ctrl(&self) -> &CH12_CTRL {
        &self.ch12_ctrl
    }
    #[doc = "0x84 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch13_ctrl(&self) -> &CH13_CTRL {
        &self.ch13_ctrl
    }
    #[doc = "0x88 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch14_ctrl(&self) -> &CH14_CTRL {
        &self.ch14_ctrl
    }
    #[doc = "0x8c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch15_ctrl(&self) -> &CH15_CTRL {
        &self.ch15_ctrl
    }
    #[doc = "0x90 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch16_ctrl(&self) -> &CH16_CTRL {
        &self.ch16_ctrl
    }
    #[doc = "0x94 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch17_ctrl(&self) -> &CH17_CTRL {
        &self.ch17_ctrl
    }
    #[doc = "0x98 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch18_ctrl(&self) -> &CH18_CTRL {
        &self.ch18_ctrl
    }
    #[doc = "0x9c - Channel Control Register"]
    #[inline(always)]
    pub const fn ch19_ctrl(&self) -> &CH19_CTRL {
        &self.ch19_ctrl
    }
    #[doc = "0xa0 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch20_ctrl(&self) -> &CH20_CTRL {
        &self.ch20_ctrl
    }
    #[doc = "0xa4 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch21_ctrl(&self) -> &CH21_CTRL {
        &self.ch21_ctrl
    }
    #[doc = "0xa8 - Channel Control Register"]
    #[inline(always)]
    pub const fn ch22_ctrl(&self) -> &CH22_CTRL {
        &self.ch22_ctrl
    }
    #[doc = "0xac - Channel Control Register"]
    #[inline(always)]
    pub const fn ch23_ctrl(&self) -> &CH23_CTRL {
        &self.ch23_ctrl
    }
}
#[doc = "SWPULSE (w) register accessor: Software Pulse Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpulse`]
module"]
pub type SWPULSE = crate::Reg<swpulse::SWPULSE_SPEC>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL (rw) register accessor: Software Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swlevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swlevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swlevel`]
module"]
pub type SWLEVEL = crate::Reg<swlevel::SWLEVEL_SPEC>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`]
module"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "ROUTELOC2 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc2`]
module"]
pub type ROUTELOC2 = crate::Reg<routeloc2::ROUTELOC2_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "ROUTELOC3 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc3`]
module"]
pub type ROUTELOC3 = crate::Reg<routeloc3::ROUTELOC3_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc3;
#[doc = "ROUTELOC4 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc4`]
module"]
pub type ROUTELOC4 = crate::Reg<routeloc4::ROUTELOC4_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc4;
#[doc = "ROUTELOC5 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc5`]
module"]
pub type ROUTELOC5 = crate::Reg<routeloc5::ROUTELOC5_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc5;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DMAREQ0 (rw) register accessor: DMA Request 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq0`]
module"]
pub type DMAREQ0 = crate::Reg<dmareq0::DMAREQ0_SPEC>;
#[doc = "DMA Request 0 Register"]
pub mod dmareq0;
#[doc = "DMAREQ1 (rw) register accessor: DMA Request 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmareq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmareq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq1`]
module"]
pub type DMAREQ1 = crate::Reg<dmareq1::DMAREQ1_SPEC>;
#[doc = "DMA Request 1 Register"]
pub mod dmareq1;
#[doc = "PEEK (r) register accessor: PRS Channel Values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek`]
module"]
pub type PEEK = crate::Reg<peek::PEEK_SPEC>;
#[doc = "PRS Channel Values"]
pub mod peek;
#[doc = "CH0_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`]
module"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`]
module"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`]
module"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`]
module"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`]
module"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`]
module"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "CH6_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`]
module"]
pub type CH6_CTRL = crate::Reg<ch6_ctrl::CH6_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch6_ctrl;
#[doc = "CH7_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`]
module"]
pub type CH7_CTRL = crate::Reg<ch7_ctrl::CH7_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch7_ctrl;
#[doc = "CH8_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ctrl`]
module"]
pub type CH8_CTRL = crate::Reg<ch8_ctrl::CH8_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch8_ctrl;
#[doc = "CH9_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch9_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ctrl`]
module"]
pub type CH9_CTRL = crate::Reg<ch9_ctrl::CH9_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch9_ctrl;
#[doc = "CH10_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch10_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ctrl`]
module"]
pub type CH10_CTRL = crate::Reg<ch10_ctrl::CH10_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch10_ctrl;
#[doc = "CH11_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch11_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ctrl`]
module"]
pub type CH11_CTRL = crate::Reg<ch11_ctrl::CH11_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch11_ctrl;
#[doc = "CH12_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch12_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch12_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_ctrl`]
module"]
pub type CH12_CTRL = crate::Reg<ch12_ctrl::CH12_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch12_ctrl;
#[doc = "CH13_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch13_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch13_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_ctrl`]
module"]
pub type CH13_CTRL = crate::Reg<ch13_ctrl::CH13_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch13_ctrl;
#[doc = "CH14_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch14_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch14_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_ctrl`]
module"]
pub type CH14_CTRL = crate::Reg<ch14_ctrl::CH14_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch14_ctrl;
#[doc = "CH15_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch15_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch15_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_ctrl`]
module"]
pub type CH15_CTRL = crate::Reg<ch15_ctrl::CH15_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch15_ctrl;
#[doc = "CH16_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch16_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch16_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch16_ctrl`]
module"]
pub type CH16_CTRL = crate::Reg<ch16_ctrl::CH16_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch16_ctrl;
#[doc = "CH17_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch17_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch17_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch17_ctrl`]
module"]
pub type CH17_CTRL = crate::Reg<ch17_ctrl::CH17_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch17_ctrl;
#[doc = "CH18_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch18_ctrl`]
module"]
pub type CH18_CTRL = crate::Reg<ch18_ctrl::CH18_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch18_ctrl;
#[doc = "CH19_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch19_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch19_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch19_ctrl`]
module"]
pub type CH19_CTRL = crate::Reg<ch19_ctrl::CH19_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch19_ctrl;
#[doc = "CH20_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch20_ctrl`]
module"]
pub type CH20_CTRL = crate::Reg<ch20_ctrl::CH20_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch20_ctrl;
#[doc = "CH21_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch21_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch21_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch21_ctrl`]
module"]
pub type CH21_CTRL = crate::Reg<ch21_ctrl::CH21_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch21_ctrl;
#[doc = "CH22_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch22_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch22_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch22_ctrl`]
module"]
pub type CH22_CTRL = crate::Reg<ch22_ctrl::CH22_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch22_ctrl;
#[doc = "CH23_CTRL (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch23_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch23_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch23_ctrl`]
module"]
pub type CH23_CTRL = crate::Reg<ch23_ctrl::CH23_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch23_ctrl;
