#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved4: [u8; 0x24],
    #[doc = "0x40 - PPU Control Register"]
    pub ppuctrl: PPUCTRL,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50 - PPU Privilege Access Type Descriptor 0"]
    pub ppupatd0: PPUPATD0,
    #[doc = "0x54 - PPU Privilege Access Type Descriptor 1"]
    pub ppupatd1: PPUPATD1,
    #[doc = "0x58 - PPU Privilege Access Type Descriptor 2"]
    pub ppupatd2: PPUPATD2,
    _reserved8: [u8; 0x34],
    #[doc = "0x90 - PPU Fault Status"]
    pub ppufs: PPUFS,
}
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
#[doc = "PPUCTRL (rw) register accessor: PPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppuctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppuctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppuctrl`] module"]
pub type PPUCTRL = crate::Reg<ppuctrl::PPUCTRL_SPEC>;
#[doc = "PPU Control Register"]
pub mod ppuctrl;
#[doc = "PPUPATD0 (rw) register accessor: PPU Privilege Access Type Descriptor 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppupatd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppupatd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppupatd0`] module"]
pub type PPUPATD0 = crate::Reg<ppupatd0::PPUPATD0_SPEC>;
#[doc = "PPU Privilege Access Type Descriptor 0"]
pub mod ppupatd0;
#[doc = "PPUPATD1 (rw) register accessor: PPU Privilege Access Type Descriptor 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppupatd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppupatd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppupatd1`] module"]
pub type PPUPATD1 = crate::Reg<ppupatd1::PPUPATD1_SPEC>;
#[doc = "PPU Privilege Access Type Descriptor 1"]
pub mod ppupatd1;
#[doc = "PPUPATD2 (rw) register accessor: PPU Privilege Access Type Descriptor 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppupatd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppupatd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppupatd2`] module"]
pub type PPUPATD2 = crate::Reg<ppupatd2::PPUPATD2_SPEC>;
#[doc = "PPU Privilege Access Type Descriptor 2"]
pub mod ppupatd2;
#[doc = "PPUFS (r) register accessor: PPU Fault Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppufs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppufs`] module"]
pub type PPUFS = crate::Reg<ppufs::PPUFS_SPEC>;
#[doc = "PPU Fault Status"]
pub mod ppufs;
