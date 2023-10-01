#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Pre-Counter Value Register"]
    pub precnt: PRECNT,
    #[doc = "0x08 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x0c - Combined Pre-Counter and Counter Value Register"]
    pub combcnt: COMBCNT,
    #[doc = "0x10 - Time of Day Register"]
    pub time: TIME,
    #[doc = "0x14 - Date Register"]
    pub date: DATE,
    #[doc = "0x18 - RTCC Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x28 - Status Register"]
    pub status: STATUS,
    #[doc = "0x2c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x30 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x34 - Retention RAM Power-down Register"]
    pub powerdown: POWERDOWN,
    #[doc = "0x38 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x3c - Wake Up Enable"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x44 - Capture/Compare Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x48 - Capture/Compare Time Register"]
    pub cc0_time: CC0_TIME,
    #[doc = "0x4c - Capture/Compare Date Register"]
    pub cc0_date: CC0_DATE,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x54 - Capture/Compare Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x58 - Capture/Compare Time Register"]
    pub cc1_time: CC1_TIME,
    #[doc = "0x5c - Capture/Compare Date Register"]
    pub cc1_date: CC1_DATE,
    #[doc = "0x60 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x64 - Capture/Compare Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x68 - Capture/Compare Time Register"]
    pub cc2_time: CC2_TIME,
    #[doc = "0x6c - Capture/Compare Date Register"]
    pub cc2_date: CC2_DATE,
    _reserved28: [u8; 0x94],
    #[doc = "0x104 - Retention Register"]
    pub ret0_reg: RET0_REG,
    #[doc = "0x108 - Retention Register"]
    pub ret1_reg: RET1_REG,
    #[doc = "0x10c - Retention Register"]
    pub ret2_reg: RET2_REG,
    #[doc = "0x110 - Retention Register"]
    pub ret3_reg: RET3_REG,
    #[doc = "0x114 - Retention Register"]
    pub ret4_reg: RET4_REG,
    #[doc = "0x118 - Retention Register"]
    pub ret5_reg: RET5_REG,
    #[doc = "0x11c - Retention Register"]
    pub ret6_reg: RET6_REG,
    #[doc = "0x120 - Retention Register"]
    pub ret7_reg: RET7_REG,
    #[doc = "0x124 - Retention Register"]
    pub ret8_reg: RET8_REG,
    #[doc = "0x128 - Retention Register"]
    pub ret9_reg: RET9_REG,
    #[doc = "0x12c - Retention Register"]
    pub ret10_reg: RET10_REG,
    #[doc = "0x130 - Retention Register"]
    pub ret11_reg: RET11_REG,
    #[doc = "0x134 - Retention Register"]
    pub ret12_reg: RET12_REG,
    #[doc = "0x138 - Retention Register"]
    pub ret13_reg: RET13_REG,
    #[doc = "0x13c - Retention Register"]
    pub ret14_reg: RET14_REG,
    #[doc = "0x140 - Retention Register"]
    pub ret15_reg: RET15_REG,
    #[doc = "0x144 - Retention Register"]
    pub ret16_reg: RET16_REG,
    #[doc = "0x148 - Retention Register"]
    pub ret17_reg: RET17_REG,
    #[doc = "0x14c - Retention Register"]
    pub ret18_reg: RET18_REG,
    #[doc = "0x150 - Retention Register"]
    pub ret19_reg: RET19_REG,
    #[doc = "0x154 - Retention Register"]
    pub ret20_reg: RET20_REG,
    #[doc = "0x158 - Retention Register"]
    pub ret21_reg: RET21_REG,
    #[doc = "0x15c - Retention Register"]
    pub ret22_reg: RET22_REG,
    #[doc = "0x160 - Retention Register"]
    pub ret23_reg: RET23_REG,
    #[doc = "0x164 - Retention Register"]
    pub ret24_reg: RET24_REG,
    #[doc = "0x168 - Retention Register"]
    pub ret25_reg: RET25_REG,
    #[doc = "0x16c - Retention Register"]
    pub ret26_reg: RET26_REG,
    #[doc = "0x170 - Retention Register"]
    pub ret27_reg: RET27_REG,
    #[doc = "0x174 - Retention Register"]
    pub ret28_reg: RET28_REG,
    #[doc = "0x178 - Retention Register"]
    pub ret29_reg: RET29_REG,
    #[doc = "0x17c - Retention Register"]
    pub ret30_reg: RET30_REG,
    #[doc = "0x180 - Retention Register"]
    pub ret31_reg: RET31_REG,
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "PRECNT (rw) register accessor: Pre-Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`precnt`] module"]
pub type PRECNT = crate::Reg<precnt::PRECNT_SPEC>;
#[doc = "Pre-Counter Value Register"]
pub mod precnt;
#[doc = "CNT (rw) register accessor: Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`] module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "COMBCNT (r) register accessor: Combined Pre-Counter and Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`combcnt`] module"]
pub type COMBCNT = crate::Reg<combcnt::COMBCNT_SPEC>;
#[doc = "Combined Pre-Counter and Counter Value Register"]
pub mod combcnt;
#[doc = "TIME (rw) register accessor: Time of Day Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time`] module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Time of Day Register"]
pub mod time;
#[doc = "DATE (rw) register accessor: Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date Register"]
pub mod date;
#[doc = "IF (r) register accessor: RTCC Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if_`] module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "RTCC Interrupt Flags"]
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
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`syncbusy`] module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "POWERDOWN (rw) register accessor: Retention RAM Power-down Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerdown::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerdown::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`powerdown`] module"]
pub type POWERDOWN = crate::Reg<powerdown::POWERDOWN_SPEC>;
#[doc = "Retention RAM Power-down Register"]
pub mod powerdown;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lock`] module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "EM4WUEN (rw) register accessor: Wake Up Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`em4wuen`] module"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "Wake Up Enable"]
pub mod em4wuen;
#[doc = "CC0_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc0_ctrl`] module"]
pub type CC0_CTRL = crate::Reg<cc0_ctrl::CC0_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "CC0_CCV (rw) register accessor: Capture/Compare Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_ccv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc0_ccv`] module"]
pub type CC0_CCV = crate::Reg<cc0_ccv::CC0_CCV_SPEC>;
#[doc = "Capture/Compare Value Register"]
pub mod cc0_ccv;
#[doc = "CC0_TIME (rw) register accessor: Capture/Compare Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc0_time`] module"]
pub type CC0_TIME = crate::Reg<cc0_time::CC0_TIME_SPEC>;
#[doc = "Capture/Compare Time Register"]
pub mod cc0_time;
#[doc = "CC0_DATE (rw) register accessor: Capture/Compare Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc0_date`] module"]
pub type CC0_DATE = crate::Reg<cc0_date::CC0_DATE_SPEC>;
#[doc = "Capture/Compare Date Register"]
pub mod cc0_date;
#[doc = "CC1_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc1_ctrl`] module"]
pub type CC1_CTRL = crate::Reg<cc1_ctrl::CC1_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "CC1_CCV (rw) register accessor: Capture/Compare Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_ccv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc1_ccv`] module"]
pub type CC1_CCV = crate::Reg<cc1_ccv::CC1_CCV_SPEC>;
#[doc = "Capture/Compare Value Register"]
pub mod cc1_ccv;
#[doc = "CC1_TIME (rw) register accessor: Capture/Compare Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc1_time`] module"]
pub type CC1_TIME = crate::Reg<cc1_time::CC1_TIME_SPEC>;
#[doc = "Capture/Compare Time Register"]
pub mod cc1_time;
#[doc = "CC1_DATE (rw) register accessor: Capture/Compare Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc1_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc1_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc1_date`] module"]
pub type CC1_DATE = crate::Reg<cc1_date::CC1_DATE_SPEC>;
#[doc = "Capture/Compare Date Register"]
pub mod cc1_date;
#[doc = "CC2_CTRL (rw) register accessor: CC Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc2_ctrl`] module"]
pub type CC2_CTRL = crate::Reg<cc2_ctrl::CC2_CTRL_SPEC>;
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "CC2_CCV (rw) register accessor: Capture/Compare Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ccv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ccv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc2_ccv`] module"]
pub type CC2_CCV = crate::Reg<cc2_ccv::CC2_CCV_SPEC>;
#[doc = "Capture/Compare Value Register"]
pub mod cc2_ccv;
#[doc = "CC2_TIME (rw) register accessor: Capture/Compare Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc2_time`] module"]
pub type CC2_TIME = crate::Reg<cc2_time::CC2_TIME_SPEC>;
#[doc = "Capture/Compare Time Register"]
pub mod cc2_time;
#[doc = "CC2_DATE (rw) register accessor: Capture/Compare Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc2_date`] module"]
pub type CC2_DATE = crate::Reg<cc2_date::CC2_DATE_SPEC>;
#[doc = "Capture/Compare Date Register"]
pub mod cc2_date;
#[doc = "RET0_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret0_reg`] module"]
pub type RET0_REG = crate::Reg<ret0_reg::RET0_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "RET1_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret1_reg`] module"]
pub type RET1_REG = crate::Reg<ret1_reg::RET1_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "RET2_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret2_reg`] module"]
pub type RET2_REG = crate::Reg<ret2_reg::RET2_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "RET3_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret3_reg`] module"]
pub type RET3_REG = crate::Reg<ret3_reg::RET3_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "RET4_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret4_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret4_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret4_reg`] module"]
pub type RET4_REG = crate::Reg<ret4_reg::RET4_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "RET5_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret5_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret5_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret5_reg`] module"]
pub type RET5_REG = crate::Reg<ret5_reg::RET5_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "RET6_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret6_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret6_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret6_reg`] module"]
pub type RET6_REG = crate::Reg<ret6_reg::RET6_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "RET7_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret7_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret7_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret7_reg`] module"]
pub type RET7_REG = crate::Reg<ret7_reg::RET7_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "RET8_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret8_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret8_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret8_reg`] module"]
pub type RET8_REG = crate::Reg<ret8_reg::RET8_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "RET9_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret9_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret9_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret9_reg`] module"]
pub type RET9_REG = crate::Reg<ret9_reg::RET9_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "RET10_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret10_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret10_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret10_reg`] module"]
pub type RET10_REG = crate::Reg<ret10_reg::RET10_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "RET11_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret11_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret11_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret11_reg`] module"]
pub type RET11_REG = crate::Reg<ret11_reg::RET11_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "RET12_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret12_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret12_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret12_reg`] module"]
pub type RET12_REG = crate::Reg<ret12_reg::RET12_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "RET13_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret13_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret13_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret13_reg`] module"]
pub type RET13_REG = crate::Reg<ret13_reg::RET13_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "RET14_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret14_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret14_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret14_reg`] module"]
pub type RET14_REG = crate::Reg<ret14_reg::RET14_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "RET15_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret15_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret15_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret15_reg`] module"]
pub type RET15_REG = crate::Reg<ret15_reg::RET15_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "RET16_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret16_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret16_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret16_reg`] module"]
pub type RET16_REG = crate::Reg<ret16_reg::RET16_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "RET17_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret17_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret17_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret17_reg`] module"]
pub type RET17_REG = crate::Reg<ret17_reg::RET17_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "RET18_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret18_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret18_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret18_reg`] module"]
pub type RET18_REG = crate::Reg<ret18_reg::RET18_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "RET19_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret19_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret19_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret19_reg`] module"]
pub type RET19_REG = crate::Reg<ret19_reg::RET19_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "RET20_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret20_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret20_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret20_reg`] module"]
pub type RET20_REG = crate::Reg<ret20_reg::RET20_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "RET21_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret21_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret21_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret21_reg`] module"]
pub type RET21_REG = crate::Reg<ret21_reg::RET21_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "RET22_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret22_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret22_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret22_reg`] module"]
pub type RET22_REG = crate::Reg<ret22_reg::RET22_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "RET23_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret23_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret23_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret23_reg`] module"]
pub type RET23_REG = crate::Reg<ret23_reg::RET23_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "RET24_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret24_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret24_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret24_reg`] module"]
pub type RET24_REG = crate::Reg<ret24_reg::RET24_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "RET25_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret25_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret25_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret25_reg`] module"]
pub type RET25_REG = crate::Reg<ret25_reg::RET25_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "RET26_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret26_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret26_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret26_reg`] module"]
pub type RET26_REG = crate::Reg<ret26_reg::RET26_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "RET27_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret27_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret27_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret27_reg`] module"]
pub type RET27_REG = crate::Reg<ret27_reg::RET27_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "RET28_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret28_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret28_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret28_reg`] module"]
pub type RET28_REG = crate::Reg<ret28_reg::RET28_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "RET29_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret29_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret29_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret29_reg`] module"]
pub type RET29_REG = crate::Reg<ret29_reg::RET29_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "RET30_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret30_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret30_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret30_reg`] module"]
pub type RET30_REG = crate::Reg<ret30_reg::RET30_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "RET31_REG (rw) register accessor: Retention Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ret31_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ret31_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ret31_reg`] module"]
pub type RET31_REG = crate::Reg<ret31_reg::RET31_REG_SPEC>;
#[doc = "Retention Register"]
pub mod ret31_reg;
