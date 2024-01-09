#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    inputsel: INPUTSEL,
    status: STATUS,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    _reserved7: [u8; 0x04],
    aportreq: APORTREQ,
    aportconflict: APORTCONFLICT,
    hysteresis0: HYSTERESIS0,
    hysteresis1: HYSTERESIS1,
    _reserved11: [u8; 0x10],
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    extifctrl: EXTIFCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Input Selection Register"]
    #[inline(always)]
    pub const fn inputsel(&self) -> &INPUTSEL {
        &self.inputsel
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
    #[doc = "0x20 - APORT Request Status Register"]
    #[inline(always)]
    pub const fn aportreq(&self) -> &APORTREQ {
        &self.aportreq
    }
    #[doc = "0x24 - APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn aportconflict(&self) -> &APORTCONFLICT {
        &self.aportconflict
    }
    #[doc = "0x28 - Hysteresis 0 Register"]
    #[inline(always)]
    pub const fn hysteresis0(&self) -> &HYSTERESIS0 {
        &self.hysteresis0
    }
    #[doc = "0x2c - Hysteresis 1 Register"]
    #[inline(always)]
    pub const fn hysteresis1(&self) -> &HYSTERESIS1 {
        &self.hysteresis1
    }
    #[doc = "0x40 - I/O Routing Pine Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x44 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x48 - External Override Interface Control"]
    #[inline(always)]
    pub const fn extifctrl(&self) -> &EXTIFCTRL {
        &self.extifctrl
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "INPUTSEL (rw) register accessor: Input Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputsel`]
module"]
pub type INPUTSEL = crate::Reg<inputsel::INPUTSEL_SPEC>;
#[doc = "Input Selection Register"]
pub mod inputsel;
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
#[doc = "APORTREQ (r) register accessor: APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportreq`]
module"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT (r) register accessor: APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportconflict`]
module"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "HYSTERESIS0 (rw) register accessor: Hysteresis 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hysteresis0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hysteresis0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hysteresis0`]
module"]
pub type HYSTERESIS0 = crate::Reg<hysteresis0::HYSTERESIS0_SPEC>;
#[doc = "Hysteresis 0 Register"]
pub mod hysteresis0;
#[doc = "HYSTERESIS1 (rw) register accessor: Hysteresis 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hysteresis1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hysteresis1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hysteresis1`]
module"]
pub type HYSTERESIS1 = crate::Reg<hysteresis1::HYSTERESIS1_SPEC>;
#[doc = "Hysteresis 1 Register"]
pub mod hysteresis1;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pine Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pine Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "EXTIFCTRL (rw) register accessor: External Override Interface Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extifctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extifctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifctrl`]
module"]
pub type EXTIFCTRL = crate::Reg<extifctrl::EXTIFCTRL_SPEC>;
#[doc = "External Override Interface Control"]
pub mod extifctrl;
