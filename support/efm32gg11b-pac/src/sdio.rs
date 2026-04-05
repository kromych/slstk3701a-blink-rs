#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdmasysaddr: Sdmasysaddr,
    blksize: Blksize,
    cmdarg1: Cmdarg1,
    tfrmode: Tfrmode,
    resp0: Resp0,
    resp2: Resp2,
    resp4: Resp4,
    resp6: Resp6,
    bufdatport: Bufdatport,
    prsstat: Prsstat,
    hostctrl1: Hostctrl1,
    clockctrl: Clockctrl,
    ifcr: Ifcr,
    ifenc: Ifenc,
    ien: Ien,
    ac12errstat: Ac12errstat,
    capab0: Capab0,
    capab2: Capab2,
    maxcurcapab: Maxcurcapab,
    _reserved19: [u8; 0x04],
    fevterrstat: Fevterrstat,
    admaes: Admaes,
    adsaddr: Adsaddr,
    _reserved22: [u8; 0x04],
    prstval0: Prstval0,
    prstval2: Prstval2,
    prstval4: Prstval4,
    prstval6: Prstval6,
    boottoctrl: Boottoctrl,
    _reserved27: [u8; 0x88],
    slotintstat: Slotintstat,
    _reserved28: [u8; 0x0700],
    ctrl: Ctrl,
    cfg0: Cfg0,
    cfg1: Cfg1,
    cfgpresetval0: Cfgpresetval0,
    cfgpresetval1: Cfgpresetval1,
    cfgpresetval2: Cfgpresetval2,
    cfgpresetval3: Cfgpresetval3,
    routeloc0: Routeloc0,
    routeloc1: Routeloc1,
    routepen: Routepen,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address Register"]
    #[inline(always)]
    pub const fn sdmasysaddr(&self) -> &Sdmasysaddr {
        &self.sdmasysaddr
    }
    #[doc = "0x04 - Block Size and Block Count Register"]
    #[inline(always)]
    pub const fn blksize(&self) -> &Blksize {
        &self.blksize
    }
    #[doc = "0x08 - SD Command Argument Register"]
    #[inline(always)]
    pub const fn cmdarg1(&self) -> &Cmdarg1 {
        &self.cmdarg1
    }
    #[doc = "0x0c - Transfer Mode and Command Register"]
    #[inline(always)]
    pub const fn tfrmode(&self) -> &Tfrmode {
        &self.tfrmode
    }
    #[doc = "0x10 - Response0 and Response1 Register"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x14 - Response2 and Response3 Register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x18 - Response4 and Response5 Register"]
    #[inline(always)]
    pub const fn resp4(&self) -> &Resp4 {
        &self.resp4
    }
    #[doc = "0x1c - Response6 and Response7 Register"]
    #[inline(always)]
    pub const fn resp6(&self) -> &Resp6 {
        &self.resp6
    }
    #[doc = "0x20 - Buffer Data Register"]
    #[inline(always)]
    pub const fn bufdatport(&self) -> &Bufdatport {
        &self.bufdatport
    }
    #[doc = "0x24 - Present State Register"]
    #[inline(always)]
    pub const fn prsstat(&self) -> &Prsstat {
        &self.prsstat
    }
    #[doc = "0x28 - Host Control1, Power, Block Gap and Wakeup-up Control Register"]
    #[inline(always)]
    pub const fn hostctrl1(&self) -> &Hostctrl1 {
        &self.hostctrl1
    }
    #[doc = "0x2c - Clock Control, Timeout Control and Software Register"]
    #[inline(always)]
    pub const fn clockctrl(&self) -> &Clockctrl {
        &self.clockctrl
    }
    #[doc = "0x30 - Normal and Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x34 - Normal and Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn ifenc(&self) -> &Ifenc {
        &self.ifenc
    }
    #[doc = "0x38 - Normal and Error Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x3c - AUTO CMD12 Error Status and Host Control2 Register"]
    #[inline(always)]
    pub const fn ac12errstat(&self) -> &Ac12errstat {
        &self.ac12errstat
    }
    #[doc = "0x40 - Capabilities Register to Hold Bits 31~0"]
    #[inline(always)]
    pub const fn capab0(&self) -> &Capab0 {
        &self.capab0
    }
    #[doc = "0x44 - Capabilities Register to Hold Bits 63~32"]
    #[inline(always)]
    pub const fn capab2(&self) -> &Capab2 {
        &self.capab2
    }
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    #[inline(always)]
    pub const fn maxcurcapab(&self) -> &Maxcurcapab {
        &self.maxcurcapab
    }
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    #[inline(always)]
    pub const fn fevterrstat(&self) -> &Fevterrstat {
        &self.fevterrstat
    }
    #[doc = "0x54 - ADMA Error Status Register"]
    #[inline(always)]
    pub const fn admaes(&self) -> &Admaes {
        &self.admaes
    }
    #[doc = "0x58 - ADMA System Address Register"]
    #[inline(always)]
    pub const fn adsaddr(&self) -> &Adsaddr {
        &self.adsaddr
    }
    #[doc = "0x60 - Preset Value for Initialization and Default Speed Mode"]
    #[inline(always)]
    pub const fn prstval0(&self) -> &Prstval0 {
        &self.prstval0
    }
    #[doc = "0x64 - Preset Value for High Speed and SDR12 Modes"]
    #[inline(always)]
    pub const fn prstval2(&self) -> &Prstval2 {
        &self.prstval2
    }
    #[doc = "0x68 - Preset Value for SDR25 and SDR50 Modes"]
    #[inline(always)]
    pub const fn prstval4(&self) -> &Prstval4 {
        &self.prstval4
    }
    #[doc = "0x6c - Preset Value for SDR104 and DDR50 Modes"]
    #[inline(always)]
    pub const fn prstval6(&self) -> &Prstval6 {
        &self.prstval6
    }
    #[doc = "0x70 - Boot Timeout Control Register"]
    #[inline(always)]
    pub const fn boottoctrl(&self) -> &Boottoctrl {
        &self.boottoctrl
    }
    #[doc = "0xfc - Slot Interrupt Status Register"]
    #[inline(always)]
    pub const fn slotintstat(&self) -> &Slotintstat {
        &self.slotintstat
    }
    #[doc = "0x800 - Core Control Signals"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x804 - Core Configuration 0"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x808 - Core Configuration 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x80c - Core Configuration Preset Value 0"]
    #[inline(always)]
    pub const fn cfgpresetval0(&self) -> &Cfgpresetval0 {
        &self.cfgpresetval0
    }
    #[doc = "0x810 - Core Configuration Preset Value 1"]
    #[inline(always)]
    pub const fn cfgpresetval1(&self) -> &Cfgpresetval1 {
        &self.cfgpresetval1
    }
    #[doc = "0x814 - Core Configuration Preset Value 2"]
    #[inline(always)]
    pub const fn cfgpresetval2(&self) -> &Cfgpresetval2 {
        &self.cfgpresetval2
    }
    #[doc = "0x818 - Core Configuration Preset Value 3"]
    #[inline(always)]
    pub const fn cfgpresetval3(&self) -> &Cfgpresetval3 {
        &self.cfgpresetval3
    }
    #[doc = "0x81c - I/O LOCATION Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x820 - I/O LOCATION Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &Routeloc1 {
        &self.routeloc1
    }
    #[doc = "0x824 - I/O LOCATION Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
}
#[doc = "SDMASYSADDR (rw) register accessor: SDMA System Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmasysaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmasysaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmasysaddr`] module"]
#[doc(alias = "SDMASYSADDR")]
pub type Sdmasysaddr = crate::Reg<sdmasysaddr::SdmasysaddrSpec>;
#[doc = "SDMA System Address Register"]
pub mod sdmasysaddr;
#[doc = "BLKSIZE (rw) register accessor: Block Size and Block Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`blksize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksize`] module"]
#[doc(alias = "BLKSIZE")]
pub type Blksize = crate::Reg<blksize::BlksizeSpec>;
#[doc = "Block Size and Block Count Register"]
pub mod blksize;
#[doc = "CMDARG1 (rw) register accessor: SD Command Argument Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdarg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdarg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg1`] module"]
#[doc(alias = "CMDARG1")]
pub type Cmdarg1 = crate::Reg<cmdarg1::Cmdarg1Spec>;
#[doc = "SD Command Argument Register"]
pub mod cmdarg1;
#[doc = "TFRMODE (rw) register accessor: Transfer Mode and Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfrmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfrmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfrmode`] module"]
#[doc(alias = "TFRMODE")]
pub type Tfrmode = crate::Reg<tfrmode::TfrmodeSpec>;
#[doc = "Transfer Mode and Command Register"]
pub mod tfrmode;
#[doc = "RESP0 (r) register accessor: Response0 and Response1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`] module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "Response0 and Response1 Register"]
pub mod resp0;
#[doc = "RESP2 (r) register accessor: Response2 and Response3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`] module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "Response2 and Response3 Register"]
pub mod resp2;
#[doc = "RESP4 (r) register accessor: Response4 and Response5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4`] module"]
#[doc(alias = "RESP4")]
pub type Resp4 = crate::Reg<resp4::Resp4Spec>;
#[doc = "Response4 and Response5 Register"]
pub mod resp4;
#[doc = "RESP6 (r) register accessor: Response6 and Response7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp6`] module"]
#[doc(alias = "RESP6")]
pub type Resp6 = crate::Reg<resp6::Resp6Spec>;
#[doc = "Response6 and Response7 Register"]
pub mod resp6;
#[doc = "BUFDATPORT (rw) register accessor: Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufdatport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufdatport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufdatport`] module"]
#[doc(alias = "BUFDATPORT")]
pub type Bufdatport = crate::Reg<bufdatport::BufdatportSpec>;
#[doc = "Buffer Data Register"]
pub mod bufdatport;
#[doc = "PRSSTAT (r) register accessor: Present State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prsstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prsstat`] module"]
#[doc(alias = "PRSSTAT")]
pub type Prsstat = crate::Reg<prsstat::PrsstatSpec>;
#[doc = "Present State Register"]
pub mod prsstat;
#[doc = "HOSTCTRL1 (rw) register accessor: Host Control1, Power, Block Gap and Wakeup-up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hostctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostctrl1`] module"]
#[doc(alias = "HOSTCTRL1")]
pub type Hostctrl1 = crate::Reg<hostctrl1::Hostctrl1Spec>;
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub mod hostctrl1;
#[doc = "CLOCKCTRL (rw) register accessor: Clock Control, Timeout Control and Software Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clockctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockctrl`] module"]
#[doc(alias = "CLOCKCTRL")]
pub type Clockctrl = crate::Reg<clockctrl::ClockctrlSpec>;
#[doc = "Clock Control, Timeout Control and Software Register"]
pub mod clockctrl;
#[doc = "IFCR (rw) register accessor: Normal and Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "Normal and Error Interrupt Status Register"]
pub mod ifcr;
#[doc = "IFENC (rw) register accessor: Normal and Error Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifenc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifenc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifenc`] module"]
#[doc(alias = "IFENC")]
pub type Ifenc = crate::Reg<ifenc::IfencSpec>;
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub mod ifenc;
#[doc = "IEN (rw) register accessor: Normal and Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub mod ien;
#[doc = "AC12ERRSTAT (rw) register accessor: AUTO CMD12 Error Status and Host Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ac12errstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac12errstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac12errstat`] module"]
#[doc(alias = "AC12ERRSTAT")]
pub type Ac12errstat = crate::Reg<ac12errstat::Ac12errstatSpec>;
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub mod ac12errstat;
#[doc = "CAPAB0 (r) register accessor: Capabilities Register to Hold Bits 31~0\n\nYou can [`read`](crate::Reg::read) this register and get [`capab0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capab0`] module"]
#[doc(alias = "CAPAB0")]
pub type Capab0 = crate::Reg<capab0::Capab0Spec>;
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub mod capab0;
#[doc = "CAPAB2 (r) register accessor: Capabilities Register to Hold Bits 63~32\n\nYou can [`read`](crate::Reg::read) this register and get [`capab2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capab2`] module"]
#[doc(alias = "CAPAB2")]
pub type Capab2 = crate::Reg<capab2::Capab2Spec>;
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub mod capab2;
#[doc = "MAXCURCAPAB (r) register accessor: Maximum Current Capabilities Register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcurcapab::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcurcapab`] module"]
#[doc(alias = "MAXCURCAPAB")]
pub type Maxcurcapab = crate::Reg<maxcurcapab::MaxcurcapabSpec>;
#[doc = "Maximum Current Capabilities Register"]
pub mod maxcurcapab;
#[doc = "FEVTERRSTAT (rw) register accessor: Force Event Register for Auto CMD Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fevterrstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fevterrstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fevterrstat`] module"]
#[doc(alias = "FEVTERRSTAT")]
pub type Fevterrstat = crate::Reg<fevterrstat::FevterrstatSpec>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod fevterrstat;
#[doc = "ADMAES (r) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`admaes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admaes`] module"]
#[doc(alias = "ADMAES")]
pub type Admaes = crate::Reg<admaes::AdmaesSpec>;
#[doc = "ADMA Error Status Register"]
pub mod admaes;
#[doc = "ADSADDR (rw) register accessor: ADMA System Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adsaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsaddr`] module"]
#[doc(alias = "ADSADDR")]
pub type Adsaddr = crate::Reg<adsaddr::AdsaddrSpec>;
#[doc = "ADMA System Address Register"]
pub mod adsaddr;
#[doc = "PRSTVAL0 (r) register accessor: Preset Value for Initialization and Default Speed Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval0`] module"]
#[doc(alias = "PRSTVAL0")]
pub type Prstval0 = crate::Reg<prstval0::Prstval0Spec>;
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub mod prstval0;
#[doc = "PRSTVAL2 (r) register accessor: Preset Value for High Speed and SDR12 Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval2`] module"]
#[doc(alias = "PRSTVAL2")]
pub type Prstval2 = crate::Reg<prstval2::Prstval2Spec>;
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub mod prstval2;
#[doc = "PRSTVAL4 (r) register accessor: Preset Value for SDR25 and SDR50 Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval4`] module"]
#[doc(alias = "PRSTVAL4")]
pub type Prstval4 = crate::Reg<prstval4::Prstval4Spec>;
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub mod prstval4;
#[doc = "PRSTVAL6 (r) register accessor: Preset Value for SDR104 and DDR50 Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`prstval6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval6`] module"]
#[doc(alias = "PRSTVAL6")]
pub type Prstval6 = crate::Reg<prstval6::Prstval6Spec>;
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub mod prstval6;
#[doc = "BOOTTOCTRL (rw) register accessor: Boot Timeout Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`boottoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boottoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boottoctrl`] module"]
#[doc(alias = "BOOTTOCTRL")]
pub type Boottoctrl = crate::Reg<boottoctrl::BoottoctrlSpec>;
#[doc = "Boot Timeout Control Register"]
pub mod boottoctrl;
#[doc = "SLOTINTSTAT (r) register accessor: Slot Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slotintstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotintstat`] module"]
#[doc(alias = "SLOTINTSTAT")]
pub type Slotintstat = crate::Reg<slotintstat::SlotintstatSpec>;
#[doc = "Slot Interrupt Status Register"]
pub mod slotintstat;
#[doc = "CTRL (rw) register accessor: Core Control Signals\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Core Control Signals"]
pub mod ctrl;
#[doc = "CFG0 (rw) register accessor: Core Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "Core Configuration 0"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: Core Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Core Configuration 1"]
pub mod cfg1;
#[doc = "CFGPRESETVAL0 (rw) register accessor: Core Configuration Preset Value 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval0`] module"]
#[doc(alias = "CFGPRESETVAL0")]
pub type Cfgpresetval0 = crate::Reg<cfgpresetval0::Cfgpresetval0Spec>;
#[doc = "Core Configuration Preset Value 0"]
pub mod cfgpresetval0;
#[doc = "CFGPRESETVAL1 (rw) register accessor: Core Configuration Preset Value 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval1`] module"]
#[doc(alias = "CFGPRESETVAL1")]
pub type Cfgpresetval1 = crate::Reg<cfgpresetval1::Cfgpresetval1Spec>;
#[doc = "Core Configuration Preset Value 1"]
pub mod cfgpresetval1;
#[doc = "CFGPRESETVAL2 (rw) register accessor: Core Configuration Preset Value 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval2`] module"]
#[doc(alias = "CFGPRESETVAL2")]
pub type Cfgpresetval2 = crate::Reg<cfgpresetval2::Cfgpresetval2Spec>;
#[doc = "Core Configuration Preset Value 2"]
pub mod cfgpresetval2;
#[doc = "CFGPRESETVAL3 (rw) register accessor: Core Configuration Preset Value 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgpresetval3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgpresetval3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval3`] module"]
#[doc(alias = "CFGPRESETVAL3")]
pub type Cfgpresetval3 = crate::Reg<cfgpresetval3::Cfgpresetval3Spec>;
#[doc = "Core Configuration Preset Value 3"]
pub mod cfgpresetval3;
#[doc = "ROUTELOC0 (rw) register accessor: I/O LOCATION Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`] module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::Routeloc0Spec>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O LOCATION Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`] module"]
#[doc(alias = "ROUTELOC1")]
pub type Routeloc1 = crate::Reg<routeloc1::Routeloc1Spec>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc1;
#[doc = "ROUTEPEN (rw) register accessor: I/O LOCATION Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O LOCATION Enable Register"]
pub mod routepen;
