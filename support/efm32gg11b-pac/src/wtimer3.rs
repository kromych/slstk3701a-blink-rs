#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    cmd: CMD,
    status: STATUS,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    top: TOP,
    topb: TOPB,
    cnt: CNT,
    _reserved10: [u8; 0x04],
    lock: LOCK,
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    _reserved13: [u8; 0x04],
    routeloc2: ROUTELOC2,
    _reserved14: [u8; 0x20],
    cc0_ctrl: CC0_CTRL,
    cc0_ccv: CC0_CCV,
    cc0_ccvp: CC0_CCVP,
    cc0_ccvb: CC0_CCVB,
    cc1_ctrl: CC1_CTRL,
    cc1_ccv: CC1_CCV,
    cc1_ccvp: CC1_CCVP,
    cc1_ccvb: CC1_CCVB,
    cc2_ctrl: CC2_CTRL,
    cc2_ccv: CC2_CCV,
    cc2_ccvp: CC2_CCVP,
    cc2_ccvb: CC2_CCVB,
    cc3_ctrl: CC3_CTRL,
    cc3_ccv: CC3_CCV,
    cc3_ccvp: CC3_CCVP,
    cc3_ccvb: CC3_CCVB,
    dtctrl: DTCTRL,
    dttime: DTTIME,
    dtfc: DTFC,
    dtogen: DTOGEN,
    dtfault: DTFAULT,
    dtfaultc: DTFAULTC,
    dtlock: DTLOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x10 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x18 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x1c - Counter Top Value Register"]
    #[inline(always)]
    pub const fn top(&self) -> &TOP {
        &self.top
    }
    #[doc = "0x20 - Counter Top Value Buffer Register"]
    #[inline(always)]
    pub const fn topb(&self) -> &TOPB {
        &self.topb
    }
    #[doc = "0x24 - Counter Value Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x2c - TIMER Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x30 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x34 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x3c - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc2(&self) -> &ROUTELOC2 {
        &self.routeloc2
    }
    #[doc = "0x60 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc0_ctrl(&self) -> &CC0_CTRL {
        &self.cc0_ctrl
    }
    #[doc = "0x64 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc0_ccv(&self) -> &CC0_CCV {
        &self.cc0_ccv
    }
    #[doc = "0x68 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc0_ccvp(&self) -> &CC0_CCVP {
        &self.cc0_ccvp
    }
    #[doc = "0x6c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc0_ccvb(&self) -> &CC0_CCVB {
        &self.cc0_ccvb
    }
    #[doc = "0x70 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc1_ctrl(&self) -> &CC1_CTRL {
        &self.cc1_ctrl
    }
    #[doc = "0x74 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc1_ccv(&self) -> &CC1_CCV {
        &self.cc1_ccv
    }
    #[doc = "0x78 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc1_ccvp(&self) -> &CC1_CCVP {
        &self.cc1_ccvp
    }
    #[doc = "0x7c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc1_ccvb(&self) -> &CC1_CCVB {
        &self.cc1_ccvb
    }
    #[doc = "0x80 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc2_ctrl(&self) -> &CC2_CTRL {
        &self.cc2_ctrl
    }
    #[doc = "0x84 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc2_ccv(&self) -> &CC2_CCV {
        &self.cc2_ccv
    }
    #[doc = "0x88 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc2_ccvp(&self) -> &CC2_CCVP {
        &self.cc2_ccvp
    }
    #[doc = "0x8c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc2_ccvb(&self) -> &CC2_CCVB {
        &self.cc2_ccvb
    }
    #[doc = "0x90 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc3_ctrl(&self) -> &CC3_CTRL {
        &self.cc3_ctrl
    }
    #[doc = "0x94 - CC Channel Value Register"]
    #[inline(always)]
    pub const fn cc3_ccv(&self) -> &CC3_CCV {
        &self.cc3_ccv
    }
    #[doc = "0x98 - CC Channel Value Peek Register"]
    #[inline(always)]
    pub const fn cc3_ccvp(&self) -> &CC3_CCVP {
        &self.cc3_ccvp
    }
    #[doc = "0x9c - CC Channel Buffer Register"]
    #[inline(always)]
    pub const fn cc3_ccvb(&self) -> &CC3_CCVB {
        &self.cc3_ccvb
    }
    #[doc = "0xa0 - DTI Control Register"]
    #[inline(always)]
    pub const fn dtctrl(&self) -> &DTCTRL {
        &self.dtctrl
    }
    #[doc = "0xa4 - DTI Time Control Register"]
    #[inline(always)]
    pub const fn dttime(&self) -> &DTTIME {
        &self.dttime
    }
    #[doc = "0xa8 - DTI Fault Configuration Register"]
    #[inline(always)]
    pub const fn dtfc(&self) -> &DTFC {
        &self.dtfc
    }
    #[doc = "0xac - DTI Output Generation Enable Register"]
    #[inline(always)]
    pub const fn dtogen(&self) -> &DTOGEN {
        &self.dtogen
    }
    #[doc = "0xb0 - DTI Fault Register"]
    #[inline(always)]
    pub const fn dtfault(&self) -> &DTFAULT {
        &self.dtfault
    }
    #[doc = "0xb4 - DTI Fault Clear Register"]
    #[inline(always)]
    pub const fn dtfaultc(&self) -> &DTFAULTC {
        &self.dtfaultc
    }
    #[doc = "0xb8 - DTI Configuration Lock Register"]
    #[inline(always)]
    pub const fn dtlock(&self) -> &DTLOCK {
        &self.dtlock
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
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
#[doc = "TOP (rw) register accessor: Counter Top Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top`]
module"]
pub type TOP = crate::Reg<top::TOP_SPEC>;
#[doc = "Counter Top Value Register"]
pub mod top;
#[doc = "TOPB (rw) register accessor: Counter Top Value Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`topb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`topb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topb`]
module"]
pub type TOPB = crate::Reg<topb::TOPB_SPEC>;
#[doc = "Counter Top Value Buffer Register"]
pub mod topb;
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "LOCK (rw) register accessor: TIMER Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "TIMER Configuration Lock Register"]
pub mod lock;
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
#[doc = "ROUTELOC2 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc2`]
module"]
pub type ROUTELOC2 = crate::Reg<routeloc2::ROUTELOC2_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc2;
#[doc = "CC0_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ctrl`]
module"]
pub type CC0_CTRL = crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC0_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccv::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccv`]
module"]
pub type CC0_CCV = crate::Reg<cc0_ccv::CC0_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc0_ccv;
#[doc = "CC0_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccvp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccvp`]
module"]
pub type CC0_CCVP = crate::Reg<cc0_ccvp::CC0_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc0_ccvp;
#[doc = "CC0_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccvb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ccvb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccvb`]
module"]
pub type CC0_CCVB = crate::Reg<cc0_ccvb::CC0_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc0_ccvb;
#[doc = "CC1_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ctrl`]
module"]
pub type CC1_CTRL = crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC1_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccv::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccv`]
module"]
pub type CC1_CCV = crate::Reg<cc1_ccv::CC1_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc1_ccv;
#[doc = "CC1_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccvp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccvp`]
module"]
pub type CC1_CCVP = crate::Reg<cc1_ccvp::CC1_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc1_ccvp;
#[doc = "CC1_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccvb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ccvb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccvb`]
module"]
pub type CC1_CCVB = crate::Reg<cc1_ccvb::CC1_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc1_ccvb;
#[doc = "CC2_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ctrl`]
module"]
pub type CC2_CTRL = crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC2_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ccv::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccv`]
module"]
pub type CC2_CCV = crate::Reg<cc2_ccv::CC2_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc2_ccv;
#[doc = "CC2_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ccvp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccvp`]
module"]
pub type CC2_CCVP = crate::Reg<cc2_ccvp::CC2_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc2_ccvp;
#[doc = "CC2_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ccvb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ccvb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccvb`]
module"]
pub type CC2_CCVB = crate::Reg<cc2_ccvb::CC2_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc2_ccvb;
#[doc = "CC3_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc3_ctrl`]
module"]
pub type CC3_CTRL = crate::Reg<cc3_ctrl::CC3_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc3_ctrl;
#[doc = "CC3_CCV (rw) register accessor: CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc3_ccv::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc3_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc3_ccv`]
module"]
pub type CC3_CCV = crate::Reg<cc3_ccv::CC3_CCV_SPEC>;
#[doc = "CC Channel Value Register"]
pub mod cc3_ccv;
#[doc = "CC3_CCVP (r) register accessor: CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc3_ccvp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc3_ccvp`]
module"]
pub type CC3_CCVP = crate::Reg<cc3_ccvp::CC3_CCVP_SPEC>;
#[doc = "CC Channel Value Peek Register"]
pub mod cc3_ccvp;
#[doc = "CC3_CCVB (rw) register accessor: CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc3_ccvb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc3_ccvb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc3_ccvb`]
module"]
pub type CC3_CCVB = crate::Reg<cc3_ccvb::CC3_CCVB_SPEC>;
#[doc = "CC Channel Buffer Register"]
pub mod cc3_ccvb;
#[doc = "DTCTRL (rw) register accessor: DTI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctrl`]
module"]
pub type DTCTRL = crate::Reg<dtctrl::DTCTRL_SPEC>;
#[doc = "DTI Control Register"]
pub mod dtctrl;
#[doc = "DTTIME (rw) register accessor: DTI Time Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dttime`]
module"]
pub type DTTIME = crate::Reg<dttime::DTTIME_SPEC>;
#[doc = "DTI Time Control Register"]
pub mod dttime;
#[doc = "DTFC (rw) register accessor: DTI Fault Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfc`]
module"]
pub type DTFC = crate::Reg<dtfc::DTFC_SPEC>;
#[doc = "DTI Fault Configuration Register"]
pub mod dtfc;
#[doc = "DTOGEN (rw) register accessor: DTI Output Generation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtogen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtogen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtogen`]
module"]
pub type DTOGEN = crate::Reg<dtogen::DTOGEN_SPEC>;
#[doc = "DTI Output Generation Enable Register"]
pub mod dtogen;
#[doc = "DTFAULT (r) register accessor: DTI Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfault::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfault`]
module"]
pub type DTFAULT = crate::Reg<dtfault::DTFAULT_SPEC>;
#[doc = "DTI Fault Register"]
pub mod dtfault;
#[doc = "DTFAULTC (w) register accessor: DTI Fault Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfaultc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfaultc`]
module"]
pub type DTFAULTC = crate::Reg<dtfaultc::DTFAULTC_SPEC>;
#[doc = "DTI Fault Clear Register"]
pub mod dtfaultc;
#[doc = "DTLOCK (rw) register accessor: DTI Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtlock`]
module"]
pub type DTLOCK = crate::Reg<dtlock::DTLOCK_SPEC>;
#[doc = "DTI Configuration Lock Register"]
pub mod dtlock;
