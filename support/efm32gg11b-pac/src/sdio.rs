#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMA System Address Register"]
    pub sdmasysaddr: SDMASYSADDR,
    #[doc = "0x04 - Block Size and Block Count Register"]
    pub blksize: BLKSIZE,
    #[doc = "0x08 - SD Command Argument Register"]
    pub cmdarg1: CMDARG1,
    #[doc = "0x0c - Transfer Mode and Command Register"]
    pub tfrmode: TFRMODE,
    #[doc = "0x10 - Response0 and Response1 Register"]
    pub resp0: RESP0,
    #[doc = "0x14 - Response2 and Response3 Register"]
    pub resp2: RESP2,
    #[doc = "0x18 - Response4 and Response5 Register"]
    pub resp4: RESP4,
    #[doc = "0x1c - Response6 and Response7 Register"]
    pub resp6: RESP6,
    #[doc = "0x20 - Buffer Data Register"]
    pub bufdatport: BUFDATPORT,
    #[doc = "0x24 - Present State Register"]
    pub prsstat: PRSSTAT,
    #[doc = "0x28 - Host Control1, Power, Block Gap and Wakeup-up Control Register"]
    pub hostctrl1: HOSTCTRL1,
    #[doc = "0x2c - Clock Control, Timeout Control and Software Register"]
    pub clockctrl: CLOCKCTRL,
    #[doc = "0x30 - Normal and Error Interrupt Status Register"]
    pub ifcr: IFCR,
    #[doc = "0x34 - Normal and Error Interrupt Status Enable Register"]
    pub ifenc: IFENC,
    #[doc = "0x38 - Normal and Error Interrupt Signal Enable Register"]
    pub ien: IEN,
    #[doc = "0x3c - AUTO CMD12 Error Status and Host Control2 Register"]
    pub ac12errstat: AC12ERRSTAT,
    #[doc = "0x40 - Capabilities Register to Hold Bits 31~0"]
    pub capab0: CAPAB0,
    #[doc = "0x44 - Capabilities Register to Hold Bits 63~32"]
    pub capab2: CAPAB2,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub maxcurcapab: MAXCURCAPAB,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub fevterrstat: FEVTERRSTAT,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub admaes: ADMAES,
    #[doc = "0x58 - ADMA System Address Register"]
    pub adsaddr: ADSADDR,
    _reserved22: [u8; 0x04],
    #[doc = "0x60 - Preset Value for Initialization and Default Speed Mode"]
    pub prstval0: PRSTVAL0,
    #[doc = "0x64 - Preset Value for High Speed and SDR12 Modes"]
    pub prstval2: PRSTVAL2,
    #[doc = "0x68 - Preset Value for SDR25 and SDR50 Modes"]
    pub prstval4: PRSTVAL4,
    #[doc = "0x6c - Preset Value for SDR104 and DDR50 Modes"]
    pub prstval6: PRSTVAL6,
    #[doc = "0x70 - Boot Timeout Control Register"]
    pub boottoctrl: BOOTTOCTRL,
    _reserved27: [u8; 0x88],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slotintstat: SLOTINTSTAT,
    _reserved28: [u8; 0x0700],
    #[doc = "0x800 - Core Control Signals"]
    pub ctrl: CTRL,
    #[doc = "0x804 - Core Configuration 0"]
    pub cfg0: CFG0,
    #[doc = "0x808 - Core Configuration 1"]
    pub cfg1: CFG1,
    #[doc = "0x80c - Core Configuration Preset Value 0"]
    pub cfgpresetval0: CFGPRESETVAL0,
    #[doc = "0x810 - Core Configuration Preset Value 1"]
    pub cfgpresetval1: CFGPRESETVAL1,
    #[doc = "0x814 - Core Configuration Preset Value 2"]
    pub cfgpresetval2: CFGPRESETVAL2,
    #[doc = "0x818 - Core Configuration Preset Value 3"]
    pub cfgpresetval3: CFGPRESETVAL3,
    #[doc = "0x81c - I/O LOCATION Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x820 - I/O LOCATION Register"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0x824 - I/O LOCATION Enable Register"]
    pub routepen: ROUTEPEN,
}
#[doc = "SDMASYSADDR (rw) register accessor: SDMA System Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmasysaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmasysaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdmasysaddr`] module"]
pub type SDMASYSADDR = crate::Reg<sdmasysaddr::SDMASYSADDR_SPEC>;
#[doc = "SDMA System Address Register"]
pub mod sdmasysaddr;
#[doc = "BLKSIZE (rw) register accessor: Block Size and Block Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`blksize`] module"]
pub type BLKSIZE = crate::Reg<blksize::BLKSIZE_SPEC>;
#[doc = "Block Size and Block Count Register"]
pub mod blksize;
#[doc = "CMDARG1 (rw) register accessor: SD Command Argument Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmdarg1`] module"]
pub type CMDARG1 = crate::Reg<cmdarg1::CMDARG1_SPEC>;
#[doc = "SD Command Argument Register"]
pub mod cmdarg1;
#[doc = "TFRMODE (rw) register accessor: Transfer Mode and Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfrmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfrmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tfrmode`] module"]
pub type TFRMODE = crate::Reg<tfrmode::TFRMODE_SPEC>;
#[doc = "Transfer Mode and Command Register"]
pub mod tfrmode;
#[doc = "RESP0 (r) register accessor: Response0 and Response1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp0`] module"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Response0 and Response1 Register"]
pub mod resp0;
#[doc = "RESP2 (r) register accessor: Response2 and Response3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp2`] module"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Response2 and Response3 Register"]
pub mod resp2;
#[doc = "RESP4 (r) register accessor: Response4 and Response5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp4`] module"]
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
#[doc = "Response4 and Response5 Register"]
pub mod resp4;
#[doc = "RESP6 (r) register accessor: Response6 and Response7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resp6`] module"]
pub type RESP6 = crate::Reg<resp6::RESP6_SPEC>;
#[doc = "Response6 and Response7 Register"]
pub mod resp6;
#[doc = "BUFDATPORT (rw) register accessor: Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufdatport::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufdatport::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bufdatport`] module"]
pub type BUFDATPORT = crate::Reg<bufdatport::BUFDATPORT_SPEC>;
#[doc = "Buffer Data Register"]
pub mod bufdatport;
#[doc = "PRSSTAT (r) register accessor: Present State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prsstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prsstat`] module"]
pub type PRSSTAT = crate::Reg<prsstat::PRSSTAT_SPEC>;
#[doc = "Present State Register"]
pub mod prsstat;
#[doc = "HOSTCTRL1 (rw) register accessor: Host Control1, Power, Block Gap and Wakeup-up Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hostctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hostctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hostctrl1`] module"]
pub type HOSTCTRL1 = crate::Reg<hostctrl1::HOSTCTRL1_SPEC>;
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub mod hostctrl1;
#[doc = "CLOCKCTRL (rw) register accessor: Clock Control, Timeout Control and Software Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clockctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clockctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clockctrl`] module"]
pub type CLOCKCTRL = crate::Reg<clockctrl::CLOCKCTRL_SPEC>;
#[doc = "Clock Control, Timeout Control and Software Register"]
pub mod clockctrl;
#[doc = "IFCR (rw) register accessor: Normal and Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifcr`] module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "Normal and Error Interrupt Status Register"]
pub mod ifcr;
#[doc = "IFENC (rw) register accessor: Normal and Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifenc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifenc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifenc`] module"]
pub type IFENC = crate::Reg<ifenc::IFENC_SPEC>;
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub mod ifenc;
#[doc = "IEN (rw) register accessor: Normal and Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ien`] module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub mod ien;
#[doc = "AC12ERRSTAT (rw) register accessor: AUTO CMD12 Error Status and Host Control2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac12errstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac12errstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ac12errstat`] module"]
pub type AC12ERRSTAT = crate::Reg<ac12errstat::AC12ERRSTAT_SPEC>;
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub mod ac12errstat;
#[doc = "CAPAB0 (r) register accessor: Capabilities Register to Hold Bits 31~0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`capab0`] module"]
pub type CAPAB0 = crate::Reg<capab0::CAPAB0_SPEC>;
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub mod capab0;
#[doc = "CAPAB2 (r) register accessor: Capabilities Register to Hold Bits 63~32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`capab2`] module"]
pub type CAPAB2 = crate::Reg<capab2::CAPAB2_SPEC>;
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub mod capab2;
#[doc = "MAXCURCAPAB (r) register accessor: Maximum Current Capabilities Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxcurcapab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maxcurcapab`] module"]
pub type MAXCURCAPAB = crate::Reg<maxcurcapab::MAXCURCAPAB_SPEC>;
#[doc = "Maximum Current Capabilities Register"]
pub mod maxcurcapab;
#[doc = "FEVTERRSTAT (rw) register accessor: Force Event Register for Auto CMD Error Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fevterrstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fevterrstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fevterrstat`] module"]
pub type FEVTERRSTAT = crate::Reg<fevterrstat::FEVTERRSTAT_SPEC>;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod fevterrstat;
#[doc = "ADMAES (r) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admaes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`admaes`] module"]
pub type ADMAES = crate::Reg<admaes::ADMAES_SPEC>;
#[doc = "ADMA Error Status Register"]
pub mod admaes;
#[doc = "ADSADDR (rw) register accessor: ADMA System Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adsaddr`] module"]
pub type ADSADDR = crate::Reg<adsaddr::ADSADDR_SPEC>;
#[doc = "ADMA System Address Register"]
pub mod adsaddr;
#[doc = "PRSTVAL0 (r) register accessor: Preset Value for Initialization and Default Speed Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prstval0`] module"]
pub type PRSTVAL0 = crate::Reg<prstval0::PRSTVAL0_SPEC>;
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub mod prstval0;
#[doc = "PRSTVAL2 (r) register accessor: Preset Value for High Speed and SDR12 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prstval2`] module"]
pub type PRSTVAL2 = crate::Reg<prstval2::PRSTVAL2_SPEC>;
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub mod prstval2;
#[doc = "PRSTVAL4 (r) register accessor: Preset Value for SDR25 and SDR50 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prstval4`] module"]
pub type PRSTVAL4 = crate::Reg<prstval4::PRSTVAL4_SPEC>;
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub mod prstval4;
#[doc = "PRSTVAL6 (r) register accessor: Preset Value for SDR104 and DDR50 Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstval6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prstval6`] module"]
pub type PRSTVAL6 = crate::Reg<prstval6::PRSTVAL6_SPEC>;
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub mod prstval6;
#[doc = "BOOTTOCTRL (rw) register accessor: Boot Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boottoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boottoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`boottoctrl`] module"]
pub type BOOTTOCTRL = crate::Reg<boottoctrl::BOOTTOCTRL_SPEC>;
#[doc = "Boot Timeout Control Register"]
pub mod boottoctrl;
#[doc = "SLOTINTSTAT (r) register accessor: Slot Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotintstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slotintstat`] module"]
pub type SLOTINTSTAT = crate::Reg<slotintstat::SLOTINTSTAT_SPEC>;
#[doc = "Slot Interrupt Status Register"]
pub mod slotintstat;
#[doc = "CTRL (rw) register accessor: Core Control Signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Core Control Signals"]
pub mod ctrl;
#[doc = "CFG0 (rw) register accessor: Core Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfg0`] module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Core Configuration 0"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: Core Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfg1`] module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Core Configuration 1"]
pub mod cfg1;
#[doc = "CFGPRESETVAL0 (rw) register accessor: Core Configuration Preset Value 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgpresetval0`] module"]
pub type CFGPRESETVAL0 = crate::Reg<cfgpresetval0::CFGPRESETVAL0_SPEC>;
#[doc = "Core Configuration Preset Value 0"]
pub mod cfgpresetval0;
#[doc = "CFGPRESETVAL1 (rw) register accessor: Core Configuration Preset Value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgpresetval1`] module"]
pub type CFGPRESETVAL1 = crate::Reg<cfgpresetval1::CFGPRESETVAL1_SPEC>;
#[doc = "Core Configuration Preset Value 1"]
pub mod cfgpresetval1;
#[doc = "CFGPRESETVAL2 (rw) register accessor: Core Configuration Preset Value 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgpresetval2`] module"]
pub type CFGPRESETVAL2 = crate::Reg<cfgpresetval2::CFGPRESETVAL2_SPEC>;
#[doc = "Core Configuration Preset Value 2"]
pub mod cfgpresetval2;
#[doc = "CFGPRESETVAL3 (rw) register accessor: Core Configuration Preset Value 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpresetval3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpresetval3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgpresetval3`] module"]
pub type CFGPRESETVAL3 = crate::Reg<cfgpresetval3::CFGPRESETVAL3_SPEC>;
#[doc = "Core Configuration Preset Value 3"]
pub mod cfgpresetval3;
#[doc = "ROUTELOC0 (rw) register accessor: I/O LOCATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`routeloc0`] module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc0;
#[doc = "ROUTELOC1 (rw) register accessor: I/O LOCATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`routeloc1`] module"]
pub type ROUTELOC1 = crate::Reg<routeloc1::ROUTELOC1_SPEC>;
#[doc = "I/O LOCATION Register"]
pub mod routeloc1;
#[doc = "ROUTEPEN (rw) register accessor: I/O LOCATION Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`routepen`] module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O LOCATION Enable Register"]
pub mod routepen;
