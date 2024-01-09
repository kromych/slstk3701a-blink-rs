#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    cmd: CMD,
    status: STATUS,
    cnt: CNT,
    comp0: COMP0,
    comp1: COMP1,
    rep0: REP0,
    rep1: REP1,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    _reserved12: [u8; 0x04],
    syncbusy: SYNCBUSY,
    _reserved13: [u8; 0x08],
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    _reserved15: [u8; 0x08],
    prssel: PRSSEL,
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
    #[doc = "0x0c - Counter Value Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x10 - Compare Value Register 0"]
    #[inline(always)]
    pub const fn comp0(&self) -> &COMP0 {
        &self.comp0
    }
    #[doc = "0x14 - Compare Value Register 1"]
    #[inline(always)]
    pub const fn comp1(&self) -> &COMP1 {
        &self.comp1
    }
    #[doc = "0x18 - Repeat Counter Register 0"]
    #[inline(always)]
    pub const fn rep0(&self) -> &REP0 {
        &self.rep0
    }
    #[doc = "0x1c - Repeat Counter Register 1"]
    #[inline(always)]
    pub const fn rep1(&self) -> &REP1 {
        &self.rep1
    }
    #[doc = "0x20 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x24 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x28 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x2c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x34 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &SYNCBUSY {
        &self.syncbusy
    }
    #[doc = "0x40 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x44 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x50 - PRS Input Select Register"]
    #[inline(always)]
    pub const fn prssel(&self) -> &PRSSEL {
        &self.prssel
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
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "COMP0 (rw) register accessor: Compare Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0`]
module"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "Compare Value Register 0"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: Compare Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1`]
module"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "Compare Value Register 1"]
pub mod comp1;
#[doc = "REP0 (rw) register accessor: Repeat Counter Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rep0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rep0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rep0`]
module"]
pub type REP0 = crate::Reg<rep0::REP0_SPEC>;
#[doc = "Repeat Counter Register 0"]
pub mod rep0;
#[doc = "REP1 (rw) register accessor: Repeat Counter Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rep1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rep1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rep1`]
module"]
pub type REP1 = crate::Reg<rep1::REP1_SPEC>;
#[doc = "Repeat Counter Register 1"]
pub mod rep1;
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
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
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
#[doc = "PRSSEL (rw) register accessor: PRS Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prssel`]
module"]
pub type PRSSEL = crate::Reg<prssel::PRSSEL_SPEC>;
#[doc = "PRS Input Select Register"]
pub mod prssel;
