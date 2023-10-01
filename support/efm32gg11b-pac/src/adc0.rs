#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Single Channel Control Register"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x14 - Single Channel Control Register Continued"]
    pub singlectrlx: SINGLECTRLX,
    #[doc = "0x18 - Scan Control Register"]
    pub scanctrl: SCANCTRL,
    #[doc = "0x1c - Scan Control Register Continued"]
    pub scanctrlx: SCANCTRLX,
    #[doc = "0x20 - Scan Sequence Input Mask Register"]
    pub scanmask: SCANMASK,
    #[doc = "0x24 - Input Selection Register for Scan Mode"]
    pub scaninputsel: SCANINPUTSEL,
    #[doc = "0x28 - Negative Input Select Register for Scan"]
    pub scannegsel: SCANNEGSEL,
    #[doc = "0x2c - Compare Threshold Register"]
    pub cmpthr: CMPTHR,
    #[doc = "0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
    pub biasprog: BIASPROG,
    #[doc = "0x34 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x3c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x40 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x48 - Single Conversion Result Data"]
    pub singledata: SINGLEDATA,
    #[doc = "0x4c - Scan Conversion Result Data"]
    pub scandata: SCANDATA,
    #[doc = "0x50 - Single Conversion Result Data Peek Register"]
    pub singledatap: SINGLEDATAP,
    #[doc = "0x54 - Scan Sequence Result Data Peek Register"]
    pub scandatap: SCANDATAP,
    _reserved21: [u8; 0x10],
    #[doc = "0x68 - Scan Sequence Result Data + Data Source Register"]
    pub scandatax: SCANDATAX,
    #[doc = "0x6c - Scan Sequence Result Data + Data Source Peek Register"]
    pub scandataxp: SCANDATAXP,
    _reserved23: [u8; 0x0c],
    #[doc = "0x7c - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x80 - APORT Conflict Status Register"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x84 - Single FIFO Count Register"]
    pub singlefifocount: SINGLEFIFOCOUNT,
    #[doc = "0x88 - Scan FIFO Count Register"]
    pub scanfifocount: SCANFIFOCOUNT,
    #[doc = "0x8c - Single FIFO Clear Register"]
    pub singlefifoclear: SINGLEFIFOCLEAR,
    #[doc = "0x90 - Scan FIFO Clear Register"]
    pub scanfifoclear: SCANFIFOCLEAR,
    #[doc = "0x94 - APORT Bus Master Disable Register"]
    pub aportmasterdis: APORTMASTERDIS,
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SINGLECTRL (rw) register accessor: Single Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`singlectrl`] module"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Channel Control Register"]
pub mod singlectrl;
#[doc = "SINGLECTRLX (rw) register accessor: Single Channel Control Register Continued\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlectrlx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlectrlx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`singlectrlx`] module"]
pub type SINGLECTRLX = crate::Reg<singlectrlx::SINGLECTRLX_SPEC>;
#[doc = "Single Channel Control Register Continued"]
pub mod singlectrlx;
#[doc = "SCANCTRL (rw) register accessor: Scan Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scanctrl`] module"]
pub type SCANCTRL = crate::Reg<scanctrl::SCANCTRL_SPEC>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "SCANCTRLX (rw) register accessor: Scan Control Register Continued\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanctrlx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanctrlx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scanctrlx`] module"]
pub type SCANCTRLX = crate::Reg<scanctrlx::SCANCTRLX_SPEC>;
#[doc = "Scan Control Register Continued"]
pub mod scanctrlx;
#[doc = "SCANMASK (rw) register accessor: Scan Sequence Input Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scanmask`] module"]
pub type SCANMASK = crate::Reg<scanmask::SCANMASK_SPEC>;
#[doc = "Scan Sequence Input Mask Register"]
pub mod scanmask;
#[doc = "SCANINPUTSEL (rw) register accessor: Input Selection Register for Scan Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scaninputsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scaninputsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scaninputsel`] module"]
pub type SCANINPUTSEL = crate::Reg<scaninputsel::SCANINPUTSEL_SPEC>;
#[doc = "Input Selection Register for Scan Mode"]
pub mod scaninputsel;
#[doc = "SCANNEGSEL (rw) register accessor: Negative Input Select Register for Scan\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scannegsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scannegsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scannegsel`] module"]
pub type SCANNEGSEL = crate::Reg<scannegsel::SCANNEGSEL_SPEC>;
#[doc = "Negative Input Select Register for Scan"]
pub mod scannegsel;
#[doc = "CMPTHR (rw) register accessor: Compare Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmpthr`] module"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "Compare Threshold Register"]
pub mod cmpthr;
#[doc = "BIASPROG (rw) register accessor: Bias Programming Register for Various Analog Blocks Used in ADC Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasprog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasprog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`biasprog`] module"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub mod biasprog;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cal`] module"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
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
#[doc = "SINGLEDATA (r) register accessor: Single Conversion Result Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singledata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`singledata`] module"]
pub type SINGLEDATA = crate::Reg<singledata::SINGLEDATA_SPEC>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA (r) register accessor: Scan Conversion Result Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandata::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scandata`] module"]
pub type SCANDATA = crate::Reg<scandata::SCANDATA_SPEC>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP (r) register accessor: Single Conversion Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singledatap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`singledatap`] module"]
pub type SINGLEDATAP = crate::Reg<singledatap::SINGLEDATAP_SPEC>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP (r) register accessor: Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandatap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scandatap`] module"]
pub type SCANDATAP = crate::Reg<scandatap::SCANDATAP_SPEC>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "SCANDATAX (r) register accessor: Scan Sequence Result Data + Data Source Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandatax::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scandatax`] module"]
pub type SCANDATAX = crate::Reg<scandatax::SCANDATAX_SPEC>;
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub mod scandatax;
#[doc = "SCANDATAXP (r) register accessor: Scan Sequence Result Data + Data Source Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandataxp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scandataxp`] module"]
pub type SCANDATAXP = crate::Reg<scandataxp::SCANDATAXP_SPEC>;
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub mod scandataxp;
#[doc = "APORTREQ (r) register accessor: APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aportreq`] module"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT (r) register accessor: APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aportconflict`] module"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "SINGLEFIFOCOUNT (r) register accessor: Single FIFO Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlefifocount::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`singlefifocount`] module"]
pub type SINGLEFIFOCOUNT = crate::Reg<singlefifocount::SINGLEFIFOCOUNT_SPEC>;
#[doc = "Single FIFO Count Register"]
pub mod singlefifocount;
#[doc = "SCANFIFOCOUNT (r) register accessor: Scan FIFO Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanfifocount::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scanfifocount`] module"]
pub type SCANFIFOCOUNT = crate::Reg<scanfifocount::SCANFIFOCOUNT_SPEC>;
#[doc = "Scan FIFO Count Register"]
pub mod scanfifocount;
#[doc = "SINGLEFIFOCLEAR (w) register accessor: Single FIFO Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlefifoclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`singlefifoclear`] module"]
pub type SINGLEFIFOCLEAR = crate::Reg<singlefifoclear::SINGLEFIFOCLEAR_SPEC>;
#[doc = "Single FIFO Clear Register"]
pub mod singlefifoclear;
#[doc = "SCANFIFOCLEAR (w) register accessor: Scan FIFO Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanfifoclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scanfifoclear`] module"]
pub type SCANFIFOCLEAR = crate::Reg<scanfifoclear::SCANFIFOCLEAR_SPEC>;
#[doc = "Scan FIFO Clear Register"]
pub mod scanfifoclear;
#[doc = "APORTMASTERDIS (rw) register accessor: APORT Bus Master Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportmasterdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aportmasterdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aportmasterdis`] module"]
pub type APORTMASTERDIS = crate::Reg<aportmasterdis::APORTMASTERDIS_SPEC>;
#[doc = "APORT Bus Master Disable Register"]
pub mod aportmasterdis;
