#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    etmcr: Etmcr,
    etmccr: Etmccr,
    etmtrigger: Etmtrigger,
    _reserved3: [u8; 0x04],
    etmsr: Etmsr,
    etmscr: Etmscr,
    _reserved5: [u8; 0x08],
    etmteevr: Etmteevr,
    etmtecr1: Etmtecr1,
    _reserved7: [u8; 0x04],
    etmfflr: Etmfflr,
    _reserved8: [u8; 0x0110],
    etmcntrldvr1: Etmcntrldvr1,
    _reserved9: [u8; 0x9c],
    etmsyncfr: Etmsyncfr,
    etmidr: Etmidr,
    etmccer: Etmccer,
    _reserved12: [u8; 0x04],
    etmtesseicr: Etmtesseicr,
    _reserved13: [u8; 0x04],
    etmtsevr: Etmtsevr,
    _reserved14: [u8; 0x04],
    etmtraceidr: Etmtraceidr,
    _reserved15: [u8; 0x04],
    etmidr2: Etmidr2,
    _reserved16: [u8; 0x0108],
    etmpdsr: Etmpdsr,
    _reserved17: [u8; 0x0bc8],
    etmiscin: Etmiscin,
    _reserved18: [u8; 0x04],
    ittrigout: Ittrigout,
    _reserved19: [u8; 0x04],
    etmitatbctr2: Etmitatbctr2,
    _reserved20: [u8; 0x04],
    etmitatbctr0: Etmitatbctr0,
    _reserved21: [u8; 0x04],
    etmitctrl: Etmitctrl,
    _reserved22: [u8; 0x9c],
    etmclaimset: Etmclaimset,
    etmclaimclr: Etmclaimclr,
    _reserved24: [u8; 0x08],
    etmlar: Etmlar,
    etmlsr: Etmlsr,
    etmauthstatus: Etmauthstatus,
    _reserved27: [u8; 0x10],
    etmdevtype: Etmdevtype,
    etmpidr4: Etmpidr4,
    etmpidr5: Etmpidr5,
    etmpidr6: Etmpidr6,
    etmpidr7: Etmpidr7,
    etmpidr0: Etmpidr0,
    etmpidr1: Etmpidr1,
    etmpidr2: Etmpidr2,
    etmpidr3: Etmpidr3,
    etmcidr0: Etmcidr0,
    etmcidr1: Etmcidr1,
    etmcidr2: Etmcidr2,
    etmcidr3: Etmcidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    #[inline(always)]
    pub const fn etmcr(&self) -> &Etmcr {
        &self.etmcr
    }
    #[doc = "0x04 - Configuration Code Register"]
    #[inline(always)]
    pub const fn etmccr(&self) -> &Etmccr {
        &self.etmccr
    }
    #[doc = "0x08 - ETM Trigger Event Register"]
    #[inline(always)]
    pub const fn etmtrigger(&self) -> &Etmtrigger {
        &self.etmtrigger
    }
    #[doc = "0x10 - ETM Status Register"]
    #[inline(always)]
    pub const fn etmsr(&self) -> &Etmsr {
        &self.etmsr
    }
    #[doc = "0x14 - ETM System Configuration Register"]
    #[inline(always)]
    pub const fn etmscr(&self) -> &Etmscr {
        &self.etmscr
    }
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    #[inline(always)]
    pub const fn etmteevr(&self) -> &Etmteevr {
        &self.etmteevr
    }
    #[doc = "0x24 - ETM Trace control Register"]
    #[inline(always)]
    pub const fn etmtecr1(&self) -> &Etmtecr1 {
        &self.etmtecr1
    }
    #[doc = "0x2c - ETM Fifo Full Level Register"]
    #[inline(always)]
    pub const fn etmfflr(&self) -> &Etmfflr {
        &self.etmfflr
    }
    #[doc = "0x140 - Counter Reload Value"]
    #[inline(always)]
    pub const fn etmcntrldvr1(&self) -> &Etmcntrldvr1 {
        &self.etmcntrldvr1
    }
    #[doc = "0x1e0 - Synchronisation Frequency Register"]
    #[inline(always)]
    pub const fn etmsyncfr(&self) -> &Etmsyncfr {
        &self.etmsyncfr
    }
    #[doc = "0x1e4 - ID Register"]
    #[inline(always)]
    pub const fn etmidr(&self) -> &Etmidr {
        &self.etmidr
    }
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    #[inline(always)]
    pub const fn etmccer(&self) -> &Etmccer {
        &self.etmccer
    }
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    #[inline(always)]
    pub const fn etmtesseicr(&self) -> &Etmtesseicr {
        &self.etmtesseicr
    }
    #[doc = "0x1f8 - Timestamp Event Register"]
    #[inline(always)]
    pub const fn etmtsevr(&self) -> &Etmtsevr {
        &self.etmtsevr
    }
    #[doc = "0x200 - CoreSight Trace ID Register"]
    #[inline(always)]
    pub const fn etmtraceidr(&self) -> &Etmtraceidr {
        &self.etmtraceidr
    }
    #[doc = "0x208 - ETM ID Register 2"]
    #[inline(always)]
    pub const fn etmidr2(&self) -> &Etmidr2 {
        &self.etmidr2
    }
    #[doc = "0x314 - Device Power-down Status Register"]
    #[inline(always)]
    pub const fn etmpdsr(&self) -> &Etmpdsr {
        &self.etmpdsr
    }
    #[doc = "0xee0 - Integration Test Miscellaneous Inputs Register"]
    #[inline(always)]
    pub const fn etmiscin(&self) -> &Etmiscin {
        &self.etmiscin
    }
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    #[inline(always)]
    pub const fn ittrigout(&self) -> &Ittrigout {
        &self.ittrigout
    }
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    #[inline(always)]
    pub const fn etmitatbctr2(&self) -> &Etmitatbctr2 {
        &self.etmitatbctr2
    }
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    #[inline(always)]
    pub const fn etmitatbctr0(&self) -> &Etmitatbctr0 {
        &self.etmitatbctr0
    }
    #[doc = "0xf00 - ETM Integration Control Register"]
    #[inline(always)]
    pub const fn etmitctrl(&self) -> &Etmitctrl {
        &self.etmitctrl
    }
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    #[inline(always)]
    pub const fn etmclaimset(&self) -> &Etmclaimset {
        &self.etmclaimset
    }
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn etmclaimclr(&self) -> &Etmclaimclr {
        &self.etmclaimclr
    }
    #[doc = "0xfb0 - ETM Lock Access Register"]
    #[inline(always)]
    pub const fn etmlar(&self) -> &Etmlar {
        &self.etmlar
    }
    #[doc = "0xfb4 - Lock Status Register"]
    #[inline(always)]
    pub const fn etmlsr(&self) -> &Etmlsr {
        &self.etmlsr
    }
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    #[inline(always)]
    pub const fn etmauthstatus(&self) -> &Etmauthstatus {
        &self.etmauthstatus
    }
    #[doc = "0xfcc - CoreSight Device Type Register"]
    #[inline(always)]
    pub const fn etmdevtype(&self) -> &Etmdevtype {
        &self.etmdevtype
    }
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    #[inline(always)]
    pub const fn etmpidr4(&self) -> &Etmpidr4 {
        &self.etmpidr4
    }
    #[doc = "0xfd4 - Peripheral ID5 Register"]
    #[inline(always)]
    pub const fn etmpidr5(&self) -> &Etmpidr5 {
        &self.etmpidr5
    }
    #[doc = "0xfd8 - Peripheral ID6 Register"]
    #[inline(always)]
    pub const fn etmpidr6(&self) -> &Etmpidr6 {
        &self.etmpidr6
    }
    #[doc = "0xfdc - Peripheral ID7 Register"]
    #[inline(always)]
    pub const fn etmpidr7(&self) -> &Etmpidr7 {
        &self.etmpidr7
    }
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    #[inline(always)]
    pub const fn etmpidr0(&self) -> &Etmpidr0 {
        &self.etmpidr0
    }
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    #[inline(always)]
    pub const fn etmpidr1(&self) -> &Etmpidr1 {
        &self.etmpidr1
    }
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    #[inline(always)]
    pub const fn etmpidr2(&self) -> &Etmpidr2 {
        &self.etmpidr2
    }
    #[doc = "0xfec - Peripheral ID3 Register"]
    #[inline(always)]
    pub const fn etmpidr3(&self) -> &Etmpidr3 {
        &self.etmpidr3
    }
    #[doc = "0xff0 - Component ID0 Register"]
    #[inline(always)]
    pub const fn etmcidr0(&self) -> &Etmcidr0 {
        &self.etmcidr0
    }
    #[doc = "0xff4 - Component ID1 Register"]
    #[inline(always)]
    pub const fn etmcidr1(&self) -> &Etmcidr1 {
        &self.etmcidr1
    }
    #[doc = "0xff8 - Component ID2 Register"]
    #[inline(always)]
    pub const fn etmcidr2(&self) -> &Etmcidr2 {
        &self.etmcidr2
    }
    #[doc = "0xffc - Component ID3 Register"]
    #[inline(always)]
    pub const fn etmcidr3(&self) -> &Etmcidr3 {
        &self.etmcidr3
    }
}
#[doc = "ETMCR (rw) register accessor: Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcr`] module"]
#[doc(alias = "ETMCR")]
pub type Etmcr = crate::Reg<etmcr::EtmcrSpec>;
#[doc = "Main Control Register"]
pub mod etmcr;
#[doc = "ETMCCR (r) register accessor: Configuration Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmccr`] module"]
#[doc(alias = "ETMCCR")]
pub type Etmccr = crate::Reg<etmccr::EtmccrSpec>;
#[doc = "Configuration Code Register"]
pub mod etmccr;
#[doc = "ETMTRIGGER (rw) register accessor: ETM Trigger Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtrigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtrigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtrigger`] module"]
#[doc(alias = "ETMTRIGGER")]
pub type Etmtrigger = crate::Reg<etmtrigger::EtmtriggerSpec>;
#[doc = "ETM Trigger Event Register"]
pub mod etmtrigger;
#[doc = "ETMSR (rw) register accessor: ETM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmsr`] module"]
#[doc(alias = "ETMSR")]
pub type Etmsr = crate::Reg<etmsr::EtmsrSpec>;
#[doc = "ETM Status Register"]
pub mod etmsr;
#[doc = "ETMSCR (r) register accessor: ETM System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmscr`] module"]
#[doc(alias = "ETMSCR")]
pub type Etmscr = crate::Reg<etmscr::EtmscrSpec>;
#[doc = "ETM System Configuration Register"]
pub mod etmscr;
#[doc = "ETMTEEVR (rw) register accessor: ETM TraceEnable Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmteevr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmteevr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmteevr`] module"]
#[doc(alias = "ETMTEEVR")]
pub type Etmteevr = crate::Reg<etmteevr::EtmteevrSpec>;
#[doc = "ETM TraceEnable Event Register"]
pub mod etmteevr;
#[doc = "ETMTECR1 (rw) register accessor: ETM Trace control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtecr1`] module"]
#[doc(alias = "ETMTECR1")]
pub type Etmtecr1 = crate::Reg<etmtecr1::Etmtecr1Spec>;
#[doc = "ETM Trace control Register"]
pub mod etmtecr1;
#[doc = "ETMFFLR (rw) register accessor: ETM Fifo Full Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmfflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmfflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmfflr`] module"]
#[doc(alias = "ETMFFLR")]
pub type Etmfflr = crate::Reg<etmfflr::EtmfflrSpec>;
#[doc = "ETM Fifo Full Level Register"]
pub mod etmfflr;
#[doc = "ETMCNTRLDVR1 (rw) register accessor: Counter Reload Value\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcntrldvr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmcntrldvr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcntrldvr1`] module"]
#[doc(alias = "ETMCNTRLDVR1")]
pub type Etmcntrldvr1 = crate::Reg<etmcntrldvr1::Etmcntrldvr1Spec>;
#[doc = "Counter Reload Value"]
pub mod etmcntrldvr1;
#[doc = "ETMSYNCFR (rw) register accessor: Synchronisation Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmsyncfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmsyncfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmsyncfr`] module"]
#[doc(alias = "ETMSYNCFR")]
pub type Etmsyncfr = crate::Reg<etmsyncfr::EtmsyncfrSpec>;
#[doc = "Synchronisation Frequency Register"]
pub mod etmsyncfr;
#[doc = "ETMIDR (r) register accessor: ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmidr`] module"]
#[doc(alias = "ETMIDR")]
pub type Etmidr = crate::Reg<etmidr::EtmidrSpec>;
#[doc = "ID Register"]
pub mod etmidr;
#[doc = "ETMCCER (r) register accessor: Configuration Code Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmccer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmccer`] module"]
#[doc(alias = "ETMCCER")]
pub type Etmccer = crate::Reg<etmccer::EtmccerSpec>;
#[doc = "Configuration Code Extension Register"]
pub mod etmccer;
#[doc = "ETMTESSEICR (rw) register accessor: TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtesseicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtesseicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtesseicr`] module"]
#[doc(alias = "ETMTESSEICR")]
pub type Etmtesseicr = crate::Reg<etmtesseicr::EtmtesseicrSpec>;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod etmtesseicr;
#[doc = "ETMTSEVR (rw) register accessor: Timestamp Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtsevr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtsevr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtsevr`] module"]
#[doc(alias = "ETMTSEVR")]
pub type Etmtsevr = crate::Reg<etmtsevr::EtmtsevrSpec>;
#[doc = "Timestamp Event Register"]
pub mod etmtsevr;
#[doc = "ETMTRACEIDR (rw) register accessor: CoreSight Trace ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmtraceidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmtraceidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmtraceidr`] module"]
#[doc(alias = "ETMTRACEIDR")]
pub type Etmtraceidr = crate::Reg<etmtraceidr::EtmtraceidrSpec>;
#[doc = "CoreSight Trace ID Register"]
pub mod etmtraceidr;
#[doc = "ETMIDR2 (r) register accessor: ETM ID Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`etmidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmidr2`] module"]
#[doc(alias = "ETMIDR2")]
pub type Etmidr2 = crate::Reg<etmidr2::Etmidr2Spec>;
#[doc = "ETM ID Register 2"]
pub mod etmidr2;
#[doc = "ETMPDSR (r) register accessor: Device Power-down Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpdsr`] module"]
#[doc(alias = "ETMPDSR")]
pub type Etmpdsr = crate::Reg<etmpdsr::EtmpdsrSpec>;
#[doc = "Device Power-down Status Register"]
pub mod etmpdsr;
#[doc = "ETMISCIN (rw) register accessor: Integration Test Miscellaneous Inputs Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmiscin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmiscin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmiscin`] module"]
#[doc(alias = "ETMISCIN")]
pub type Etmiscin = crate::Reg<etmiscin::EtmiscinSpec>;
#[doc = "Integration Test Miscellaneous Inputs Register"]
pub mod etmiscin;
#[doc = "ITTRIGOUT (rw) register accessor: Integration Test Trigger Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrigout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrigout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrigout`] module"]
#[doc(alias = "ITTRIGOUT")]
pub type Ittrigout = crate::Reg<ittrigout::IttrigoutSpec>;
#[doc = "Integration Test Trigger Out Register"]
pub mod ittrigout;
#[doc = "ETMITATBCTR2 (r) register accessor: ETM Integration Test ATB Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmitatbctr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmitatbctr2`] module"]
#[doc(alias = "ETMITATBCTR2")]
pub type Etmitatbctr2 = crate::Reg<etmitatbctr2::Etmitatbctr2Spec>;
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod etmitatbctr2;
#[doc = "ETMITATBCTR0 (rw) register accessor: ETM Integration Test ATB Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmitatbctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmitatbctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmitatbctr0`] module"]
#[doc(alias = "ETMITATBCTR0")]
pub type Etmitatbctr0 = crate::Reg<etmitatbctr0::Etmitatbctr0Spec>;
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod etmitatbctr0;
#[doc = "ETMITCTRL (rw) register accessor: ETM Integration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmitctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmitctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmitctrl`] module"]
#[doc(alias = "ETMITCTRL")]
pub type Etmitctrl = crate::Reg<etmitctrl::EtmitctrlSpec>;
#[doc = "ETM Integration Control Register"]
pub mod etmitctrl;
#[doc = "ETMCLAIMSET (rw) register accessor: ETM Claim Tag Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmclaimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmclaimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmclaimset`] module"]
#[doc(alias = "ETMCLAIMSET")]
pub type Etmclaimset = crate::Reg<etmclaimset::EtmclaimsetSpec>;
#[doc = "ETM Claim Tag Set Register"]
pub mod etmclaimset;
#[doc = "ETMCLAIMCLR (rw) register accessor: ETM Claim Tag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmclaimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmclaimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmclaimclr`] module"]
#[doc(alias = "ETMCLAIMCLR")]
pub type Etmclaimclr = crate::Reg<etmclaimclr::EtmclaimclrSpec>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod etmclaimclr;
#[doc = "ETMLAR (rw) register accessor: ETM Lock Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmlar`] module"]
#[doc(alias = "ETMLAR")]
pub type Etmlar = crate::Reg<etmlar::EtmlarSpec>;
#[doc = "ETM Lock Access Register"]
pub mod etmlar;
#[doc = "ETMLSR (r) register accessor: Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmlsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmlsr`] module"]
#[doc(alias = "ETMLSR")]
pub type Etmlsr = crate::Reg<etmlsr::EtmlsrSpec>;
#[doc = "Lock Status Register"]
pub mod etmlsr;
#[doc = "ETMAUTHSTATUS (r) register accessor: ETM Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmauthstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmauthstatus`] module"]
#[doc(alias = "ETMAUTHSTATUS")]
pub type Etmauthstatus = crate::Reg<etmauthstatus::EtmauthstatusSpec>;
#[doc = "ETM Authentication Status Register"]
pub mod etmauthstatus;
#[doc = "ETMDEVTYPE (r) register accessor: CoreSight Device Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmdevtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmdevtype`] module"]
#[doc(alias = "ETMDEVTYPE")]
pub type Etmdevtype = crate::Reg<etmdevtype::EtmdevtypeSpec>;
#[doc = "CoreSight Device Type Register"]
pub mod etmdevtype;
#[doc = "ETMPIDR4 (r) register accessor: Peripheral ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr4`] module"]
#[doc(alias = "ETMPIDR4")]
pub type Etmpidr4 = crate::Reg<etmpidr4::Etmpidr4Spec>;
#[doc = "Peripheral ID4 Register"]
pub mod etmpidr4;
#[doc = "ETMPIDR5 (w) register accessor: Peripheral ID5 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmpidr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr5`] module"]
#[doc(alias = "ETMPIDR5")]
pub type Etmpidr5 = crate::Reg<etmpidr5::Etmpidr5Spec>;
#[doc = "Peripheral ID5 Register"]
pub mod etmpidr5;
#[doc = "ETMPIDR6 (w) register accessor: Peripheral ID6 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmpidr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr6`] module"]
#[doc(alias = "ETMPIDR6")]
pub type Etmpidr6 = crate::Reg<etmpidr6::Etmpidr6Spec>;
#[doc = "Peripheral ID6 Register"]
pub mod etmpidr6;
#[doc = "ETMPIDR7 (w) register accessor: Peripheral ID7 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmpidr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr7`] module"]
#[doc(alias = "ETMPIDR7")]
pub type Etmpidr7 = crate::Reg<etmpidr7::Etmpidr7Spec>;
#[doc = "Peripheral ID7 Register"]
pub mod etmpidr7;
#[doc = "ETMPIDR0 (r) register accessor: Peripheral ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr0`] module"]
#[doc(alias = "ETMPIDR0")]
pub type Etmpidr0 = crate::Reg<etmpidr0::Etmpidr0Spec>;
#[doc = "Peripheral ID0 Register"]
pub mod etmpidr0;
#[doc = "ETMPIDR1 (r) register accessor: Peripheral ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr1`] module"]
#[doc(alias = "ETMPIDR1")]
pub type Etmpidr1 = crate::Reg<etmpidr1::Etmpidr1Spec>;
#[doc = "Peripheral ID1 Register"]
pub mod etmpidr1;
#[doc = "ETMPIDR2 (r) register accessor: Peripheral ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr2`] module"]
#[doc(alias = "ETMPIDR2")]
pub type Etmpidr2 = crate::Reg<etmpidr2::Etmpidr2Spec>;
#[doc = "Peripheral ID2 Register"]
pub mod etmpidr2;
#[doc = "ETMPIDR3 (r) register accessor: Peripheral ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmpidr3`] module"]
#[doc(alias = "ETMPIDR3")]
pub type Etmpidr3 = crate::Reg<etmpidr3::Etmpidr3Spec>;
#[doc = "Peripheral ID3 Register"]
pub mod etmpidr3;
#[doc = "ETMCIDR0 (r) register accessor: Component ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr0`] module"]
#[doc(alias = "ETMCIDR0")]
pub type Etmcidr0 = crate::Reg<etmcidr0::Etmcidr0Spec>;
#[doc = "Component ID0 Register"]
pub mod etmcidr0;
#[doc = "ETMCIDR1 (r) register accessor: Component ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr1`] module"]
#[doc(alias = "ETMCIDR1")]
pub type Etmcidr1 = crate::Reg<etmcidr1::Etmcidr1Spec>;
#[doc = "Component ID1 Register"]
pub mod etmcidr1;
#[doc = "ETMCIDR2 (r) register accessor: Component ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr2`] module"]
#[doc(alias = "ETMCIDR2")]
pub type Etmcidr2 = crate::Reg<etmcidr2::Etmcidr2Spec>;
#[doc = "Component ID2 Register"]
pub mod etmcidr2;
#[doc = "ETMCIDR3 (r) register accessor: Component ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etmcidr3`] module"]
#[doc(alias = "ETMCIDR3")]
pub type Etmcidr3 = crate::Reg<etmcidr3::Etmcidr3Spec>;
#[doc = "Component ID3 Register"]
pub mod etmcidr3;
