#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x04],
    cmd: Cmd,
    status: Status,
    singlectrl: Singlectrl,
    singlectrlx: Singlectrlx,
    scanctrl: Scanctrl,
    scanctrlx: Scanctrlx,
    scanmask: Scanmask,
    scaninputsel: Scaninputsel,
    scannegsel: Scannegsel,
    cmpthr: Cmpthr,
    biasprog: Biasprog,
    cal: Cal,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    singledata: Singledata,
    scandata: Scandata,
    singledatap: Singledatap,
    scandatap: Scandatap,
    _reserved21: [u8; 0x10],
    scandatax: Scandatax,
    scandataxp: Scandataxp,
    _reserved23: [u8; 0x0c],
    aportreq: Aportreq,
    aportconflict: Aportconflict,
    singlefifocount: Singlefifocount,
    scanfifocount: Scanfifocount,
    singlefifoclear: Singlefifoclear,
    scanfifoclear: Scanfifoclear,
    aportmasterdis: Aportmasterdis,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Single Channel Control Register"]
    #[inline(always)]
    pub const fn singlectrl(&self) -> &Singlectrl {
        &self.singlectrl
    }
    #[doc = "0x14 - Single Channel Control Register Continued"]
    #[inline(always)]
    pub const fn singlectrlx(&self) -> &Singlectrlx {
        &self.singlectrlx
    }
    #[doc = "0x18 - Scan Control Register"]
    #[inline(always)]
    pub const fn scanctrl(&self) -> &Scanctrl {
        &self.scanctrl
    }
    #[doc = "0x1c - Scan Control Register Continued"]
    #[inline(always)]
    pub const fn scanctrlx(&self) -> &Scanctrlx {
        &self.scanctrlx
    }
    #[doc = "0x20 - Scan Sequence Input Mask Register"]
    #[inline(always)]
    pub const fn scanmask(&self) -> &Scanmask {
        &self.scanmask
    }
    #[doc = "0x24 - Input Selection Register for Scan Mode"]
    #[inline(always)]
    pub const fn scaninputsel(&self) -> &Scaninputsel {
        &self.scaninputsel
    }
    #[doc = "0x28 - Negative Input Select Register for Scan"]
    #[inline(always)]
    pub const fn scannegsel(&self) -> &Scannegsel {
        &self.scannegsel
    }
    #[doc = "0x2c - Compare Threshold Register"]
    #[inline(always)]
    pub const fn cmpthr(&self) -> &Cmpthr {
        &self.cmpthr
    }
    #[doc = "0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
    #[inline(always)]
    pub const fn biasprog(&self) -> &Biasprog {
        &self.biasprog
    }
    #[doc = "0x34 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0x38 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x3c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x40 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x44 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x48 - Single Conversion Result Data"]
    #[inline(always)]
    pub const fn singledata(&self) -> &Singledata {
        &self.singledata
    }
    #[doc = "0x4c - Scan Conversion Result Data"]
    #[inline(always)]
    pub const fn scandata(&self) -> &Scandata {
        &self.scandata
    }
    #[doc = "0x50 - Single Conversion Result Data Peek Register"]
    #[inline(always)]
    pub const fn singledatap(&self) -> &Singledatap {
        &self.singledatap
    }
    #[doc = "0x54 - Scan Sequence Result Data Peek Register"]
    #[inline(always)]
    pub const fn scandatap(&self) -> &Scandatap {
        &self.scandatap
    }
    #[doc = "0x68 - Scan Sequence Result Data + Data Source Register"]
    #[inline(always)]
    pub const fn scandatax(&self) -> &Scandatax {
        &self.scandatax
    }
    #[doc = "0x6c - Scan Sequence Result Data + Data Source Peek Register"]
    #[inline(always)]
    pub const fn scandataxp(&self) -> &Scandataxp {
        &self.scandataxp
    }
    #[doc = "0x7c - APORT Request Status Register"]
    #[inline(always)]
    pub const fn aportreq(&self) -> &Aportreq {
        &self.aportreq
    }
    #[doc = "0x80 - APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn aportconflict(&self) -> &Aportconflict {
        &self.aportconflict
    }
    #[doc = "0x84 - Single FIFO Count Register"]
    #[inline(always)]
    pub const fn singlefifocount(&self) -> &Singlefifocount {
        &self.singlefifocount
    }
    #[doc = "0x88 - Scan FIFO Count Register"]
    #[inline(always)]
    pub const fn scanfifocount(&self) -> &Scanfifocount {
        &self.scanfifocount
    }
    #[doc = "0x8c - Single FIFO Clear Register"]
    #[inline(always)]
    pub const fn singlefifoclear(&self) -> &Singlefifoclear {
        &self.singlefifoclear
    }
    #[doc = "0x90 - Scan FIFO Clear Register"]
    #[inline(always)]
    pub const fn scanfifoclear(&self) -> &Scanfifoclear {
        &self.scanfifoclear
    }
    #[doc = "0x94 - APORT Bus Master Disable Register"]
    #[inline(always)]
    pub const fn aportmasterdis(&self) -> &Aportmasterdis {
        &self.aportmasterdis
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
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
#[doc = "SINGLECTRL (rw) register accessor: Single Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlectrl`] module"]
#[doc(alias = "SINGLECTRL")]
pub type Singlectrl = crate::Reg<singlectrl::SinglectrlSpec>;
#[doc = "Single Channel Control Register"]
pub mod singlectrl;
#[doc = "SINGLECTRLX (rw) register accessor: Single Channel Control Register Continued\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlectrlx`] module"]
#[doc(alias = "SINGLECTRLX")]
pub type Singlectrlx = crate::Reg<singlectrlx::SinglectrlxSpec>;
#[doc = "Single Channel Control Register Continued"]
pub mod singlectrlx;
#[doc = "SCANCTRL (rw) register accessor: Scan Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanctrl`] module"]
#[doc(alias = "SCANCTRL")]
pub type Scanctrl = crate::Reg<scanctrl::ScanctrlSpec>;
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "SCANCTRLX (rw) register accessor: Scan Control Register Continued\n\nYou can [`read`](crate::Reg::read) this register and get [`scanctrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanctrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanctrlx`] module"]
#[doc(alias = "SCANCTRLX")]
pub type Scanctrlx = crate::Reg<scanctrlx::ScanctrlxSpec>;
#[doc = "Scan Control Register Continued"]
pub mod scanctrlx;
#[doc = "SCANMASK (rw) register accessor: Scan Sequence Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanmask`] module"]
#[doc(alias = "SCANMASK")]
pub type Scanmask = crate::Reg<scanmask::ScanmaskSpec>;
#[doc = "Scan Sequence Input Mask Register"]
pub mod scanmask;
#[doc = "SCANINPUTSEL (rw) register accessor: Input Selection Register for Scan Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`scaninputsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scaninputsel`] module"]
#[doc(alias = "SCANINPUTSEL")]
pub type Scaninputsel = crate::Reg<scaninputsel::ScaninputselSpec>;
#[doc = "Input Selection Register for Scan Mode"]
pub mod scaninputsel;
#[doc = "SCANNEGSEL (rw) register accessor: Negative Input Select Register for Scan\n\nYou can [`read`](crate::Reg::read) this register and get [`scannegsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scannegsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scannegsel`] module"]
#[doc(alias = "SCANNEGSEL")]
pub type Scannegsel = crate::Reg<scannegsel::ScannegselSpec>;
#[doc = "Negative Input Select Register for Scan"]
pub mod scannegsel;
#[doc = "CMPTHR (rw) register accessor: Compare Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpthr`] module"]
#[doc(alias = "CMPTHR")]
pub type Cmpthr = crate::Reg<cmpthr::CmpthrSpec>;
#[doc = "Compare Threshold Register"]
pub mod cmpthr;
#[doc = "BIASPROG (rw) register accessor: Bias Programming Register for Various Analog Blocks Used in ADC Operation\n\nYou can [`read`](crate::Reg::read) this register and get [`biasprog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasprog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasprog`] module"]
#[doc(alias = "BIASPROG")]
pub type Biasprog = crate::Reg<biasprog::BiasprogSpec>;
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub mod biasprog;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "Calibration Register"]
pub mod cal;
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
#[doc = "SINGLEDATA (r) register accessor: Single Conversion Result Data\n\nYou can [`read`](crate::Reg::read) this register and get [`singledata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@singledata`] module"]
#[doc(alias = "SINGLEDATA")]
pub type Singledata = crate::Reg<singledata::SingledataSpec>;
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "SCANDATA (r) register accessor: Scan Conversion Result Data\n\nYou can [`read`](crate::Reg::read) this register and get [`scandata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@scandata`] module"]
#[doc(alias = "SCANDATA")]
pub type Scandata = crate::Reg<scandata::ScandataSpec>;
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "SINGLEDATAP (r) register accessor: Single Conversion Result Data Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singledatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singledatap`] module"]
#[doc(alias = "SINGLEDATAP")]
pub type Singledatap = crate::Reg<singledatap::SingledatapSpec>;
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "SCANDATAP (r) register accessor: Scan Sequence Result Data Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scandatap`] module"]
#[doc(alias = "SCANDATAP")]
pub type Scandatap = crate::Reg<scandatap::ScandatapSpec>;
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "SCANDATAX (r) register accessor: Scan Sequence Result Data + Data Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@scandatax`] module"]
#[doc(alias = "SCANDATAX")]
pub type Scandatax = crate::Reg<scandatax::ScandataxSpec>;
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub mod scandatax;
#[doc = "SCANDATAXP (r) register accessor: Scan Sequence Result Data + Data Source Peek Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scandataxp`] module"]
#[doc(alias = "SCANDATAXP")]
pub type Scandataxp = crate::Reg<scandataxp::ScandataxpSpec>;
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub mod scandataxp;
#[doc = "APORTREQ (r) register accessor: APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportreq`] module"]
#[doc(alias = "APORTREQ")]
pub type Aportreq = crate::Reg<aportreq::AportreqSpec>;
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORTCONFLICT (r) register accessor: APORT Conflict Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportconflict`] module"]
#[doc(alias = "APORTCONFLICT")]
pub type Aportconflict = crate::Reg<aportconflict::AportconflictSpec>;
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "SINGLEFIFOCOUNT (r) register accessor: Single FIFO Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifocount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlefifocount`] module"]
#[doc(alias = "SINGLEFIFOCOUNT")]
pub type Singlefifocount = crate::Reg<singlefifocount::SinglefifocountSpec>;
#[doc = "Single FIFO Count Register"]
pub mod singlefifocount;
#[doc = "SCANFIFOCOUNT (r) register accessor: Scan FIFO Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanfifocount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanfifocount`] module"]
#[doc(alias = "SCANFIFOCOUNT")]
pub type Scanfifocount = crate::Reg<scanfifocount::ScanfifocountSpec>;
#[doc = "Scan FIFO Count Register"]
pub mod scanfifocount;
#[doc = "SINGLEFIFOCLEAR (w) register accessor: Single FIFO Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlefifoclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlefifoclear`] module"]
#[doc(alias = "SINGLEFIFOCLEAR")]
pub type Singlefifoclear = crate::Reg<singlefifoclear::SinglefifoclearSpec>;
#[doc = "Single FIFO Clear Register"]
pub mod singlefifoclear;
#[doc = "SCANFIFOCLEAR (w) register accessor: Scan FIFO Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanfifoclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanfifoclear`] module"]
#[doc(alias = "SCANFIFOCLEAR")]
pub type Scanfifoclear = crate::Reg<scanfifoclear::ScanfifoclearSpec>;
#[doc = "Scan FIFO Clear Register"]
pub mod scanfifoclear;
#[doc = "APORTMASTERDIS (rw) register accessor: APORT Bus Master Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportmasterdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aportmasterdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportmasterdis`] module"]
#[doc(alias = "APORTMASTERDIS")]
pub type Aportmasterdis = crate::Reg<aportmasterdis::AportmasterdisSpec>;
#[doc = "APORT Bus Master Disable Register"]
pub mod aportmasterdis;
