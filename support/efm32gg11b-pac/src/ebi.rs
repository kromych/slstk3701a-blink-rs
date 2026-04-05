#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    addrtiming: Addrtiming,
    rdtiming: Rdtiming,
    wrtiming: Wrtiming,
    polarity: Polarity,
    _reserved5: [u8; 0x04],
    addrtiming1: Addrtiming1,
    rdtiming1: Rdtiming1,
    wrtiming1: Wrtiming1,
    polarity1: Polarity1,
    addrtiming2: Addrtiming2,
    rdtiming2: Rdtiming2,
    wrtiming2: Wrtiming2,
    polarity2: Polarity2,
    addrtiming3: Addrtiming3,
    rdtiming3: Rdtiming3,
    wrtiming3: Wrtiming3,
    polarity3: Polarity3,
    pagectrl: Pagectrl,
    nandctrl: Nandctrl,
    cmd: Cmd,
    status: Status,
    eccparity: Eccparity,
    tftctrl: Tftctrl,
    tftstatus: Tftstatus,
    tftcolorformat: Tftcolorformat,
    tftframebase: Tftframebase,
    _reserved26: [u8; 0x04],
    tftstride: Tftstride,
    tftsize: Tftsize,
    tfthporch: Tfthporch,
    tftvporch: Tftvporch,
    tfttiming: Tfttiming,
    tftpolarity: Tftpolarity,
    tftdd: Tftdd,
    tftalpha: Tftalpha,
    tftpixel0: Tftpixel0,
    tftpixel1: Tftpixel1,
    tftpixel: Tftpixel,
    tftmask: Tftmask,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    routepen: Routepen,
    routeloc0: Routeloc0,
    routeloc1: Routeloc1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Address Timing Register"]
    #[inline(always)]
    pub const fn addrtiming(&self) -> &Addrtiming {
        &self.addrtiming
    }
    #[doc = "0x08 - Read Timing Register"]
    #[inline(always)]
    pub const fn rdtiming(&self) -> &Rdtiming {
        &self.rdtiming
    }
    #[doc = "0x0c - Write Timing Register"]
    #[inline(always)]
    pub const fn wrtiming(&self) -> &Wrtiming {
        &self.wrtiming
    }
    #[doc = "0x10 - Polarity Register"]
    #[inline(always)]
    pub const fn polarity(&self) -> &Polarity {
        &self.polarity
    }
    #[doc = "0x18 - Address Timing Register 1"]
    #[inline(always)]
    pub const fn addrtiming1(&self) -> &Addrtiming1 {
        &self.addrtiming1
    }
    #[doc = "0x1c - Read Timing Register 1"]
    #[inline(always)]
    pub const fn rdtiming1(&self) -> &Rdtiming1 {
        &self.rdtiming1
    }
    #[doc = "0x20 - Write Timing Register 1"]
    #[inline(always)]
    pub const fn wrtiming1(&self) -> &Wrtiming1 {
        &self.wrtiming1
    }
    #[doc = "0x24 - Polarity Register 1"]
    #[inline(always)]
    pub const fn polarity1(&self) -> &Polarity1 {
        &self.polarity1
    }
    #[doc = "0x28 - Address Timing Register 2"]
    #[inline(always)]
    pub const fn addrtiming2(&self) -> &Addrtiming2 {
        &self.addrtiming2
    }
    #[doc = "0x2c - Read Timing Register 2"]
    #[inline(always)]
    pub const fn rdtiming2(&self) -> &Rdtiming2 {
        &self.rdtiming2
    }
    #[doc = "0x30 - Write Timing Register 2"]
    #[inline(always)]
    pub const fn wrtiming2(&self) -> &Wrtiming2 {
        &self.wrtiming2
    }
    #[doc = "0x34 - Polarity Register 2"]
    #[inline(always)]
    pub const fn polarity2(&self) -> &Polarity2 {
        &self.polarity2
    }
    #[doc = "0x38 - Address Timing Register 3"]
    #[inline(always)]
    pub const fn addrtiming3(&self) -> &Addrtiming3 {
        &self.addrtiming3
    }
    #[doc = "0x3c - Read Timing Register 3"]
    #[inline(always)]
    pub const fn rdtiming3(&self) -> &Rdtiming3 {
        &self.rdtiming3
    }
    #[doc = "0x40 - Write Timing Register 3"]
    #[inline(always)]
    pub const fn wrtiming3(&self) -> &Wrtiming3 {
        &self.wrtiming3
    }
    #[doc = "0x44 - Polarity Register 3"]
    #[inline(always)]
    pub const fn polarity3(&self) -> &Polarity3 {
        &self.polarity3
    }
    #[doc = "0x48 - Page Control Register"]
    #[inline(always)]
    pub const fn pagectrl(&self) -> &Pagectrl {
        &self.pagectrl
    }
    #[doc = "0x4c - NAND Control Register"]
    #[inline(always)]
    pub const fn nandctrl(&self) -> &Nandctrl {
        &self.nandctrl
    }
    #[doc = "0x50 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x54 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x58 - ECC Parity Register"]
    #[inline(always)]
    pub const fn eccparity(&self) -> &Eccparity {
        &self.eccparity
    }
    #[doc = "0x5c - TFT Control Register"]
    #[inline(always)]
    pub const fn tftctrl(&self) -> &Tftctrl {
        &self.tftctrl
    }
    #[doc = "0x60 - TFT Status Register"]
    #[inline(always)]
    pub const fn tftstatus(&self) -> &Tftstatus {
        &self.tftstatus
    }
    #[doc = "0x64 - Color Format Register"]
    #[inline(always)]
    pub const fn tftcolorformat(&self) -> &Tftcolorformat {
        &self.tftcolorformat
    }
    #[doc = "0x68 - TFT Frame Base Register"]
    #[inline(always)]
    pub const fn tftframebase(&self) -> &Tftframebase {
        &self.tftframebase
    }
    #[doc = "0x70 - TFT Stride Register"]
    #[inline(always)]
    pub const fn tftstride(&self) -> &Tftstride {
        &self.tftstride
    }
    #[doc = "0x74 - TFT Size Register"]
    #[inline(always)]
    pub const fn tftsize(&self) -> &Tftsize {
        &self.tftsize
    }
    #[doc = "0x78 - TFT Horizontal Porch Register"]
    #[inline(always)]
    pub const fn tfthporch(&self) -> &Tfthporch {
        &self.tfthporch
    }
    #[doc = "0x7c - TFT Vertical Porch Register"]
    #[inline(always)]
    pub const fn tftvporch(&self) -> &Tftvporch {
        &self.tftvporch
    }
    #[doc = "0x80 - TFT Timing Register"]
    #[inline(always)]
    pub const fn tfttiming(&self) -> &Tfttiming {
        &self.tfttiming
    }
    #[doc = "0x84 - TFT Polarity Register"]
    #[inline(always)]
    pub const fn tftpolarity(&self) -> &Tftpolarity {
        &self.tftpolarity
    }
    #[doc = "0x88 - TFT Direct Drive Data Register"]
    #[inline(always)]
    pub const fn tftdd(&self) -> &Tftdd {
        &self.tftdd
    }
    #[doc = "0x8c - TFT Alpha Blending Register"]
    #[inline(always)]
    pub const fn tftalpha(&self) -> &Tftalpha {
        &self.tftalpha
    }
    #[doc = "0x90 - TFT Pixel 0 Register"]
    #[inline(always)]
    pub const fn tftpixel0(&self) -> &Tftpixel0 {
        &self.tftpixel0
    }
    #[doc = "0x94 - TFT Pixel 1 Register"]
    #[inline(always)]
    pub const fn tftpixel1(&self) -> &Tftpixel1 {
        &self.tftpixel1
    }
    #[doc = "0x98 - TFT Alpha Blending Result Pixel Register"]
    #[inline(always)]
    pub const fn tftpixel(&self) -> &Tftpixel {
        &self.tftpixel
    }
    #[doc = "0x9c - TFT Masking Register"]
    #[inline(always)]
    pub const fn tftmask(&self) -> &Tftmask {
        &self.tftmask
    }
    #[doc = "0xa0 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0xac - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0xb0 - I/O Routing Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0xb4 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0xb8 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ADDRTIMING (rw) register accessor: Address Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addrtiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrtiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming`] module"]
