#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cnt: Cnt,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved6: [u8; 0x08],
    compa_comp: CompaComp,
    compb_comp: CompbComp,
    compc_comp: CompcComp,
    compd_comp: CompdComp,
    compe_comp: CompeComp,
    compf_comp: CompfComp,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Counter Value Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x08 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x0c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x20 - Compare Value Register X"]
    #[inline(always)]
    pub const fn compa_comp(&self) -> &CompaComp {
        &self.compa_comp
    }
    #[doc = "0x24 - Compare Value Register X"]
    #[inline(always)]
    pub const fn compb_comp(&self) -> &CompbComp {
        &self.compb_comp
    }
    #[doc = "0x28 - Compare Value Register X"]
    #[inline(always)]
    pub const fn compc_comp(&self) -> &CompcComp {
        &self.compc_comp
    }
    #[doc = "0x2c - Compare Value Register X"]
    #[inline(always)]
    pub const fn compd_comp(&self) -> &CompdComp {
        &self.compd_comp
    }
    #[doc = "0x30 - Compare Value Register X"]
    #[inline(always)]
    pub const fn compe_comp(&self) -> &CompeComp {
        &self.compe_comp
    }
    #[doc = "0x34 - Compare Value Register X"]
    #[inline(always)]
    pub const fn compf_comp(&self) -> &CompfComp {
        &self.compf_comp
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Value Register"]
pub mod cnt;
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
#[doc = "COMPA_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compa_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compa_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compa_comp`] module"]
#[doc(alias = "COMPA_COMP")]
pub type CompaComp = crate::Reg<compa_comp::CompaCompSpec>;
#[doc = "Compare Value Register X"]
pub mod compa_comp;
#[doc = "COMPB_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compb_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compb_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compb_comp`] module"]
#[doc(alias = "COMPB_COMP")]
pub type CompbComp = crate::Reg<compb_comp::CompbCompSpec>;
#[doc = "Compare Value Register X"]
pub mod compb_comp;
#[doc = "COMPC_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compc_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compc_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compc_comp`] module"]
#[doc(alias = "COMPC_COMP")]
pub type CompcComp = crate::Reg<compc_comp::CompcCompSpec>;
#[doc = "Compare Value Register X"]
pub mod compc_comp;
#[doc = "COMPD_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compd_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compd_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compd_comp`] module"]
#[doc(alias = "COMPD_COMP")]
pub type CompdComp = crate::Reg<compd_comp::CompdCompSpec>;
#[doc = "Compare Value Register X"]
pub mod compd_comp;
#[doc = "COMPE_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compe_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compe_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compe_comp`] module"]
#[doc(alias = "COMPE_COMP")]
pub type CompeComp = crate::Reg<compe_comp::CompeCompSpec>;
#[doc = "Compare Value Register X"]
pub mod compe_comp;
#[doc = "COMPF_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compf_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compf_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compf_comp`] module"]
#[doc(alias = "COMPF_COMP")]
pub type CompfComp = crate::Reg<compf_comp::CompfCompSpec>;
#[doc = "Compare Value Register X"]
pub mod compf_comp;
