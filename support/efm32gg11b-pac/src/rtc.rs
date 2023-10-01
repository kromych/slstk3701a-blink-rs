#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Compare Value Register X"]
    pub compa_comp: COMPA_COMP,
    #[doc = "0x24 - Compare Value Register X"]
    pub compb_comp: COMPB_COMP,
    #[doc = "0x28 - Compare Value Register X"]
    pub compc_comp: COMPC_COMP,
    #[doc = "0x2c - Compare Value Register X"]
    pub compd_comp: COMPD_COMP,
    #[doc = "0x30 - Compare Value Register X"]
    pub compe_comp: COMPE_COMP,
    #[doc = "0x34 - Compare Value Register X"]
    pub compf_comp: COMPF_COMP,
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`] module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if_`] module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifs`] module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifc`] module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ien`] module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "COMPA_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compa_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compa_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`compa_comp`] module"]
pub type COMPA_COMP = crate::Reg<compa_comp::COMPA_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compa_comp;
#[doc = "COMPB_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compb_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compb_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`compb_comp`] module"]
pub type COMPB_COMP = crate::Reg<compb_comp::COMPB_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compb_comp;
#[doc = "COMPC_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compc_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compc_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`compc_comp`] module"]
pub type COMPC_COMP = crate::Reg<compc_comp::COMPC_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compc_comp;
#[doc = "COMPD_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compd_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compd_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`compd_comp`] module"]
pub type COMPD_COMP = crate::Reg<compd_comp::COMPD_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compd_comp;
#[doc = "COMPE_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compe_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compe_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`compe_comp`] module"]
pub type COMPE_COMP = crate::Reg<compe_comp::COMPE_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compe_comp;
#[doc = "COMPF_COMP (rw) register accessor: Compare Value Register X\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compf_comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`compf_comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`compf_comp`] module"]
pub type COMPF_COMP = crate::Reg<compf_comp::COMPF_COMP_SPEC>;
#[doc = "Compare Value Register X"]
pub mod compf_comp;
