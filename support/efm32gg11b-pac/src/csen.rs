#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    timctrl: Timctrl,
    cmd: Cmd,
    status: Status,
    prssel: Prssel,
    data: Data,
    scanmask0: Scanmask0,
    scaninputsel0: Scaninputsel0,
    scanmask1: Scanmask1,
    scaninputsel1: Scaninputsel1,
    aportreq: Aportreq,
    aportconflict: Aportconflict,
    cmpthr: Cmpthr,
    ema: Ema,
    emactrl: Emactrl,
    singlectrl: Singlectrl,
    dmbaseline: Dmbaseline,
    dmcfg: Dmcfg,
    anactrl: Anactrl,
    _reserved19: [u8; 0x08],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Timing Control"]
    #[inline(always)]
    pub const fn timctrl(&self) -> &Timctrl {
        &self.timctrl
    }
    #[doc = "0x08 - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x0c - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - PRS Select"]
    #[inline(always)]
    pub const fn prssel(&self) -> &Prssel {
        &self.prssel
    }
    #[doc = "0x14 - Output Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x18 - Scan Channel Mask 0"]
    #[inline(always)]
    pub const fn scanmask0(&self) -> &Scanmask0 {
        &self.scanmask0
    }
    #[doc = "0x1c - Scan Input Selection 0"]
    #[inline(always)]
    pub const fn scaninputsel0(&self) -> &Scaninputsel0 {
        &self.scaninputsel0
    }
    #[doc = "0x20 - Scan Channel Mask 1"]
    #[inline(always)]
    pub const fn scanmask1(&self) -> &Scanmask1 {
        &self.scanmask1
    }
    #[doc = "0x24 - Scan Input Selection 1"]
    #[inline(always)]
    pub const fn scaninputsel1(&self) -> &Scaninputsel1 {
        &self.scaninputsel1
    }
    #[doc = "0x28 - APORT Request Status"]
    #[inline(always)]
    pub const fn aportreq(&self) -> &Aportreq {
        &self.aportreq
    }
    #[doc = "0x2c - APORT Request Conflict"]
    #[inline(always)]
    pub const fn aportconflict(&self) -> &Aportconflict {
        &self.aportconflict
    }
    #[doc = "0x30 - Comparator Threshold"]
    #[inline(always)]
    pub const fn cmpthr(&self) -> &Cmpthr {
        &self.cmpthr
    }
    #[doc = "0x34 - Exponential Moving Average"]
    #[inline(always)]
    pub const fn ema(&self) -> &Ema {
        &self.ema
    }
    #[doc = "0x38 - Exponential Moving Average Control"]
    #[inline(always)]
    pub const fn emactrl(&self) -> &Emactrl {
        &self.emactrl
    }
    #[doc = "0x3c - Single Conversion Control"]
    #[inline(always)]
    pub const fn singlectrl(&self) -> &Singlectrl {
        &self.singlectrl
    }
    #[doc = "0x40 - Delta Modulation Baseline"]
    #[inline(always)]
    pub const fn dmbaseline(&self) -> &Dmbaseline {
        &self.dmbaseline
    }
    #[doc = "0x44 - Delta Modulation Configuration"]
    #[inline(always)]
    pub const fn dmcfg(&self) -> &Dmcfg {
        &self.dmcfg
    }
    #[doc = "0x48 - Analog Control"]
    #[inline(always)]
    pub const fn anactrl(&self) -> &Anactrl {
        &self.anactrl
    }
    #[doc = "0x54 - Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x58 - Interrupt Flag Set"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x5c - Interrupt Flag Clear"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x60 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "TIMCTRL (rw) register accessor: Timing Control\n\nYou can [`read`](crate::Reg::read) this register and get [`timctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timctrl`] module"]
