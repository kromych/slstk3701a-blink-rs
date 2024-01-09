#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    lock: LOCK,
    ram0ctrl: RAM0CTRL,
    cmd: CMD,
    _reserved5: [u8; 0x04],
    em4ctrl: EM4CTRL,
    templimits: TEMPLIMITS,
    temp: TEMP,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    pwrlock: PWRLOCK,
    _reserved13: [u8; 0x04],
    pwrctrl: PWRCTRL,
    dcdcctrl: DCDCCTRL,
    _reserved15: [u8; 0x08],
    dcdcmiscctrl: DCDCMISCCTRL,
    dcdczdetctrl: DCDCZDETCTRL,
    dcdcclimctrl: DCDCCLIMCTRL,
    dcdclncompctrl: DCDCLNCOMPCTRL,
    dcdclnvctrl: DCDCLNVCTRL,
    _reserved20: [u8; 0x04],
    dcdclpvctrl: DCDCLPVCTRL,
    _reserved21: [u8; 0x04],
    dcdclpctrl: DCDCLPCTRL,
    dcdclnfreqctrl: DCDCLNFREQCTRL,
    _reserved23: [u8; 0x04],
    dcdcsync: DCDCSYNC,
    _reserved24: [u8; 0x14],
    vmonavddctrl: VMONAVDDCTRL,
    vmonaltavddctrl: VMONALTAVDDCTRL,
    vmondvddctrl: VMONDVDDCTRL,
    vmonio0ctrl: VMONIO0CTRL,
    vmonio1ctrl: VMONIO1CTRL,
    vmonbuvddctrl: VMONBUVDDCTRL,
    _reserved30: [u8; 0x0c],
    ram1ctrl: RAM1CTRL,
    ram2ctrl: RAM2CTRL,
    buctrl: BUCTRL,
    _reserved33: [u8; 0x08],
    r5vctrl: R5VCTRL,
    r5vadcctrl: R5VADCCTRL,
    r5voutlevel: R5VOUTLEVEL,
    _reserved36: [u8; 0x08],
    r5vdetctrl: R5VDETCTRL,
    _reserved37: [u8; 0x0c],
    dcdclpem01cfg: DCDCLPEM01CFG,
    r5vstatus: R5VSTATUS,
    _reserved39: [u8; 0x04],
    r5vsync: R5VSYNC,
    _reserved40: [u8; 0x04],
    em23pernoretaincmd: EM23PERNORETAINCMD,
    em23pernoretainstatus: EM23PERNORETAINSTATUS,
    em23pernoretainctrl: EM23PERNORETAINCTRL,
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
    #[doc = "0x08 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x0c - Memory Control Register"]
    #[inline(always)]
    pub const fn ram0ctrl(&self) -> &RAM0CTRL {
        &self.ram0ctrl
    }
    #[doc = "0x10 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x18 - EM4 Control Register"]
    #[inline(always)]
    pub const fn em4ctrl(&self) -> &EM4CTRL {
        &self.em4ctrl
    }
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    #[inline(always)]
    pub const fn templimits(&self) -> &TEMPLIMITS {
        &self.templimits
    }
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    #[inline(always)]
    pub const fn temp(&self) -> &TEMP {
        &self.temp
    }
    #[doc = "0x24 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x28 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x30 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    #[inline(always)]
    pub const fn pwrlock(&self) -> &PWRLOCK {
        &self.pwrlock
    }
    #[doc = "0x3c - Power Control Register"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &PWRCTRL {
        &self.pwrctrl
    }
    #[doc = "0x40 - DCDC Control"]
    #[inline(always)]
    pub const fn dcdcctrl(&self) -> &DCDCCTRL {
        &self.dcdcctrl
    }
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn dcdcmiscctrl(&self) -> &DCDCMISCCTRL {
        &self.dcdcmiscctrl
    }
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    #[inline(always)]
    pub const fn dcdczdetctrl(&self) -> &DCDCZDETCTRL {
        &self.dcdczdetctrl
    }
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    #[inline(always)]
    pub const fn dcdcclimctrl(&self) -> &DCDCCLIMCTRL {
        &self.dcdcclimctrl
    }
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    #[inline(always)]
    pub const fn dcdclncompctrl(&self) -> &DCDCLNCOMPCTRL {
        &self.dcdclncompctrl
    }
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    #[inline(always)]
    pub const fn dcdclnvctrl(&self) -> &DCDCLNVCTRL {
        &self.dcdclnvctrl
    }
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    #[inline(always)]
    pub const fn dcdclpvctrl(&self) -> &DCDCLPVCTRL {
        &self.dcdclpvctrl
    }
    #[doc = "0x6c - DCDC Low Power Control Register"]
    #[inline(always)]
    pub const fn dcdclpctrl(&self) -> &DCDCLPCTRL {
        &self.dcdclpctrl
    }
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    #[inline(always)]
    pub const fn dcdclnfreqctrl(&self) -> &DCDCLNFREQCTRL {
        &self.dcdclnfreqctrl
    }
    #[doc = "0x78 - DCDC Read Status Register"]
    #[inline(always)]
    pub const fn dcdcsync(&self) -> &DCDCSYNC {
        &self.dcdcsync
    }
    #[doc = "0x90 - VMON AVDD Channel Control"]
    #[inline(always)]
    pub const fn vmonavddctrl(&self) -> &VMONAVDDCTRL {
        &self.vmonavddctrl
    }
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    #[inline(always)]
    pub const fn vmonaltavddctrl(&self) -> &VMONALTAVDDCTRL {
        &self.vmonaltavddctrl
    }
    #[doc = "0x98 - VMON DVDD Channel Control"]
    #[inline(always)]
    pub const fn vmondvddctrl(&self) -> &VMONDVDDCTRL {
        &self.vmondvddctrl
    }
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    #[inline(always)]
    pub const fn vmonio0ctrl(&self) -> &VMONIO0CTRL {
        &self.vmonio0ctrl
    }
    #[doc = "0xa0 - VMON IOVDD1 Channel Control"]
    #[inline(always)]
    pub const fn vmonio1ctrl(&self) -> &VMONIO1CTRL {
        &self.vmonio1ctrl
    }
    #[doc = "0xa4 - VMON BUVDD Channel Control"]
    #[inline(always)]
    pub const fn vmonbuvddctrl(&self) -> &VMONBUVDDCTRL {
        &self.vmonbuvddctrl
    }
    #[doc = "0xb4 - Memory Control Register"]
    #[inline(always)]
    pub const fn ram1ctrl(&self) -> &RAM1CTRL {
        &self.ram1ctrl
    }
    #[doc = "0xb8 - Memory Control Register"]
    #[inline(always)]
    pub const fn ram2ctrl(&self) -> &RAM2CTRL {
        &self.ram2ctrl
    }
    #[doc = "0xbc - Backup Power Configuration Register"]
    #[inline(always)]
    pub const fn buctrl(&self) -> &BUCTRL {
        &self.buctrl
    }
    #[doc = "0xc8 - 5V Regulator Control"]
    #[inline(always)]
    pub const fn r5vctrl(&self) -> &R5VCTRL {
        &self.r5vctrl
    }
    #[doc = "0xcc - 5V Regulator Control"]
    #[inline(always)]
    pub const fn r5vadcctrl(&self) -> &R5VADCCTRL {
        &self.r5vadcctrl
    }
    #[doc = "0xd0 - 5V Regulator Voltage Select"]
    #[inline(always)]
    pub const fn r5voutlevel(&self) -> &R5VOUTLEVEL {
        &self.r5voutlevel
    }
    #[doc = "0xdc - 5V Detector Enables"]
    #[inline(always)]
    pub const fn r5vdetctrl(&self) -> &R5VDETCTRL {
        &self.r5vdetctrl
    }
    #[doc = "0xec - Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
    #[inline(always)]
    pub const fn dcdclpem01cfg(&self) -> &DCDCLPEM01CFG {
        &self.dcdclpem01cfg
    }
    #[doc = "0xf0 - 5V Detector Status Register"]
    #[inline(always)]
    pub const fn r5vstatus(&self) -> &R5VSTATUS {
        &self.r5vstatus
    }
    #[doc = "0xf8 - 5V Read Status Register"]
    #[inline(always)]
    pub const fn r5vsync(&self) -> &R5VSYNC {
        &self.r5vsync
    }
    #[doc = "0x100 - Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
    #[inline(always)]
    pub const fn em23pernoretaincmd(&self) -> &EM23PERNORETAINCMD {
        &self.em23pernoretaincmd
    }
    #[doc = "0x104 - Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
    #[inline(always)]
    pub const fn em23pernoretainstatus(&self) -> &EM23PERNORETAINSTATUS {
        &self.em23pernoretainstatus
    }
    #[doc = "0x108 - When Set Corresponding Peripherals May Get Powered Down in EM23"]
    #[inline(always)]
    pub const fn em23pernoretainctrl(&self) -> &EM23PERNORETAINCTRL {
        &self.em23pernoretainctrl
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
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "RAM0CTRL (rw) register accessor: Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram0ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram0ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0ctrl`]
module"]
pub type RAM0CTRL = crate::Reg<ram0ctrl::RAM0CTRL_SPEC>;
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4CTRL (rw) register accessor: EM4 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4ctrl`]
module"]
pub type EM4CTRL = crate::Reg<em4ctrl::EM4CTRL_SPEC>;
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "TEMPLIMITS (rw) register accessor: Temperature Limits for Interrupt Generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`templimits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`templimits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@templimits`]
module"]
pub type TEMPLIMITS = crate::Reg<templimits::TEMPLIMITS_SPEC>;
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "TEMP (r) register accessor: Value of Last Temperature Measurement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`]
module"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
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
#[doc = "PWRLOCK (rw) register accessor: Regulator and Supply Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrlock`]
module"]
pub type PWRLOCK = crate::Reg<pwrlock::PWRLOCK_SPEC>;
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "PWRCTRL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`]
module"]
pub type PWRCTRL = crate::Reg<pwrctrl::PWRCTRL_SPEC>;
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDCCTRL (rw) register accessor: DCDC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcctrl`]
module"]
pub type DCDCCTRL = crate::Reg<dcdcctrl::DCDCCTRL_SPEC>;
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDCMISCCTRL (rw) register accessor: DCDC Miscellaneous Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcmiscctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcmiscctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcmiscctrl`]
module"]
pub type DCDCMISCCTRL = crate::Reg<dcdcmiscctrl::DCDCMISCCTRL_SPEC>;
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDCZDETCTRL (rw) register accessor: DCDC Power Train NFET Zero Current Detector Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdczdetctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdczdetctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdczdetctrl`]
module"]
pub type DCDCZDETCTRL = crate::Reg<dcdczdetctrl::DCDCZDETCTRL_SPEC>;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDCCLIMCTRL (rw) register accessor: DCDC Power Train PFET Current Limiter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcclimctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdcclimctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcclimctrl`]
module"]
pub type DCDCCLIMCTRL = crate::Reg<dcdcclimctrl::DCDCCLIMCTRL_SPEC>;
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDCLNCOMPCTRL (rw) register accessor: DCDC Low Noise Compensator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclncompctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclncompctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclncompctrl`]
module"]
pub type DCDCLNCOMPCTRL = crate::Reg<dcdclncompctrl::DCDCLNCOMPCTRL_SPEC>;
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDCLNVCTRL (rw) register accessor: DCDC Low Noise Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclnvctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclnvctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclnvctrl`]
module"]
pub type DCDCLNVCTRL = crate::Reg<dcdclnvctrl::DCDCLNVCTRL_SPEC>;
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDCLPVCTRL (rw) register accessor: DCDC Low Power Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclpvctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclpvctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclpvctrl`]
module"]
pub type DCDCLPVCTRL = crate::Reg<dcdclpvctrl::DCDCLPVCTRL_SPEC>;
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDCLPCTRL (rw) register accessor: DCDC Low Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclpctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclpctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclpctrl`]
module"]
pub type DCDCLPCTRL = crate::Reg<dcdclpctrl::DCDCLPCTRL_SPEC>;
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDCLNFREQCTRL (rw) register accessor: DCDC Low Noise Controller Frequency Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclnfreqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclnfreqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclnfreqctrl`]
module"]
pub type DCDCLNFREQCTRL = crate::Reg<dcdclnfreqctrl::DCDCLNFREQCTRL_SPEC>;
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDCSYNC (r) register accessor: DCDC Read Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcsync::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcsync`]
module"]
pub type DCDCSYNC = crate::Reg<dcdcsync::DCDCSYNC_SPEC>;
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMONAVDDCTRL (rw) register accessor: VMON AVDD Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonavddctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonavddctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonavddctrl`]
module"]
pub type VMONAVDDCTRL = crate::Reg<vmonavddctrl::VMONAVDDCTRL_SPEC>;
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "VMONALTAVDDCTRL (rw) register accessor: Alternate VMON AVDD Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonaltavddctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonaltavddctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonaltavddctrl`]
module"]
pub type VMONALTAVDDCTRL = crate::Reg<vmonaltavddctrl::VMONALTAVDDCTRL_SPEC>;
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMONDVDDCTRL (rw) register accessor: VMON DVDD Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmondvddctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmondvddctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmondvddctrl`]
module"]
pub type VMONDVDDCTRL = crate::Reg<vmondvddctrl::VMONDVDDCTRL_SPEC>;
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMONIO0CTRL (rw) register accessor: VMON IOVDD0 Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonio0ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonio0ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonio0ctrl`]
module"]
pub type VMONIO0CTRL = crate::Reg<vmonio0ctrl::VMONIO0CTRL_SPEC>;
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "VMONIO1CTRL (rw) register accessor: VMON IOVDD1 Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonio1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonio1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonio1ctrl`]
module"]
pub type VMONIO1CTRL = crate::Reg<vmonio1ctrl::VMONIO1CTRL_SPEC>;
#[doc = "VMON IOVDD1 Channel Control"]
pub mod vmonio1ctrl;
#[doc = "VMONBUVDDCTRL (rw) register accessor: VMON BUVDD Channel Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmonbuvddctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmonbuvddctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonbuvddctrl`]
module"]
pub type VMONBUVDDCTRL = crate::Reg<vmonbuvddctrl::VMONBUVDDCTRL_SPEC>;
#[doc = "VMON BUVDD Channel Control"]
pub mod vmonbuvddctrl;
#[doc = "RAM1CTRL (rw) register accessor: Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1ctrl`]
module"]
pub type RAM1CTRL = crate::Reg<ram1ctrl::RAM1CTRL_SPEC>;
#[doc = "Memory Control Register"]
pub mod ram1ctrl;
#[doc = "RAM2CTRL (rw) register accessor: Memory Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2ctrl`]
module"]
pub type RAM2CTRL = crate::Reg<ram2ctrl::RAM2CTRL_SPEC>;
#[doc = "Memory Control Register"]
pub mod ram2ctrl;
#[doc = "BUCTRL (rw) register accessor: Backup Power Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buctrl`]
module"]
pub type BUCTRL = crate::Reg<buctrl::BUCTRL_SPEC>;
#[doc = "Backup Power Configuration Register"]
pub mod buctrl;
#[doc = "R5VCTRL (rw) register accessor: 5V Regulator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5vctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vctrl`]
module"]
pub type R5VCTRL = crate::Reg<r5vctrl::R5VCTRL_SPEC>;
#[doc = "5V Regulator Control"]
pub mod r5vctrl;
#[doc = "R5VADCCTRL (rw) register accessor: 5V Regulator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vadcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5vadcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vadcctrl`]
module"]
pub type R5VADCCTRL = crate::Reg<r5vadcctrl::R5VADCCTRL_SPEC>;
#[doc = "5V Regulator Control"]
pub mod r5vadcctrl;
#[doc = "R5VOUTLEVEL (rw) register accessor: 5V Regulator Voltage Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5voutlevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5voutlevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5voutlevel`]
module"]
pub type R5VOUTLEVEL = crate::Reg<r5voutlevel::R5VOUTLEVEL_SPEC>;
#[doc = "5V Regulator Voltage Select"]
pub mod r5voutlevel;
#[doc = "R5VDETCTRL (rw) register accessor: 5V Detector Enables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vdetctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5vdetctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vdetctrl`]
module"]
pub type R5VDETCTRL = crate::Reg<r5vdetctrl::R5VDETCTRL_SPEC>;
#[doc = "5V Detector Enables"]
pub mod r5vdetctrl;
#[doc = "DCDCLPEM01CFG (rw) register accessor: Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclpem01cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclpem01cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclpem01cfg`]
module"]
pub type DCDCLPEM01CFG = crate::Reg<dcdclpem01cfg::DCDCLPEM01CFG_SPEC>;
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
pub mod dcdclpem01cfg;
#[doc = "R5VSTATUS (r) register accessor: 5V Detector Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vstatus`]
module"]
pub type R5VSTATUS = crate::Reg<r5vstatus::R5VSTATUS_SPEC>;
#[doc = "5V Detector Status Register"]
pub mod r5vstatus;
#[doc = "R5VSYNC (r) register accessor: 5V Read Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vsync::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vsync`]
module"]
pub type R5VSYNC = crate::Reg<r5vsync::R5VSYNC_SPEC>;
#[doc = "5V Read Status Register"]
pub mod r5vsync;
#[doc = "EM23PERNORETAINCMD (w) register accessor: Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em23pernoretaincmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23pernoretaincmd`]
module"]
pub type EM23PERNORETAINCMD = crate::Reg<em23pernoretaincmd::EM23PERNORETAINCMD_SPEC>;
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
pub mod em23pernoretaincmd;
#[doc = "EM23PERNORETAINSTATUS (r) register accessor: Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em23pernoretainstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23pernoretainstatus`]
module"]
pub type EM23PERNORETAINSTATUS = crate::Reg<em23pernoretainstatus::EM23PERNORETAINSTATUS_SPEC>;
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
pub mod em23pernoretainstatus;
#[doc = "EM23PERNORETAINCTRL (rw) register accessor: When Set Corresponding Peripherals May Get Powered Down in EM23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em23pernoretainctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em23pernoretainctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23pernoretainctrl`]
module"]
pub type EM23PERNORETAINCTRL = crate::Reg<em23pernoretainctrl::EM23PERNORETAINCTRL_SPEC>;
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23"]
pub mod em23pernoretainctrl;
