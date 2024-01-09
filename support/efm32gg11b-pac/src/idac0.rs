#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    curprog: CURPROG,
    _reserved2: [u8; 0x04],
    dutyconfig: DUTYCONFIG,
    _reserved3: [u8; 0x08],
    status: STATUS,
    _reserved4: [u8; 0x04],
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    _reserved8: [u8; 0x04],
    aportreq: APORTREQ,
    aportconflict: APORTCONFLICT,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Current Programming Register"]
    #[inline(always)]
    pub const fn curprog(&self) -> &CURPROG {
        &self.curprog
    }
    #[doc = "0x0c - Duty Cycle Configuration Register"]
    #[inline(always)]
    pub const fn dutyconfig(&self) -> &DUTYCONFIG {
        &self.dutyconfig
    }
    #[doc = "0x18 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
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
    #[doc = "0x34 - APORT Request Status Register"]
    #[inline(always)]
    pub const fn aportreq(&self) -> &APORTREQ {
        &self.aportreq
    }
    #[doc = "0x38 - APORT Request Status Register"]
    #[inline(always)]
    pub const fn aportconflict(&self) -> &APORTCONFLICT {
        &self.aportconflict
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CURPROG (rw) register accessor: Current Programming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curprog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curprog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curprog`]
module"]
pub type CURPROG = crate::Reg<curprog::CURPROG_SPEC>;
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "DUTYCONFIG (rw) register accessor: Duty Cycle Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dutyconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dutyconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dutyconfig`]
module"]
pub type DUTYCONFIG = crate::Reg<dutyconfig::DUTYCONFIG_SPEC>;
#[doc = "Duty Cycle Configuration Register"]
pub mod dutyconfig;
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
#[doc = "APORTCONFLICT (r) register accessor: APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportconflict`]
module"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportconflict;