#[doc(alias = "TIMCTRL")]
pub type Timctrl = crate::Reg<timctrl::TimctrlSpec>;
#[doc = "Timing Control"]
pub mod timctrl;
#[doc = "CMD (w) register accessor: Command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "PRSSEL (rw) register accessor: PRS Select\n\nYou can [`read`](crate::Reg::read) this register and get [`prssel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prssel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prssel`] module"]
#[doc(alias = "PRSSEL")]
pub type Prssel = crate::Reg<prssel::PrsselSpec>;
#[doc = "PRS Select"]
pub mod prssel;
#[doc = "DATA (rw) register accessor: Output Data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Output Data"]
pub mod data;
#[doc = "SCANMASK0 (rw) register accessor: Scan Channel Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scanmask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanmask0`] module"]
#[doc(alias = "SCANMASK0")]
pub type Scanmask0 = crate::Reg<scanmask0::Scanmask0Spec>;
#[doc = "Scan Channel Mask 0"]
pub mod scanmask0;
#[doc = "SCANINPUTSEL0 (rw) register accessor: Scan Input Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scaninputsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scaninputsel0`] module"]
#[doc(alias = "SCANINPUTSEL0")]
pub type Scaninputsel0 = crate::Reg<scaninputsel0::Scaninputsel0Spec>;
#[doc = "Scan Input Selection 0"]
pub mod scaninputsel0;
#[doc = "SCANMASK1 (rw) register accessor: Scan Channel Mask 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scanmask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanmask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanmask1`] module"]
#[doc(alias = "SCANMASK1")]
pub type Scanmask1 = crate::Reg<scanmask1::Scanmask1Spec>;
#[doc = "Scan Channel Mask 1"]
pub mod scanmask1;
#[doc = "SCANINPUTSEL1 (rw) register accessor: Scan Input Selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scaninputsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scaninputsel1`] module"]
#[doc(alias = "SCANINPUTSEL1")]
pub type Scaninputsel1 = crate::Reg<scaninputsel1::Scaninputsel1Spec>;
#[doc = "Scan Input Selection 1"]
pub mod scaninputsel1;
#[doc = "APORTREQ (r) register accessor: APORT Request Status\n\nYou can [`read`](crate::Reg::read) this register and get [`aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportreq`] module"]
#[doc(alias = "APORTREQ")]
pub type Aportreq = crate::Reg<aportreq::AportreqSpec>;
#[doc = "APORT Request Status"]
pub mod aportreq;
#[doc = "APORTCONFLICT (r) register accessor: APORT Request Conflict\n\nYou can [`read`](crate::Reg::read) this register and get [`aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportconflict`] module"]
#[doc(alias = "APORTCONFLICT")]
pub type Aportconflict = crate::Reg<aportconflict::AportconflictSpec>;
#[doc = "APORT Request Conflict"]
pub mod aportconflict;
#[doc = "CMPTHR (rw) register accessor: Comparator Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpthr`] module"]
#[doc(alias = "CMPTHR")]
pub type Cmpthr = crate::Reg<cmpthr::CmpthrSpec>;
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "EMA (rw) register accessor: Exponential Moving Average\n\nYou can [`read`](crate::Reg::read) this register and get [`ema::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ema::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema`] module"]
#[doc(alias = "EMA")]
pub type Ema = crate::Reg<ema::EmaSpec>;
#[doc = "Exponential Moving Average"]
pub mod ema;
#[doc = "EMACTRL (rw) register accessor: Exponential Moving Average Control\n\nYou can [`read`](crate::Reg::read) this register and get [`emactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emactrl`] module"]
#[doc(alias = "EMACTRL")]
pub type Emactrl = crate::Reg<emactrl::EmactrlSpec>;
#[doc = "Exponential Moving Average Control"]
pub mod emactrl;
#[doc = "SINGLECTRL (rw) register accessor: Single Conversion Control\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlectrl`] module"]
#[doc(alias = "SINGLECTRL")]
pub type Singlectrl = crate::Reg<singlectrl::SinglectrlSpec>;
#[doc = "Single Conversion Control"]
pub mod singlectrl;
#[doc = "DMBASELINE (rw) register accessor: Delta Modulation Baseline\n\nYou can [`read`](crate::Reg::read) this register and get [`dmbaseline::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmbaseline::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmbaseline`] module"]
#[doc(alias = "DMBASELINE")]
pub type Dmbaseline = crate::Reg<dmbaseline::DmbaselineSpec>;
#[doc = "Delta Modulation Baseline"]
pub mod dmbaseline;
#[doc = "DMCFG (rw) register accessor: Delta Modulation Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcfg`] module"]
#[doc(alias = "DMCFG")]
pub type Dmcfg = crate::Reg<dmcfg::DmcfgSpec>;
#[doc = "Delta Modulation Configuration"]
pub mod dmcfg;
#[doc = "ANACTRL (rw) register accessor: Analog Control\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anactrl`] module"]
#[doc(alias = "ANACTRL")]
pub type Anactrl = crate::Reg<anactrl::AnactrlSpec>;
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "IF (r) register accessor: Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`] module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IfsSpec>;
#[doc = "Interrupt Flag Set"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`] module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IfcSpec>;
#[doc = "Interrupt Flag Clear"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable"]
pub mod ien;
