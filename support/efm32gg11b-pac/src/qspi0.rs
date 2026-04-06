#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    devinstrrdconfig: Devinstrrdconfig,
    devinstrwrconfig: Devinstrwrconfig,
    devdelay: Devdelay,
    rddatacapture: Rddatacapture,
    devsizeconfig: Devsizeconfig,
    srampartitioncfg: Srampartitioncfg,
    indahbaddrtrigger: Indahbaddrtrigger,
    _reserved8: [u8; 0x04],
    remapaddr: Remapaddr,
    modebitconfig: Modebitconfig,
    sramfill: Sramfill,
    txthresh: Txthresh,
    rxthresh: Rxthresh,
    writecompletionctrl: Writecompletionctrl,
    noofpollsbefexp: Noofpollsbefexp,
    irqstatus: Irqstatus,
    irqmask: Irqmask,
    _reserved17: [u8; 0x08],
    lowerwrprot: Lowerwrprot,
    upperwrprot: Upperwrprot,
    wrprotctrl: Wrprotctrl,
    _reserved20: [u8; 0x04],
    indirectreadxferctrl: Indirectreadxferctrl,
    indirectreadxferwatermark: Indirectreadxferwatermark,
    indirectreadxferstart: Indirectreadxferstart,
    indirectreadxfernumbytes: Indirectreadxfernumbytes,
    indirectwritexferctrl: Indirectwritexferctrl,
    indirectwritexferwatermark: Indirectwritexferwatermark,
    indirectwritexferstart: Indirectwritexferstart,
    indirectwritexfernumbytes: Indirectwritexfernumbytes,
    indirecttriggeraddrrange: Indirecttriggeraddrrange,
    _reserved29: [u8; 0x08],
    flashcommandctrlmem: Flashcommandctrlmem,
    flashcmdctrl: Flashcmdctrl,
    flashcmdaddr: Flashcmdaddr,
    _reserved32: [u8; 0x08],
    flashrddatalower: Flashrddatalower,
    flashrddataupper: Flashrddataupper,
    flashwrdatalower: Flashwrdatalower,
    flashwrdataupper: Flashwrdataupper,
    pollingflashstatus: Pollingflashstatus,
    phyconfiguration: Phyconfiguration,
    _reserved38: [u8; 0x28],
    opcodeextlower: Opcodeextlower,
    opcodeextupper: Opcodeextupper,
    _reserved40: [u8; 0x14],
    moduleid: Moduleid,
    _reserved41: [u8; 0x04],
    routepen: Routepen,
    routeloc0: Routeloc0,
}
impl RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    #[inline(always)]
    pub const fn devinstrrdconfig(&self) -> &Devinstrrdconfig {
        &self.devinstrrdconfig
    }
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    #[inline(always)]
    pub const fn devinstrwrconfig(&self) -> &Devinstrwrconfig {
        &self.devinstrwrconfig
    }
    #[doc = "0x0c - Device Delay Register"]
    #[inline(always)]
    pub const fn devdelay(&self) -> &Devdelay {
        &self.devdelay
    }
    #[doc = "0x10 - Read Data Capture Register"]
    #[inline(always)]
    pub const fn rddatacapture(&self) -> &Rddatacapture {
        &self.rddatacapture
    }
    #[doc = "0x14 - Device Size Configuration Register"]
    #[inline(always)]
    pub const fn devsizeconfig(&self) -> &Devsizeconfig {
        &self.devsizeconfig
    }
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    #[inline(always)]
    pub const fn srampartitioncfg(&self) -> &Srampartitioncfg {
        &self.srampartitioncfg
    }
    #[doc = "0x1c - Indirect Address Trigger Register"]
    #[inline(always)]
    pub const fn indahbaddrtrigger(&self) -> &Indahbaddrtrigger {
        &self.indahbaddrtrigger
    }
    #[doc = "0x24 - Remap Address Register"]
    #[inline(always)]
    pub const fn remapaddr(&self) -> &Remapaddr {
        &self.remapaddr
    }
    #[doc = "0x28 - Mode Bit Configuration Register"]
    #[inline(always)]
    pub const fn modebitconfig(&self) -> &Modebitconfig {
        &self.modebitconfig
    }
    #[doc = "0x2c - SRAM Fill Register"]
    #[inline(always)]
    pub const fn sramfill(&self) -> &Sramfill {
        &self.sramfill
    }
    #[doc = "0x30 - TX Threshold Register"]
    #[inline(always)]
    pub const fn txthresh(&self) -> &Txthresh {
        &self.txthresh
    }
    #[doc = "0x34 - RX Threshold Register"]
    #[inline(always)]
    pub const fn rxthresh(&self) -> &Rxthresh {
        &self.rxthresh
    }
    #[doc = "0x38 - Write Completion Control Register"]
    #[inline(always)]
    pub const fn writecompletionctrl(&self) -> &Writecompletionctrl {
        &self.writecompletionctrl
    }
    #[doc = "0x3c - Polling Expiration Register"]
    #[inline(always)]
    pub const fn noofpollsbefexp(&self) -> &Noofpollsbefexp {
        &self.noofpollsbefexp
    }
    #[doc = "0x40 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn irqstatus(&self) -> &Irqstatus {
        &self.irqstatus
    }
    #[doc = "0x44 - Interrupt Mask"]
    #[inline(always)]
    pub const fn irqmask(&self) -> &Irqmask {
        &self.irqmask
    }
    #[doc = "0x50 - Lower Write Protection Register"]
    #[inline(always)]
    pub const fn lowerwrprot(&self) -> &Lowerwrprot {
        &self.lowerwrprot
    }
    #[doc = "0x54 - Upper Write Protection Register"]
    #[inline(always)]
    pub const fn upperwrprot(&self) -> &Upperwrprot {
        &self.upperwrprot
    }
    #[doc = "0x58 - Write Protection Control Register"]
    #[inline(always)]
    pub const fn wrprotctrl(&self) -> &Wrprotctrl {
        &self.wrprotctrl
    }
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    #[inline(always)]
    pub const fn indirectreadxferctrl(&self) -> &Indirectreadxferctrl {
        &self.indirectreadxferctrl
    }
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    #[inline(always)]
    pub const fn indirectreadxferwatermark(&self) -> &Indirectreadxferwatermark {
        &self.indirectreadxferwatermark
    }
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    #[inline(always)]
    pub const fn indirectreadxferstart(&self) -> &Indirectreadxferstart {
        &self.indirectreadxferstart
    }
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    #[inline(always)]
    pub const fn indirectreadxfernumbytes(&self) -> &Indirectreadxfernumbytes {
        &self.indirectreadxfernumbytes
    }
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    #[inline(always)]
    pub const fn indirectwritexferctrl(&self) -> &Indirectwritexferctrl {
        &self.indirectwritexferctrl
    }
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    #[inline(always)]
    pub const fn indirectwritexferwatermark(&self) -> &Indirectwritexferwatermark {
        &self.indirectwritexferwatermark
    }
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    #[inline(always)]
    pub const fn indirectwritexferstart(&self) -> &Indirectwritexferstart {
        &self.indirectwritexferstart
    }
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    #[inline(always)]
    pub const fn indirectwritexfernumbytes(&self) -> &Indirectwritexfernumbytes {
        &self.indirectwritexfernumbytes
    }
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    #[inline(always)]
    pub const fn indirecttriggeraddrrange(&self) -> &Indirecttriggeraddrrange {
        &self.indirecttriggeraddrrange
    }
    #[doc = "0x8c - Flash Command Control Memory Register (STIG)"]
    #[inline(always)]
    pub const fn flashcommandctrlmem(&self) -> &Flashcommandctrlmem {
        &self.flashcommandctrlmem
    }
    #[doc = "0x90 - Flash Command Control Register (STIG)"]
    #[inline(always)]
    pub const fn flashcmdctrl(&self) -> &Flashcmdctrl {
        &self.flashcmdctrl
    }
    #[doc = "0x94 - Flash Command Address Register (STIG)"]
    #[inline(always)]
    pub const fn flashcmdaddr(&self) -> &Flashcmdaddr {
        &self.flashcmdaddr
    }
    #[doc = "0xa0 - Flash Command Read Data Register (Lower) (STIG)"]
    #[inline(always)]
    pub const fn flashrddatalower(&self) -> &Flashrddatalower {
        &self.flashrddatalower
    }
    #[doc = "0xa4 - Flash Command Read Data Register (Upper) (STIG)"]
    #[inline(always)]
    pub const fn flashrddataupper(&self) -> &Flashrddataupper {
        &self.flashrddataupper
    }
    #[doc = "0xa8 - Flash Command Write Data Register (Lower) (STIG)"]
    #[inline(always)]
    pub const fn flashwrdatalower(&self) -> &Flashwrdatalower {
        &self.flashwrdatalower
    }
    #[doc = "0xac - Flash Command Write Data Register (Upper) (STIG)"]
    #[inline(always)]
    pub const fn flashwrdataupper(&self) -> &Flashwrdataupper {
        &self.flashwrdataupper
    }
    #[doc = "0xb0 - Polling Flash Status Register"]
    #[inline(always)]
    pub const fn pollingflashstatus(&self) -> &Pollingflashstatus {
        &self.pollingflashstatus
    }
    #[doc = "0xb4 - PHY Configuration Register"]
    #[inline(always)]
    pub const fn phyconfiguration(&self) -> &Phyconfiguration {
        &self.phyconfiguration
    }
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    #[inline(always)]
    pub const fn opcodeextlower(&self) -> &Opcodeextlower {
        &self.opcodeextlower
    }
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    #[inline(always)]
    pub const fn opcodeextupper(&self) -> &Opcodeextupper {
        &self.opcodeextupper
    }
    #[doc = "0xfc - Module ID Register"]
    #[inline(always)]
    pub const fn moduleid(&self) -> &Moduleid {
        &self.moduleid
    }
    #[doc = "0x104 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x108 - I/O Route Location Register 0"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
}
#[doc = "CONFIG (rw) register accessor: Octal-SPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Octal-SPI Configuration Register"]
pub mod config;
#[doc = "DEVINSTRRDCONFIG (rw) register accessor: Device Read Instruction Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devinstrrdconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinstrrdconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devinstrrdconfig`] module"]
#[doc(alias = "DEVINSTRRDCONFIG")]
pub type Devinstrrdconfig = crate::Reg<devinstrrdconfig::DevinstrrdconfigSpec>;
#[doc = "Device Read Instruction Configuration Register"]
pub mod devinstrrdconfig;
#[doc = "DEVINSTRWRCONFIG (rw) register accessor: Device Write Instruction Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devinstrwrconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinstrwrconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devinstrwrconfig`] module"]
#[doc(alias = "DEVINSTRWRCONFIG")]
pub type Devinstrwrconfig = crate::Reg<devinstrwrconfig::DevinstrwrconfigSpec>;
#[doc = "Device Write Instruction Configuration Register"]
pub mod devinstrwrconfig;
#[doc = "DEVDELAY (rw) register accessor: Device Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdelay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdelay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdelay`] module"]
#[doc(alias = "DEVDELAY")]
pub type Devdelay = crate::Reg<devdelay::DevdelaySpec>;
#[doc = "Device Delay Register"]
pub mod devdelay;
#[doc = "RDDATACAPTURE (rw) register accessor: Read Data Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rddatacapture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rddatacapture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rddatacapture`] module"]
#[doc(alias = "RDDATACAPTURE")]
pub type Rddatacapture = crate::Reg<rddatacapture::RddatacaptureSpec>;
#[doc = "Read Data Capture Register"]
pub mod rddatacapture;
#[doc = "DEVSIZECONFIG (rw) register accessor: Device Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devsizeconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devsizeconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devsizeconfig`] module"]
#[doc(alias = "DEVSIZECONFIG")]
pub type Devsizeconfig = crate::Reg<devsizeconfig::DevsizeconfigSpec>;
#[doc = "Device Size Configuration Register"]
pub mod devsizeconfig;
#[doc = "SRAMPARTITIONCFG (rw) register accessor: SRAM Partition Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srampartitioncfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srampartitioncfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srampartitioncfg`] module"]
#[doc(alias = "SRAMPARTITIONCFG")]
pub type Srampartitioncfg = crate::Reg<srampartitioncfg::SrampartitioncfgSpec>;
#[doc = "SRAM Partition Configuration Register"]
pub mod srampartitioncfg;
#[doc = "INDAHBADDRTRIGGER (rw) register accessor: Indirect Address Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indahbaddrtrigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indahbaddrtrigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indahbaddrtrigger`] module"]
#[doc(alias = "INDAHBADDRTRIGGER")]
pub type Indahbaddrtrigger = crate::Reg<indahbaddrtrigger::IndahbaddrtriggerSpec>;
#[doc = "Indirect Address Trigger Register"]
pub mod indahbaddrtrigger;
#[doc = "REMAPADDR (rw) register accessor: Remap Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remapaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remapaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remapaddr`] module"]
#[doc(alias = "REMAPADDR")]
pub type Remapaddr = crate::Reg<remapaddr::RemapaddrSpec>;
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "MODEBITCONFIG (rw) register accessor: Mode Bit Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modebitconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modebitconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modebitconfig`] module"]
#[doc(alias = "MODEBITCONFIG")]
pub type Modebitconfig = crate::Reg<modebitconfig::ModebitconfigSpec>;
#[doc = "Mode Bit Configuration Register"]
pub mod modebitconfig;
#[doc = "SRAMFILL (r) register accessor: SRAM Fill Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sramfill::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramfill`] module"]
#[doc(alias = "SRAMFILL")]
pub type Sramfill = crate::Reg<sramfill::SramfillSpec>;
#[doc = "SRAM Fill Register"]
pub mod sramfill;
#[doc = "TXTHRESH (rw) register accessor: TX Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txthresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txthresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txthresh`] module"]
#[doc(alias = "TXTHRESH")]
pub type Txthresh = crate::Reg<txthresh::TxthreshSpec>;
#[doc = "TX Threshold Register"]
pub mod txthresh;
#[doc = "RXTHRESH (rw) register accessor: RX Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxthresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxthresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxthresh`] module"]
#[doc(alias = "RXTHRESH")]
pub type Rxthresh = crate::Reg<rxthresh::RxthreshSpec>;
#[doc = "RX Threshold Register"]
pub mod rxthresh;
#[doc = "WRITECOMPLETIONCTRL (rw) register accessor: Write Completion Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`writecompletionctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecompletionctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writecompletionctrl`] module"]
#[doc(alias = "WRITECOMPLETIONCTRL")]
pub type Writecompletionctrl = crate::Reg<writecompletionctrl::WritecompletionctrlSpec>;
#[doc = "Write Completion Control Register"]
pub mod writecompletionctrl;
#[doc = "NOOFPOLLSBEFEXP (rw) register accessor: Polling Expiration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`noofpollsbefexp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noofpollsbefexp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noofpollsbefexp`] module"]
#[doc(alias = "NOOFPOLLSBEFEXP")]
pub type Noofpollsbefexp = crate::Reg<noofpollsbefexp::NoofpollsbefexpSpec>;
#[doc = "Polling Expiration Register"]
pub mod noofpollsbefexp;
#[doc = "IRQSTATUS (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstatus`] module"]
#[doc(alias = "IRQSTATUS")]
pub type Irqstatus = crate::Reg<irqstatus::IrqstatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod irqstatus;
#[doc = "IRQMASK (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`irqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqmask`] module"]
#[doc(alias = "IRQMASK")]
pub type Irqmask = crate::Reg<irqmask::IrqmaskSpec>;
#[doc = "Interrupt Mask"]
pub mod irqmask;
#[doc = "LOWERWRPROT (rw) register accessor: Lower Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lowerwrprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowerwrprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowerwrprot`] module"]
#[doc(alias = "LOWERWRPROT")]
pub type Lowerwrprot = crate::Reg<lowerwrprot::LowerwrprotSpec>;
#[doc = "Lower Write Protection Register"]
pub mod lowerwrprot;
#[doc = "UPPERWRPROT (rw) register accessor: Upper Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`upperwrprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upperwrprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upperwrprot`] module"]
#[doc(alias = "UPPERWRPROT")]
pub type Upperwrprot = crate::Reg<upperwrprot::UpperwrprotSpec>;
#[doc = "Upper Write Protection Register"]
pub mod upperwrprot;
#[doc = "WRPROTCTRL (rw) register accessor: Write Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrprotctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrprotctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrprotctrl`] module"]
#[doc(alias = "WRPROTCTRL")]
pub type Wrprotctrl = crate::Reg<wrprotctrl::WrprotctrlSpec>;
#[doc = "Write Protection Control Register"]
pub mod wrprotctrl;
#[doc = "INDIRECTREADXFERCTRL (rw) register accessor: Indirect Read Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxferctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxferctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxferctrl`] module"]
#[doc(alias = "INDIRECTREADXFERCTRL")]
pub type Indirectreadxferctrl = crate::Reg<indirectreadxferctrl::IndirectreadxferctrlSpec>;
#[doc = "Indirect Read Transfer Control Register"]
pub mod indirectreadxferctrl;
#[doc = "INDIRECTREADXFERWATERMARK (rw) register accessor: Indirect Read Transfer Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxferwatermark::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxferwatermark::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxferwatermark`] module"]
#[doc(alias = "INDIRECTREADXFERWATERMARK")]
pub type Indirectreadxferwatermark =
    crate::Reg<indirectreadxferwatermark::IndirectreadxferwatermarkSpec>;
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod indirectreadxferwatermark;
#[doc = "INDIRECTREADXFERSTART (rw) register accessor: Indirect Read Transfer Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxferstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxferstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxferstart`] module"]
#[doc(alias = "INDIRECTREADXFERSTART")]
pub type Indirectreadxferstart = crate::Reg<indirectreadxferstart::IndirectreadxferstartSpec>;
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod indirectreadxferstart;
#[doc = "INDIRECTREADXFERNUMBYTES (rw) register accessor: Indirect Read Transfer Number Bytes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxfernumbytes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxfernumbytes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxfernumbytes`] module"]
#[doc(alias = "INDIRECTREADXFERNUMBYTES")]
pub type Indirectreadxfernumbytes =
    crate::Reg<indirectreadxfernumbytes::IndirectreadxfernumbytesSpec>;
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod indirectreadxfernumbytes;
#[doc = "INDIRECTWRITEXFERCTRL (rw) register accessor: Indirect Write Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexferctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexferctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexferctrl`] module"]
#[doc(alias = "INDIRECTWRITEXFERCTRL")]
pub type Indirectwritexferctrl = crate::Reg<indirectwritexferctrl::IndirectwritexferctrlSpec>;
#[doc = "Indirect Write Transfer Control Register"]
pub mod indirectwritexferctrl;
#[doc = "INDIRECTWRITEXFERWATERMARK (rw) register accessor: Indirect Write Transfer Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexferwatermark::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexferwatermark::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexferwatermark`] module"]
#[doc(alias = "INDIRECTWRITEXFERWATERMARK")]
pub type Indirectwritexferwatermark =
    crate::Reg<indirectwritexferwatermark::IndirectwritexferwatermarkSpec>;
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod indirectwritexferwatermark;
#[doc = "INDIRECTWRITEXFERSTART (rw) register accessor: Indirect Write Transfer Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexferstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexferstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexferstart`] module"]
#[doc(alias = "INDIRECTWRITEXFERSTART")]
pub type Indirectwritexferstart = crate::Reg<indirectwritexferstart::IndirectwritexferstartSpec>;
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod indirectwritexferstart;
#[doc = "INDIRECTWRITEXFERNUMBYTES (rw) register accessor: Indirect Write Transfer Number Bytes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexfernumbytes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexfernumbytes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexfernumbytes`] module"]
#[doc(alias = "INDIRECTWRITEXFERNUMBYTES")]
pub type Indirectwritexfernumbytes =
    crate::Reg<indirectwritexfernumbytes::IndirectwritexfernumbytesSpec>;
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod indirectwritexfernumbytes;
#[doc = "INDIRECTTRIGGERADDRRANGE (rw) register accessor: Indirect Trigger Address Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirecttriggeraddrrange::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirecttriggeraddrrange::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirecttriggeraddrrange`] module"]
#[doc(alias = "INDIRECTTRIGGERADDRRANGE")]
pub type Indirecttriggeraddrrange =
    crate::Reg<indirecttriggeraddrrange::IndirecttriggeraddrrangeSpec>;
