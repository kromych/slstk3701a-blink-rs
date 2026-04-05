#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    lock: Lock,
    ram0ctrl: Ram0ctrl,
    cmd: Cmd,
    _reserved5: [u8; 0x04],
    em4ctrl: Em4ctrl,
    templimits: Templimits,
    temp: Temp,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    pwrlock: Pwrlock,
    _reserved13: [u8; 0x04],
    pwrctrl: Pwrctrl,
    dcdcctrl: Dcdcctrl,
    _reserved15: [u8; 0x08],
    dcdcmiscctrl: Dcdcmiscctrl,
    dcdczdetctrl: Dcdczdetctrl,
    dcdcclimctrl: Dcdcclimctrl,
    dcdclncompctrl: Dcdclncompctrl,
    dcdclnvctrl: Dcdclnvctrl,
    _reserved20: [u8; 0x04],
    dcdclpvctrl: Dcdclpvctrl,
    _reserved21: [u8; 0x04],
    dcdclpctrl: Dcdclpctrl,
    dcdclnfreqctrl: Dcdclnfreqctrl,
    _reserved23: [u8; 0x04],
    dcdcsync: Dcdcsync,
    _reserved24: [u8; 0x14],
    vmonavddctrl: Vmonavddctrl,
    vmonaltavddctrl: Vmonaltavddctrl,
    vmondvddctrl: Vmondvddctrl,
    vmonio0ctrl: Vmonio0ctrl,
    vmonio1ctrl: Vmonio1ctrl,
    vmonbuvddctrl: Vmonbuvddctrl,
    _reserved30: [u8; 0x0c],
    ram1ctrl: Ram1ctrl,
    ram2ctrl: Ram2ctrl,
    buctrl: Buctrl,
    _reserved33: [u8; 0x08],
    r5vctrl: R5vctrl,
    r5vadcctrl: R5vadcctrl,
    r5voutlevel: R5voutlevel,
    _reserved36: [u8; 0x08],
    r5vdetctrl: R5vdetctrl,
    _reserved37: [u8; 0x0c],
    dcdclpem01cfg: Dcdclpem01cfg,
    r5vstatus: R5vstatus,
    _reserved39: [u8; 0x04],
    r5vsync: R5vsync,
    _reserved40: [u8; 0x04],
    em23pernoretaincmd: Em23pernoretaincmd,
    em23pernoretainstatus: Em23pernoretainstatus,
    em23pernoretainctrl: Em23pernoretainctrl,
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
    #[doc = "0x08 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x0c - Memory Control Register"]
    #[inline(always)]
    pub const fn ram0ctrl(&self) -> &Ram0ctrl {
        &self.ram0ctrl
    }
    #[doc = "0x10 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x18 - EM4 Control Register"]
    #[inline(always)]
    pub const fn em4ctrl(&self) -> &Em4ctrl {
        &self.em4ctrl
    }
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    #[inline(always)]
    pub const fn templimits(&self) -> &Templimits {
        &self.templimits
    }
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0x24 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x28 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x30 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    #[inline(always)]
    pub const fn pwrlock(&self) -> &Pwrlock {
        &self.pwrlock
    }
    #[doc = "0x3c - Power Control Register"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &Pwrctrl {
        &self.pwrctrl
    }
    #[doc = "0x40 - DCDC Control"]
    #[inline(always)]
    pub const fn dcdcctrl(&self) -> &Dcdcctrl {
        &self.dcdcctrl
    }
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn dcdcmiscctrl(&self) -> &Dcdcmiscctrl {
        &self.dcdcmiscctrl
    }
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    #[inline(always)]
    pub const fn dcdczdetctrl(&self) -> &Dcdczdetctrl {
        &self.dcdczdetctrl
    }
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    #[inline(always)]
    pub const fn dcdcclimctrl(&self) -> &Dcdcclimctrl {
        &self.dcdcclimctrl
    }
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    #[inline(always)]
    pub const fn dcdclncompctrl(&self) -> &Dcdclncompctrl {
        &self.dcdclncompctrl
    }
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    #[inline(always)]
    pub const fn dcdclnvctrl(&self) -> &Dcdclnvctrl {
        &self.dcdclnvctrl
    }
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    #[inline(always)]
    pub const fn dcdclpvctrl(&self) -> &Dcdclpvctrl {
        &self.dcdclpvctrl
    }
    #[doc = "0x6c - DCDC Low Power Control Register"]
    #[inline(always)]
    pub const fn dcdclpctrl(&self) -> &Dcdclpctrl {
        &self.dcdclpctrl
    }
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    #[inline(always)]
    pub const fn dcdclnfreqctrl(&self) -> &Dcdclnfreqctrl {
        &self.dcdclnfreqctrl
    }
    #[doc = "0x78 - DCDC Read Status Register"]
    #[inline(always)]
    pub const fn dcdcsync(&self) -> &Dcdcsync {
        &self.dcdcsync
    }
    #[doc = "0x90 - VMON AVDD Channel Control"]
    #[inline(always)]
    pub const fn vmonavddctrl(&self) -> &Vmonavddctrl {
        &self.vmonavddctrl
    }
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    #[inline(always)]
    pub const fn vmonaltavddctrl(&self) -> &Vmonaltavddctrl {
        &self.vmonaltavddctrl
    }
    #[doc = "0x98 - VMON DVDD Channel Control"]
    #[inline(always)]
    pub const fn vmondvddctrl(&self) -> &Vmondvddctrl {
        &self.vmondvddctrl
    }
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    #[inline(always)]
    pub const fn vmonio0ctrl(&self) -> &Vmonio0ctrl {
        &self.vmonio0ctrl
    }
    #[doc = "0xa0 - VMON IOVDD1 Channel Control"]
    #[inline(always)]
    pub const fn vmonio1ctrl(&self) -> &Vmonio1ctrl {
        &self.vmonio1ctrl
    }
    #[doc = "0xa4 - VMON BUVDD Channel Control"]
    #[inline(always)]
    pub const fn vmonbuvddctrl(&self) -> &Vmonbuvddctrl {
        &self.vmonbuvddctrl
    }
    #[doc = "0xb4 - Memory Control Register"]
    #[inline(always)]
    pub const fn ram1ctrl(&self) -> &Ram1ctrl {
        &self.ram1ctrl
    }
    #[doc = "0xb8 - Memory Control Register"]
    #[inline(always)]
    pub const fn ram2ctrl(&self) -> &Ram2ctrl {
        &self.ram2ctrl
    }
    #[doc = "0xbc - Backup Power Configuration Register"]
    #[inline(always)]
    pub const fn buctrl(&self) -> &Buctrl {
        &self.buctrl
    }
    #[doc = "0xc8 - 5V Regulator Control"]
    #[inline(always)]
    pub const fn r5vctrl(&self) -> &R5vctrl {
        &self.r5vctrl
    }
    #[doc = "0xcc - 5V Regulator Control"]
    #[inline(always)]
    pub const fn r5vadcctrl(&self) -> &R5vadcctrl {
        &self.r5vadcctrl
    }
    #[doc = "0xd0 - 5V Regulator Voltage Select"]
    #[inline(always)]
    pub const fn r5voutlevel(&self) -> &R5voutlevel {
        &self.r5voutlevel
    }
    #[doc = "0xdc - 5V Detector Enables"]
    #[inline(always)]
    pub const fn r5vdetctrl(&self) -> &R5vdetctrl {
        &self.r5vdetctrl
    }
    #[doc = "0xec - Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
    #[inline(always)]
    pub const fn dcdclpem01cfg(&self) -> &Dcdclpem01cfg {
        &self.dcdclpem01cfg
    }
    #[doc = "0xf0 - 5V Detector Status Register"]
    #[inline(always)]
    pub const fn r5vstatus(&self) -> &R5vstatus {
        &self.r5vstatus
    }
    #[doc = "0xf8 - 5V Read Status Register"]
    #[inline(always)]
    pub const fn r5vsync(&self) -> &R5vsync {
        &self.r5vsync
    }
    #[doc = "0x100 - Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
    #[inline(always)]
    pub const fn em23pernoretaincmd(&self) -> &Em23pernoretaincmd {
        &self.em23pernoretaincmd
    }
    #[doc = "0x104 - Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
    #[inline(always)]
    pub const fn em23pernoretainstatus(&self) -> &Em23pernoretainstatus {
        &self.em23pernoretainstatus
    }
    #[doc = "0x108 - When Set Corresponding Peripherals May Get Powered Down in EM23"]
    #[inline(always)]
    pub const fn em23pernoretainctrl(&self) -> &Em23pernoretainctrl {
        &self.em23pernoretainctrl
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
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "RAM0CTRL (rw) register accessor: Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0ctrl`] module"]
#[doc(alias = "RAM0CTRL")]
pub type Ram0ctrl = crate::Reg<ram0ctrl::Ram0ctrlSpec>;
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4CTRL (rw) register accessor: EM4 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4ctrl`] module"]
#[doc(alias = "EM4CTRL")]
pub type Em4ctrl = crate::Reg<em4ctrl::Em4ctrlSpec>;
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "TEMPLIMITS (rw) register accessor: Temperature Limits for Interrupt Generation\n\nYou can [`read`](crate::Reg::read) this register and get [`templimits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`templimits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@templimits`] module"]
#[doc(alias = "TEMPLIMITS")]
pub type Templimits = crate::Reg<templimits::TemplimitsSpec>;
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "TEMP (r) register accessor: Value of Last Temperature Measurement\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`] module"]
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TempSpec>;
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
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
#[doc = "PWRLOCK (rw) register accessor: Regulator and Supply Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrlock`] module"]
#[doc(alias = "PWRLOCK")]
pub type Pwrlock = crate::Reg<pwrlock::PwrlockSpec>;
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "PWRCTRL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`] module"]
#[doc(alias = "PWRCTRL")]
pub type Pwrctrl = crate::Reg<pwrctrl::PwrctrlSpec>;
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDCCTRL (rw) register accessor: DCDC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcctrl`] module"]
#[doc(alias = "DCDCCTRL")]
pub type Dcdcctrl = crate::Reg<dcdcctrl::DcdcctrlSpec>;
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDCMISCCTRL (rw) register accessor: DCDC Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcmiscctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcmiscctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcmiscctrl`] module"]
#[doc(alias = "DCDCMISCCTRL")]
pub type Dcdcmiscctrl = crate::Reg<dcdcmiscctrl::DcdcmiscctrlSpec>;
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDCZDETCTRL (rw) register accessor: DCDC Power Train NFET Zero Current Detector Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdczdetctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdczdetctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdczdetctrl`] module"]
#[doc(alias = "DCDCZDETCTRL")]
pub type Dcdczdetctrl = crate::Reg<dcdczdetctrl::DcdczdetctrlSpec>;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDCCLIMCTRL (rw) register accessor: DCDC Power Train PFET Current Limiter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcclimctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcclimctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcclimctrl`] module"]
#[doc(alias = "DCDCCLIMCTRL")]
pub type Dcdcclimctrl = crate::Reg<dcdcclimctrl::DcdcclimctrlSpec>;
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDCLNCOMPCTRL (rw) register accessor: DCDC Low Noise Compensator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclncompctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclncompctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclncompctrl`] module"]
#[doc(alias = "DCDCLNCOMPCTRL")]
pub type Dcdclncompctrl = crate::Reg<dcdclncompctrl::DcdclncompctrlSpec>;
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDCLNVCTRL (rw) register accessor: DCDC Low Noise Voltage Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclnvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclnvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclnvctrl`] module"]
#[doc(alias = "DCDCLNVCTRL")]
pub type Dcdclnvctrl = crate::Reg<dcdclnvctrl::DcdclnvctrlSpec>;
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDCLPVCTRL (rw) register accessor: DCDC Low Power Voltage Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclpvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclpvctrl`] module"]
#[doc(alias = "DCDCLPVCTRL")]
pub type Dcdclpvctrl = crate::Reg<dcdclpvctrl::DcdclpvctrlSpec>;
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDCLPCTRL (rw) register accessor: DCDC Low Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclpctrl`] module"]
#[doc(alias = "DCDCLPCTRL")]
pub type Dcdclpctrl = crate::Reg<dcdclpctrl::DcdclpctrlSpec>;
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDCLNFREQCTRL (rw) register accessor: DCDC Low Noise Controller Frequency Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclnfreqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclnfreqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclnfreqctrl`] module"]
#[doc(alias = "DCDCLNFREQCTRL")]
pub type Dcdclnfreqctrl = crate::Reg<dcdclnfreqctrl::DcdclnfreqctrlSpec>;
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDCSYNC (r) register accessor: DCDC Read Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcsync::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcsync`] module"]
#[doc(alias = "DCDCSYNC")]
pub type Dcdcsync = crate::Reg<dcdcsync::DcdcsyncSpec>;
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMONAVDDCTRL (rw) register accessor: VMON AVDD Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonavddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonavddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonavddctrl`] module"]
#[doc(alias = "VMONAVDDCTRL")]
pub type Vmonavddctrl = crate::Reg<vmonavddctrl::VmonavddctrlSpec>;
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "VMONALTAVDDCTRL (rw) register accessor: Alternate VMON AVDD Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonaltavddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonaltavddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonaltavddctrl`] module"]
#[doc(alias = "VMONALTAVDDCTRL")]
pub type Vmonaltavddctrl = crate::Reg<vmonaltavddctrl::VmonaltavddctrlSpec>;
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMONDVDDCTRL (rw) register accessor: VMON DVDD Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmondvddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmondvddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmondvddctrl`] module"]
#[doc(alias = "VMONDVDDCTRL")]
pub type Vmondvddctrl = crate::Reg<vmondvddctrl::VmondvddctrlSpec>;
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMONIO0CTRL (rw) register accessor: VMON IOVDD0 Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonio0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonio0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonio0ctrl`] module"]
#[doc(alias = "VMONIO0CTRL")]
pub type Vmonio0ctrl = crate::Reg<vmonio0ctrl::Vmonio0ctrlSpec>;
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "VMONIO1CTRL (rw) register accessor: VMON IOVDD1 Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonio1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonio1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonio1ctrl`] module"]
#[doc(alias = "VMONIO1CTRL")]
pub type Vmonio1ctrl = crate::Reg<vmonio1ctrl::Vmonio1ctrlSpec>;
#[doc = "VMON IOVDD1 Channel Control"]
pub mod vmonio1ctrl;
#[doc = "VMONBUVDDCTRL (rw) register accessor: VMON BUVDD Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vmonbuvddctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmonbuvddctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmonbuvddctrl`] module"]
#[doc(alias = "VMONBUVDDCTRL")]
pub type Vmonbuvddctrl = crate::Reg<vmonbuvddctrl::VmonbuvddctrlSpec>;
#[doc = "VMON BUVDD Channel Control"]
pub mod vmonbuvddctrl;
#[doc = "RAM1CTRL (rw) register accessor: Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1ctrl`] module"]
#[doc(alias = "RAM1CTRL")]
pub type Ram1ctrl = crate::Reg<ram1ctrl::Ram1ctrlSpec>;
#[doc = "Memory Control Register"]
pub mod ram1ctrl;
#[doc = "RAM2CTRL (rw) register accessor: Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2ctrl`] module"]
#[doc(alias = "RAM2CTRL")]
pub type Ram2ctrl = crate::Reg<ram2ctrl::Ram2ctrlSpec>;
#[doc = "Memory Control Register"]
pub mod ram2ctrl;
#[doc = "BUCTRL (rw) register accessor: Backup Power Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buctrl`] module"]
#[doc(alias = "BUCTRL")]
pub type Buctrl = crate::Reg<buctrl::BuctrlSpec>;
#[doc = "Backup Power Configuration Register"]
pub mod buctrl;
#[doc = "R5VCTRL (rw) register accessor: 5V Regulator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5vctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vctrl`] module"]
#[doc(alias = "R5VCTRL")]
pub type R5vctrl = crate::Reg<r5vctrl::R5vctrlSpec>;
#[doc = "5V Regulator Control"]
pub mod r5vctrl;
#[doc = "R5VADCCTRL (rw) register accessor: 5V Regulator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vadcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5vadcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vadcctrl`] module"]
#[doc(alias = "R5VADCCTRL")]
pub type R5vadcctrl = crate::Reg<r5vadcctrl::R5vadcctrlSpec>;
#[doc = "5V Regulator Control"]
pub mod r5vadcctrl;
#[doc = "R5VOUTLEVEL (rw) register accessor: 5V Regulator Voltage Select\n\nYou can [`read`](crate::Reg::read) this register and get [`r5voutlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5voutlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5voutlevel`] module"]
#[doc(alias = "R5VOUTLEVEL")]
pub type R5voutlevel = crate::Reg<r5voutlevel::R5voutlevelSpec>;
#[doc = "5V Regulator Voltage Select"]
pub mod r5voutlevel;
#[doc = "R5VDETCTRL (rw) register accessor: 5V Detector Enables\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vdetctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5vdetctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vdetctrl`] module"]
#[doc(alias = "R5VDETCTRL")]
pub type R5vdetctrl = crate::Reg<r5vdetctrl::R5vdetctrlSpec>;
#[doc = "5V Detector Enables"]
pub mod r5vdetctrl;
#[doc = "DCDCLPEM01CFG (rw) register accessor: Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclpem01cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpem01cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdclpem01cfg`] module"]
#[doc(alias = "DCDCLPEM01CFG")]
pub type Dcdclpem01cfg = crate::Reg<dcdclpem01cfg::Dcdclpem01cfgSpec>;
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
pub mod dcdclpem01cfg;
#[doc = "R5VSTATUS (r) register accessor: 5V Detector Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vstatus`] module"]
#[doc(alias = "R5VSTATUS")]
pub type R5vstatus = crate::Reg<r5vstatus::R5vstatusSpec>;
#[doc = "5V Detector Status Register"]
pub mod r5vstatus;
#[doc = "R5VSYNC (r) register accessor: 5V Read Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r5vsync::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r5vsync`] module"]
#[doc(alias = "R5VSYNC")]
pub type R5vsync = crate::Reg<r5vsync::R5vsyncSpec>;
#[doc = "5V Read Status Register"]
pub mod r5vsync;
#[doc = "EM23PERNORETAINCMD (w) register accessor: Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23pernoretaincmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23pernoretaincmd`] module"]
#[doc(alias = "EM23PERNORETAINCMD")]
pub type Em23pernoretaincmd = crate::Reg<em23pernoretaincmd::Em23pernoretaincmdSpec>;
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
pub mod em23pernoretaincmd;
#[doc = "EM23PERNORETAINSTATUS (r) register accessor: Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It\n\nYou can [`read`](crate::Reg::read) this register and get [`em23pernoretainstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23pernoretainstatus`] module"]
#[doc(alias = "EM23PERNORETAINSTATUS")]
pub type Em23pernoretainstatus = crate::Reg<em23pernoretainstatus::Em23pernoretainstatusSpec>;
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
pub mod em23pernoretainstatus;
#[doc = "EM23PERNORETAINCTRL (rw) register accessor: When Set Corresponding Peripherals May Get Powered Down in EM23\n\nYou can [`read`](crate::Reg::read) this register and get [`em23pernoretainctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23pernoretainctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23pernoretainctrl`] module"]
#[doc(alias = "EM23PERNORETAINCTRL")]
pub type Em23pernoretainctrl = crate::Reg<em23pernoretainctrl::Em23pernoretainctrlSpec>;
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23"]
pub mod em23pernoretainctrl;
