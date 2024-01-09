#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    etmcr: ETMCR,
    etmccr: ETMCCR,
    etmtrigger: ETMTRIGGER,
    _reserved3: [u8; 0x04],
    etmsr: ETMSR,
    etmscr: ETMSCR,
    _reserved5: [u8; 0x08],
    etmteevr: ETMTEEVR,
    etmtecr1: ETMTECR1,
    _reserved7: [u8; 0x04],
    etmfflr: ETMFFLR,
    _reserved8: [u8; 0x0110],
    etmcntrldvr1: ETMCNTRLDVR1,
    _reserved9: [u8; 0x9c],
    etmsyncfr: ETMSYNCFR,
    etmidr: ETMIDR,
    etmccer: ETMCCER,
    _reserved12: [u8; 0x04],
    etmtesseicr: ETMTESSEICR,
    _reserved13: [u8; 0x04],
    etmtsevr: ETMTSEVR,
    _reserved14: [u8; 0x04],
    etmtraceidr: ETMTRACEIDR,
    _reserved15: [u8; 0x04],
    etmidr2: ETMIDR2,
    _reserved16: [u8; 0x0108],
    etmpdsr: ETMPDSR,
    _reserved17: [u8; 0x0bc8],
    etmiscin: ETMISCIN,
    _reserved18: [u8; 0x04],
    ittrigout: ITTRIGOUT,
    _reserved19: [u8; 0x04],
    etmitatbctr2: ETMITATBCTR2,
    _reserved20: [u8; 0x04],
    etmitatbctr0: ETMITATBCTR0,
    _reserved21: [u8; 0x04],
    etmitctrl: ETMITCTRL,
    _reserved22: [u8; 0x9c],
    etmclaimset: ETMCLAIMSET,
    etmclaimclr: ETMCLAIMCLR,
    _reserved24: [u8; 0x08],
    etmlar: ETMLAR,
    etmlsr: ETMLSR,
    etmauthstatus: ETMAUTHSTATUS,
    _reserved27: [u8; 0x10],
    etmdevtype: ETMDEVTYPE,
    etmpidr4: ETMPIDR4,
    etmpidr5: ETMPIDR5,
    etmpidr6: ETMPIDR6,
    etmpidr7: ETMPIDR7,
    etmpidr0: ETMPIDR0,
    etmpidr1: ETMPIDR1,
    etmpidr2: ETMPIDR2,
    etmpidr3: ETMPIDR3,
    etmcidr0: ETMCIDR0,
    etmcidr1: ETMCIDR1,
    etmcidr2: ETMCIDR2,
    etmcidr3: ETMCIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    #[inline(always)]
    pub const fn etmcr(&self) -> &ETMCR {
        &self.etmcr
    }
    #[doc = "0x04 - Configuration Code Register"]
    #[inline(always)]
    pub const fn etmccr(&self) -> &ETMCCR {
        &self.etmccr
    }
    #[doc = "0x08 - ETM Trigger Event Register"]
    #[inline(always)]
    pub const fn etmtrigger(&self) -> &ETMTRIGGER {
        &self.etmtrigger
    }
    #[doc = "0x10 - ETM Status Register"]
    #[inline(always)]
    pub const fn etmsr(&self) -> &ETMSR {
        &self.etmsr
    }
    #[doc = "0x14 - ETM System Configuration Register"]
    #[inline(always)]
    pub const fn etmscr(&self) -> &ETMSCR {
        &self.etmscr
    }
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    #[inline(always)]
    pub const fn etmteevr(&self) -> &ETMTEEVR {
        &self.etmteevr
    }
    #[doc = "0x24 - ETM Trace control Register"]
    #[inline(always)]
    pub const fn etmtecr1(&self) -> &ETMTECR1 {
        &self.etmtecr1
    }
    #[doc = "0x2c - ETM Fifo Full Level Register"]
    #[inline(always)]
    pub const fn etmfflr(&self) -> &ETMFFLR {
        &self.etmfflr
    }
    #[doc = "0x140 - Counter Reload Value"]
    #[inline(always)]
    pub const fn etmcntrldvr1(&self) -> &ETMCNTRLDVR1 {
        &self.etmcntrldvr1
    }
    #[doc = "0x1e0 - Synchronisation Frequency Register"]
    #[inline(always)]
    pub const fn etmsyncfr(&self) -> &ETMSYNCFR {
        &self.etmsyncfr
    }
    #[doc = "0x1e4 - ID Register"]
    #[inline(always)]
    pub const fn etmidr(&self) -> &ETMIDR {
        &self.etmidr
    }
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    #[inline(always)]
    pub const fn etmccer(&self) -> &ETMCCER {
        &self.etmccer
    }
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    #[inline(always)]
    pub const fn etmtesseicr(&self) -> &ETMTESSEICR {
        &self.etmtesseicr
    }
    #[doc = "0x1f8 - Timestamp Event Register"]
    #[inline(always)]
    pub const fn etmtsevr(&self) -> &ETMTSEVR {
        &self.etmtsevr
    }
    #[doc = "0x200 - CoreSight Trace ID Register"]
    #[inline(always)]
    pub const fn etmtraceidr(&self) -> &ETMTRACEIDR {
        &self.etmtraceidr
    }
    #[doc = "0x208 - ETM ID Register 2"]
    #[inline(always)]
    pub const fn etmidr2(&self) -> &ETMIDR2 {
        &self.etmidr2
    }
    #[doc = "0x314 - Device Power-down Status Register"]
    #[inline(always)]
    pub const fn etmpdsr(&self) -> &ETMPDSR {
        &self.etmpdsr
    }
    #[doc = "0xee0 - Integration Test Miscellaneous Inputs Register"]
    #[inline(always)]
    pub const fn etmiscin(&self) -> &ETMISCIN {
        &self.etmiscin
    }
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    #[inline(always)]
    pub const fn ittrigout(&self) -> &ITTRIGOUT {
        &self.ittrigout
    }
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    #[inline(always)]
    pub const fn etmitatbctr2(&self) -> &ETMITATBCTR2 {
        &self.etmitatbctr2
    }
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    #[inline(always)]
    pub const fn etmitatbctr0(&self) -> &ETMITATBCTR0 {
        &self.etmitatbctr0
    }
    #[doc = "0xf00 - ETM Integration Control Register"]
    #[inline(always)]
    pub const fn etmitctrl(&self) -> &ETMITCTRL {
        &self.etmitctrl
    }
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    #[inline(always)]
    pub const fn etmclaimset(&self) -> &ETMCLAIMSET {
        &self.etmclaimset
    }
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn etmclaimclr(&self) -> &ETMCLAIMCLR {
        &self.etmclaimclr
    }
    #[doc = "0xfb0 - ETM Lock Access Register"]
    #[inline(always)]
    pub const fn etmlar(&self) -> &ETMLAR {
        &self.etmlar
    }
    #[doc = "0xfb4 - Lock Status Register"]
    #[inline(always)]
    pub const fn etmlsr(&self) -> &ETMLSR {
        &self.etmlsr
    }
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    #[inline(always)]
    pub const fn etmauthstatus(&self) -> &ETMAUTHSTATUS {
        &self.etmauthstatus
    }
    #[doc = "0xfcc - CoreSight Device Type Register"]
    #[inline(always)]
    pub const fn etmdevtype(&self) -> &ETMDEVTYPE {
        &self.etmdevtype
    }
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    #[inline(always)]
    pub const fn etmpidr4(&self) -> &ETMPIDR4 {
        &self.etmpidr4
    }
    #[doc = "0xfd4 - Peripheral ID5 Register"]
    #[inline(always)]
    pub const fn etmpidr5(&self) -> &ETMPIDR5 {
        &self.etmpidr5
    }
    #[doc = "0xfd8 - Peripheral ID6 Register"]
    #[inline(always)]
    pub const fn etmpidr6(&self) -> &ETMPIDR6 {
        &self.etmpidr6
    }
    #[doc = "0xfdc - Peripheral ID7 Register"]
    #[inline(always)]
    pub const fn etmpidr7(&self) -> &ETMPIDR7 {
        &self.etmpidr7
    }
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    #[inline(always)]
    pub const fn etmpidr0(&self) -> &ETMPIDR0 {
        &self.etmpidr0
    }
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    #[inline(always)]
    pub const fn etmpidr1(&self) -> &ETMPIDR1 {
        &self.etmpidr1
    }
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    #[inline(always)]
    pub const fn etmpidr2(&self) -> &ETMPIDR2 {
        &self.etmpidr2
    }
    #[doc = "0xfec - Peripheral ID3 Register"]
    #[inline(always)]
    pub const fn etmpidr3(&self) -> &ETMPIDR3 {
        &self.etmpidr3
    }
    #[doc = "0xff0 - Component ID0 Register"]
    #[inline(always)]
    pub const fn etmcidr0(&self) -> &ETMCIDR0 {
        &self.etmcidr0
    }
    #[doc = "0xff4 - Component ID1 Register"]
    #[inline(always)]
    pub const fn etmcidr1(&self) -> &ETMCIDR1 {
        &self.etmcidr1
    }
    #[doc = "0xff8 - Component ID2 Register"]
    #[inline(always)]
    pub const fn etmcidr2(&self) -> &ETMCIDR2 {
        &self.etmcidr2
    }
    #[doc = "0xffc - Component ID3 Register"]
    #[inline(always)]
    pub const fn etmcidr3(&self) -> &ETMCIDR3 {
        &self.etmcidr3
    }
}
#[doc = "ETMCR (rw) register accessor: Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcr`]
module"]
pub type ETMCR = crate::Reg<etmcr::ETMCR_SPEC>;
#[doc = "Main Control Register"]
pub mod etmcr;
#[doc = "ETMCCR (r) register accessor: Configuration Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmccr`]
module"]
pub type ETMCCR = crate::Reg<etmccr::ETMCCR_SPEC>;
#[doc = "Configuration Code Register"]
pub mod etmccr;
#[doc = "ETMTRIGGER (rw) register accessor: ETM Trigger Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtrigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtrigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtrigger`]
module"]
pub type ETMTRIGGER = crate::Reg<etmtrigger::ETMTRIGGER_SPEC>;
#[doc = "ETM Trigger Event Register"]
pub mod etmtrigger;
#[doc = "ETMSR (rw) register accessor: ETM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmsr`]
module"]
pub type ETMSR = crate::Reg<etmsr::ETMSR_SPEC>;
#[doc = "ETM Status Register"]
pub mod etmsr;
#[doc = "ETMSCR (r) register accessor: ETM System Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmscr`]
module"]
pub type ETMSCR = crate::Reg<etmscr::ETMSCR_SPEC>;
#[doc = "ETM System Configuration Register"]
pub mod etmscr;
#[doc = "ETMTEEVR (rw) register accessor: ETM TraceEnable Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmteevr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmteevr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmteevr`]
module"]
pub type ETMTEEVR = crate::Reg<etmteevr::ETMTEEVR_SPEC>;
#[doc = "ETM TraceEnable Event Register"]
pub mod etmteevr;
#[doc = "ETMTECR1 (rw) register accessor: ETM Trace control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtecr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtecr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtecr1`]
module"]
pub type ETMTECR1 = crate::Reg<etmtecr1::ETMTECR1_SPEC>;
#[doc = "ETM Trace control Register"]
pub mod etmtecr1;
#[doc = "ETMFFLR (rw) register accessor: ETM Fifo Full Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmfflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmfflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmfflr`]
module"]
pub type ETMFFLR = crate::Reg<etmfflr::ETMFFLR_SPEC>;
#[doc = "ETM Fifo Full Level Register"]
pub mod etmfflr;
#[doc = "ETMCNTRLDVR1 (rw) register accessor: Counter Reload Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcntrldvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmcntrldvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcntrldvr1`]
module"]
pub type ETMCNTRLDVR1 = crate::Reg<etmcntrldvr1::ETMCNTRLDVR1_SPEC>;
#[doc = "Counter Reload Value"]
pub mod etmcntrldvr1;
#[doc = "ETMSYNCFR (rw) register accessor: Synchronisation Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmsyncfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmsyncfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmsyncfr`]
module"]
pub type ETMSYNCFR = crate::Reg<etmsyncfr::ETMSYNCFR_SPEC>;
#[doc = "Synchronisation Frequency Register"]
pub mod etmsyncfr;
#[doc = "ETMIDR (r) register accessor: ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmidr`]
module"]
pub type ETMIDR = crate::Reg<etmidr::ETMIDR_SPEC>;
#[doc = "ID Register"]
pub mod etmidr;
#[doc = "ETMCCER (r) register accessor: Configuration Code Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmccer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmccer`]
module"]
pub type ETMCCER = crate::Reg<etmccer::ETMCCER_SPEC>;
#[doc = "Configuration Code Extension Register"]
pub mod etmccer;
#[doc = "ETMTESSEICR (rw) register accessor: TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtesseicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtesseicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtesseicr`]
module"]
pub type ETMTESSEICR = crate::Reg<etmtesseicr::ETMTESSEICR_SPEC>;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod etmtesseicr;
#[doc = "ETMTSEVR (rw) register accessor: Timestamp Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtsevr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtsevr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtsevr`]
module"]
pub type ETMTSEVR = crate::Reg<etmtsevr::ETMTSEVR_SPEC>;
#[doc = "Timestamp Event Register"]
pub mod etmtsevr;
#[doc = "ETMTRACEIDR (rw) register accessor: CoreSight Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtraceidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtraceidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtraceidr`]
module"]
pub type ETMTRACEIDR = crate::Reg<etmtraceidr::ETMTRACEIDR_SPEC>;
#[doc = "CoreSight Trace ID Register"]
pub mod etmtraceidr;
#[doc = "ETMIDR2 (r) register accessor: ETM ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmidr2`]
module"]
pub type ETMIDR2 = crate::Reg<etmidr2::ETMIDR2_SPEC>;
#[doc = "ETM ID Register 2"]
pub mod etmidr2;
#[doc = "ETMPDSR (r) register accessor: Device Power-down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpdsr`]
module"]
pub type ETMPDSR = crate::Reg<etmpdsr::ETMPDSR_SPEC>;
#[doc = "Device Power-down Status Register"]
pub mod etmpdsr;
#[doc = "ETMISCIN (rw) register accessor: Integration Test Miscellaneous Inputs Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmiscin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmiscin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmiscin`]
module"]
pub type ETMISCIN = crate::Reg<etmiscin::ETMISCIN_SPEC>;
#[doc = "Integration Test Miscellaneous Inputs Register"]
pub mod etmiscin;
#[doc = "ITTRIGOUT (rw) register accessor: Integration Test Trigger Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ittrigout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ittrigout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrigout`]
module"]
pub type ITTRIGOUT = crate::Reg<ittrigout::ITTRIGOUT_SPEC>;
#[doc = "Integration Test Trigger Out Register"]
pub mod ittrigout;
#[doc = "ETMITATBCTR2 (r) register accessor: ETM Integration Test ATB Control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmitatbctr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmitatbctr2`]
module"]
pub type ETMITATBCTR2 = crate::Reg<etmitatbctr2::ETMITATBCTR2_SPEC>;
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod etmitatbctr2;
#[doc = "ETMITATBCTR0 (rw) register accessor: ETM Integration Test ATB Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmitatbctr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmitatbctr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmitatbctr0`]
module"]
pub type ETMITATBCTR0 = crate::Reg<etmitatbctr0::ETMITATBCTR0_SPEC>;
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod etmitatbctr0;
#[doc = "ETMITCTRL (rw) register accessor: ETM Integration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmitctrl`]
module"]
pub type ETMITCTRL = crate::Reg<etmitctrl::ETMITCTRL_SPEC>;
#[doc = "ETM Integration Control Register"]
pub mod etmitctrl;
#[doc = "ETMCLAIMSET (rw) register accessor: ETM Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmclaimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmclaimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmclaimset`]
module"]
pub type ETMCLAIMSET = crate::Reg<etmclaimset::ETMCLAIMSET_SPEC>;
#[doc = "ETM Claim Tag Set Register"]
pub mod etmclaimset;
#[doc = "ETMCLAIMCLR (rw) register accessor: ETM Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmclaimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmclaimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmclaimclr`]
module"]
pub type ETMCLAIMCLR = crate::Reg<etmclaimclr::ETMCLAIMCLR_SPEC>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod etmclaimclr;
#[doc = "ETMLAR (rw) register accessor: ETM Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmlar`]
module"]
pub type ETMLAR = crate::Reg<etmlar::ETMLAR_SPEC>;
#[doc = "ETM Lock Access Register"]
pub mod etmlar;
#[doc = "ETMLSR (r) register accessor: Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmlsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmlsr`]
module"]
pub type ETMLSR = crate::Reg<etmlsr::ETMLSR_SPEC>;
#[doc = "Lock Status Register"]
pub mod etmlsr;
#[doc = "ETMAUTHSTATUS (r) register accessor: ETM Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmauthstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmauthstatus`]
module"]
pub type ETMAUTHSTATUS = crate::Reg<etmauthstatus::ETMAUTHSTATUS_SPEC>;
#[doc = "ETM Authentication Status Register"]
pub mod etmauthstatus;
#[doc = "ETMDEVTYPE (r) register accessor: CoreSight Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmdevtype::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmdevtype`]
module"]
pub type ETMDEVTYPE = crate::Reg<etmdevtype::ETMDEVTYPE_SPEC>;
#[doc = "CoreSight Device Type Register"]
pub mod etmdevtype;
#[doc = "ETMPIDR4 (r) register accessor: Peripheral ID4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr4`]
module"]
pub type ETMPIDR4 = crate::Reg<etmpidr4::ETMPIDR4_SPEC>;
#[doc = "Peripheral ID4 Register"]
pub mod etmpidr4;
#[doc = "ETMPIDR5 (w) register accessor: Peripheral ID5 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmpidr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr5`]
module"]
pub type ETMPIDR5 = crate::Reg<etmpidr5::ETMPIDR5_SPEC>;
#[doc = "Peripheral ID5 Register"]
pub mod etmpidr5;
#[doc = "ETMPIDR6 (w) register accessor: Peripheral ID6 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmpidr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr6`]
module"]
pub type ETMPIDR6 = crate::Reg<etmpidr6::ETMPIDR6_SPEC>;
#[doc = "Peripheral ID6 Register"]
pub mod etmpidr6;
#[doc = "ETMPIDR7 (w) register accessor: Peripheral ID7 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmpidr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr7`]
module"]
pub type ETMPIDR7 = crate::Reg<etmpidr7::ETMPIDR7_SPEC>;
#[doc = "Peripheral ID7 Register"]
pub mod etmpidr7;
#[doc = "ETMPIDR0 (r) register accessor: Peripheral ID0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr0`]
module"]
pub type ETMPIDR0 = crate::Reg<etmpidr0::ETMPIDR0_SPEC>;
#[doc = "Peripheral ID0 Register"]
pub mod etmpidr0;
#[doc = "ETMPIDR1 (r) register accessor: Peripheral ID1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr1`]
module"]
pub type ETMPIDR1 = crate::Reg<etmpidr1::ETMPIDR1_SPEC>;
#[doc = "Peripheral ID1 Register"]
pub mod etmpidr1;
#[doc = "ETMPIDR2 (r) register accessor: Peripheral ID2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr2`]
module"]
pub type ETMPIDR2 = crate::Reg<etmpidr2::ETMPIDR2_SPEC>;
#[doc = "Peripheral ID2 Register"]
pub mod etmpidr2;
#[doc = "ETMPIDR3 (r) register accessor: Peripheral ID3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr3`]
module"]
pub type ETMPIDR3 = crate::Reg<etmpidr3::ETMPIDR3_SPEC>;
#[doc = "Peripheral ID3 Register"]
pub mod etmpidr3;
#[doc = "ETMCIDR0 (r) register accessor: Component ID0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr0`]
module"]
pub type ETMCIDR0 = crate::Reg<etmcidr0::ETMCIDR0_SPEC>;
#[doc = "Component ID0 Register"]
pub mod etmcidr0;
#[doc = "ETMCIDR1 (r) register accessor: Component ID1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr1`]
module"]
pub type ETMCIDR1 = crate::Reg<etmcidr1::ETMCIDR1_SPEC>;
#[doc = "Component ID1 Register"]
pub mod etmcidr1;
#[doc = "ETMCIDR2 (r) register accessor: Component ID2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr2`]
module"]
pub type ETMCIDR2 = crate::Reg<etmcidr2::ETMCIDR2_SPEC>;
#[doc = "Component ID2 Register"]
pub mod etmcidr2;
#[doc = "ETMCIDR3 (r) register accessor: Component ID3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr3`]
module"]
pub type ETMCIDR3 = crate::Reg<etmcidr3::ETMCIDR3_SPEC>;
#[doc = "Component ID3 Register"]
pub mod etmcidr3;
