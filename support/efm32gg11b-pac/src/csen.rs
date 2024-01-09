#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    timctrl: TIMCTRL,
    cmd: CMD,
    status: STATUS,
    prssel: PRSSEL,
    data: DATA,
    scanmask0: SCANMASK0,
    scaninputsel0: SCANINPUTSEL0,
    scanmask1: SCANMASK1,
    scaninputsel1: SCANINPUTSEL1,
    aportreq: APORTREQ,
    aportconflict: APORTCONFLICT,
    cmpthr: CMPTHR,
    ema: EMA,
    emactrl: EMACTRL,
    singlectrl: SINGLECTRL,
    dmbaseline: DMBASELINE,
    dmcfg: DMCFG,
    anactrl: ANACTRL,
    _reserved19: [u8; 0x08],
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Timing Control"]
    #[inline(always)]
    pub const fn timctrl(&self) -> &TIMCTRL {
        &self.timctrl
    }
    #[doc = "0x08 - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x0c - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x10 - PRS Select"]
    #[inline(always)]
    pub const fn prssel(&self) -> &PRSSEL {
        &self.prssel
    }
    #[doc = "0x14 - Output Data"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x18 - Scan Channel Mask 0"]
    #[inline(always)]
    pub const fn scanmask0(&self) -> &SCANMASK0 {
        &self.scanmask0
    }
    #[doc = "0x1c - Scan Input Selection 0"]
    #[inline(always)]
    pub const fn scaninputsel0(&self) -> &SCANINPUTSEL0 {
        &self.scaninputsel0
    }
    #[doc = "0x20 - Scan Channel Mask 1"]
    #[inline(always)]
    pub const fn scanmask1(&self) -> &SCANMASK1 {
        &self.scanmask1
    }
    #[doc = "0x24 - Scan Input Selection 1"]
    #[inline(always)]
    pub const fn scaninputsel1(&self) -> &SCANINPUTSEL1 {
        &self.scaninputsel1
    }
    #[doc = "0x28 - APORT Request Status"]
    #[inline(always)]
    pub const fn aportreq(&self) -> &APORTREQ {
        &self.aportreq
    }
    #[doc = "0x2c - APORT Request Conflict"]
    #[inline(always)]
    pub const fn aportconflict(&self) -> &APORTCONFLICT {
        &self.aportconflict
    }
    #[doc = "0x30 - Comparator Threshold"]
    #[inline(always)]
    pub const fn cmpthr(&self) -> &CMPTHR {
        &self.cmpthr
    }
    #[doc = "0x34 - Exponential Moving Average"]
    #[inline(always)]
    pub const fn ema(&self) -> &EMA {
        &self.ema
    }
    #[doc = "0x38 - Exponential Moving Average Control"]
    #[inline(always)]
    pub const fn emactrl(&self) -> &EMACTRL {
        &self.emactrl
    }
    #[doc = "0x3c - Single Conversion Control"]
    #[inline(always)]
    pub const fn singlectrl(&self) -> &SINGLECTRL {
        &self.singlectrl
    }
    #[doc = "0x40 - Delta Modulation Baseline"]
    #[inline(always)]
    pub const fn dmbaseline(&self) -> &DMBASELINE {
        &self.dmbaseline
    }
    #[doc = "0x44 - Delta Modulation Configuration"]
    #[inline(always)]
    pub const fn dmcfg(&self) -> &DMCFG {
        &self.dmcfg
    }
    #[doc = "0x48 - Analog Control"]
    #[inline(always)]
    pub const fn anactrl(&self) -> &ANACTRL {
        &self.anactrl
    }
    #[doc = "0x54 - Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x58 - Interrupt Flag Set"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x5c - Interrupt Flag Clear"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x60 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "TIMCTRL (rw) register accessor: Timing Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timctrl`]
