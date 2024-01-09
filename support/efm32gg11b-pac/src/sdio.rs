#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sdmasysaddr: SDMASYSADDR,
    blksize: BLKSIZE,
    cmdarg1: CMDARG1,
    tfrmode: TFRMODE,
    resp0: RESP0,
    resp2: RESP2,
    resp4: RESP4,
    resp6: RESP6,
    bufdatport: BUFDATPORT,
    prsstat: PRSSTAT,
    hostctrl1: HOSTCTRL1,
    clockctrl: CLOCKCTRL,
    ifcr: IFCR,
    ifenc: IFENC,
    ien: IEN,
    ac12errstat: AC12ERRSTAT,
    capab0: CAPAB0,
    capab2: CAPAB2,
    maxcurcapab: MAXCURCAPAB,
    _reserved19: [u8; 0x04],
    fevterrstat: FEVTERRSTAT,
    admaes: ADMAES,
    adsaddr: ADSADDR,
    _reserved22: [u8; 0x04],
    prstval0: PRSTVAL0,
    prstval2: PRSTVAL2,
    prstval4: PRSTVAL4,
    prstval6: PRSTVAL6,
    boottoctrl: BOOTTOCTRL,
    _reserved27: [u8; 0x88],
    slotintstat: SLOTINTSTAT,
    _reserved28: [u8; 0x0700],
    ctrl: CTRL,
    cfg0: CFG0,
    cfg1: CFG1,
    cfgpresetval0: CFGPRESETVAL0,
    cfgpresetval1: CFGPRESETVAL1,
    cfgpresetval2: CFGPRESETVAL2,
    cfgpresetval3: CFGPRESETVAL3,
    routeloc0: ROUTELOC0,
    routeloc1: ROUTELOC1,
    routepen: ROUTEPEN,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address Register"]
    #[inline(always)]
    pub const fn sdmasysaddr(&self) -> &SDMASYSADDR {
        &self.sdmasysaddr
    }
    #[doc = "0x04 - Block Size and Block Count Register"]
    #[inline(always)]
    pub const fn blksize(&self) -> &BLKSIZE {
        &self.blksize
    }
    #[doc = "0x08 - SD Command Argument Register"]
    #[inline(always)]
    pub const fn cmdarg1(&self) -> &CMDARG1 {
        &self.cmdarg1
    }
    #[doc = "0x0c - Transfer Mode and Command Register"]
    #[inline(always)]
    pub const fn tfrmode(&self) -> &TFRMODE {
        &self.tfrmode
    }
    #[doc = "0x10 - Response0 and Response1 Register"]
    #[inline(always)]
    pub const fn resp0(&self) -> &RESP0 {
        &self.resp0
    }
    #[doc = "0x14 - Response2 and Response3 Register"]
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP2 {
        &self.resp2
    }
    #[doc = "0x18 - Response4 and Response5 Register"]
    #[inline(always)]
    pub const fn resp4(&self) -> &RESP4 {
        &self.resp4
    }
    #[doc = "0x1c - Response6 and Response7 Register"]
    #[inline(always)]
    pub const fn resp6(&self) -> &RESP6 {
        &self.resp6
    }
    #[doc = "0x20 - Buffer Data Register"]
    #[inline(always)]
    pub const fn bufdatport(&self) -> &BUFDATPORT {
        &self.bufdatport
    }
    #[doc = "0x24 - Present State Register"]
    #[inline(always)]
    pub const fn prsstat(&self) -> &PRSSTAT {
        &self.prsstat
    }
    #[doc = "0x28 - Host Control1, Power, Block Gap and Wakeup-up Control Register"]
    #[inline(always)]
    pub const fn hostctrl1(&self) -> &HOSTCTRL1 {
        &self.hostctrl1
    }
    #[doc = "0x2c - Clock Control, Timeout Control and Software Register"]
    #[inline(always)]
    pub const fn clockctrl(&self) -> &CLOCKCTRL {
        &self.clockctrl
    }
    #[doc = "0x30 - Normal and Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x34 - Normal and Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn ifenc(&self) -> &IFENC {
        &self.ifenc
    }
    #[doc = "0x38 - Normal and Error Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x3c - AUTO CMD12 Error Status and Host Control2 Register"]
    #[inline(always)]
    pub const fn ac12errstat(&self) -> &AC12ERRSTAT {
        &self.ac12errstat
    }
    #[doc = "0x40 - Capabilities Register to Hold Bits 31~0"]
    #[inline(always)]
    pub const fn capab0(&self) -> &CAPAB0 {
        &self.capab0
    }
    #[doc = "0x44 - Capabilities Register to Hold Bits 63~32"]
    #[inline(always)]
    pub const fn capab2(&self) -> &CAPAB2 {
        &self.capab2
    }
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    #[inline(always)]
    pub const fn maxcurcapab(&self) -> &MAXCURCAPAB {
        &self.maxcurcapab
    }
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    #[inline(always)]
    pub const fn fevterrstat(&self) -> &FEVTERRSTAT {
        &self.fevterrstat
    }
    #[doc = "0x54 - ADMA Error Status Register"]
    #[inline(always)]
    pub const fn admaes(&self) -> &ADMAES {
        &self.admaes
    }
    #[doc = "0x58 - ADMA System Address Register"]
    #[inline(always)]
    pub const fn adsaddr(&self) -> &ADSADDR {
        &self.adsaddr
    }
    #[doc = "0x60 - Preset Value for Initialization and Default Speed Mode"]
    #[inline(always)]
    pub const fn prstval0(&self) -> &PRSTVAL0 {
        &self.prstval0
    }
    #[doc = "0x64 - Preset Value for High Speed and SDR12 Modes"]
    #[inline(always)]
    pub const fn prstval2(&self) -> &PRSTVAL2 {
        &self.prstval2
    }
    #[doc = "0x68 - Preset Value for SDR25 and SDR50 Modes"]
    #[inline(always)]
    pub const fn prstval4(&self) -> &PRSTVAL4 {
        &self.prstval4
    }
    #[doc = "0x6c - Preset Value for SDR104 and DDR50 Modes"]
    #[inline(always)]
    pub const fn prstval6(&self) -> &PRSTVAL6 {
        &self.prstval6
    }
    #[doc = "0x70 - Boot Timeout Control Register"]
    #[inline(always)]
    pub const fn boottoctrl(&self) -> &BOOTTOCTRL {
        &self.boottoctrl
    }
    #[doc = "0xfc - Slot Interrupt Status Register"]
    #[inline(always)]
    pub const fn slotintstat(&self) -> &SLOTINTSTAT {
        &self.slotintstat
    }
    #[doc = "0x800 - Core Control Signals"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x804 - Core Configuration 0"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &CFG0 {
        &self.cfg0
    }
    #[doc = "0x808 - Core Configuration 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x80c - Core Configuration Preset Value 0"]
    #[inline(always)]
    pub const fn cfgpresetval0(&self) -> &CFGPRESETVAL0 {
        &self.cfgpresetval0
    }
    #[doc = "0x810 - Core Configuration Preset Value 1"]
    #[inline(always)]
    pub const fn cfgpresetval1(&self) -> &CFGPRESETVAL1 {
        &self.cfgpresetval1
    }
    #[doc = "0x814 - Core Configuration Preset Value 2"]
    #[inline(always)]
    pub const fn cfgpresetval2(&self) -> &CFGPRESETVAL2 {
        &self.cfgpresetval2
    }
    #[doc = "0x818 - Core Configuration Preset Value 3"]
    #[inline(always)]
    pub const fn cfgpresetval3(&self) -> &CFGPRESETVAL3 {
        &self.cfgpresetval3
    }
    #[doc = "0x81c - I/O LOCATION Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x820 - I/O LOCATION Register"]
    #[inline(always)]
    pub const fn routeloc1(&self) -> &ROUTELOC1 {
        &self.routeloc1
    }
    #[doc = "0x824 - I/O LOCATION Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
}
#[doc = "SDMASYSADDR (rw) register accessor: SDMA System Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmasysaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmasysaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmasysaddr`]
module"]
pub type SDMASYSADDR = crate::Reg<sdmasysaddr::SDMASYSADDR_SPEC>;
#[doc = "SDMA System Address Register"]
pub mod sdmasysaddr;
#[doc = "BLKSIZE (rw) register accessor: Block Size and Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksize`]
module"]
pub type BLKSIZE = crate::Reg<blksize::BLKSIZE_SPEC>;
#[doc = "Block Size and Block Count Register"]
pub mod blksize;
#[doc = "CMDARG1 (rw) register accessor: SD Command Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg1`]
module"]
pub type CMDARG1 = crate::Reg<cmdarg1::CMDARG1_SPEC>;
#[doc = "SD Command Argument Register"]
pub mod cmdarg1;
#[doc = "TFRMODE (rw) register accessor: Transfer Mode and Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfrmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfrmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfrmode`]
module"]
pub type TFRMODE = crate::Reg<tfrmode::TFRMODE_SPEC>;
#[doc = "Transfer Mode and Command Register"]
pub mod tfrmode;
#[doc = "RESP0 (r) register accessor: Response0 and Response1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`]
module"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response0 and Response1 Register"]
pub mod resp0;
#[doc = "RESP2 (r) register accessor: Response2 and Response3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Response2 and Response3 Register"]
pub mod resp2;
#[doc = "RESP4 (r) register accessor: Response4 and Response5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp4`]
module"]
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
#[doc = "Response4 and Response5 Register"]
pub mod resp4;
#[doc = "RESP6 (r) register accessor: Response6 and Response7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp6`]
module"]
pub type RESP6 = crate::Reg<resp6::RESP6_SPEC>;
#[doc = "Response6 and Response7 Register"]
pub mod resp6;
#[doc = "BUFDATPORT (rw) register accessor: Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufdatport::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufdatport::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufdatport`]
module"]
pub type BUFDATPORT = crate::Reg<bufdatport::BUFDATPORT_SPEC>;
#[doc = "Buffer Data Register"]
pub mod bufdatport;
#[doc = "PRSSTAT (r) register accessor: Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prsstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prsstat`]
module"]
pub type PRSSTAT = crate::Reg<prsstat::PRSSTAT_SPEC>;
#[doc = "Present State Register"]
pub mod prsstat;
#[doc = "HOSTCTRL1 (rw) register accessor: Host Control1, Power, Block Gap and Wakeup-up Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hostctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hostctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostctrl1`]
module"]
pub type HOSTCTRL1 = crate::Reg<hostctrl1::HOSTCTRL1_SPEC>;
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub mod hostctrl1;
#[doc = "CLOCKCTRL (rw) register accessor: Clock Control, Timeout Control and Software Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clockctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clockctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockctrl`]
module"]
pub type CLOCKCTRL = crate::Reg<clockctrl::CLOCKCTRL_SPEC>;
#[doc = "Clock Control, Timeout Control and Software Register"]
pub mod clockctrl;
#[doc = "IFCR (rw) register accessor: Normal and Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Normal and Error Interrupt Status Register"]
pub mod ifcr;
#[doc = "IFENC (rw) register accessor: Normal and Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifenc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifenc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifenc`]
module"]
pub type IFENC = crate::Reg<ifenc::IFENC_SPEC>;
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub mod ifenc;
#[doc = "IEN (rw) register accessor: Normal and Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub mod ien;
#[doc = "AC12ERRSTAT (rw) register accessor: AUTO CMD12 Error Status and Host Control2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac12errstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac12errstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac12errstat`]
module"]
pub type AC12ERRSTAT = crate::Reg<ac12errstat::AC12ERRSTAT_SPEC>;
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub mod ac12errstat;
#[doc = "CAPAB0 (r) register accessor: Capabilities Register to Hold Bits 31~0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capab0`]
module"]
pub type CAPAB0 = crate::Reg<capab0::CAPAB0_SPEC>;
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub mod capab0;
#[doc = "CAPAB2 (r) register accessor: Capabilities Register to Hold Bits 63~32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capab2`]
module"]
pub type CAPAB2 = crate::Reg<capab2::CAPAB2_SPEC>;
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub mod capab2;
#[doc = "MAXCURCAPAB (r) register accessor: Maximum Current Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxcurcapab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcurcapab`]
module"]
pub type MAXCURCAPAB = crate::Reg<maxcurcapab::MAXCURCAPAB_SPEC>;
#[doc = "Maximum Current Capabilities Register"]
pub mod maxcurcapab;
#[doc = "FEVTERRSTAT (rw) register accessor: Force Event Register for Auto CMD Error Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fevterrstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fevterrstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fevterrstat`]
module"]
pub type FEVTERRSTAT = crate::Reg<fevterrstat::FEVTERRSTAT_SPEC>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod fevterrstat;
#[doc = "ADMAES (r) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admaes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@admaes`]
module"]
pub type ADMAES = crate::Reg<admaes::ADMAES_SPEC>;
#[doc = "ADMA Error Status Register"]
pub mod admaes;
#[doc = "ADSADDR (rw) register accessor: ADMA System Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsaddr`]
module"]
pub type ADSADDR = crate::Reg<adsaddr::ADSADDR_SPEC>;
#[doc = "ADMA System Address Register"]
pub mod adsaddr;
#[doc = "PRSTVAL0 (r) register accessor: Preset Value for Initialization and Default Speed Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval0`]
module"]
pub type PRSTVAL0 = crate::Reg<prstval0::PRSTVAL0_SPEC>;
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub mod prstval0;
#[doc = "PRSTVAL2 (r) register accessor: Preset Value for High Speed and SDR12 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval2`]
module"]
pub type PRSTVAL2 = crate::Reg<prstval2::PRSTVAL2_SPEC>;
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub mod prstval2;
#[doc = "PRSTVAL4 (r) register accessor: Preset Value for SDR25 and SDR50 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval4`]
module"]
pub type PRSTVAL4 = crate::Reg<prstval4::PRSTVAL4_SPEC>;
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub mod prstval4;
#[doc = "PRSTVAL6 (r) register accessor: Preset Value for SDR104 and DDR50 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstval6`]
module"]
pub type PRSTVAL6 = crate::Reg<prstval6::PRSTVAL6_SPEC>;
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub mod prstval6;
#[doc = "BOOTTOCTRL (rw) register accessor: Boot Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boottoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boottoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boottoctrl`]
module"]
pub type BOOTTOCTRL = crate::Reg<boottoctrl::BOOTTOCTRL_SPEC>;
#[doc = "Boot Timeout Control Register"]
pub mod boottoctrl;
#[doc = "SLOTINTSTAT (r) register accessor: Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotintstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotintstat`]
module"]
pub type SLOTINTSTAT = crate::Reg<slotintstat::SLOTINTSTAT_SPEC>;
#[doc = "Slot Interrupt Status Register"]
pub mod slotintstat;
#[doc = "CTRL (rw) register accessor: Core Control Signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Core Control Signals"]
pub mod ctrl;
#[doc = "CFG0 (rw) register accessor: Core Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Core Configuration 0"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: Core Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Core Configuration 1"]
pub mod cfg1;
#[doc = "CFGPRESETVAL0 (rw) register accessor: Core Configuration Preset Value 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval0`]
module"]
pub type CFGPRESETVAL0 = crate::Reg<cfgpresetval0::CFGPRESETVAL0_SPEC>;
#[doc = "Core Configuration Preset Value 0"]
pub mod cfgpresetval0;
#[doc = "CFGPRESETVAL1 (rw) register accessor: Core Configuration Preset Value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval1`]
module"]
pub type CFGPRESETVAL1 = crate::Reg<cfgpresetval1::CFGPRESETVAL1_SPEC>;
#[doc = "Core Configuration Preset Value 1"]
pub mod cfgpresetval1;
#[doc = "CFGPRESETVAL2 (rw) register accessor: Core Configuration Preset Value 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval2`]
module"]
pub type CFGPRESETVAL2 = crate::Reg<cfgpresetval2::CFGPRESETVAL2_SPEC>;
#[doc = "Core Configuration Preset Value 2"]
pub mod cfgpresetval2;
#[doc = "CFGPRESETVAL3 (rw) register accessor: Core Configuration Preset Value 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpresetval3`]
module"]
pub type CFGPRESETVAL3 = crate::Reg<cfgpresetval3::CFGPRESETVAL3_SPEC>;
#[doc = "Core Configuration Preset Value 3"]
pub mod cfgpresetval3;
#[doc = "ROUTELOC0 (rw) register accessor: I/O LOCATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O LOCATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc1`]
module"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc1;
#[doc = "ROUTEPEN (rw) register accessor: I/O LOCATION Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O LOCATION Enable Register"]
pub mod routepen;
