#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    addrtiming: ADDRTIMING,
    rdtiming: RDTIMING,
    wrtiming: WRTIMING,
    polarity: POLARITY,
    _reserved5: [u8; 0x04],
    addrtiming1: ADDRTIMING1,
    rdtiming1: RDTIMING1,
    wrtiming1: WRTIMING1,
    polarity1: POLARITY1,
    addrtiming2: ADDRTIMING2,
    rdtiming2: RDTIMING2,
    wrtiming2: WRTIMING2,
    polarity2: POLARITY2,
    addrtiming3: ADDRTIMING3,
    rdtiming3: RDTIMING3,
    wrtiming3: WRTIMING3,
    polarity3: POLARITY3,
    pagectrl: PAGECTRL,
    nandctrl: NANDCTRL,
    cmd: CMD,
    status: STATUS,
    eccparity: ECCPARITY,
    tftctrl: TFTCTRL,
    tftstatus: TFTSTATUS,
    tftcolorformat: TFTCOLORFORMAT,
    tftframebase: TFTFRAMEBASE,
    _reserved26: [u8; 0x04],
    tftstride: TFTSTRIDE,
    tftsize: TFTSIZE,
    tfthporch: TFTHPORCH,
    tftvporch: TFTVPORCH,
    tfttiming: TFTTIMING,
    tftpolarity: TFTPOLARITY,
    tftdd: TFTDD,
    tftalpha: TFTALPHA,
    tftpixel0: TFTPIXEL0,
    tftpixel1: TFTPIXEL1,
    tftpixel: TFTPIXEL,
    tftmask: TFTMASK,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    routeloc1: ROUTELOC1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Address Timing Register"]
    #[inline(always)]
    pub const fn addrtiming(&self) -> &ADDRTIMING {
        &self.addrtiming
    }
    #[doc = "0x08 - Read Timing Register"]
    #[inline(always)]
    pub const fn rdtiming(&self) -> &RDTIMING {
        &self.rdtiming
    }
    #[doc = "0x0c - Write Timing Register"]
    #[inline(always)]
    pub const fn wrtiming(&self) -> &WRTIMING {
        &self.wrtiming
    }
    #[doc = "0x10 - Polarity Register"]
    #[inline(always)]
    pub const fn polarity(&self) -> &POLARITY {
        &self.polarity
    }
    #[doc = "0x18 - Address Timing Register 1"]
    #[inline(always)]
    pub const fn addrtiming1(&self) -> &ADDRTIMING1 {
        &self.addrtiming1
    }
    #[doc = "0x1c - Read Timing Register 1"]
    #[inline(always)]
    pub const fn rdtiming1(&self) -> &RDTIMING1 {
        &self.rdtiming1
    }
    #[doc = "0x20 - Write Timing Register 1"]
    #[inline(always)]
    pub const fn wrtiming1(&self) -> &WRTIMING1 {
        &self.wrtiming1
    }
    #[doc = "0x24 - Polarity Register 1"]
    #[inline(always)]
    pub const fn polarity1(&self) -> &POLARITY1 {
        &self.polarity1
    }
    #[doc = "0x28 - Address Timing Register 2"]
    #[inline(always)]
    pub const fn addrtiming2(&self) -> &ADDRTIMING2 {
        &self.addrtiming2
    }
    #[doc = "0x2c - Read Timing Register 2"]
    #[inline(always)]
    pub const fn rdtiming2(&self) -> &RDTIMING2 {
        &self.rdtiming2
    }
    #[doc = "0x30 - Write Timing Register 2"]
    #[inline(always)]
    pub const fn wrtiming2(&self) -> &WRTIMING2 {
        &self.wrtiming2
    }
    #[doc = "0x34 - Polarity Register 2"]
    #[inline(always)]
    pub const fn polarity2(&self) -> &POLARITY2 {
        &self.polarity2
    }
    #[doc = "0x38 - Address Timing Register 3"]
    #[inline(always)]
    pub const fn addrtiming3(&self) -> &ADDRTIMING3 {
        &self.addrtiming3
    }
    #[doc = "0x3c - Read Timing Register 3"]
    #[inline(always)]
    pub const fn rdtiming3(&self) -> &RDTIMING3 {
        &self.rdtiming3
    }
    #[doc = "0x40 - Write Timing Register 3"]
    #[inline(always)]
    pub const fn wrtiming3(&self) -> &WRTIMING3 {
        &self.wrtiming3
    }
    #[doc = "0x44 - Polarity Register 3"]
    #[inline(always)]
    pub const fn polarity3(&self) -> &POLARITY3 {
        &self.polarity3
    }
    #[doc = "0x48 - Page Control Register"]
    #[inline(always)]
    pub const fn pagectrl(&self) -> &PAGECTRL {
        &self.pagectrl
    }
    #[doc = "0x4c - NAND Control Register"]
    #[inline(always)]
    pub const fn nandctrl(&self) -> &NANDCTRL {
        &self.nandctrl
    }
    #[doc = "0x50 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x54 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x58 - ECC Parity Register"]
    #[inline(always)]
    pub const fn eccparity(&self) -> &ECCPARITY {
        &self.eccparity
    }
    #[doc = "0x5c - TFT Control Register"]
    #[inline(always)]
    pub const fn tftctrl(&self) -> &TFTCTRL {
        &self.tftctrl
    }
    #[doc = "0x60 - TFT Status Register"]
    #[inline(always)]
    pub const fn tftstatus(&self) -> &TFTSTATUS {
        &self.tftstatus
    }
    #[doc = "0x64 - Color Format Register"]
    #[inline(always)]
    pub const fn tftcolorformat(&self) -> &TFTCOLORFORMAT {
        &self.tftcolorformat
    }
    #[doc = "0x68 - TFT Frame Base Register"]
    #[inline(always)]
    pub const fn tftframebase(&self) -> &TFTFRAMEBASE {
        &self.tftframebase
    }
    #[doc = "0x70 - TFT Stride Register"]
    #[inline(always)]
    pub const fn tftstride(&self) -> &TFTSTRIDE {
        &self.tftstride
    }
    #[doc = "0x74 - TFT Size Register"]
    #[inline(always)]
    pub const fn tftsize(&self) -> &TFTSIZE {
        &self.tftsize
    }
    #[doc = "0x78 - TFT Horizontal Porch Register"]
    #[inline(always)]
    pub const fn tfthporch(&self) -> &TFTHPORCH {
        &self.tfthporch
    }
    #[doc = "0x7c - TFT Vertical Porch Register"]
    #[inline(always)]
    pub const fn tftvporch(&self) -> &TFTVPORCH {
        &self.tftvporch
    }
    #[doc = "0x80 - TFT Timing Register"]
    #[inline(always)]
    pub const fn tfttiming(&self) -> &TFTTIMING {
        &self.tfttiming
    }
    #[doc = "0x84 - TFT Polarity Register"]
    #[inline(always)]
    pub const fn tftpolarity(&self) -> &TFTPOLARITY {
        &self.tftpolarity
    }
    #[doc = "0x88 - TFT Direct Drive Data Register"]
    #[inline(always)]
    pub const fn tftdd(&self) -> &TFTDD {
        &self.tftdd
    }
    #[doc = "0x8c - TFT Alpha Blending Register"]
    #[inline(always)]
    pub const fn tftalpha(&self) -> &TFTALPHA {
        &self.tftalpha
    }
    #[doc = "0x90 - TFT Pixel 0 Register"]
    #[inline(always)]
    pub const fn tftpixel0(&self) -> &TFTPIXEL0 {
        &self.tftpixel0
    }
    #[doc = "0x94 - TFT Pixel 1 Register"]
    #[inline(always)]
    pub const fn tftpixel1(&self) -> &TFTPIXEL1 {
        &self.tftpixel1
    }
    #[doc = "0x98 - TFT Alpha Blending Result Pixel Register"]
    #[inline(always)]
    pub const fn tftpixel(&self) -> &TFTPIXEL {
        &self.tftpixel
    }
    #[doc = "0x9c - TFT Masking Register"]
    #[inline(always)]
    pub const fn tftmask(&self) -> &TFTMASK {
        &self.tftmask
    }
    #[doc = "0xa0 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0xac - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0xb0 - I/O Routing Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0xb4 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0xb8 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &ROUTELOC1 {
        &self.routeloc1
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ADDRTIMING (rw) register accessor: Address Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrtiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrtiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming`]
module"]
pub type ADDRTIMING = crate::Reg<addrtiming::ADDRTIMING_SPEC>;
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "RDTIMING (rw) register accessor: Read Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdtiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming`]
module"]
pub type RDTIMING = crate::Reg<rdtiming::RDTIMING_SPEC>;
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "WRTIMING (rw) register accessor: Write Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming`]
module"]
pub type WRTIMING = crate::Reg<wrtiming::WRTIMING_SPEC>;
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "POLARITY (rw) register accessor: Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polarity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polarity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity`]
module"]
pub type POLARITY = crate::Reg<polarity::POLARITY_SPEC>;
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "ADDRTIMING1 (rw) register accessor: Address Timing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrtiming1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrtiming1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming1`]
module"]
pub type ADDRTIMING1 = crate::Reg<addrtiming1::ADDRTIMING1_SPEC>;
#[doc = "Address Timing Register 1"]
pub mod addrtiming1;
#[doc = "RDTIMING1 (rw) register accessor: Read Timing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtiming1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdtiming1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming1`]
module"]
pub type RDTIMING1 = crate::Reg<rdtiming1::RDTIMING1_SPEC>;
#[doc = "Read Timing Register 1"]
pub mod rdtiming1;
#[doc = "WRTIMING1 (rw) register accessor: Write Timing Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtiming1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtiming1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming1`]
module"]
pub type WRTIMING1 = crate::Reg<wrtiming1::WRTIMING1_SPEC>;
#[doc = "Write Timing Register 1"]
pub mod wrtiming1;
#[doc = "POLARITY1 (rw) register accessor: Polarity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polarity1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polarity1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity1`]
module"]
pub type POLARITY1 = crate::Reg<polarity1::POLARITY1_SPEC>;
#[doc = "Polarity Register 1"]
pub mod polarity1;
#[doc = "ADDRTIMING2 (rw) register accessor: Address Timing Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrtiming2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrtiming2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming2`]
module"]
pub type ADDRTIMING2 = crate::Reg<addrtiming2::ADDRTIMING2_SPEC>;
#[doc = "Address Timing Register 2"]
pub mod addrtiming2;
#[doc = "RDTIMING2 (rw) register accessor: Read Timing Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtiming2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdtiming2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming2`]
module"]
pub type RDTIMING2 = crate::Reg<rdtiming2::RDTIMING2_SPEC>;
#[doc = "Read Timing Register 2"]
pub mod rdtiming2;
#[doc = "WRTIMING2 (rw) register accessor: Write Timing Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtiming2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtiming2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming2`]
module"]
pub type WRTIMING2 = crate::Reg<wrtiming2::WRTIMING2_SPEC>;
#[doc = "Write Timing Register 2"]
pub mod wrtiming2;
#[doc = "POLARITY2 (rw) register accessor: Polarity Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polarity2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polarity2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity2`]
module"]
pub type POLARITY2 = crate::Reg<polarity2::POLARITY2_SPEC>;
#[doc = "Polarity Register 2"]
pub mod polarity2;
#[doc = "ADDRTIMING3 (rw) register accessor: Address Timing Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrtiming3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrtiming3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrtiming3`]
module"]
pub type ADDRTIMING3 = crate::Reg<addrtiming3::ADDRTIMING3_SPEC>;
#[doc = "Address Timing Register 3"]
pub mod addrtiming3;
#[doc = "RDTIMING3 (rw) register accessor: Read Timing Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtiming3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdtiming3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtiming3`]
module"]
pub type RDTIMING3 = crate::Reg<rdtiming3::RDTIMING3_SPEC>;
#[doc = "Read Timing Register 3"]
pub mod rdtiming3;
#[doc = "WRTIMING3 (rw) register accessor: Write Timing Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtiming3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtiming3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtiming3`]
module"]
pub type WRTIMING3 = crate::Reg<wrtiming3::WRTIMING3_SPEC>;
#[doc = "Write Timing Register 3"]
pub mod wrtiming3;
#[doc = "POLARITY3 (rw) register accessor: Polarity Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polarity3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polarity3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polarity3`]
module"]
pub type POLARITY3 = crate::Reg<polarity3::POLARITY3_SPEC>;
#[doc = "Polarity Register 3"]
pub mod polarity3;
#[doc = "PAGECTRL (rw) register accessor: Page Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pagectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pagectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pagectrl`]
module"]
pub type PAGECTRL = crate::Reg<pagectrl::PAGECTRL_SPEC>;
#[doc = "Page Control Register"]
pub mod pagectrl;
#[doc = "NANDCTRL (rw) register accessor: NAND Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nandctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nandctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nandctrl`]
module"]
pub type NANDCTRL = crate::Reg<nandctrl::NANDCTRL_SPEC>;
#[doc = "NAND Control Register"]
pub mod nandctrl;
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
#[doc = "ECCPARITY (r) register accessor: ECC Parity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccparity::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccparity`]
module"]
pub type ECCPARITY = crate::Reg<eccparity::ECCPARITY_SPEC>;
#[doc = "ECC Parity Register"]
pub mod eccparity;
#[doc = "TFTCTRL (rw) register accessor: TFT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftctrl`]
module"]
pub type TFTCTRL = crate::Reg<tftctrl::TFTCTRL_SPEC>;
#[doc = "TFT Control Register"]
pub mod tftctrl;
#[doc = "TFTSTATUS (r) register accessor: TFT Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftstatus`]
module"]
pub type TFTSTATUS = crate::Reg<tftstatus::TFTSTATUS_SPEC>;
#[doc = "TFT Status Register"]
pub mod tftstatus;
#[doc = "TFTCOLORFORMAT (rw) register accessor: Color Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftcolorformat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftcolorformat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftcolorformat`]
module"]
pub type TFTCOLORFORMAT = crate::Reg<tftcolorformat::TFTCOLORFORMAT_SPEC>;
#[doc = "Color Format Register"]
pub mod tftcolorformat;
#[doc = "TFTFRAMEBASE (rw) register accessor: TFT Frame Base Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftframebase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftframebase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftframebase`]
module"]
pub type TFTFRAMEBASE = crate::Reg<tftframebase::TFTFRAMEBASE_SPEC>;
#[doc = "TFT Frame Base Register"]
pub mod tftframebase;
#[doc = "TFTSTRIDE (rw) register accessor: TFT Stride Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftstride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftstride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftstride`]
module"]
pub type TFTSTRIDE = crate::Reg<tftstride::TFTSTRIDE_SPEC>;
#[doc = "TFT Stride Register"]
pub mod tftstride;
#[doc = "TFTSIZE (rw) register accessor: TFT Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftsize`]
module"]
pub type TFTSIZE = crate::Reg<tftsize::TFTSIZE_SPEC>;
#[doc = "TFT Size Register"]
pub mod tftsize;
#[doc = "TFTHPORCH (rw) register accessor: TFT Horizontal Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfthporch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfthporch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfthporch`]
module"]
pub type TFTHPORCH = crate::Reg<tfthporch::TFTHPORCH_SPEC>;
#[doc = "TFT Horizontal Porch Register"]
pub mod tfthporch;
#[doc = "TFTVPORCH (rw) register accessor: TFT Vertical Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftvporch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftvporch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftvporch`]
module"]
pub type TFTVPORCH = crate::Reg<tftvporch::TFTVPORCH_SPEC>;
#[doc = "TFT Vertical Porch Register"]
pub mod tftvporch;
#[doc = "TFTTIMING (rw) register accessor: TFT Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfttiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfttiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfttiming`]
module"]
pub type TFTTIMING = crate::Reg<tfttiming::TFTTIMING_SPEC>;
#[doc = "TFT Timing Register"]
pub mod tfttiming;
#[doc = "TFTPOLARITY (rw) register accessor: TFT Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpolarity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftpolarity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpolarity`]
module"]
pub type TFTPOLARITY = crate::Reg<tftpolarity::TFTPOLARITY_SPEC>;
#[doc = "TFT Polarity Register"]
pub mod tftpolarity;
#[doc = "TFTDD (rw) register accessor: TFT Direct Drive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftdd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftdd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftdd`]
module"]
pub type TFTDD = crate::Reg<tftdd::TFTDD_SPEC>;
#[doc = "TFT Direct Drive Data Register"]
pub mod tftdd;
#[doc = "TFTALPHA (rw) register accessor: TFT Alpha Blending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftalpha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftalpha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftalpha`]
module"]
pub type TFTALPHA = crate::Reg<tftalpha::TFTALPHA_SPEC>;
#[doc = "TFT Alpha Blending Register"]
pub mod tftalpha;
#[doc = "TFTPIXEL0 (rw) register accessor: TFT Pixel 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpixel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftpixel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpixel0`]
module"]
pub type TFTPIXEL0 = crate::Reg<tftpixel0::TFTPIXEL0_SPEC>;
#[doc = "TFT Pixel 0 Register"]
pub mod tftpixel0;
#[doc = "TFTPIXEL1 (rw) register accessor: TFT Pixel 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpixel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftpixel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpixel1`]
module"]
pub type TFTPIXEL1 = crate::Reg<tftpixel1::TFTPIXEL1_SPEC>;
#[doc = "TFT Pixel 1 Register"]
pub mod tftpixel1;
#[doc = "TFTPIXEL (r) register accessor: TFT Alpha Blending Result Pixel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpixel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftpixel`]
module"]
pub type TFTPIXEL = crate::Reg<tftpixel::TFTPIXEL_SPEC>;
#[doc = "TFT Alpha Blending Result Pixel Register"]
pub mod tftpixel;
#[doc = "TFTMASK (rw) register accessor: TFT Masking Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tftmask`]
module"]
pub type TFTMASK = crate::Reg<tftmask::TFTMASK_SPEC>;
#[doc = "TFT Masking Register"]
pub mod tftmask;
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
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`]
module"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
