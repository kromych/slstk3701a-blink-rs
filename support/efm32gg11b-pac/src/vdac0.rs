#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    ch0ctrl: CH0CTRL,
    ch1ctrl: CH1CTRL,
    cmd: CMD,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    ch0data: CH0DATA,
    ch1data: CH1DATA,
    combdata: COMBDATA,
    cal: CAL,
    _reserved13: [u8; 0x6c],
    opa0_aportreq: OPA0_APORTREQ,
    opa0_aportconflict: OPA0_APORTCONFLICT,
    opa0_ctrl: OPA0_CTRL,
    opa0_timer: OPA0_TIMER,
    opa0_mux: OPA0_MUX,
    opa0_out: OPA0_OUT,
    opa0_cal: OPA0_CAL,
    _reserved20: [u8; 0x04],
    opa1_aportreq: OPA1_APORTREQ,
    opa1_aportconflict: OPA1_APORTCONFLICT,
    opa1_ctrl: OPA1_CTRL,
    opa1_timer: OPA1_TIMER,
    opa1_mux: OPA1_MUX,
    opa1_out: OPA1_OUT,
    opa1_cal: OPA1_CAL,
    _reserved27: [u8; 0x04],
    opa2_aportreq: OPA2_APORTREQ,
    opa2_aportconflict: OPA2_APORTCONFLICT,
    opa2_ctrl: OPA2_CTRL,
    opa2_timer: OPA2_TIMER,
    opa2_mux: OPA2_MUX,
    opa2_out: OPA2_OUT,
    opa2_cal: OPA2_CAL,
    _reserved34: [u8; 0x04],
    opa3_aportreq: OPA3_APORTREQ,
    opa3_aportconflict: OPA3_APORTCONFLICT,
    opa3_ctrl: OPA3_CTRL,
    opa3_timer: OPA3_TIMER,
    opa3_mux: OPA3_MUX,
    opa3_out: OPA3_OUT,
    opa3_cal: OPA3_CAL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Channel 0 Control Register"]
    #[inline(always)]
    pub const fn ch0ctrl(&self) -> &CH0CTRL {
        &self.ch0ctrl
    }
    #[doc = "0x0c - Channel 1 Control Register"]
    #[inline(always)]
    pub const fn ch1ctrl(&self) -> &CH1CTRL {
        &self.ch1ctrl
    }
    #[doc = "0x10 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x14 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x18 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x20 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x24 - Channel 0 Data Register"]
    #[inline(always)]
    pub const fn ch0data(&self) -> &CH0DATA {
        &self.ch0data
    }
    #[doc = "0x28 - Channel 1 Data Register"]
    #[inline(always)]
    pub const fn ch1data(&self) -> &CH1DATA {
        &self.ch1data
    }
    #[doc = "0x2c - Combined Data Register"]
    #[inline(always)]
    pub const fn combdata(&self) -> &COMBDATA {
        &self.combdata
    }
    #[doc = "0x30 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &CAL {
        &self.cal
    }
    #[doc = "0xa0 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa0_aportreq(&self) -> &OPA0_APORTREQ {
        &self.opa0_aportreq
    }
    #[doc = "0xa4 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa0_aportconflict(&self) -> &OPA0_APORTCONFLICT {
        &self.opa0_aportconflict
    }
    #[doc = "0xa8 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa0_ctrl(&self) -> &OPA0_CTRL {
        &self.opa0_ctrl
    }
    #[doc = "0xac - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa0_timer(&self) -> &OPA0_TIMER {
        &self.opa0_timer
    }
    #[doc = "0xb0 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa0_mux(&self) -> &OPA0_MUX {
        &self.opa0_mux
    }
    #[doc = "0xb4 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa0_out(&self) -> &OPA0_OUT {
        &self.opa0_out
    }
    #[doc = "0xb8 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa0_cal(&self) -> &OPA0_CAL {
        &self.opa0_cal
    }
    #[doc = "0xc0 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa1_aportreq(&self) -> &OPA1_APORTREQ {
        &self.opa1_aportreq
    }
    #[doc = "0xc4 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa1_aportconflict(&self) -> &OPA1_APORTCONFLICT {
        &self.opa1_aportconflict
    }
    #[doc = "0xc8 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa1_ctrl(&self) -> &OPA1_CTRL {
        &self.opa1_ctrl
    }
    #[doc = "0xcc - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa1_timer(&self) -> &OPA1_TIMER {
        &self.opa1_timer
    }
    #[doc = "0xd0 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa1_mux(&self) -> &OPA1_MUX {
        &self.opa1_mux
    }
    #[doc = "0xd4 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa1_out(&self) -> &OPA1_OUT {
        &self.opa1_out
    }
    #[doc = "0xd8 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa1_cal(&self) -> &OPA1_CAL {
        &self.opa1_cal
    }
    #[doc = "0xe0 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa2_aportreq(&self) -> &OPA2_APORTREQ {
        &self.opa2_aportreq
    }
    #[doc = "0xe4 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa2_aportconflict(&self) -> &OPA2_APORTCONFLICT {
        &self.opa2_aportconflict
    }
    #[doc = "0xe8 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa2_ctrl(&self) -> &OPA2_CTRL {
        &self.opa2_ctrl
    }
    #[doc = "0xec - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa2_timer(&self) -> &OPA2_TIMER {
        &self.opa2_timer
    }
    #[doc = "0xf0 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa2_mux(&self) -> &OPA2_MUX {
        &self.opa2_mux
    }
    #[doc = "0xf4 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa2_out(&self) -> &OPA2_OUT {
        &self.opa2_out
    }
    #[doc = "0xf8 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa2_cal(&self) -> &OPA2_CAL {
        &self.opa2_cal
    }
    #[doc = "0x100 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa3_aportreq(&self) -> &OPA3_APORTREQ {
        &self.opa3_aportreq
    }
    #[doc = "0x104 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa3_aportconflict(&self) -> &OPA3_APORTCONFLICT {
        &self.opa3_aportconflict
    }
    #[doc = "0x108 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa3_ctrl(&self) -> &OPA3_CTRL {
        &self.opa3_ctrl
    }
    #[doc = "0x10c - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa3_timer(&self) -> &OPA3_TIMER {
        &self.opa3_timer
    }
    #[doc = "0x110 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa3_mux(&self) -> &OPA3_MUX {
        &self.opa3_mux
    }
    #[doc = "0x114 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa3_out(&self) -> &OPA3_OUT {
        &self.opa3_out
    }
    #[doc = "0x118 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa3_cal(&self) -> &OPA3_CAL {
        &self.opa3_cal
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CH0CTRL (rw) register accessor: Channel 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctrl`]
module"]
pub type CH0CTRL = crate::Reg<ch0ctrl::CH0CTRL_SPEC>;
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "CH1CTRL (rw) register accessor: Channel 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctrl`]
module"]
pub type CH1CTRL = crate::Reg<ch1ctrl::CH1CTRL_SPEC>;
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
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
#[doc = "CH0DATA (rw) register accessor: Channel 0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0data`]
module"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "CH1DATA (rw) register accessor: Channel 1 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1data`]
module"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "COMBDATA (rw) register accessor: Combined Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`combdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@combdata`]
module"]
pub type COMBDATA = crate::Reg<combdata::COMBDATA_SPEC>;
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`]
module"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "OPA0_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_aportreq`]
module"]
pub type OPA0_APORTREQ = crate::Reg<opa0_aportreq::OPA0_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa0_aportreq;
#[doc = "OPA0_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_aportconflict`]
module"]
pub type OPA0_APORTCONFLICT = crate::Reg<opa0_aportconflict::OPA0_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa0_aportconflict;
#[doc = "OPA0_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_ctrl`]
module"]
pub type OPA0_CTRL = crate::Reg<opa0_ctrl::OPA0_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa0_ctrl;
#[doc = "OPA0_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa0_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_timer`]
module"]
pub type OPA0_TIMER = crate::Reg<opa0_timer::OPA0_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa0_timer;
#[doc = "OPA0_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa0_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_mux`]
module"]
pub type OPA0_MUX = crate::Reg<opa0_mux::OPA0_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0_mux;
#[doc = "OPA0_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa0_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_out`]
module"]
pub type OPA0_OUT = crate::Reg<opa0_out::OPA0_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa0_out;
#[doc = "OPA0_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa0_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_cal`]
module"]
pub type OPA0_CAL = crate::Reg<opa0_cal::OPA0_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa0_cal;
#[doc = "OPA1_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_aportreq`]
module"]
pub type OPA1_APORTREQ = crate::Reg<opa1_aportreq::OPA1_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa1_aportreq;
#[doc = "OPA1_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_aportconflict`]
module"]
pub type OPA1_APORTCONFLICT = crate::Reg<opa1_aportconflict::OPA1_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa1_aportconflict;
#[doc = "OPA1_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_ctrl`]
module"]
pub type OPA1_CTRL = crate::Reg<opa1_ctrl::OPA1_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa1_ctrl;
#[doc = "OPA1_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa1_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_timer`]
module"]
pub type OPA1_TIMER = crate::Reg<opa1_timer::OPA1_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa1_timer;
#[doc = "OPA1_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa1_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_mux`]
module"]
pub type OPA1_MUX = crate::Reg<opa1_mux::OPA1_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1_mux;
#[doc = "OPA1_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa1_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_out`]
module"]
pub type OPA1_OUT = crate::Reg<opa1_out::OPA1_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa1_out;
#[doc = "OPA1_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa1_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_cal`]
module"]
pub type OPA1_CAL = crate::Reg<opa1_cal::OPA1_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa1_cal;
#[doc = "OPA2_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_aportreq`]
module"]
pub type OPA2_APORTREQ = crate::Reg<opa2_aportreq::OPA2_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa2_aportreq;
#[doc = "OPA2_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_aportconflict`]
module"]
pub type OPA2_APORTCONFLICT = crate::Reg<opa2_aportconflict::OPA2_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa2_aportconflict;
#[doc = "OPA2_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_ctrl`]
module"]
pub type OPA2_CTRL = crate::Reg<opa2_ctrl::OPA2_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa2_ctrl;
#[doc = "OPA2_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa2_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_timer`]
module"]
pub type OPA2_TIMER = crate::Reg<opa2_timer::OPA2_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa2_timer;
#[doc = "OPA2_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa2_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_mux`]
module"]
pub type OPA2_MUX = crate::Reg<opa2_mux::OPA2_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2_mux;
#[doc = "OPA2_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa2_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_out`]
module"]
pub type OPA2_OUT = crate::Reg<opa2_out::OPA2_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa2_out;
#[doc = "OPA2_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa2_cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa2_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_cal`]
module"]
pub type OPA2_CAL = crate::Reg<opa2_cal::OPA2_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa2_cal;
#[doc = "OPA3_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_aportreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_aportreq`]
module"]
pub type OPA3_APORTREQ = crate::Reg<opa3_aportreq::OPA3_APORTREQ_SPEC>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa3_aportreq;
#[doc = "OPA3_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_aportconflict::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_aportconflict`]
module"]
pub type OPA3_APORTCONFLICT = crate::Reg<opa3_aportconflict::OPA3_APORTCONFLICT_SPEC>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa3_aportconflict;
#[doc = "OPA3_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_ctrl`]
module"]
pub type OPA3_CTRL = crate::Reg<opa3_ctrl::OPA3_CTRL_SPEC>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa3_ctrl;
#[doc = "OPA3_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa3_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_timer`]
module"]
pub type OPA3_TIMER = crate::Reg<opa3_timer::OPA3_TIMER_SPEC>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa3_timer;
#[doc = "OPA3_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_mux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa3_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_mux`]
module"]
pub type OPA3_MUX = crate::Reg<opa3_mux::OPA3_MUX_SPEC>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa3_mux;
#[doc = "OPA3_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa3_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_out`]
module"]
pub type OPA3_OUT = crate::Reg<opa3_out::OPA3_OUT_SPEC>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa3_out;
#[doc = "OPA3_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa3_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_cal`]
module"]
pub type OPA3_CAL = crate::Reg<opa3_cal::OPA3_CAL_SPEC>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa3_cal;
