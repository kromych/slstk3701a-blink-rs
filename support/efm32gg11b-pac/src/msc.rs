#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    readctrl: Readctrl,
    writectrl: Writectrl,
    writecmd: Writecmd,
    addrb: Addrb,
    _reserved5: [u8; 0x04],
    wdata: Wdata,
    status: Status,
    _reserved7: [u8; 0x10],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    lock: Lock,
    cachecmd: Cachecmd,
    cachehits: Cachehits,
    cachemisses: Cachemisses,
    _reserved15: [u8; 0x04],
    masslock: Masslock,
    _reserved16: [u8; 0x04],
    startup: Startup,
    _reserved17: [u8; 0x10],
    bankswitchlock: Bankswitchlock,
    cmd: Cmd,
    _reserved19: [u8; 0x18],
    bootloaderctrl: Bootloaderctrl,
    aapunlockcmd: Aapunlockcmd,
    cacheconfig0: Cacheconfig0,
    _reserved22: [u8; 0x64],
    ramctrl: Ramctrl,
    eccctrl: Eccctrl,
    rameccaddr: Rameccaddr,
    ram1eccaddr: Ram1eccaddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Memory System Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Read Control Register"]
    #[inline(always)]
    pub const fn readctrl(&self) -> &Readctrl {
        &self.readctrl
    }
    #[doc = "0x08 - Write Control Register"]
    #[inline(always)]
    pub const fn writectrl(&self) -> &Writectrl {
        &self.writectrl
    }
    #[doc = "0x0c - Write Command Register"]
    #[inline(always)]
    pub const fn writecmd(&self) -> &Writecmd {
        &self.writecmd
    }
    #[doc = "0x10 - Page Erase/Write Address Buffer"]
    #[inline(always)]
    pub const fn addrb(&self) -> &Addrb {
        &self.addrb
    }
    #[doc = "0x18 - Write Data Register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x30 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x34 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x38 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x3c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x40 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x44 - Flash Cache Command Register"]
    #[inline(always)]
    pub const fn cachecmd(&self) -> &Cachecmd {
        &self.cachecmd
    }
    #[doc = "0x48 - Cache Hits Performance Counter"]
    #[inline(always)]
    pub const fn cachehits(&self) -> &Cachehits {
        &self.cachehits
    }
    #[doc = "0x4c - Cache Misses Performance Counter"]
    #[inline(always)]
    pub const fn cachemisses(&self) -> &Cachemisses {
        &self.cachemisses
    }
    #[doc = "0x54 - Mass Erase Lock Register"]
    #[inline(always)]
    pub const fn masslock(&self) -> &Masslock {
        &self.masslock
    }
    #[doc = "0x5c - Startup Control"]
    #[inline(always)]
    pub const fn startup(&self) -> &Startup {
        &self.startup
    }
    #[doc = "0x70 - Bank Switching Lock Register"]
    #[inline(always)]
    pub const fn bankswitchlock(&self) -> &Bankswitchlock {
        &self.bankswitchlock
    }
    #[doc = "0x74 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x90 - Bootloader Read and Write Enable, Write Once Register"]
    #[inline(always)]
    pub const fn bootloaderctrl(&self) -> &Bootloaderctrl {
        &self.bootloaderctrl
    }
    #[doc = "0x94 - Software Unlock AAP Command Register"]
    #[inline(always)]
    pub const fn aapunlockcmd(&self) -> &Aapunlockcmd {
        &self.aapunlockcmd
    }
    #[doc = "0x98 - Cache Configuration Register 0"]
    #[inline(always)]
    pub const fn cacheconfig0(&self) -> &Cacheconfig0 {
        &self.cacheconfig0
    }
    #[doc = "0x100 - RAM Control Enable Register"]
    #[inline(always)]
    pub const fn ramctrl(&self) -> &Ramctrl {
        &self.ramctrl
    }
    #[doc = "0x104 - RAM ECC Control Register"]
    #[inline(always)]
    pub const fn eccctrl(&self) -> &Eccctrl {
        &self.eccctrl
    }
    #[doc = "0x108 - RAM ECC Error Address Register"]
    #[inline(always)]
    pub const fn rameccaddr(&self) -> &Rameccaddr {
        &self.rameccaddr
    }
    #[doc = "0x10c - RAM1 ECC Error Address Register"]
    #[inline(always)]
    pub const fn ram1eccaddr(&self) -> &Ram1eccaddr {
        &self.ram1eccaddr
    }
}
#[doc = "CTRL (rw) register accessor: Memory System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Memory System Control Register"]
pub mod ctrl;
#[doc = "READCTRL (rw) register accessor: Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`readctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readctrl`] module"]
#[doc(alias = "READCTRL")]
pub type Readctrl = crate::Reg<readctrl::ReadctrlSpec>;
#[doc = "Read Control Register"]
pub mod readctrl;
#[doc = "WRITECTRL (rw) register accessor: Write Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`writectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writectrl`] module"]
#[doc(alias = "WRITECTRL")]
pub type Writectrl = crate::Reg<writectrl::WritectrlSpec>;
#[doc = "Write Control Register"]
pub mod writectrl;
#[doc = "WRITECMD (w) register accessor: Write Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writecmd`] module"]
#[doc(alias = "WRITECMD")]
pub type Writecmd = crate::Reg<writecmd::WritecmdSpec>;
#[doc = "Write Command Register"]
pub mod writecmd;
#[doc = "ADDRB (rw) register accessor: Page Erase/Write Address Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`addrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrb`] module"]
#[doc(alias = "ADDRB")]
pub type Addrb = crate::Reg<addrb::AddrbSpec>;
#[doc = "Page Erase/Write Address Buffer"]
pub mod addrb;
#[doc = "WDATA (rw) register accessor: Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`] module"]
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WdataSpec>;
#[doc = "Write Data Register"]
pub mod wdata;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
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
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "CACHECMD (w) register accessor: Flash Cache Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachecmd`] module"]
#[doc(alias = "CACHECMD")]
pub type Cachecmd = crate::Reg<cachecmd::CachecmdSpec>;
#[doc = "Flash Cache Command Register"]
pub mod cachecmd;
#[doc = "CACHEHITS (r) register accessor: Cache Hits Performance Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cachehits::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachehits`] module"]
#[doc(alias = "CACHEHITS")]
pub type Cachehits = crate::Reg<cachehits::CachehitsSpec>;
#[doc = "Cache Hits Performance Counter"]
pub mod cachehits;
#[doc = "CACHEMISSES (r) register accessor: Cache Misses Performance Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cachemisses::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cachemisses`] module"]
#[doc(alias = "CACHEMISSES")]
pub type Cachemisses = crate::Reg<cachemisses::CachemissesSpec>;
#[doc = "Cache Misses Performance Counter"]
pub mod cachemisses;
#[doc = "MASSLOCK (rw) register accessor: Mass Erase Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`masslock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masslock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masslock`] module"]
#[doc(alias = "MASSLOCK")]
pub type Masslock = crate::Reg<masslock::MasslockSpec>;
#[doc = "Mass Erase Lock Register"]
pub mod masslock;
#[doc = "STARTUP (rw) register accessor: Startup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`startup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startup`] module"]
#[doc(alias = "STARTUP")]
pub type Startup = crate::Reg<startup::StartupSpec>;
#[doc = "Startup Control"]
pub mod startup;
#[doc = "BANKSWITCHLOCK (rw) register accessor: Bank Switching Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bankswitchlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bankswitchlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bankswitchlock`] module"]
#[doc(alias = "BANKSWITCHLOCK")]
pub type Bankswitchlock = crate::Reg<bankswitchlock::BankswitchlockSpec>;
#[doc = "Bank Switching Lock Register"]
pub mod bankswitchlock;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "BOOTLOADERCTRL (rw) register accessor: Bootloader Read and Write Enable, Write Once Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bootloaderctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootloaderctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootloaderctrl`] module"]
#[doc(alias = "BOOTLOADERCTRL")]
pub type Bootloaderctrl = crate::Reg<bootloaderctrl::BootloaderctrlSpec>;
#[doc = "Bootloader Read and Write Enable, Write Once Register"]
pub mod bootloaderctrl;
#[doc = "AAPUNLOCKCMD (w) register accessor: Software Unlock AAP Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aapunlockcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aapunlockcmd`] module"]
#[doc(alias = "AAPUNLOCKCMD")]
pub type Aapunlockcmd = crate::Reg<aapunlockcmd::AapunlockcmdSpec>;
#[doc = "Software Unlock AAP Command Register"]
pub mod aapunlockcmd;
#[doc = "CACHECONFIG0 (rw) register accessor: Cache Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cacheconfig0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacheconfig0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacheconfig0`] module"]
#[doc(alias = "CACHECONFIG0")]
pub type Cacheconfig0 = crate::Reg<cacheconfig0::Cacheconfig0Spec>;
#[doc = "Cache Configuration Register 0"]
pub mod cacheconfig0;
#[doc = "RAMCTRL (rw) register accessor: RAM Control Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ramctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramctrl`] module"]
#[doc(alias = "RAMCTRL")]
pub type Ramctrl = crate::Reg<ramctrl::RamctrlSpec>;
#[doc = "RAM Control Enable Register"]
pub mod ramctrl;
#[doc = "ECCCTRL (rw) register accessor: RAM ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccctrl`] module"]
#[doc(alias = "ECCCTRL")]
pub type Eccctrl = crate::Reg<eccctrl::EccctrlSpec>;
#[doc = "RAM ECC Control Register"]
pub mod eccctrl;
#[doc = "RAMECCADDR (r) register accessor: RAM ECC Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rameccaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rameccaddr`] module"]
#[doc(alias = "RAMECCADDR")]
pub type Rameccaddr = crate::Reg<rameccaddr::RameccaddrSpec>;
#[doc = "RAM ECC Error Address Register"]
pub mod rameccaddr;
#[doc = "RAM1ECCADDR (r) register accessor: RAM1 ECC Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1eccaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1eccaddr`] module"]
#[doc(alias = "RAM1ECCADDR")]
pub type Ram1eccaddr = crate::Reg<ram1eccaddr::Ram1eccaddrSpec>;
#[doc = "RAM1 ECC Error Address Register"]
pub mod ram1eccaddr;
