#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved4: [u8; 0x24],
    ppuctrl: Ppuctrl,
    _reserved5: [u8; 0x0c],
    ppupatd0: Ppupatd0,
    ppupatd1: Ppupatd1,
    ppupatd2: Ppupatd2,
    _reserved8: [u8; 0x34],
    ppufs: Ppufs,
}
impl RegisterBlock {
    #[doc = "0x0c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x10 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x18 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x40 - PPU Control Register"]
    #[inline(always)]
    pub const fn ppuctrl(&self) -> &Ppuctrl {
        &self.ppuctrl
    }
    #[doc = "0x50 - PPU Privilege Access Type Descriptor 0"]
    #[inline(always)]
    pub const fn ppupatd0(&self) -> &Ppupatd0 {
        &self.ppupatd0
    }
    #[doc = "0x54 - PPU Privilege Access Type Descriptor 1"]
    #[inline(always)]
    pub const fn ppupatd1(&self) -> &Ppupatd1 {
        &self.ppupatd1
    }
    #[doc = "0x58 - PPU Privilege Access Type Descriptor 2"]
    #[inline(always)]
    pub const fn ppupatd2(&self) -> &Ppupatd2 {
        &self.ppupatd2
    }
    #[doc = "0x90 - PPU Fault Status"]
    #[inline(always)]
    pub const fn ppufs(&self) -> &Ppufs {
        &self.ppufs
    }
}
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
#[doc = "PPUCTRL (rw) register accessor: PPU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppuctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppuctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppuctrl`] module"]
#[doc(alias = "PPUCTRL")]
pub type Ppuctrl = crate::Reg<ppuctrl::PpuctrlSpec>;
#[doc = "PPU Control Register"]
pub mod ppuctrl;
#[doc = "PPUPATD0 (rw) register accessor: PPU Privilege Access Type Descriptor 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppupatd0`] module"]
#[doc(alias = "PPUPATD0")]
pub type Ppupatd0 = crate::Reg<ppupatd0::Ppupatd0Spec>;
#[doc = "PPU Privilege Access Type Descriptor 0"]
pub mod ppupatd0;
#[doc = "PPUPATD1 (rw) register accessor: PPU Privilege Access Type Descriptor 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppupatd1`] module"]
#[doc(alias = "PPUPATD1")]
pub type Ppupatd1 = crate::Reg<ppupatd1::Ppupatd1Spec>;
#[doc = "PPU Privilege Access Type Descriptor 1"]
pub mod ppupatd1;
#[doc = "PPUPATD2 (rw) register accessor: PPU Privilege Access Type Descriptor 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppupatd2`] module"]
#[doc(alias = "PPUPATD2")]
pub type Ppupatd2 = crate::Reg<ppupatd2::Ppupatd2Spec>;
#[doc = "PPU Privilege Access Type Descriptor 2"]
pub mod ppupatd2;
#[doc = "PPUFS (r) register accessor: PPU Fault Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ppufs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppufs`] module"]
#[doc(alias = "PPUFS")]
pub type Ppufs = crate::Reg<ppufs::PpufsSpec>;
#[doc = "PPU Fault Status"]
pub mod ppufs;