#[doc = "Indirect Trigger Address Range Register"]
pub mod indirecttriggeraddrrange;
#[doc = "FLASHCOMMANDCTRLMEM (rw) register accessor: Flash Command Control Memory Register (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcommandctrlmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcommandctrlmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcommandctrlmem`] module"]
#[doc(alias = "FLASHCOMMANDCTRLMEM")]
pub type Flashcommandctrlmem = crate::Reg<flashcommandctrlmem::FlashcommandctrlmemSpec>;
#[doc = "Flash Command Control Memory Register (STIG)"]
pub mod flashcommandctrlmem;
#[doc = "FLASHCMDCTRL (rw) register accessor: Flash Command Control Register (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdctrl`] module"]
#[doc(alias = "FLASHCMDCTRL")]
pub type Flashcmdctrl = crate::Reg<flashcmdctrl::FlashcmdctrlSpec>;
#[doc = "Flash Command Control Register (STIG)"]
pub mod flashcmdctrl;
#[doc = "FLASHCMDADDR (rw) register accessor: Flash Command Address Register (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdaddr`] module"]
#[doc(alias = "FLASHCMDADDR")]
pub type Flashcmdaddr = crate::Reg<flashcmdaddr::FlashcmdaddrSpec>;
#[doc = "Flash Command Address Register (STIG)"]
pub mod flashcmdaddr;
#[doc = "FLASHRDDATALOWER (r) register accessor: Flash Command Read Data Register (Lower) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrddatalower::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrddatalower`] module"]
#[doc(alias = "FLASHRDDATALOWER")]
pub type Flashrddatalower = crate::Reg<flashrddatalower::FlashrddatalowerSpec>;
#[doc = "Flash Command Read Data Register (Lower) (STIG)"]
pub mod flashrddatalower;
#[doc = "FLASHRDDATAUPPER (r) register accessor: Flash Command Read Data Register (Upper) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrddataupper::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrddataupper`] module"]
#[doc(alias = "FLASHRDDATAUPPER")]
pub type Flashrddataupper = crate::Reg<flashrddataupper::FlashrddataupperSpec>;
#[doc = "Flash Command Read Data Register (Upper) (STIG)"]
pub mod flashrddataupper;
#[doc = "FLASHWRDATALOWER (rw) register accessor: Flash Command Write Data Register (Lower) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwrdatalower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwrdatalower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwrdatalower`] module"]
#[doc(alias = "FLASHWRDATALOWER")]
pub type Flashwrdatalower = crate::Reg<flashwrdatalower::FlashwrdatalowerSpec>;
#[doc = "Flash Command Write Data Register (Lower) (STIG)"]
pub mod flashwrdatalower;
#[doc = "FLASHWRDATAUPPER (rw) register accessor: Flash Command Write Data Register (Upper) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashwrdataupper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashwrdataupper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwrdataupper`] module"]
#[doc(alias = "FLASHWRDATAUPPER")]
pub type Flashwrdataupper = crate::Reg<flashwrdataupper::FlashwrdataupperSpec>;
#[doc = "Flash Command Write Data Register (Upper) (STIG)"]
pub mod flashwrdataupper;
#[doc = "POLLINGFLASHSTATUS (rw) register accessor: Polling Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pollingflashstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pollingflashstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pollingflashstatus`] module"]
#[doc(alias = "POLLINGFLASHSTATUS")]
pub type Pollingflashstatus = crate::Reg<pollingflashstatus::PollingflashstatusSpec>;
#[doc = "Polling Flash Status Register"]
pub mod pollingflashstatus;
#[doc = "PHYCONFIGURATION (rw) register accessor: PHY Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`phyconfiguration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phyconfiguration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phyconfiguration`] module"]
#[doc(alias = "PHYCONFIGURATION")]
pub type Phyconfiguration = crate::Reg<phyconfiguration::PhyconfigurationSpec>;
#[doc = "PHY Configuration Register"]
pub mod phyconfiguration;
#[doc = "OPCODEEXTLOWER (rw) register accessor: Opcode Extension Register (Lower)\n\nYou can [`read`](crate::Reg::read) this register and get [`opcodeextlower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcodeextlower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcodeextlower`] module"]
#[doc(alias = "OPCODEEXTLOWER")]
pub type Opcodeextlower = crate::Reg<opcodeextlower::OpcodeextlowerSpec>;
#[doc = "Opcode Extension Register (Lower)"]
pub mod opcodeextlower;
#[doc = "OPCODEEXTUPPER (rw) register accessor: Opcode Extension Register (Upper)\n\nYou can [`read`](crate::Reg::read) this register and get [`opcodeextupper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcodeextupper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcodeextupper`] module"]
#[doc(alias = "OPCODEEXTUPPER")]
pub type Opcodeextupper = crate::Reg<opcodeextupper::OpcodeextupperSpec>;
#[doc = "Opcode Extension Register (Upper)"]
pub mod opcodeextupper;
#[doc = "MODULEID (r) register accessor: Module ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`moduleid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moduleid`] module"]
#[doc(alias = "MODULEID")]
pub type Moduleid = crate::Reg<moduleid::ModuleidSpec>;
#[doc = "Module ID Register"]
pub mod moduleid;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Route Location Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`] module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::Routeloc0Spec>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
