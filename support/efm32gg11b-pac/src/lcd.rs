#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    dispctrl: Dispctrl,
    segen: Segen,
    bactrl: Bactrl,
    status: Status,
    arega: Arega,
    aregb: Aregb,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    _reserved11: [u8; 0x04],
    biasctrl: Biasctrl,
    _reserved12: [u8; 0x0c],
    segd0l: Segd0l,
    segd1l: Segd1l,
    segd2l: Segd2l,
    segd3l: Segd3l,
    segd0h: Segd0h,
    segd1h: Segd1h,
    segd2h: Segd2h,
    segd3h: Segd3h,
    segd4l: Segd4l,
    segd5l: Segd5l,
    segd6l: Segd6l,
    segd7l: Segd7l,
    segd4h: Segd4h,
    segd5h: Segd5h,
    segd6h: Segd6h,
    segd7h: Segd7h,
    _reserved28: [u8; 0x40],
    freeze: Freeze,
    syncbusy: Syncbusy,
    _reserved30: [u8; 0x28],
    framerate: Framerate,
    segen2: Segen2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Display Control Register"]
    #[inline(always)]
    pub const fn dispctrl(&self) -> &Dispctrl {
        &self.dispctrl
    }
    #[doc = "0x08 - Segment Enable Register"]
    #[inline(always)]
    pub const fn segen(&self) -> &Segen {
        &self.segen
    }
    #[doc = "0x0c - Blink and Animation Control Register"]
    #[inline(always)]
    pub const fn bactrl(&self) -> &Bactrl {
        &self.bactrl
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Animation Register a"]
    #[inline(always)]
    pub const fn arega(&self) -> &Arega {
        &self.arega
    }
    #[doc = "0x18 - Animation Register B"]
    #[inline(always)]
    pub const fn aregb(&self) -> &Aregb {
        &self.aregb
    }
    #[doc = "0x1c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x20 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x30 - Analog BIAS Control"]
    #[inline(always)]
    pub const fn biasctrl(&self) -> &Biasctrl {
        &self.biasctrl
    }
    #[doc = "0x40 - Segment Data Low Register 0"]
    #[inline(always)]
    pub const fn segd0l(&self) -> &Segd0l {
        &self.segd0l
    }
    #[doc = "0x44 - Segment Data Low Register 1"]
    #[inline(always)]
    pub const fn segd1l(&self) -> &Segd1l {
        &self.segd1l
    }
    #[doc = "0x48 - Segment Data Low Register 2"]
    #[inline(always)]
    pub const fn segd2l(&self) -> &Segd2l {
        &self.segd2l
    }
    #[doc = "0x4c - Segment Data Low Register 3"]
    #[inline(always)]
    pub const fn segd3l(&self) -> &Segd3l {
        &self.segd3l
    }
    #[doc = "0x50 - Segment Data High Register 0"]
    #[inline(always)]
    pub const fn segd0h(&self) -> &Segd0h {
        &self.segd0h
    }
    #[doc = "0x54 - Segment Data High Register 1"]
    #[inline(always)]
    pub const fn segd1h(&self) -> &Segd1h {
        &self.segd1h
    }
    #[doc = "0x58 - Segment Data High Register 2"]
    #[inline(always)]
    pub const fn segd2h(&self) -> &Segd2h {
        &self.segd2h
    }
    #[doc = "0x5c - Segment Data High Register 3"]
    #[inline(always)]
    pub const fn segd3h(&self) -> &Segd3h {
        &self.segd3h
    }
    #[doc = "0x60 - Segment Data Low Register 4"]
    #[inline(always)]
    pub const fn segd4l(&self) -> &Segd4l {
        &self.segd4l
    }
    #[doc = "0x64 - Segment Data Low Register 5"]
    #[inline(always)]
    pub const fn segd5l(&self) -> &Segd5l {
        &self.segd5l
    }
    #[doc = "0x68 - Segment Data Low Register 6"]
    #[inline(always)]
    pub const fn segd6l(&self) -> &Segd6l {
        &self.segd6l
    }
    #[doc = "0x6c - Segment Data Low Register 7"]
    #[inline(always)]
    pub const fn segd7l(&self) -> &Segd7l {
        &self.segd7l
    }
    #[doc = "0x70 - Segment Data High Register 4"]
    #[inline(always)]
    pub const fn segd4h(&self) -> &Segd4h {
        &self.segd4h
    }
    #[doc = "0x74 - Segment Data High Register 5"]
    #[inline(always)]
    pub const fn segd5h(&self) -> &Segd5h {
        &self.segd5h
    }
    #[doc = "0x78 - Segment Data High Register 6"]
    #[inline(always)]
    pub const fn segd6h(&self) -> &Segd6h {
        &self.segd6h
    }
    #[doc = "0x7c - Segment Data High Register 7"]
    #[inline(always)]
    pub const fn segd7h(&self) -> &Segd7h {
        &self.segd7h
    }
    #[doc = "0xc0 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &Freeze {
        &self.freeze
    }
    #[doc = "0xc4 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0xf0 - Frame Rate"]
    #[inline(always)]
    pub const fn framerate(&self) -> &Framerate {
        &self.framerate
    }
    #[doc = "0xf4 - Segment Enable (32 to 39)"]
    #[inline(always)]
    pub const fn segen2(&self) -> &Segen2 {
        &self.segen2
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DISPCTRL (rw) register accessor: Display Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dispctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dispctrl`] module"]
#[doc(alias = "DISPCTRL")]
pub type Dispctrl = crate::Reg<dispctrl::DispctrlSpec>;
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "SEGEN (rw) register accessor: Segment Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`segen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segen`] module"]
#[doc(alias = "SEGEN")]
pub type Segen = crate::Reg<segen::SegenSpec>;
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "BACTRL (rw) register accessor: Blink and Animation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bactrl`] module"]
#[doc(alias = "BACTRL")]
pub type Bactrl = crate::Reg<bactrl::BactrlSpec>;
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "AREGA (rw) register accessor: Animation Register a\n\nYou can [`read`](crate::Reg::read) this register and get [`arega::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arega::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arega`] module"]
#[doc(alias = "AREGA")]
pub type Arega = crate::Reg<arega::AregaSpec>;
#[doc = "Animation Register a"]
pub mod arega;
#[doc = "AREGB (rw) register accessor: Animation Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`aregb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aregb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aregb`] module"]
#[doc(alias = "AREGB")]
pub type Aregb = crate::Reg<aregb::AregbSpec>;
#[doc = "Animation Register B"]
pub mod aregb;
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
#[doc = "BIASCTRL (rw) register accessor: Analog BIAS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`biasctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasctrl`] module"]
#[doc(alias = "BIASCTRL")]
pub type Biasctrl = crate::Reg<biasctrl::BiasctrlSpec>;
#[doc = "Analog BIAS Control"]
pub mod biasctrl;
#[doc = "SEGD0L (rw) register accessor: Segment Data Low Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`segd0l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd0l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd0l`] module"]
#[doc(alias = "SEGD0L")]
pub type Segd0l = crate::Reg<segd0l::Segd0lSpec>;
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "SEGD1L (rw) register accessor: Segment Data Low Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`segd1l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd1l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd1l`] module"]
#[doc(alias = "SEGD1L")]
pub type Segd1l = crate::Reg<segd1l::Segd1lSpec>;
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "SEGD2L (rw) register accessor: Segment Data Low Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`segd2l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd2l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd2l`] module"]
#[doc(alias = "SEGD2L")]
pub type Segd2l = crate::Reg<segd2l::Segd2lSpec>;
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "SEGD3L (rw) register accessor: Segment Data Low Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`segd3l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd3l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd3l`] module"]
#[doc(alias = "SEGD3L")]
pub type Segd3l = crate::Reg<segd3l::Segd3lSpec>;
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "SEGD0H (rw) register accessor: Segment Data High Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`segd0h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd0h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd0h`] module"]
#[doc(alias = "SEGD0H")]
pub type Segd0h = crate::Reg<segd0h::Segd0hSpec>;
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "SEGD1H (rw) register accessor: Segment Data High Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`segd1h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd1h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd1h`] module"]
#[doc(alias = "SEGD1H")]
pub type Segd1h = crate::Reg<segd1h::Segd1hSpec>;
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "SEGD2H (rw) register accessor: Segment Data High Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`segd2h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd2h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd2h`] module"]
#[doc(alias = "SEGD2H")]
pub type Segd2h = crate::Reg<segd2h::Segd2hSpec>;
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "SEGD3H (rw) register accessor: Segment Data High Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`segd3h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd3h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd3h`] module"]
#[doc(alias = "SEGD3H")]
pub type Segd3h = crate::Reg<segd3h::Segd3hSpec>;
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "SEGD4L (rw) register accessor: Segment Data Low Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`segd4l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd4l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd4l`] module"]
#[doc(alias = "SEGD4L")]
pub type Segd4l = crate::Reg<segd4l::Segd4lSpec>;
#[doc = "Segment Data Low Register 4"]
pub mod segd4l;
#[doc = "SEGD5L (rw) register accessor: Segment Data Low Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`segd5l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd5l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd5l`] module"]
#[doc(alias = "SEGD5L")]
pub type Segd5l = crate::Reg<segd5l::Segd5lSpec>;
#[doc = "Segment Data Low Register 5"]
pub mod segd5l;
#[doc = "SEGD6L (rw) register accessor: Segment Data Low Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`segd6l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd6l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd6l`] module"]
#[doc(alias = "SEGD6L")]
pub type Segd6l = crate::Reg<segd6l::Segd6lSpec>;
#[doc = "Segment Data Low Register 6"]
pub mod segd6l;
#[doc = "SEGD7L (rw) register accessor: Segment Data Low Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`segd7l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd7l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd7l`] module"]
#[doc(alias = "SEGD7L")]
pub type Segd7l = crate::Reg<segd7l::Segd7lSpec>;
#[doc = "Segment Data Low Register 7"]
pub mod segd7l;
#[doc = "SEGD4H (rw) register accessor: Segment Data High Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`segd4h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd4h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd4h`] module"]
#[doc(alias = "SEGD4H")]
pub type Segd4h = crate::Reg<segd4h::Segd4hSpec>;
#[doc = "Segment Data High Register 4"]
pub mod segd4h;
#[doc = "SEGD5H (rw) register accessor: Segment Data High Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`segd5h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd5h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd5h`] module"]
#[doc(alias = "SEGD5H")]
pub type Segd5h = crate::Reg<segd5h::Segd5hSpec>;
#[doc = "Segment Data High Register 5"]
pub mod segd5h;
#[doc = "SEGD6H (rw) register accessor: Segment Data High Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`segd6h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd6h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd6h`] module"]
#[doc(alias = "SEGD6H")]
pub type Segd6h = crate::Reg<segd6h::Segd6hSpec>;
#[doc = "Segment Data High Register 6"]
pub mod segd6h;
#[doc = "SEGD7H (rw) register accessor: Segment Data High Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`segd7h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd7h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd7h`] module"]
#[doc(alias = "SEGD7H")]
pub type Segd7h = crate::Reg<segd7h::Segd7hSpec>;
#[doc = "Segment Data High Register 7"]
pub mod segd7h;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`freeze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`] module"]
#[doc(alias = "FREEZE")]
pub type Freeze = crate::Reg<freeze::FreezeSpec>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FRAMERATE (rw) register accessor: Frame Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`framerate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framerate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framerate`] module"]
#[doc(alias = "FRAMERATE")]
pub type Framerate = crate::Reg<framerate::FramerateSpec>;
#[doc = "Frame Rate"]
pub mod framerate;
#[doc = "SEGEN2 (rw) register accessor: Segment Enable (32 to 39)\n\nYou can [`read`](crate::Reg::read) this register and get [`segen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segen2`] module"]
#[doc(alias = "SEGEN2")]
pub type Segen2 = crate::Reg<segen2::Segen2Spec>;
#[doc = "Segment Enable (32 to 39)"]
pub mod segen2;