#[doc(alias = "ADDRTIMING")]
pub type Addrtiming = crate::Reg<addrtiming::AddrtimingSpec>;
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "RDTIMING (rw) register accessor: Read Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdtiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdtiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming`] module"]
#[doc(alias = "RDTIMING")]
pub type Rdtiming = crate::Reg<rdtiming::RdtimingSpec>;
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "WRTIMING (rw) register accessor: Write Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming`] module"]
#[doc(alias = "WRTIMING")]
pub type Wrtiming = crate::Reg<wrtiming::WrtimingSpec>;
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "POLARITY (rw) register accessor: Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity`] module"]
#[doc(alias = "POLARITY")]
pub type Polarity = crate::Reg<polarity::PolaritySpec>;
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "ADDRTIMING1 (rw) register accessor: Address Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`addrtiming1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrtiming1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming1`] module"]
#[doc(alias = "ADDRTIMING1")]
pub type Addrtiming1 = crate::Reg<addrtiming1::Addrtiming1Spec>;
#[doc = "Address Timing Register 1"]
pub mod addrtiming1;
#[doc = "RDTIMING1 (rw) register accessor: Read Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rdtiming1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdtiming1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming1`] module"]
#[doc(alias = "RDTIMING1")]
pub type Rdtiming1 = crate::Reg<rdtiming1::Rdtiming1Spec>;
#[doc = "Read Timing Register 1"]
pub mod rdtiming1;
#[doc = "WRTIMING1 (rw) register accessor: Write Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtiming1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtiming1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming1`] module"]
#[doc(alias = "WRTIMING1")]
pub type Wrtiming1 = crate::Reg<wrtiming1::Wrtiming1Spec>;
#[doc = "Write Timing Register 1"]
pub mod wrtiming1;
#[doc = "POLARITY1 (rw) register accessor: Polarity Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity1`] module"]
#[doc(alias = "POLARITY1")]
pub type Polarity1 = crate::Reg<polarity1::Polarity1Spec>;
#[doc = "Polarity Register 1"]
pub mod polarity1;
#[doc = "ADDRTIMING2 (rw) register accessor: Address Timing Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`addrtiming2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrtiming2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming2`] module"]
#[doc(alias = "ADDRTIMING2")]
pub type Addrtiming2 = crate::Reg<addrtiming2::Addrtiming2Spec>;
#[doc = "Address Timing Register 2"]
pub mod addrtiming2;
#[doc = "RDTIMING2 (rw) register accessor: Read Timing Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rdtiming2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdtiming2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming2`] module"]
#[doc(alias = "RDTIMING2")]
pub type Rdtiming2 = crate::Reg<rdtiming2::Rdtiming2Spec>;
#[doc = "Read Timing Register 2"]
pub mod rdtiming2;
#[doc = "WRTIMING2 (rw) register accessor: Write Timing Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtiming2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtiming2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming2`] module"]
#[doc(alias = "WRTIMING2")]
pub type Wrtiming2 = crate::Reg<wrtiming2::Wrtiming2Spec>;
#[doc = "Write Timing Register 2"]
pub mod wrtiming2;
#[doc = "POLARITY2 (rw) register accessor: Polarity Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity2`] module"]
#[doc(alias = "POLARITY2")]
pub type Polarity2 = crate::Reg<polarity2::Polarity2Spec>;
#[doc = "Polarity Register 2"]
pub mod polarity2;
#[doc = "ADDRTIMING3 (rw) register accessor: Address Timing Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`addrtiming3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrtiming3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming3`] module"]
#[doc(alias = "ADDRTIMING3")]
pub type Addrtiming3 = crate::Reg<addrtiming3::Addrtiming3Spec>;
#[doc = "Address Timing Register 3"]
pub mod addrtiming3;
#[doc = "RDTIMING3 (rw) register accessor: Read Timing Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`rdtiming3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdtiming3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming3`] module"]
#[doc(alias = "RDTIMING3")]
pub type Rdtiming3 = crate::Reg<rdtiming3::Rdtiming3Spec>;
#[doc = "Read Timing Register 3"]
pub mod rdtiming3;
#[doc = "WRTIMING3 (rw) register accessor: Write Timing Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtiming3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtiming3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming3`] module"]
#[doc(alias = "WRTIMING3")]
pub type Wrtiming3 = crate::Reg<wrtiming3::Wrtiming3Spec>;
#[doc = "Write Timing Register 3"]
pub mod wrtiming3;
#[doc = "POLARITY3 (rw) register accessor: Polarity Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity3`] module"]
#[doc(alias = "POLARITY3")]
pub type Polarity3 = crate::Reg<polarity3::Polarity3Spec>;
#[doc = "Polarity Register 3"]
pub mod polarity3;
#[doc = "PAGECTRL (rw) register accessor: Page Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pagectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pagectrl`] module"]
#[doc(alias = "PAGECTRL")]
pub type Pagectrl = crate::Reg<pagectrl::PagectrlSpec>;
#[doc = "Page Control Register"]
pub mod pagectrl;
#[doc = "NANDCTRL (rw) register accessor: NAND Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nandctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nandctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nandctrl`] module"]
#[doc(alias = "NANDCTRL")]
pub type Nandctrl = crate::Reg<nandctrl::NandctrlSpec>;
#[doc = "NAND Control Register"]
pub mod nandctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ECCPARITY (r) register accessor: ECC Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccparity::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccparity`] module"]
#[doc(alias = "ECCPARITY")]
pub type Eccparity = crate::Reg<eccparity::EccparitySpec>;
#[doc = "ECC Parity Register"]
pub mod eccparity;
#[doc = "TFTCTRL (rw) register accessor: TFT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftctrl`] module"]
#[doc(alias = "TFTCTRL")]
pub type Tftctrl = crate::Reg<tftctrl::TftctrlSpec>;
#[doc = "TFT Control Register"]
pub mod tftctrl;
#[doc = "TFTSTATUS (r) register accessor: TFT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftstatus`] module"]
#[doc(alias = "TFTSTATUS")]
pub type Tftstatus = crate::Reg<tftstatus::TftstatusSpec>;
#[doc = "TFT Status Register"]
pub mod tftstatus;
#[doc = "TFTCOLORFORMAT (rw) register accessor: Color Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftcolorformat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftcolorformat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftcolorformat`] module"]
#[doc(alias = "TFTCOLORFORMAT")]
pub type Tftcolorformat = crate::Reg<tftcolorformat::TftcolorformatSpec>;
#[doc = "Color Format Register"]
pub mod tftcolorformat;
#[doc = "TFTFRAMEBASE (rw) register accessor: TFT Frame Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftframebase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftframebase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftframebase`] module"]
#[doc(alias = "TFTFRAMEBASE")]
pub type Tftframebase = crate::Reg<tftframebase::TftframebaseSpec>;
#[doc = "TFT Frame Base Register"]
pub mod tftframebase;
#[doc = "TFTSTRIDE (rw) register accessor: TFT Stride Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftstride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftstride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftstride`] module"]
#[doc(alias = "TFTSTRIDE")]
pub type Tftstride = crate::Reg<tftstride::TftstrideSpec>;
#[doc = "TFT Stride Register"]
pub mod tftstride;
#[doc = "TFTSIZE (rw) register accessor: TFT Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftsize`] module"]
#[doc(alias = "TFTSIZE")]
pub type Tftsize = crate::Reg<tftsize::TftsizeSpec>;
#[doc = "TFT Size Register"]
pub mod tftsize;
#[doc = "TFTHPORCH (rw) register accessor: TFT Horizontal Porch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfthporch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfthporch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfthporch`] module"]
#[doc(alias = "TFTHPORCH")]
pub type Tfthporch = crate::Reg<tfthporch::TfthporchSpec>;
#[doc = "TFT Horizontal Porch Register"]
pub mod tfthporch;
#[doc = "TFTVPORCH (rw) register accessor: TFT Vertical Porch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftvporch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftvporch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftvporch`] module"]
#[doc(alias = "TFTVPORCH")]
pub type Tftvporch = crate::Reg<tftvporch::TftvporchSpec>;
#[doc = "TFT Vertical Porch Register"]
pub mod tftvporch;
#[doc = "TFTTIMING (rw) register accessor: TFT Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfttiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfttiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfttiming`] module"]
#[doc(alias = "TFTTIMING")]
pub type Tfttiming = crate::Reg<tfttiming::TfttimingSpec>;
#[doc = "TFT Timing Register"]
pub mod tfttiming;
#[doc = "TFTPOLARITY (rw) register accessor: TFT Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpolarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftpolarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpolarity`] module"]
#[doc(alias = "TFTPOLARITY")]
pub type Tftpolarity = crate::Reg<tftpolarity::TftpolaritySpec>;
#[doc = "TFT Polarity Register"]
pub mod tftpolarity;
#[doc = "TFTDD (rw) register accessor: TFT Direct Drive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftdd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftdd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftdd`] module"]
#[doc(alias = "TFTDD")]
pub type Tftdd = crate::Reg<tftdd::TftddSpec>;
#[doc = "TFT Direct Drive Data Register"]
pub mod tftdd;
#[doc = "TFTALPHA (rw) register accessor: TFT Alpha Blending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftalpha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftalpha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftalpha`] module"]
#[doc(alias = "TFTALPHA")]
pub type Tftalpha = crate::Reg<tftalpha::TftalphaSpec>;
#[doc = "TFT Alpha Blending Register"]
pub mod tftalpha;
#[doc = "TFTPIXEL0 (rw) register accessor: TFT Pixel 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpixel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftpixel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpixel0`] module"]
#[doc(alias = "TFTPIXEL0")]
pub type Tftpixel0 = crate::Reg<tftpixel0::Tftpixel0Spec>;
#[doc = "TFT Pixel 0 Register"]
pub mod tftpixel0;
#[doc = "TFTPIXEL1 (rw) register accessor: TFT Pixel 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpixel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftpixel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpixel1`] module"]
#[doc(alias = "TFTPIXEL1")]
pub type Tftpixel1 = crate::Reg<tftpixel1::Tftpixel1Spec>;
#[doc = "TFT Pixel 1 Register"]
pub mod tftpixel1;
#[doc = "TFTPIXEL (r) register accessor: TFT Alpha Blending Result Pixel Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpixel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpixel`] module"]
#[doc(alias = "TFTPIXEL")]
pub type Tftpixel = crate::Reg<tftpixel::TftpixelSpec>;
#[doc = "TFT Alpha Blending Result Pixel Register"]
pub mod tftpixel;
#[doc = "TFTMASK (rw) register accessor: TFT Masking Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftmask`] module"]
#[doc(alias = "TFTMASK")]
pub type Tftmask = crate::Reg<tftmask::TftmaskSpec>;
#[doc = "TFT Masking Register"]
pub mod tftmask;
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
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`] module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::Routeloc0Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`] module"]
#[doc(alias = "ROUTELOC1")]
pub type Routeloc1 = crate::Reg<routeloc1::Routeloc1Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