module"]
pub type TIMCTRL = crate::Reg<timctrl::TIMCTRL_SPEC>;
#[doc = "Timing Control"]
pub mod timctrl;
#[doc = "CMD (w) register accessor: Command\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "PRSSEL (rw) register accessor: PRS Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prssel`]
module"]
pub type PRSSEL = crate::Reg<prssel::PRSSEL_SPEC>;
#[doc = "PRS Select"]
pub mod prssel;
#[doc = "DATA (rw) register accessor: Output Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Output Data"]
pub mod data;
#[doc = "SCANMASK0 (rw) register accessor: Scan Channel Mask 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanmask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanmask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanmask0`]
module"]
pub type SCANMASK0 = crate::Reg<scanmask0::SCANMASK0_SPEC>;
#[doc = "Scan Channel Mask 0"]
pub mod scanmask0;
#[doc = "SCANINPUTSEL0 (rw) register accessor: Scan Input Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scaninputsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scaninputsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scaninputsel0`]
module"]
pub type SCANINPUTSEL0 = crate::Reg<scaninputsel0::SCANINPUTSEL0_SPEC>;
#[doc = "Scan Input Selection 0"]
pub mod scaninputsel0;
#[doc = "SCANMASK1 (rw) register accessor: Scan Channel Mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanmask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanmask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanmask1`]
module"]
pub type SCANMASK1 = crate::Reg<scanmask1::SCANMASK1_SPEC>;
#[doc = "Scan Channel Mask 1"]
pub mod scanmask1;
#[doc = "SCANINPUTSEL1 (rw) register accessor: Scan Input Selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scaninputsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scaninputsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scaninputsel1`]
module"]
pub type SCANINPUTSEL1 = crate::Reg<scaninputsel1::SCANINPUTSEL1_SPEC>;
#[doc = "Scan Input Selection 1"]
pub mod scaninputsel1;
#[doc = "APORTREQ (r) register accessor: APORT Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportreq`]
module"]
pub type APORTREQ = crate::Reg<aportreq::APORTREQ_SPEC>;
#[doc = "APORT Request Status"]
pub mod aportreq;
#[doc = "APORTCONFLICT (r) register accessor: APORT Request Conflict\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aportconflict`]
module"]
pub type APORTCONFLICT = crate::Reg<aportconflict::APORTCONFLICT_SPEC>;
#[doc = "APORT Request Conflict"]
pub mod aportconflict;
#[doc = "CMPTHR (rw) register accessor: Comparator Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpthr`]
module"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "EMA (rw) register accessor: Exponential Moving Average\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ema`]
module"]
pub type EMA = crate::Reg<ema::EMA_SPEC>;
#[doc = "Exponential Moving Average"]
pub mod ema;
#[doc = "EMACTRL (rw) register accessor: Exponential Moving Average Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emactrl`]
module"]
pub type EMACTRL = crate::Reg<emactrl::EMACTRL_SPEC>;
#[doc = "Exponential Moving Average Control"]
pub mod emactrl;
#[doc = "SINGLECTRL (rw) register accessor: Single Conversion Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlectrl`]
module"]
pub type SINGLECTRL = crate::Reg<singlectrl::SINGLECTRL_SPEC>;
#[doc = "Single Conversion Control"]
pub mod singlectrl;
#[doc = "DMBASELINE (rw) register accessor: Delta Modulation Baseline\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmbaseline::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmbaseline::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmbaseline`]
module"]
pub type DMBASELINE = crate::Reg<dmbaseline::DMBASELINE_SPEC>;
#[doc = "Delta Modulation Baseline"]
pub mod dmbaseline;
#[doc = "DMCFG (rw) register accessor: Delta Modulation Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmcfg`]
module"]
pub type DMCFG = crate::Reg<dmcfg::DMCFG_SPEC>;
#[doc = "Delta Modulation Configuration"]
pub mod dmcfg;
#[doc = "ANACTRL (rw) register accessor: Analog Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anactrl`]
module"]
pub type ANACTRL = crate::Reg<anactrl::ANACTRL_SPEC>;
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "IF (r) register accessor: Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ien;
