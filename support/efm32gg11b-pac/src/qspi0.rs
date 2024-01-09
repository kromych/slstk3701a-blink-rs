#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    devinstrrdconfig: DEVINSTRRDCONFIG,
    devinstrwrconfig: DEVINSTRWRCONFIG,
    devdelay: DEVDELAY,
    rddatacapture: RDDATACAPTURE,
    devsizeconfig: DEVSIZECONFIG,
    srampartitioncfg: SRAMPARTITIONCFG,
    indahbaddrtrigger: INDAHBADDRTRIGGER,
    _reserved8: [u8; 0x04],
    remapaddr: REMAPADDR,
    modebitconfig: MODEBITCONFIG,
    sramfill: SRAMFILL,
    txthresh: TXTHRESH,
    rxthresh: RXTHRESH,
    writecompletionctrl: WRITECOMPLETIONCTRL,
    noofpollsbefexp: NOOFPOLLSBEFEXP,
    irqstatus: IRQSTATUS,
    irqmask: IRQMASK,
    _reserved17: [u8; 0x08],
    lowerwrprot: LOWERWRPROT,
    upperwrprot: UPPERWRPROT,
    wrprotctrl: WRPROTCTRL,
    _reserved20: [u8; 0x04],
    indirectreadxferctrl: INDIRECTREADXFERCTRL,
    indirectreadxferwatermark: INDIRECTREADXFERWATERMARK,
    indirectreadxferstart: INDIRECTREADXFERSTART,
    indirectreadxfernumbytes: INDIRECTREADXFERNUMBYTES,
    indirectwritexferctrl: INDIRECTWRITEXFERCTRL,
    indirectwritexferwatermark: INDIRECTWRITEXFERWATERMARK,
    indirectwritexferstart: INDIRECTWRITEXFERSTART,
    indirectwritexfernumbytes: INDIRECTWRITEXFERNUMBYTES,
    indirecttriggeraddrrange: INDIRECTTRIGGERADDRRANGE,
    _reserved29: [u8; 0x08],
    flashcommandctrlmem: FLASHCOMMANDCTRLMEM,
    flashcmdctrl: FLASHCMDCTRL,
    flashcmdaddr: FLASHCMDADDR,
    _reserved32: [u8; 0x08],
    flashrddatalower: FLASHRDDATALOWER,
    flashrddataupper: FLASHRDDATAUPPER,
    flashwrdatalower: FLASHWRDATALOWER,
    flashwrdataupper: FLASHWRDATAUPPER,
    pollingflashstatus: POLLINGFLASHSTATUS,
    phyconfiguration: PHYCONFIGURATION,
    _reserved38: [u8; 0x28],
    opcodeextlower: OPCODEEXTLOWER,
    opcodeextupper: OPCODEEXTUPPER,
    _reserved40: [u8; 0x14],
    moduleid: MODULEID,
    _reserved41: [u8; 0x04],
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
}
impl RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    #[inline(always)]
    pub const fn devinstrrdconfig(&self) -> &DEVINSTRRDCONFIG {
        &self.devinstrrdconfig
    }
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    #[inline(always)]
    pub const fn devinstrwrconfig(&self) -> &DEVINSTRWRCONFIG {
        &self.devinstrwrconfig
    }
    #[doc = "0x0c - Device Delay Register"]
    #[inline(always)]
    pub const fn devdelay(&self) -> &DEVDELAY {
        &self.devdelay
    }
    #[doc = "0x10 - Read Data Capture Register"]
    #[inline(always)]
    pub const fn rddatacapture(&self) -> &RDDATACAPTURE {
        &self.rddatacapture
    }
    #[doc = "0x14 - Device Size Configuration Register"]
    #[inline(always)]
    pub const fn devsizeconfig(&self) -> &DEVSIZECONFIG {
        &self.devsizeconfig
    }
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    #[inline(always)]
    pub const fn srampartitioncfg(&self) -> &SRAMPARTITIONCFG {
        &self.srampartitioncfg
    }
    #[doc = "0x1c - Indirect Address Trigger Register"]
    #[inline(always)]
    pub const fn indahbaddrtrigger(&self) -> &INDAHBADDRTRIGGER {
        &self.indahbaddrtrigger
    }
    #[doc = "0x24 - Remap Address Register"]
    #[inline(always)]
    pub const fn remapaddr(&self) -> &REMAPADDR {
        &self.remapaddr
    }
    #[doc = "0x28 - Mode Bit Configuration Register"]
    #[inline(always)]
    pub const fn modebitconfig(&self) -> &MODEBITCONFIG {
        &self.modebitconfig
    }
    #[doc = "0x2c - SRAM Fill Register"]
    #[inline(always)]
    pub const fn sramfill(&self) -> &SRAMFILL {
        &self.sramfill
    }
    #[doc = "0x30 - TX Threshold Register"]
    #[inline(always)]
    pub const fn txthresh(&self) -> &TXTHRESH {
        &self.txthresh
    }
    #[doc = "0x34 - RX Threshold Register"]
    #[inline(always)]
    pub const fn rxthresh(&self) -> &RXTHRESH {
        &self.rxthresh
    }
    #[doc = "0x38 - Write Completion Control Register"]
    #[inline(always)]
    pub const fn writecompletionctrl(&self) -> &WRITECOMPLETIONCTRL {
        &self.writecompletionctrl
    }
    #[doc = "0x3c - Polling Expiration Register"]
    #[inline(always)]
    pub const fn noofpollsbefexp(&self) -> &NOOFPOLLSBEFEXP {
        &self.noofpollsbefexp
    }
    #[doc = "0x40 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn irqstatus(&self) -> &IRQSTATUS {
        &self.irqstatus
    }
    #[doc = "0x44 - Interrupt Mask"]
    #[inline(always)]
    pub const fn irqmask(&self) -> &IRQMASK {
        &self.irqmask
    }
    #[doc = "0x50 - Lower Write Protection Register"]
    #[inline(always)]
    pub const fn lowerwrprot(&self) -> &LOWERWRPROT {
        &self.lowerwrprot
    }
    #[doc = "0x54 - Upper Write Protection Register"]
    #[inline(always)]
    pub const fn upperwrprot(&self) -> &UPPERWRPROT {
        &self.upperwrprot
    }
    #[doc = "0x58 - Write Protection Control Register"]
    #[inline(always)]
    pub const fn wrprotctrl(&self) -> &WRPROTCTRL {
        &self.wrprotctrl
    }
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    #[inline(always)]
    pub const fn indirectreadxferctrl(&self) -> &INDIRECTREADXFERCTRL {
        &self.indirectreadxferctrl
    }
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    #[inline(always)]
    pub const fn indirectreadxferwatermark(&self) -> &INDIRECTREADXFERWATERMARK {
        &self.indirectreadxferwatermark
    }
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    #[inline(always)]
    pub const fn indirectreadxferstart(&self) -> &INDIRECTREADXFERSTART {
        &self.indirectreadxferstart
    }
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    #[inline(always)]
    pub const fn indirectreadxfernumbytes(&self) -> &INDIRECTREADXFERNUMBYTES {
        &self.indirectreadxfernumbytes
    }
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    #[inline(always)]
    pub const fn indirectwritexferctrl(&self) -> &INDIRECTWRITEXFERCTRL {
        &self.indirectwritexferctrl
    }
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    #[inline(always)]
    pub const fn indirectwritexferwatermark(&self) -> &INDIRECTWRITEXFERWATERMARK {
        &self.indirectwritexferwatermark
    }
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    #[inline(always)]
    pub const fn indirectwritexferstart(&self) -> &INDIRECTWRITEXFERSTART {
        &self.indirectwritexferstart
    }
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    #[inline(always)]
    pub const fn indirectwritexfernumbytes(&self) -> &INDIRECTWRITEXFERNUMBYTES {
        &self.indirectwritexfernumbytes
    }
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    #[inline(always)]
    pub const fn indirecttriggeraddrrange(&self) -> &INDIRECTTRIGGERADDRRANGE {
        &self.indirecttriggeraddrrange
    }
    #[doc = "0x8c - Flash Command Control Memory Register (STIG)"]
    #[inline(always)]
    pub const fn flashcommandctrlmem(&self) -> &FLASHCOMMANDCTRLMEM {
        &self.flashcommandctrlmem
    }
    #[doc = "0x90 - Flash Command Control Register (STIG)"]
    #[inline(always)]
    pub const fn flashcmdctrl(&self) -> &FLASHCMDCTRL {
        &self.flashcmdctrl
    }
    #[doc = "0x94 - Flash Command Address Register (STIG)"]
    #[inline(always)]
    pub const fn flashcmdaddr(&self) -> &FLASHCMDADDR {
        &self.flashcmdaddr
    }
    #[doc = "0xa0 - Flash Command Read Data Register (Lower) (STIG)"]
    #[inline(always)]
    pub const fn flashrddatalower(&self) -> &FLASHRDDATALOWER {
        &self.flashrddatalower
    }
    #[doc = "0xa4 - Flash Command Read Data Register (Upper) (STIG)"]
    #[inline(always)]
    pub const fn flashrddataupper(&self) -> &FLASHRDDATAUPPER {
        &self.flashrddataupper
    }
    #[doc = "0xa8 - Flash Command Write Data Register (Lower) (STIG)"]
    #[inline(always)]
    pub const fn flashwrdatalower(&self) -> &FLASHWRDATALOWER {
        &self.flashwrdatalower
    }
    #[doc = "0xac - Flash Command Write Data Register (Upper) (STIG)"]
    #[inline(always)]
    pub const fn flashwrdataupper(&self) -> &FLASHWRDATAUPPER {
        &self.flashwrdataupper
    }
    #[doc = "0xb0 - Polling Flash Status Register"]
    #[inline(always)]
    pub const fn pollingflashstatus(&self) -> &POLLINGFLASHSTATUS {
        &self.pollingflashstatus
    }
    #[doc = "0xb4 - PHY Configuration Register"]
    #[inline(always)]
    pub const fn phyconfiguration(&self) -> &PHYCONFIGURATION {
        &self.phyconfiguration
    }
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    #[inline(always)]
    pub const fn opcodeextlower(&self) -> &OPCODEEXTLOWER {
        &self.opcodeextlower
    }
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    #[inline(always)]
    pub const fn opcodeextupper(&self) -> &OPCODEEXTUPPER {
        &self.opcodeextupper
    }
    #[doc = "0xfc - Module ID Register"]
    #[inline(always)]
    pub const fn moduleid(&self) -> &MODULEID {
        &self.moduleid
    }
    #[doc = "0x104 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x108 - I/O Route Location Register 0"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
}
#[doc = "CONFIG (rw) register accessor: Octal-SPI Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Octal-SPI Configuration Register"]
pub mod config;
#[doc = "DEVINSTRRDCONFIG (rw) register accessor: Device Read Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devinstrrdconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devinstrrdconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devinstrrdconfig`]
module"]
pub type DEVINSTRRDCONFIG = crate::Reg<devinstrrdconfig::DEVINSTRRDCONFIG_SPEC>;
#[doc = "Device Read Instruction Configuration Register"]
pub mod devinstrrdconfig;
#[doc = "DEVINSTRWRCONFIG (rw) register accessor: Device Write Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devinstrwrconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devinstrwrconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devinstrwrconfig`]
module"]
pub type DEVINSTRWRCONFIG = crate::Reg<devinstrwrconfig::DEVINSTRWRCONFIG_SPEC>;
#[doc = "Device Write Instruction Configuration Register"]
pub mod devinstrwrconfig;
#[doc = "DEVDELAY (rw) register accessor: Device Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdelay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdelay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devdelay`]
module"]
pub type DEVDELAY = crate::Reg<devdelay::DEVDELAY_SPEC>;
#[doc = "Device Delay Register"]
pub mod devdelay;
#[doc = "RDDATACAPTURE (rw) register accessor: Read Data Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rddatacapture::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rddatacapture::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rddatacapture`]
module"]
pub type RDDATACAPTURE = crate::Reg<rddatacapture::RDDATACAPTURE_SPEC>;
#[doc = "Read Data Capture Register"]
pub mod rddatacapture;
#[doc = "DEVSIZECONFIG (rw) register accessor: Device Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devsizeconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devsizeconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devsizeconfig`]
module"]
pub type DEVSIZECONFIG = crate::Reg<devsizeconfig::DEVSIZECONFIG_SPEC>;
#[doc = "Device Size Configuration Register"]
pub mod devsizeconfig;
#[doc = "SRAMPARTITIONCFG (rw) register accessor: SRAM Partition Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srampartitioncfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srampartitioncfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srampartitioncfg`]
module"]
pub type SRAMPARTITIONCFG = crate::Reg<srampartitioncfg::SRAMPARTITIONCFG_SPEC>;
#[doc = "SRAM Partition Configuration Register"]
pub mod srampartitioncfg;
#[doc = "INDAHBADDRTRIGGER (rw) register accessor: Indirect Address Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indahbaddrtrigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indahbaddrtrigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indahbaddrtrigger`]
module"]
pub type INDAHBADDRTRIGGER = crate::Reg<indahbaddrtrigger::INDAHBADDRTRIGGER_SPEC>;
#[doc = "Indirect Address Trigger Register"]
pub mod indahbaddrtrigger;
#[doc = "REMAPADDR (rw) register accessor: Remap Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remapaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remapaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remapaddr`]
module"]
pub type REMAPADDR = crate::Reg<remapaddr::REMAPADDR_SPEC>;
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "MODEBITCONFIG (rw) register accessor: Mode Bit Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modebitconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modebitconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modebitconfig`]
module"]
pub type MODEBITCONFIG = crate::Reg<modebitconfig::MODEBITCONFIG_SPEC>;
#[doc = "Mode Bit Configuration Register"]
pub mod modebitconfig;
#[doc = "SRAMFILL (r) register accessor: SRAM Fill Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sramfill::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramfill`]
module"]
pub type SRAMFILL = crate::Reg<sramfill::SRAMFILL_SPEC>;
#[doc = "SRAM Fill Register"]
pub mod sramfill;
#[doc = "TXTHRESH (rw) register accessor: TX Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txthresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txthresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txthresh`]
module"]
pub type TXTHRESH = crate::Reg<txthresh::TXTHRESH_SPEC>;
#[doc = "TX Threshold Register"]
pub mod txthresh;
#[doc = "RXTHRESH (rw) register accessor: RX Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxthresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxthresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxthresh`]
module"]
pub type RXTHRESH = crate::Reg<rxthresh::RXTHRESH_SPEC>;
#[doc = "RX Threshold Register"]
pub mod rxthresh;
#[doc = "WRITECOMPLETIONCTRL (rw) register accessor: Write Completion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writecompletionctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writecompletionctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writecompletionctrl`]
module"]
pub type WRITECOMPLETIONCTRL = crate::Reg<writecompletionctrl::WRITECOMPLETIONCTRL_SPEC>;
#[doc = "Write Completion Control Register"]
pub mod writecompletionctrl;
#[doc = "NOOFPOLLSBEFEXP (rw) register accessor: Polling Expiration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noofpollsbefexp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noofpollsbefexp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noofpollsbefexp`]
module"]
pub type NOOFPOLLSBEFEXP = crate::Reg<noofpollsbefexp::NOOFPOLLSBEFEXP_SPEC>;
#[doc = "Polling Expiration Register"]
pub mod noofpollsbefexp;
#[doc = "IRQSTATUS (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstatus`]
module"]
pub type IRQSTATUS = crate::Reg<irqstatus::IRQSTATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod irqstatus;
#[doc = "IRQMASK (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqmask`]
module"]
pub type IRQMASK = crate::Reg<irqmask::IRQMASK_SPEC>;
#[doc = "Interrupt Mask"]
pub mod irqmask;
#[doc = "LOWERWRPROT (rw) register accessor: Lower Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowerwrprot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lowerwrprot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowerwrprot`]
module"]
pub type LOWERWRPROT = crate::Reg<lowerwrprot::LOWERWRPROT_SPEC>;
#[doc = "Lower Write Protection Register"]
pub mod lowerwrprot;
#[doc = "UPPERWRPROT (rw) register accessor: Upper Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upperwrprot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upperwrprot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upperwrprot`]
module"]
pub type UPPERWRPROT = crate::Reg<upperwrprot::UPPERWRPROT_SPEC>;
#[doc = "Upper Write Protection Register"]
pub mod upperwrprot;
#[doc = "WRPROTCTRL (rw) register accessor: Write Protection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrprotctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrprotctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrprotctrl`]
module"]
pub type WRPROTCTRL = crate::Reg<wrprotctrl::WRPROTCTRL_SPEC>;
#[doc = "Write Protection Control Register"]
pub mod wrprotctrl;
#[doc = "INDIRECTREADXFERCTRL (rw) register accessor: Indirect Read Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectreadxferctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectreadxferctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxferctrl`]
module"]
pub type INDIRECTREADXFERCTRL = crate::Reg<indirectreadxferctrl::INDIRECTREADXFERCTRL_SPEC>;
#[doc = "Indirect Read Transfer Control Register"]
pub mod indirectreadxferctrl;
#[doc = "INDIRECTREADXFERWATERMARK (rw) register accessor: Indirect Read Transfer Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectreadxferwatermark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectreadxferwatermark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxferwatermark`]
module"]
pub type INDIRECTREADXFERWATERMARK =
    crate::Reg<indirectreadxferwatermark::INDIRECTREADXFERWATERMARK_SPEC>;
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod indirectreadxferwatermark;
#[doc = "INDIRECTREADXFERSTART (rw) register accessor: Indirect Read Transfer Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectreadxferstart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectreadxferstart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxferstart`]
module"]
pub type INDIRECTREADXFERSTART = crate::Reg<indirectreadxferstart::INDIRECTREADXFERSTART_SPEC>;
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod indirectreadxferstart;
#[doc = "INDIRECTREADXFERNUMBYTES (rw) register accessor: Indirect Read Transfer Number Bytes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectreadxfernumbytes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectreadxfernumbytes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectreadxfernumbytes`]
module"]
pub type INDIRECTREADXFERNUMBYTES =
    crate::Reg<indirectreadxfernumbytes::INDIRECTREADXFERNUMBYTES_SPEC>;
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod indirectreadxfernumbytes;
#[doc = "INDIRECTWRITEXFERCTRL (rw) register accessor: Indirect Write Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexferctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexferctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexferctrl`]
module"]
pub type INDIRECTWRITEXFERCTRL = crate::Reg<indirectwritexferctrl::INDIRECTWRITEXFERCTRL_SPEC>;
#[doc = "Indirect Write Transfer Control Register"]
pub mod indirectwritexferctrl;
#[doc = "INDIRECTWRITEXFERWATERMARK (rw) register accessor: Indirect Write Transfer Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexferwatermark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexferwatermark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexferwatermark`]
module"]
pub type INDIRECTWRITEXFERWATERMARK =
    crate::Reg<indirectwritexferwatermark::INDIRECTWRITEXFERWATERMARK_SPEC>;
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod indirectwritexferwatermark;
#[doc = "INDIRECTWRITEXFERSTART (rw) register accessor: Indirect Write Transfer Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexferstart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexferstart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexferstart`]
module"]
pub type INDIRECTWRITEXFERSTART = crate::Reg<indirectwritexferstart::INDIRECTWRITEXFERSTART_SPEC>;
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod indirectwritexferstart;
#[doc = "INDIRECTWRITEXFERNUMBYTES (rw) register accessor: Indirect Write Transfer Number Bytes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexfernumbytes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexfernumbytes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirectwritexfernumbytes`]
module"]
pub type INDIRECTWRITEXFERNUMBYTES =
    crate::Reg<indirectwritexfernumbytes::INDIRECTWRITEXFERNUMBYTES_SPEC>;
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod indirectwritexfernumbytes;
#[doc = "INDIRECTTRIGGERADDRRANGE (rw) register accessor: Indirect Trigger Address Range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirecttriggeraddrrange::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirecttriggeraddrrange::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirecttriggeraddrrange`]
module"]
pub type INDIRECTTRIGGERADDRRANGE =
    crate::Reg<indirecttriggeraddrrange::INDIRECTTRIGGERADDRRANGE_SPEC>;
