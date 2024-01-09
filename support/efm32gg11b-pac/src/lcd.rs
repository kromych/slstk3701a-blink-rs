#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    dispctrl: DISPCTRL,
    segen: SEGEN,
    bactrl: BACTRL,
    status: STATUS,
    arega: AREGA,
    aregb: AREGB,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    _reserved11: [u8; 0x04],
    biasctrl: BIASCTRL,
    _reserved12: [u8; 0x0c],
    segd0l: SEGD0L,
    segd1l: SEGD1L,
    segd2l: SEGD2L,
    segd3l: SEGD3L,
    segd0h: SEGD0H,
    segd1h: SEGD1H,
    segd2h: SEGD2H,
    segd3h: SEGD3H,
    segd4l: SEGD4L,
    segd5l: SEGD5L,
    segd6l: SEGD6L,
    segd7l: SEGD7L,
    segd4h: SEGD4H,
    segd5h: SEGD5H,
    segd6h: SEGD6H,
    segd7h: SEGD7H,
    _reserved28: [u8; 0x40],
    freeze: FREEZE,
    syncbusy: SYNCBUSY,
    _reserved30: [u8; 0x28],
    framerate: FRAMERATE,
    segen2: SEGEN2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Display Control Register"]
    #[inline(always)]
    pub const fn dispctrl(&self) -> &DISPCTRL {
        &self.dispctrl
    }
    #[doc = "0x08 - Segment Enable Register"]
    #[inline(always)]
    pub const fn segen(&self) -> &SEGEN {
        &self.segen
    }
    #[doc = "0x0c - Blink and Animation Control Register"]
    #[inline(always)]
    pub const fn bactrl(&self) -> &BACTRL {
        &self.bactrl
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x14 - Animation Register a"]
    #[inline(always)]
    pub const fn arega(&self) -> &AREGA {
        &self.arega
    }
    #[doc = "0x18 - Animation Register B"]
    #[inline(always)]
    pub const fn aregb(&self) -> &AREGB {
        &self.aregb
    }
    #[doc = "0x1c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x20 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x30 - Analog BIAS Control"]
    #[inline(always)]
    pub const fn biasctrl(&self) -> &BIASCTRL {
        &self.biasctrl
    }
    #[doc = "0x40 - Segment Data Low Register 0"]
    #[inline(always)]
    pub const fn segd0l(&self) -> &SEGD0L {
        &self.segd0l
    }
    #[doc = "0x44 - Segment Data Low Register 1"]
    #[inline(always)]
    pub const fn segd1l(&self) -> &SEGD1L {
        &self.segd1l
    }
    #[doc = "0x48 - Segment Data Low Register 2"]
    #[inline(always)]
    pub const fn segd2l(&self) -> &SEGD2L {
        &self.segd2l
    }
    #[doc = "0x4c - Segment Data Low Register 3"]
    #[inline(always)]
    pub const fn segd3l(&self) -> &SEGD3L {
        &self.segd3l
    }
    #[doc = "0x50 - Segment Data High Register 0"]
    #[inline(always)]
    pub const fn segd0h(&self) -> &SEGD0H {
        &self.segd0h
    }
    #[doc = "0x54 - Segment Data High Register 1"]
    #[inline(always)]
    pub const fn segd1h(&self) -> &SEGD1H {
        &self.segd1h
    }
    #[doc = "0x58 - Segment Data High Register 2"]
    #[inline(always)]
    pub const fn segd2h(&self) -> &SEGD2H {
        &self.segd2h
    }
    #[doc = "0x5c - Segment Data High Register 3"]
    #[inline(always)]
    pub const fn segd3h(&self) -> &SEGD3H {
        &self.segd3h
    }
    #[doc = "0x60 - Segment Data Low Register 4"]
    #[inline(always)]
    pub const fn segd4l(&self) -> &SEGD4L {
        &self.segd4l
    }
    #[doc = "0x64 - Segment Data Low Register 5"]
    #[inline(always)]
    pub const fn segd5l(&self) -> &SEGD5L {
        &self.segd5l
    }
    #[doc = "0x68 - Segment Data Low Register 6"]
    #[inline(always)]
    pub const fn segd6l(&self) -> &SEGD6L {
        &self.segd6l
    }
    #[doc = "0x6c - Segment Data Low Register 7"]
    #[inline(always)]
    pub const fn segd7l(&self) -> &SEGD7L {
        &self.segd7l
    }
    #[doc = "0x70 - Segment Data High Register 4"]
    #[inline(always)]
    pub const fn segd4h(&self) -> &SEGD4H {
        &self.segd4h
    }
    #[doc = "0x74 - Segment Data High Register 5"]
    #[inline(always)]
    pub const fn segd5h(&self) -> &SEGD5H {
        &self.segd5h
    }
    #[doc = "0x78 - Segment Data High Register 6"]
    #[inline(always)]
    pub const fn segd6h(&self) -> &SEGD6H {
        &self.segd6h
    }
    #[doc = "0x7c - Segment Data High Register 7"]
    #[inline(always)]
    pub const fn segd7h(&self) -> &SEGD7H {
        &self.segd7h
    }
    #[doc = "0xc0 - Freeze Register"]
    #[inline(always)]
    pub const fn freeze(&self) -> &FREEZE {
        &self.freeze
    }
    #[doc = "0xc4 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &SYNCBUSY {
        &self.syncbusy
    }
    #[doc = "0xf0 - Frame Rate"]
    #[inline(always)]
    pub const fn framerate(&self) -> &FRAMERATE {
        &self.framerate
    }
    #[doc = "0xf4 - Segment Enable (32 to 39)"]
    #[inline(always)]
    pub const fn segen2(&self) -> &SEGEN2 {
        &self.segen2
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "DISPCTRL (rw) register accessor: Display Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dispctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dispctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dispctrl`]
module"]
pub type DISPCTRL = crate::Reg<dispctrl::DISPCTRL_SPEC>;
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "SEGEN (rw) register accessor: Segment Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segen`]
module"]
pub type SEGEN = crate::Reg<segen::SEGEN_SPEC>;
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "BACTRL (rw) register accessor: Blink and Animation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bactrl`]
module"]
pub type BACTRL = crate::Reg<bactrl::BACTRL_SPEC>;
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "AREGA (rw) register accessor: Animation Register a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arega::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arega::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arega`]
module"]
pub type AREGA = crate::Reg<arega::AREGA_SPEC>;
#[doc = "Animation Register a"]
pub mod arega;
#[doc = "AREGB (rw) register accessor: Animation Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aregb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aregb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aregb`]
module"]
pub type AREGB = crate::Reg<aregb::AREGB_SPEC>;
#[doc = "Animation Register B"]
pub mod aregb;
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
#[doc = "BIASCTRL (rw) register accessor: Analog BIAS Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasctrl`]
module"]
pub type BIASCTRL = crate::Reg<biasctrl::BIASCTRL_SPEC>;
#[doc = "Analog BIAS Control"]
pub mod biasctrl;
#[doc = "SEGD0L (rw) register accessor: Segment Data Low Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd0l`]
module"]
pub type SEGD0L = crate::Reg<segd0l::SEGD0L_SPEC>;
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "SEGD1L (rw) register accessor: Segment Data Low Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd1l`]
module"]
pub type SEGD1L = crate::Reg<segd1l::SEGD1L_SPEC>;
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "SEGD2L (rw) register accessor: Segment Data Low Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd2l`]
module"]
pub type SEGD2L = crate::Reg<segd2l::SEGD2L_SPEC>;
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "SEGD3L (rw) register accessor: Segment Data Low Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd3l`]
module"]
pub type SEGD3L = crate::Reg<segd3l::SEGD3L_SPEC>;
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "SEGD0H (rw) register accessor: Segment Data High Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd0h`]
module"]
pub type SEGD0H = crate::Reg<segd0h::SEGD0H_SPEC>;
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "SEGD1H (rw) register accessor: Segment Data High Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd1h`]
module"]
pub type SEGD1H = crate::Reg<segd1h::SEGD1H_SPEC>;
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "SEGD2H (rw) register accessor: Segment Data High Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd2h`]
module"]
pub type SEGD2H = crate::Reg<segd2h::SEGD2H_SPEC>;
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "SEGD3H (rw) register accessor: Segment Data High Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd3h`]
module"]
pub type SEGD3H = crate::Reg<segd3h::SEGD3H_SPEC>;
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "SEGD4L (rw) register accessor: Segment Data Low Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd4l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd4l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd4l`]
module"]
pub type SEGD4L = crate::Reg<segd4l::SEGD4L_SPEC>;
#[doc = "Segment Data Low Register 4"]
pub mod segd4l;
#[doc = "SEGD5L (rw) register accessor: Segment Data Low Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd5l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd5l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd5l`]
module"]
pub type SEGD5L = crate::Reg<segd5l::SEGD5L_SPEC>;
#[doc = "Segment Data Low Register 5"]
pub mod segd5l;
#[doc = "SEGD6L (rw) register accessor: Segment Data Low Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd6l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd6l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd6l`]
module"]
pub type SEGD6L = crate::Reg<segd6l::SEGD6L_SPEC>;
#[doc = "Segment Data Low Register 6"]
pub mod segd6l;
#[doc = "SEGD7L (rw) register accessor: Segment Data Low Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd7l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd7l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd7l`]
module"]
pub type SEGD7L = crate::Reg<segd7l::SEGD7L_SPEC>;
#[doc = "Segment Data Low Register 7"]
pub mod segd7l;
#[doc = "SEGD4H (rw) register accessor: Segment Data High Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd4h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd4h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd4h`]
module"]
pub type SEGD4H = crate::Reg<segd4h::SEGD4H_SPEC>;
#[doc = "Segment Data High Register 4"]
pub mod segd4h;
#[doc = "SEGD5H (rw) register accessor: Segment Data High Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd5h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd5h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd5h`]
module"]
pub type SEGD5H = crate::Reg<segd5h::SEGD5H_SPEC>;
#[doc = "Segment Data High Register 5"]
pub mod segd5h;
#[doc = "SEGD6H (rw) register accessor: Segment Data High Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd6h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd6h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd6h`]
module"]
pub type SEGD6H = crate::Reg<segd6h::SEGD6H_SPEC>;
#[doc = "Segment Data High Register 6"]
pub mod segd6h;
#[doc = "SEGD7H (rw) register accessor: Segment Data High Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd7h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd7h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd7h`]
module"]
pub type SEGD7H = crate::Reg<segd7h::SEGD7H_SPEC>;
#[doc = "Segment Data High Register 7"]
pub mod segd7h;
#[doc = "FREEZE (rw) register accessor: Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freeze`]
module"]
pub type FREEZE = crate::Reg<freeze::FREEZE_SPEC>;
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "FRAMERATE (rw) register accessor: Frame Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framerate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framerate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framerate`]
module"]
pub type FRAMERATE = crate::Reg<framerate::FRAMERATE_SPEC>;
#[doc = "Frame Rate"]
pub mod framerate;
#[doc = "SEGEN2 (rw) register accessor: Segment Enable (32 to 39)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segen2`]
module"]
pub type SEGEN2 = crate::Reg<segen2::SEGEN2_SPEC>;
#[doc = "Segment Enable (32 to 39)"]
pub mod segen2;
