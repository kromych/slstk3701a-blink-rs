#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    precnt: Precnt,
    cnt: Cnt,
    combcnt: Combcnt,
    time: Time,
    date: Date,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    status: Status,
    cmd: Cmd,
    syncbusy: Syncbusy,
    powerdown: Powerdown,
    lock: Lock,
    em4wuen: Em4wuen,
    cc0_ctrl: Cc0Ctrl,
    cc0_ccv: Cc0Ccv,
    cc0_time: Cc0Time,
    cc0_date: Cc0Date,
    cc1_ctrl: Cc1Ctrl,
    cc1_ccv: Cc1Ccv,
    cc1_time: Cc1Time,
    cc1_date: Cc1Date,
    cc2_ctrl: Cc2Ctrl,
    cc2_ccv: Cc2Ccv,
    cc2_time: Cc2Time,
    cc2_date: Cc2Date,
    _reserved28: [u8; 0x94],
    ret0_reg: Ret0Reg,
    ret1_reg: Ret1Reg,
    ret2_reg: Ret2Reg,
    ret3_reg: Ret3Reg,
    ret4_reg: Ret4Reg,
    ret5_reg: Ret5Reg,
    ret6_reg: Ret6Reg,
    ret7_reg: Ret7Reg,
    ret8_reg: Ret8Reg,
    ret9_reg: Ret9Reg,
    ret10_reg: Ret10Reg,
    ret11_reg: Ret11Reg,
    ret12_reg: Ret12Reg,
    ret13_reg: Ret13Reg,
    ret14_reg: Ret14Reg,
    ret15_reg: Ret15Reg,
    ret16_reg: Ret16Reg,
    ret17_reg: Ret17Reg,
    ret18_reg: Ret18Reg,
    ret19_reg: Ret19Reg,
    ret20_reg: Ret20Reg,
    ret21_reg: Ret21Reg,
    ret22_reg: Ret22Reg,
    ret23_reg: Ret23Reg,
    ret24_reg: Ret24Reg,
    ret25_reg: Ret25Reg,
    ret26_reg: Ret26Reg,
    ret27_reg: Ret27Reg,
    ret28_reg: Ret28Reg,
    ret29_reg: Ret29Reg,
    ret30_reg: Ret30Reg,
    ret31_reg: Ret31Reg,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Pre-Counter Value Register"]
    #[inline(always)]
    pub const fn precnt(&self) -> &Precnt {
        &self.precnt
    }
    #[doc = "0x08 - Counter Value Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x0c - Combined Pre-Counter and Counter Value Register"]
    #[inline(always)]
    pub const fn combcnt(&self) -> &Combcnt {
        &self.combcnt
    }
    #[doc = "0x10 - Time of Day Register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x14 - Date Register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x18 - RTCC Interrupt Flags"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x1c - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x24 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x28 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x2c - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x30 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x34 - Retention RAM Power-down Register"]
    #[inline(always)]
    pub const fn powerdown(&self) -> &Powerdown {
        &self.powerdown
    }
    #[doc = "0x38 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x3c - Wake Up Enable"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x40 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc0_ctrl(&self) -> &Cc0Ctrl {
        &self.cc0_ctrl
    }
    #[doc = "0x44 - Capture/Compare Value Register"]
    #[inline(always)]
    pub const fn cc0_ccv(&self) -> &Cc0Ccv {
        &self.cc0_ccv
    }
    #[doc = "0x48 - Capture/Compare Time Register"]
    #[inline(always)]
    pub const fn cc0_time(&self) -> &Cc0Time {
        &self.cc0_time
    }
    #[doc = "0x4c - Capture/Compare Date Register"]
    #[inline(always)]
    pub const fn cc0_date(&self) -> &Cc0Date {
        &self.cc0_date
    }
    #[doc = "0x50 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc1_ctrl(&self) -> &Cc1Ctrl {
        &self.cc1_ctrl
    }
    #[doc = "0x54 - Capture/Compare Value Register"]
    #[inline(always)]
    pub const fn cc1_ccv(&self) -> &Cc1Ccv {
        &self.cc1_ccv
    }
    #[doc = "0x58 - Capture/Compare Time Register"]
    #[inline(always)]
    pub const fn cc1_time(&self) -> &Cc1Time {
        &self.cc1_time
    }
    #[doc = "0x5c - Capture/Compare Date Register"]
    #[inline(always)]
    pub const fn cc1_date(&self) -> &Cc1Date {
        &self.cc1_date
    }
    #[doc = "0x60 - CC Channel Control Register"]
    #[inline(always)]
    pub const fn cc2_ctrl(&self) -> &Cc2Ctrl {
        &self.cc2_ctrl
    }
    #[doc = "0x64 - Capture/Compare Value Register"]
    #[inline(always)]
    pub const fn cc2_ccv(&self) -> &Cc2Ccv {
        &self.cc2_ccv
    }
    #[doc = "0x68 - Capture/Compare Time Register"]
    #[inline(always)]
    pub const fn cc2_time(&self) -> &Cc2Time {
        &self.cc2_time
    }
    #[doc = "0x6c - Capture/Compare Date Register"]
    #[inline(always)]
    pub const fn cc2_date(&self) -> &Cc2Date {
        &self.cc2_date
    }
    #[doc = "0x104 - Retention Register"]
    #[inline(always)]
    pub const fn ret0_reg(&self) -> &Ret0Reg {
        &self.ret0_reg
    }
    #[doc = "0x108 - Retention Register"]
    #[inline(always)]
    pub const fn ret1_reg(&self) -> &Ret1Reg {
        &self.ret1_reg
    }
    #[doc = "0x10c - Retention Register"]
    #[inline(always)]
    pub const fn ret2_reg(&self) -> &Ret2Reg {
        &self.ret2_reg
    }
    #[doc = "0x110 - Retention Register"]
    #[inline(always)]
    pub const fn ret3_reg(&self) -> &Ret3Reg {
        &self.ret3_reg
    }
    #[doc = "0x114 - Retention Register"]
    #[inline(always)]
    pub const fn ret4_reg(&self) -> &Ret4Reg {
        &self.ret4_reg
    }
    #[doc = "0x118 - Retention Register"]
    #[inline(always)]
    pub const fn ret5_reg(&self) -> &Ret5Reg {
        &self.ret5_reg
    }
    #[doc = "0x11c - Retention Register"]
    #[inline(always)]
    pub const fn ret6_reg(&self) -> &Ret6Reg {
        &self.ret6_reg
    }
    #[doc = "0x120 - Retention Register"]
    #[inline(always)]
    pub const fn ret7_reg(&self) -> &Ret7Reg {
        &self.ret7_reg
    }
    #[doc = "0x124 - Retention Register"]
    #[inline(always)]
    pub const fn ret8_reg(&self) -> &Ret8Reg {
        &self.ret8_reg
    }
    #[doc = "0x128 - Retention Register"]
    #[inline(always)]
    pub const fn ret9_reg(&self) -> &Ret9Reg {
        &self.ret9_reg
    }
    #[doc = "0x12c - Retention Register"]
    #[inline(always)]
    pub const fn ret10_reg(&self) -> &Ret10Reg {
        &self.ret10_reg
    }
    #[doc = "0x130 - Retention Register"]
    #[inline(always)]
    pub const fn ret11_reg(&self) -> &Ret11Reg {
        &self.ret11_reg
    }
    #[doc = "0x134 - Retention Register"]
    #[inline(always)]
    pub const fn ret12_reg(&self) -> &Ret12Reg {
        &self.ret12_reg
    }
    #[doc = "0x138 - Retention Register"]
    #[inline(always)]
    pub const fn ret13_reg(&self) -> &Ret13Reg {
        &self.ret13_reg
    }
    #[doc = "0x13c - Retention Register"]
    #[inline(always)]
    pub const fn ret14_reg(&self) -> &Ret14Reg {
        &self.ret14_reg
    }
    #[doc = "0x140 - Retention Register"]
    #[inline(always)]
    pub const fn ret15_reg(&self) -> &Ret15Reg {
        &self.ret15_reg
    }
    #[doc = "0x144 - Retention Register"]
    #[inline(always)]
    pub const fn ret16_reg(&self) -> &Ret16Reg {
        &self.ret16_reg
    }
    #[doc = "0x148 - Retention Register"]
    #[inline(always)]
    pub const fn ret17_reg(&self) -> &Ret17Reg {
        &self.ret17_reg
    }
    #[doc = "0x14c - Retention Register"]
    #[inline(always)]
    pub const fn ret18_reg(&self) -> &Ret18Reg {
        &self.ret18_reg
    }
    #[doc = "0x150 - Retention Register"]
    #[inline(always)]
    pub const fn ret19_reg(&self) -> &Ret19Reg {
        &self.ret19_reg
    }
    #[doc = "0x154 - Retention Register"]
    #[inline(always)]
    pub const fn ret20_reg(&self) -> &Ret20Reg {
        &self.ret20_reg
    }
    #[doc = "0x158 - Retention Register"]
    #[inline(always)]
    pub const fn ret21_reg(&self) -> &Ret21Reg {
        &self.ret21_reg
    }
    #[doc = "0x15c - Retention Register"]
    #[inline(always)]
    pub const fn ret22_reg(&self) -> &Ret22Reg {
        &self.ret22_reg
    }
    #[doc = "0x160 - Retention Register"]
    #[inline(always)]
    pub const fn ret23_reg(&self) -> &Ret23Reg {
        &self.ret23_reg
    }
    #[doc = "0x164 - Retention Register"]
    #[inline(always)]
    pub const fn ret24_reg(&self) -> &Ret24Reg {
        &self.ret24_reg
    }
    #[doc = "0x168 - Retention Register"]
    #[inline(always)]
    pub const fn ret25_reg(&self) -> &Ret25Reg {
        &self.ret25_reg
    }
    #[doc = "0x16c - Retention Register"]
    #[inline(always)]
    pub const fn ret26_reg(&self) -> &Ret26Reg {
        &self.ret26_reg
    }
    #[doc = "0x170 - Retention Register"]
    #[inline(always)]
    pub const fn ret27_reg(&self) -> &Ret27Reg {
        &self.ret27_reg
    }
    #[doc = "0x174 - Retention Register"]
    #[inline(always)]
    pub const fn ret28_reg(&self) -> &Ret28Reg {
        &self.ret28_reg
    }
    #[doc = "0x178 - Retention Register"]
    #[inline(always)]
    pub const fn ret29_reg(&self) -> &Ret29Reg {
        &self.ret29_reg
    }
    #[doc = "0x17c - Retention Register"]
    #[inline(always)]
    pub const fn ret30_reg(&self) -> &Ret30Reg {
        &self.ret30_reg
    }
    #[doc = "0x180 - Retention Register"]
    #[inline(always)]
    pub const fn ret31_reg(&self) -> &Ret31Reg {
        &self.ret31_reg
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "PRECNT (rw) register accessor: Pre-Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`precnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`precnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@precnt`] module"]
#[doc(alias = "PRECNT")]
pub type Precnt = crate::Reg<precnt::PrecntSpec>;
#[doc = "Pre-Counter Value Register"]
pub mod precnt;
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "COMBCNT (r) register accessor: Combined Pre-Counter and Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`combcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@combcnt`] module"]
#[doc(alias = "COMBCNT")]
pub type Combcnt = crate::Reg<combcnt::CombcntSpec>;
#[doc = "Combined Pre-Counter and Counter Value Register"]
pub mod combcnt;
#[doc = "TIME (rw) register accessor: Time of Day Register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`] module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Time of Day Register"]
pub mod time;
#[doc = "DATE (rw) register accessor: Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "Date Register"]
pub mod date;
#[doc = "IF (r) register accessor: RTCC Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "RTCC Interrupt Flags"]
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
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "POWERDOWN (rw) register accessor: Retention RAM Power-down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`powerdown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerdown`] module"]
#[doc(alias = "POWERDOWN")]
pub type Powerdown = crate::Reg<powerdown::PowerdownSpec>;
#[doc = "Retention RAM Power-down Register"]
pub mod powerdown;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "EM4WUEN (rw) register accessor: Wake Up Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`] module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::Em4wuenSpec>;
#[doc = "Wake Up Enable"]
pub mod em4wuen;
#[doc = "CC0_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ctrl`] module"]
#[doc(alias = "CC0_CTRL")]
pub type Cc0Ctrl = crate::Reg<cc0_ctrl::Cc0CtrlSpec>;
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC0_CCV (rw) register accessor: Capture/Compare Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ccv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ccv`] module"]
#[doc(alias = "CC0_CCV")]
pub type Cc0Ccv = crate::Reg<cc0_ccv::Cc0CcvSpec>;
#[doc = "Capture/Compare Value Register"]
pub mod cc0_ccv;
#[doc = "CC0_TIME (rw) register accessor: Capture/Compare Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_time`] module"]
#[doc(alias = "CC0_TIME")]
pub type Cc0Time = crate::Reg<cc0_time::Cc0TimeSpec>;
#[doc = "Capture/Compare Time Register"]
pub mod cc0_time;
#[doc = "CC0_DATE (rw) register accessor: Capture/Compare Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_date`] module"]
#[doc(alias = "CC0_DATE")]
pub type Cc0Date = crate::Reg<cc0_date::Cc0DateSpec>;
#[doc = "Capture/Compare Date Register"]
pub mod cc0_date;
#[doc = "CC1_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ctrl`] module"]
#[doc(alias = "CC1_CTRL")]
pub type Cc1Ctrl = crate::Reg<cc1_ctrl::Cc1CtrlSpec>;
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC1_CCV (rw) register accessor: Capture/Compare Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ccv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ccv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ccv`] module"]
#[doc(alias = "CC1_CCV")]
pub type Cc1Ccv = crate::Reg<cc1_ccv::Cc1CcvSpec>;
#[doc = "Capture/Compare Value Register"]
pub mod cc1_ccv;
#[doc = "CC1_TIME (rw) register accessor: Capture/Compare Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_time`] module"]
#[doc(alias = "CC1_TIME")]
pub type Cc1Time = crate::Reg<cc1_time::Cc1TimeSpec>;
#[doc = "Capture/Compare Time Register"]
pub mod cc1_time;
#[doc = "CC1_DATE (rw) register accessor: Capture/Compare Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_date`] module"]
#[doc(alias = "CC1_DATE")]
pub type Cc1Date = crate::Reg<cc1_date::Cc1DateSpec>;
#[doc = "Capture/Compare Date Register"]
pub mod cc1_date;
#[doc = "CC2_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ctrl`] module"]
#[doc(alias = "CC2_CTRL")]
pub type Cc2Ctrl = crate::Reg<cc2_ctrl::Cc2CtrlSpec>;
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC2_CCV (rw) register accessor: Capture/Compare Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ccv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ccv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ccv`] module"]
#[doc(alias = "CC2_CCV")]
pub type Cc2Ccv = crate::Reg<cc2_ccv::Cc2CcvSpec>;
#[doc = "Capture/Compare Value Register"]
pub mod cc2_ccv;
#[doc = "CC2_TIME (rw) register accessor: Capture/Compare Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_time`] module"]
#[doc(alias = "CC2_TIME")]
pub type Cc2Time = crate::Reg<cc2_time::Cc2TimeSpec>;
#[doc = "Capture/Compare Time Register"]
pub mod cc2_time;
#[doc = "CC2_DATE (rw) register accessor: Capture/Compare Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_date`] module"]
#[doc(alias = "CC2_DATE")]
pub type Cc2Date = crate::Reg<cc2_date::Cc2DateSpec>;
#[doc = "Capture/Compare Date Register"]
pub mod cc2_date;
#[doc = "RET0_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret0_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret0_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret0_reg`] module"]
#[doc(alias = "RET0_REG")]
pub type Ret0Reg = crate::Reg<ret0_reg::Ret0RegSpec>;
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "RET1_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret1_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret1_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret1_reg`] module"]
#[doc(alias = "RET1_REG")]
pub type Ret1Reg = crate::Reg<ret1_reg::Ret1RegSpec>;
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "RET2_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret2_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret2_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret2_reg`] module"]
#[doc(alias = "RET2_REG")]
pub type Ret2Reg = crate::Reg<ret2_reg::Ret2RegSpec>;
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "RET3_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret3_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret3_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret3_reg`] module"]
#[doc(alias = "RET3_REG")]
pub type Ret3Reg = crate::Reg<ret3_reg::Ret3RegSpec>;
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "RET4_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret4_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret4_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret4_reg`] module"]
#[doc(alias = "RET4_REG")]
pub type Ret4Reg = crate::Reg<ret4_reg::Ret4RegSpec>;
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "RET5_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret5_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret5_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret5_reg`] module"]
#[doc(alias = "RET5_REG")]
pub type Ret5Reg = crate::Reg<ret5_reg::Ret5RegSpec>;
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "RET6_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret6_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret6_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret6_reg`] module"]
#[doc(alias = "RET6_REG")]
pub type Ret6Reg = crate::Reg<ret6_reg::Ret6RegSpec>;
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "RET7_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret7_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret7_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret7_reg`] module"]
#[doc(alias = "RET7_REG")]
pub type Ret7Reg = crate::Reg<ret7_reg::Ret7RegSpec>;
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "RET8_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret8_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret8_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret8_reg`] module"]
#[doc(alias = "RET8_REG")]
pub type Ret8Reg = crate::Reg<ret8_reg::Ret8RegSpec>;
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "RET9_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret9_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret9_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret9_reg`] module"]
#[doc(alias = "RET9_REG")]
pub type Ret9Reg = crate::Reg<ret9_reg::Ret9RegSpec>;
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "RET10_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret10_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret10_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret10_reg`] module"]
#[doc(alias = "RET10_REG")]
pub type Ret10Reg = crate::Reg<ret10_reg::Ret10RegSpec>;
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "RET11_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret11_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret11_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret11_reg`] module"]
#[doc(alias = "RET11_REG")]
pub type Ret11Reg = crate::Reg<ret11_reg::Ret11RegSpec>;
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "RET12_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret12_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret12_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret12_reg`] module"]
#[doc(alias = "RET12_REG")]
pub type Ret12Reg = crate::Reg<ret12_reg::Ret12RegSpec>;
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "RET13_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret13_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret13_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret13_reg`] module"]
#[doc(alias = "RET13_REG")]
pub type Ret13Reg = crate::Reg<ret13_reg::Ret13RegSpec>;
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "RET14_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret14_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret14_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret14_reg`] module"]
#[doc(alias = "RET14_REG")]
pub type Ret14Reg = crate::Reg<ret14_reg::Ret14RegSpec>;
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "RET15_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret15_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret15_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret15_reg`] module"]
#[doc(alias = "RET15_REG")]
pub type Ret15Reg = crate::Reg<ret15_reg::Ret15RegSpec>;
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "RET16_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret16_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret16_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret16_reg`] module"]
#[doc(alias = "RET16_REG")]
pub type Ret16Reg = crate::Reg<ret16_reg::Ret16RegSpec>;
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "RET17_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret17_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret17_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret17_reg`] module"]
#[doc(alias = "RET17_REG")]
pub type Ret17Reg = crate::Reg<ret17_reg::Ret17RegSpec>;
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "RET18_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret18_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret18_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret18_reg`] module"]
#[doc(alias = "RET18_REG")]
pub type Ret18Reg = crate::Reg<ret18_reg::Ret18RegSpec>;
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "RET19_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret19_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret19_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret19_reg`] module"]
#[doc(alias = "RET19_REG")]
pub type Ret19Reg = crate::Reg<ret19_reg::Ret19RegSpec>;
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "RET20_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret20_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret20_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret20_reg`] module"]
#[doc(alias = "RET20_REG")]
pub type Ret20Reg = crate::Reg<ret20_reg::Ret20RegSpec>;
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "RET21_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret21_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret21_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret21_reg`] module"]
#[doc(alias = "RET21_REG")]
pub type Ret21Reg = crate::Reg<ret21_reg::Ret21RegSpec>;
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "RET22_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret22_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret22_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret22_reg`] module"]
#[doc(alias = "RET22_REG")]
pub type Ret22Reg = crate::Reg<ret22_reg::Ret22RegSpec>;
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "RET23_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret23_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret23_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret23_reg`] module"]
#[doc(alias = "RET23_REG")]
pub type Ret23Reg = crate::Reg<ret23_reg::Ret23RegSpec>;
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "RET24_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret24_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret24_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret24_reg`] module"]
#[doc(alias = "RET24_REG")]
pub type Ret24Reg = crate::Reg<ret24_reg::Ret24RegSpec>;
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "RET25_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret25_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret25_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret25_reg`] module"]
#[doc(alias = "RET25_REG")]
pub type Ret25Reg = crate::Reg<ret25_reg::Ret25RegSpec>;
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "RET26_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret26_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret26_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret26_reg`] module"]
#[doc(alias = "RET26_REG")]
pub type Ret26Reg = crate::Reg<ret26_reg::Ret26RegSpec>;
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "RET27_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret27_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret27_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret27_reg`] module"]
#[doc(alias = "RET27_REG")]
pub type Ret27Reg = crate::Reg<ret27_reg::Ret27RegSpec>;
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "RET28_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret28_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret28_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret28_reg`] module"]
#[doc(alias = "RET28_REG")]
pub type Ret28Reg = crate::Reg<ret28_reg::Ret28RegSpec>;
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "RET29_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret29_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret29_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret29_reg`] module"]
#[doc(alias = "RET29_REG")]
pub type Ret29Reg = crate::Reg<ret29_reg::Ret29RegSpec>;
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "RET30_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret30_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret30_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret30_reg`] module"]
#[doc(alias = "RET30_REG")]
pub type Ret30Reg = crate::Reg<ret30_reg::Ret30RegSpec>;
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "RET31_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ret31_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret31_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ret31_reg`] module"]
#[doc(alias = "RET31_REG")]
pub type Ret31Reg = crate::Reg<ret31_reg::Ret31RegSpec>;
#[doc = "Retention Register"]
pub mod ret31_reg;