#[doc = "Indirect Trigger Address Range Register"]
pub mod indirecttriggeraddrrange;
#[doc = "FLASHCOMMANDCTRLMEM (rw) register accessor: Flash Command Control Memory Register (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcommandctrlmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcommandctrlmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcommandctrlmem`]
module"]
pub type FLASHCOMMANDCTRLMEM = crate::Reg<flashcommandctrlmem::FLASHCOMMANDCTRLMEM_SPEC>;
#[doc = "Flash Command Control Memory Register (STIG)"]
pub mod flashcommandctrlmem;
#[doc = "FLASHCMDCTRL (rw) register accessor: Flash Command Control Register (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcmdctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcmdctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdctrl`]
module"]
pub type FLASHCMDCTRL = crate::Reg<flashcmdctrl::FLASHCMDCTRL_SPEC>;
#[doc = "Flash Command Control Register (STIG)"]
pub mod flashcmdctrl;
#[doc = "FLASHCMDADDR (rw) register accessor: Flash Command Address Register (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcmdaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcmdaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcmdaddr`]
module"]
pub type FLASHCMDADDR = crate::Reg<flashcmdaddr::FLASHCMDADDR_SPEC>;
#[doc = "Flash Command Address Register (STIG)"]
pub mod flashcmdaddr;
#[doc = "FLASHRDDATALOWER (r) register accessor: Flash Command Read Data Register (Lower) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashrddatalower::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrddatalower`]
module"]
pub type FLASHRDDATALOWER = crate::Reg<flashrddatalower::FLASHRDDATALOWER_SPEC>;
#[doc = "Flash Command Read Data Register (Lower) (STIG)"]
pub mod flashrddatalower;
#[doc = "FLASHRDDATAUPPER (r) register accessor: Flash Command Read Data Register (Upper) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashrddataupper::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrddataupper`]
module"]
pub type FLASHRDDATAUPPER = crate::Reg<flashrddataupper::FLASHRDDATAUPPER_SPEC>;
#[doc = "Flash Command Read Data Register (Upper) (STIG)"]
pub mod flashrddataupper;
#[doc = "FLASHWRDATALOWER (rw) register accessor: Flash Command Write Data Register (Lower) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashwrdatalower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashwrdatalower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwrdatalower`]
module"]
pub type FLASHWRDATALOWER = crate::Reg<flashwrdatalower::FLASHWRDATALOWER_SPEC>;
#[doc = "Flash Command Write Data Register (Lower) (STIG)"]
pub mod flashwrdatalower;
#[doc = "FLASHWRDATAUPPER (rw) register accessor: Flash Command Write Data Register (Upper) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashwrdataupper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashwrdataupper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashwrdataupper`]
module"]
pub type FLASHWRDATAUPPER = crate::Reg<flashwrdataupper::FLASHWRDATAUPPER_SPEC>;
#[doc = "Flash Command Write Data Register (Upper) (STIG)"]
pub mod flashwrdataupper;
#[doc = "POLLINGFLASHSTATUS (rw) register accessor: Polling Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pollingflashstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pollingflashstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pollingflashstatus`]
module"]
pub type POLLINGFLASHSTATUS = crate::Reg<pollingflashstatus::POLLINGFLASHSTATUS_SPEC>;
#[doc = "Polling Flash Status Register"]
pub mod pollingflashstatus;
#[doc = "PHYCONFIGURATION (rw) register accessor: PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyconfiguration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phyconfiguration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phyconfiguration`]
module"]
pub type PHYCONFIGURATION = crate::Reg<phyconfiguration::PHYCONFIGURATION_SPEC>;
#[doc = "PHY Configuration Register"]
pub mod phyconfiguration;
#[doc = "OPCODEEXTLOWER (rw) register accessor: Opcode Extension Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opcodeextlower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opcodeextlower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcodeextlower`]
module"]
pub type OPCODEEXTLOWER = crate::Reg<opcodeextlower::OPCODEEXTLOWER_SPEC>;
#[doc = "Opcode Extension Register (Lower)"]
pub mod opcodeextlower;
#[doc = "OPCODEEXTUPPER (rw) register accessor: Opcode Extension Register (Upper)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opcodeextupper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opcodeextupper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcodeextupper`]
module"]
pub type OPCODEEXTUPPER = crate::Reg<opcodeextupper::OPCODEEXTUPPER_SPEC>;
#[doc = "Opcode Extension Register (Upper)"]
pub mod opcodeextupper;
#[doc = "MODULEID (r) register accessor: Module ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moduleid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moduleid`]
module"]
pub type MODULEID = crate::Reg<moduleid::MODULEID_SPEC>;
#[doc = "Module ID Register"]
pub mod moduleid;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Route Location Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
