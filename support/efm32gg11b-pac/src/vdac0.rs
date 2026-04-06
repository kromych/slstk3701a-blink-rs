#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    ch0ctrl: Ch0ctrl,
    ch1ctrl: Ch1ctrl,
    cmd: Cmd,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    ch0data: Ch0data,
    ch1data: Ch1data,
    combdata: Combdata,
    cal: Cal,
    _reserved13: [u8; 0x6c],
    opa0_aportreq: Opa0Aportreq,
    opa0_aportconflict: Opa0Aportconflict,
    opa0_ctrl: Opa0Ctrl,
    opa0_timer: Opa0Timer,
    opa0_mux: Opa0Mux,
    opa0_out: Opa0Out,
    opa0_cal: Opa0Cal,
    _reserved20: [u8; 0x04],
    opa1_aportreq: Opa1Aportreq,
    opa1_aportconflict: Opa1Aportconflict,
    opa1_ctrl: Opa1Ctrl,
    opa1_timer: Opa1Timer,
    opa1_mux: Opa1Mux,
    opa1_out: Opa1Out,
    opa1_cal: Opa1Cal,
    _reserved27: [u8; 0x04],
    opa2_aportreq: Opa2Aportreq,
    opa2_aportconflict: Opa2Aportconflict,
    opa2_ctrl: Opa2Ctrl,
    opa2_timer: Opa2Timer,
    opa2_mux: Opa2Mux,
    opa2_out: Opa2Out,
    opa2_cal: Opa2Cal,
    _reserved34: [u8; 0x04],
    opa3_aportreq: Opa3Aportreq,
    opa3_aportconflict: Opa3Aportconflict,
    opa3_ctrl: Opa3Ctrl,
    opa3_timer: Opa3Timer,
    opa3_mux: Opa3Mux,
    opa3_out: Opa3Out,
    opa3_cal: Opa3Cal,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Channel 0 Control Register"]
    #[inline(always)]
    pub const fn ch0ctrl(&self) -> &Ch0ctrl {
        &self.ch0ctrl
    }
    #[doc = "0x0c - Channel 1 Control Register"]
    #[inline(always)]
    pub const fn ch1ctrl(&self) -> &Ch1ctrl {
        &self.ch1ctrl
    }
    #[doc = "0x10 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x14 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x18 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x20 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x24 - Channel 0 Data Register"]
    #[inline(always)]
    pub const fn ch0data(&self) -> &Ch0data {
        &self.ch0data
    }
    #[doc = "0x28 - Channel 1 Data Register"]
    #[inline(always)]
    pub const fn ch1data(&self) -> &Ch1data {
        &self.ch1data
    }
    #[doc = "0x2c - Combined Data Register"]
    #[inline(always)]
    pub const fn combdata(&self) -> &Combdata {
        &self.combdata
    }
    #[doc = "0x30 - Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0xa0 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa0_aportreq(&self) -> &Opa0Aportreq {
        &self.opa0_aportreq
    }
    #[doc = "0xa4 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa0_aportconflict(&self) -> &Opa0Aportconflict {
        &self.opa0_aportconflict
    }
    #[doc = "0xa8 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa0_ctrl(&self) -> &Opa0Ctrl {
        &self.opa0_ctrl
    }
    #[doc = "0xac - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa0_timer(&self) -> &Opa0Timer {
        &self.opa0_timer
    }
    #[doc = "0xb0 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa0_mux(&self) -> &Opa0Mux {
        &self.opa0_mux
    }
    #[doc = "0xb4 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa0_out(&self) -> &Opa0Out {
        &self.opa0_out
    }
    #[doc = "0xb8 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa0_cal(&self) -> &Opa0Cal {
        &self.opa0_cal
    }
    #[doc = "0xc0 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa1_aportreq(&self) -> &Opa1Aportreq {
        &self.opa1_aportreq
    }
    #[doc = "0xc4 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa1_aportconflict(&self) -> &Opa1Aportconflict {
        &self.opa1_aportconflict
    }
    #[doc = "0xc8 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa1_ctrl(&self) -> &Opa1Ctrl {
        &self.opa1_ctrl
    }
    #[doc = "0xcc - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa1_timer(&self) -> &Opa1Timer {
        &self.opa1_timer
    }
    #[doc = "0xd0 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa1_mux(&self) -> &Opa1Mux {
        &self.opa1_mux
    }
    #[doc = "0xd4 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa1_out(&self) -> &Opa1Out {
        &self.opa1_out
    }
    #[doc = "0xd8 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa1_cal(&self) -> &Opa1Cal {
        &self.opa1_cal
    }
    #[doc = "0xe0 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa2_aportreq(&self) -> &Opa2Aportreq {
        &self.opa2_aportreq
    }
    #[doc = "0xe4 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa2_aportconflict(&self) -> &Opa2Aportconflict {
        &self.opa2_aportconflict
    }
    #[doc = "0xe8 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa2_ctrl(&self) -> &Opa2Ctrl {
        &self.opa2_ctrl
    }
    #[doc = "0xec - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa2_timer(&self) -> &Opa2Timer {
        &self.opa2_timer
    }
    #[doc = "0xf0 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa2_mux(&self) -> &Opa2Mux {
        &self.opa2_mux
    }
    #[doc = "0xf4 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa2_out(&self) -> &Opa2Out {
        &self.opa2_out
    }
    #[doc = "0xf8 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa2_cal(&self) -> &Opa2Cal {
        &self.opa2_cal
    }
    #[doc = "0x100 - Operational Amplifier APORT Request Status Register"]
    #[inline(always)]
    pub const fn opa3_aportreq(&self) -> &Opa3Aportreq {
        &self.opa3_aportreq
    }
    #[doc = "0x104 - Operational Amplifier APORT Conflict Status Register"]
    #[inline(always)]
    pub const fn opa3_aportconflict(&self) -> &Opa3Aportconflict {
        &self.opa3_aportconflict
    }
    #[doc = "0x108 - Operational Amplifier Control Register"]
    #[inline(always)]
    pub const fn opa3_ctrl(&self) -> &Opa3Ctrl {
        &self.opa3_ctrl
    }
    #[doc = "0x10c - Operational Amplifier Timer Control Register"]
    #[inline(always)]
    pub const fn opa3_timer(&self) -> &Opa3Timer {
        &self.opa3_timer
    }
    #[doc = "0x110 - Operational Amplifier Mux Configuration Register"]
    #[inline(always)]
    pub const fn opa3_mux(&self) -> &Opa3Mux {
        &self.opa3_mux
    }
    #[doc = "0x114 - Operational Amplifier Output Configuration Register"]
    #[inline(always)]
    pub const fn opa3_out(&self) -> &Opa3Out {
        &self.opa3_out
    }
    #[doc = "0x118 - Operational Amplifier Calibration Register"]
    #[inline(always)]
    pub const fn opa3_cal(&self) -> &Opa3Cal {
        &self.opa3_cal
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CH0CTRL (rw) register accessor: Channel 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctrl`] module"]
#[doc(alias = "CH0CTRL")]
pub type Ch0ctrl = crate::Reg<ch0ctrl::Ch0ctrlSpec>;
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "CH1CTRL (rw) register accessor: Channel 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctrl`] module"]
#[doc(alias = "CH1CTRL")]
pub type Ch1ctrl = crate::Reg<ch1ctrl::Ch1ctrlSpec>;
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
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
#[doc = "CH0DATA (rw) register accessor: Channel 0 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0data`] module"]
#[doc(alias = "CH0DATA")]
pub type Ch0data = crate::Reg<ch0data::Ch0dataSpec>;
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "CH1DATA (rw) register accessor: Channel 1 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1data`] module"]
#[doc(alias = "CH1DATA")]
pub type Ch1data = crate::Reg<ch1data::Ch1dataSpec>;
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "COMBDATA (rw) register accessor: Combined Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`combdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`combdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@combdata`] module"]
#[doc(alias = "COMBDATA")]
pub type Combdata = crate::Reg<combdata::CombdataSpec>;
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "CAL (rw) register accessor: Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "OPA0_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_aportreq`] module"]
#[doc(alias = "OPA0_APORTREQ")]
pub type Opa0Aportreq = crate::Reg<opa0_aportreq::Opa0AportreqSpec>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa0_aportreq;
#[doc = "OPA0_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_aportconflict`] module"]
#[doc(alias = "OPA0_APORTCONFLICT")]
pub type Opa0Aportconflict = crate::Reg<opa0_aportconflict::Opa0AportconflictSpec>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa0_aportconflict;
#[doc = "OPA0_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_ctrl`] module"]
#[doc(alias = "OPA0_CTRL")]
pub type Opa0Ctrl = crate::Reg<opa0_ctrl::Opa0CtrlSpec>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa0_ctrl;
#[doc = "OPA0_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa0_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_timer`] module"]
#[doc(alias = "OPA0_TIMER")]
pub type Opa0Timer = crate::Reg<opa0_timer::Opa0TimerSpec>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa0_timer;
#[doc = "OPA0_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa0_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_mux`] module"]
#[doc(alias = "OPA0_MUX")]
pub type Opa0Mux = crate::Reg<opa0_mux::Opa0MuxSpec>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0_mux;
#[doc = "OPA0_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa0_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_out`] module"]
#[doc(alias = "OPA0_OUT")]
pub type Opa0Out = crate::Reg<opa0_out::Opa0OutSpec>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa0_out;
#[doc = "OPA0_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa0_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa0_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa0_cal`] module"]
#[doc(alias = "OPA0_CAL")]
pub type Opa0Cal = crate::Reg<opa0_cal::Opa0CalSpec>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa0_cal;
#[doc = "OPA1_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_aportreq`] module"]
#[doc(alias = "OPA1_APORTREQ")]
pub type Opa1Aportreq = crate::Reg<opa1_aportreq::Opa1AportreqSpec>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa1_aportreq;
#[doc = "OPA1_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_aportconflict`] module"]
#[doc(alias = "OPA1_APORTCONFLICT")]
pub type Opa1Aportconflict = crate::Reg<opa1_aportconflict::Opa1AportconflictSpec>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa1_aportconflict;
#[doc = "OPA1_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_ctrl`] module"]
#[doc(alias = "OPA1_CTRL")]
pub type Opa1Ctrl = crate::Reg<opa1_ctrl::Opa1CtrlSpec>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa1_ctrl;
#[doc = "OPA1_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa1_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_timer`] module"]
#[doc(alias = "OPA1_TIMER")]
pub type Opa1Timer = crate::Reg<opa1_timer::Opa1TimerSpec>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa1_timer;
#[doc = "OPA1_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa1_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_mux`] module"]
#[doc(alias = "OPA1_MUX")]
pub type Opa1Mux = crate::Reg<opa1_mux::Opa1MuxSpec>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1_mux;
#[doc = "OPA1_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa1_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_out`] module"]
#[doc(alias = "OPA1_OUT")]
pub type Opa1Out = crate::Reg<opa1_out::Opa1OutSpec>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa1_out;
#[doc = "OPA1_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa1_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa1_cal`] module"]
#[doc(alias = "OPA1_CAL")]
pub type Opa1Cal = crate::Reg<opa1_cal::Opa1CalSpec>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa1_cal;
#[doc = "OPA2_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_aportreq`] module"]
#[doc(alias = "OPA2_APORTREQ")]
pub type Opa2Aportreq = crate::Reg<opa2_aportreq::Opa2AportreqSpec>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa2_aportreq;
#[doc = "OPA2_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_aportconflict`] module"]
#[doc(alias = "OPA2_APORTCONFLICT")]
pub type Opa2Aportconflict = crate::Reg<opa2_aportconflict::Opa2AportconflictSpec>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa2_aportconflict;
#[doc = "OPA2_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_ctrl`] module"]
#[doc(alias = "OPA2_CTRL")]
pub type Opa2Ctrl = crate::Reg<opa2_ctrl::Opa2CtrlSpec>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa2_ctrl;
#[doc = "OPA2_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa2_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_timer`] module"]
#[doc(alias = "OPA2_TIMER")]
pub type Opa2Timer = crate::Reg<opa2_timer::Opa2TimerSpec>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa2_timer;
#[doc = "OPA2_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa2_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_mux`] module"]
#[doc(alias = "OPA2_MUX")]
pub type Opa2Mux = crate::Reg<opa2_mux::Opa2MuxSpec>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2_mux;
#[doc = "OPA2_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa2_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_out`] module"]
#[doc(alias = "OPA2_OUT")]
pub type Opa2Out = crate::Reg<opa2_out::Opa2OutSpec>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa2_out;
#[doc = "OPA2_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa2_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa2_cal`] module"]
#[doc(alias = "OPA2_CAL")]
pub type Opa2Cal = crate::Reg<opa2_cal::Opa2CalSpec>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa2_cal;
#[doc = "OPA3_APORTREQ (r) register accessor: Operational Amplifier APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_aportreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_aportreq`] module"]
#[doc(alias = "OPA3_APORTREQ")]
pub type Opa3Aportreq = crate::Reg<opa3_aportreq::Opa3AportreqSpec>;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa3_aportreq;
#[doc = "OPA3_APORTCONFLICT (r) register accessor: Operational Amplifier APORT Conflict Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_aportconflict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_aportconflict`] module"]
#[doc(alias = "OPA3_APORTCONFLICT")]
pub type Opa3Aportconflict = crate::Reg<opa3_aportconflict::Opa3AportconflictSpec>;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa3_aportconflict;
#[doc = "OPA3_CTRL (rw) register accessor: Operational Amplifier Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_ctrl`] module"]
#[doc(alias = "OPA3_CTRL")]
pub type Opa3Ctrl = crate::Reg<opa3_ctrl::Opa3CtrlSpec>;
#[doc = "Operational Amplifier Control Register"]
pub mod opa3_ctrl;
#[doc = "OPA3_TIMER (rw) register accessor: Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_timer`] module"]
#[doc(alias = "OPA3_TIMER")]
pub type Opa3Timer = crate::Reg<opa3_timer::Opa3TimerSpec>;
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa3_timer;
#[doc = "OPA3_MUX (rw) register accessor: Operational Amplifier Mux Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_mux`] module"]
#[doc(alias = "OPA3_MUX")]
pub type Opa3Mux = crate::Reg<opa3_mux::Opa3MuxSpec>;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa3_mux;
#[doc = "OPA3_OUT (rw) register accessor: Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_out`] module"]
#[doc(alias = "OPA3_OUT")]
pub type Opa3Out = crate::Reg<opa3_out::Opa3OutSpec>;
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa3_out;
#[doc = "OPA3_CAL (rw) register accessor: Operational Amplifier Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa3_cal`] module"]
#[doc(alias = "OPA3_CAL")]
pub type Opa3Cal = crate::Reg<opa3_cal::Opa3CalSpec>;
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa3_cal;
