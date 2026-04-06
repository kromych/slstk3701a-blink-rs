#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    errcnt: Errcnt,
    bittiming: Bittiming,
    intid: Intid,
    test: Test,
    brpe: Brpe,
    transreq: Transreq,
    messagedata: Messagedata,
    _reserved9: [u8; 0x04],
    messagestate: Messagestate,
    config: Config,
    if0if: If0if,
    if0ifs: If0ifs,
    if0ifc: If0ifc,
    if0ien: If0ien,
    if1if: If1if,
    if1ifs: If1ifs,
    if1ifc: If1ifc,
    if1ien: If1ien,
    route: Route,
    _reserved20: [u8; 0x0c],
    mir0_cmdmask: Mir0Cmdmask,
    mir0_mask: Mir0Mask,
    mir0_arb: Mir0Arb,
    mir0_ctrl: Mir0Ctrl,
    mir0_datal: Mir0Datal,
    mir0_datah: Mir0Datah,
    mir0_cmdreq: Mir0Cmdreq,
    _reserved27: [u8; 0x04],
    mir1_cmdmask: Mir1Cmdmask,
    mir1_mask: Mir1Mask,
    mir1_arb: Mir1Arb,
    mir1_ctrl: Mir1Ctrl,
    mir1_datal: Mir1Datal,
    mir1_datah: Mir1Datah,
    mir1_cmdreq: Mir1Cmdreq,
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
    #[doc = "0x08 - Error Count Register"]
    #[inline(always)]
    pub const fn errcnt(&self) -> &Errcnt {
        &self.errcnt
    }
    #[doc = "0x0c - Bit Timing Register"]
    #[inline(always)]
    pub const fn bittiming(&self) -> &Bittiming {
        &self.bittiming
    }
    #[doc = "0x10 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn intid(&self) -> &Intid {
        &self.intid
    }
    #[doc = "0x14 - Test Register"]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x18 - BRP Extension Register"]
    #[inline(always)]
    pub const fn brpe(&self) -> &Brpe {
        &self.brpe
    }
    #[doc = "0x1c - Transmission Request Register"]
    #[inline(always)]
    pub const fn transreq(&self) -> &Transreq {
        &self.transreq
    }
    #[doc = "0x20 - New Data Register"]
    #[inline(always)]
    pub const fn messagedata(&self) -> &Messagedata {
        &self.messagedata
    }
    #[doc = "0x28 - Message Valid Register"]
    #[inline(always)]
    pub const fn messagestate(&self) -> &Messagestate {
        &self.messagestate
    }
    #[doc = "0x2c - Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x30 - Message Object Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if0if(&self) -> &If0if {
        &self.if0if
    }
    #[doc = "0x34 - Message Object Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn if0ifs(&self) -> &If0ifs {
        &self.if0ifs
    }
    #[doc = "0x38 - Message Object Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn if0ifc(&self) -> &If0ifc {
        &self.if0ifc
    }
    #[doc = "0x3c - Message Object Interrupt Enable Register"]
    #[inline(always)]
    pub const fn if0ien(&self) -> &If0ien {
        &self.if0ien
    }
    #[doc = "0x40 - Status Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if1if(&self) -> &If1if {
        &self.if1if
    }
    #[doc = "0x44 - Message Object Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn if1ifs(&self) -> &If1ifs {
        &self.if1ifs
    }
    #[doc = "0x48 - Message Object Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn if1ifc(&self) -> &If1ifc {
        &self.if1ifc
    }
    #[doc = "0x4c - Status Interrupt Enable Register"]
    #[inline(always)]
    pub const fn if1ien(&self) -> &If1ien {
        &self.if1ien
    }
    #[doc = "0x50 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &Route {
        &self.route
    }
    #[doc = "0x60 - Interface Command Mask Register"]
    #[inline(always)]
    pub const fn mir0_cmdmask(&self) -> &Mir0Cmdmask {
        &self.mir0_cmdmask
    }
    #[doc = "0x64 - Interface Mask Register"]
    #[inline(always)]
    pub const fn mir0_mask(&self) -> &Mir0Mask {
        &self.mir0_mask
    }
    #[doc = "0x68 - Interface Arbitration Register"]
    #[inline(always)]
    pub const fn mir0_arb(&self) -> &Mir0Arb {
        &self.mir0_arb
    }
    #[doc = "0x6c - Interface Message Control Register"]
    #[inline(always)]
    pub const fn mir0_ctrl(&self) -> &Mir0Ctrl {
        &self.mir0_ctrl
    }
    #[doc = "0x70 - Interface Data a Register"]
    #[inline(always)]
    pub const fn mir0_datal(&self) -> &Mir0Datal {
        &self.mir0_datal
    }
    #[doc = "0x74 - Interface Data B Register"]
    #[inline(always)]
    pub const fn mir0_datah(&self) -> &Mir0Datah {
        &self.mir0_datah
    }
    #[doc = "0x78 - Interface Command Request Register"]
    #[inline(always)]
    pub const fn mir0_cmdreq(&self) -> &Mir0Cmdreq {
        &self.mir0_cmdreq
    }
    #[doc = "0x80 - Interface Command Mask Register"]
    #[inline(always)]
    pub const fn mir1_cmdmask(&self) -> &Mir1Cmdmask {
        &self.mir1_cmdmask
    }
    #[doc = "0x84 - Interface Mask Register"]
    #[inline(always)]
    pub const fn mir1_mask(&self) -> &Mir1Mask {
        &self.mir1_mask
    }
    #[doc = "0x88 - Interface Arbitration Register"]
    #[inline(always)]
    pub const fn mir1_arb(&self) -> &Mir1Arb {
        &self.mir1_arb
    }
    #[doc = "0x8c - Interface Message Control Register"]
    #[inline(always)]
    pub const fn mir1_ctrl(&self) -> &Mir1Ctrl {
        &self.mir1_ctrl
    }
    #[doc = "0x90 - Interface Data a Register"]
    #[inline(always)]
    pub const fn mir1_datal(&self) -> &Mir1Datal {
        &self.mir1_datal
    }
    #[doc = "0x94 - Interface Data B Register"]
    #[inline(always)]
    pub const fn mir1_datah(&self) -> &Mir1Datah {
        &self.mir1_datah
    }
    #[doc = "0x98 - Interface Command Request Register"]
    #[inline(always)]
    pub const fn mir1_cmdreq(&self) -> &Mir1Cmdreq {
        &self.mir1_cmdreq
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ERRCNT (r) register accessor: Error Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcnt`] module"]
#[doc(alias = "ERRCNT")]
pub type Errcnt = crate::Reg<errcnt::ErrcntSpec>;
#[doc = "Error Count Register"]
pub mod errcnt;
#[doc = "BITTIMING (rw) register accessor: Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bittiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bittiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bittiming`] module"]
#[doc(alias = "BITTIMING")]
pub type Bittiming = crate::Reg<bittiming::BittimingSpec>;
#[doc = "Bit Timing Register"]
pub mod bittiming;
#[doc = "INTID (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intid`] module"]
#[doc(alias = "INTID")]
pub type Intid = crate::Reg<intid::IntidSpec>;
#[doc = "Interrupt Identification Register"]
pub mod intid;
#[doc = "TEST (rw) register accessor: Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Test Register"]
pub mod test;
#[doc = "BRPE (rw) register accessor: BRP Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brpe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brpe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brpe`] module"]
#[doc(alias = "BRPE")]
pub type Brpe = crate::Reg<brpe::BrpeSpec>;
#[doc = "BRP Extension Register"]
pub mod brpe;
#[doc = "TRANSREQ (r) register accessor: Transmission Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`transreq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transreq`] module"]
#[doc(alias = "TRANSREQ")]
pub type Transreq = crate::Reg<transreq::TransreqSpec>;
#[doc = "Transmission Request Register"]
pub mod transreq;
#[doc = "MESSAGEDATA (r) register accessor: New Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`messagedata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@messagedata`] module"]
#[doc(alias = "MESSAGEDATA")]
pub type Messagedata = crate::Reg<messagedata::MessagedataSpec>;
#[doc = "New Data Register"]
pub mod messagedata;
#[doc = "MESSAGESTATE (r) register accessor: Message Valid Register\n\nYou can [`read`](crate::Reg::read) this register and get [`messagestate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@messagestate`] module"]
#[doc(alias = "MESSAGESTATE")]
pub type Messagestate = crate::Reg<messagestate::MessagestateSpec>;
#[doc = "Message Valid Register"]
pub mod messagestate;
#[doc = "CONFIG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "IF0IF (r) register accessor: Message Object Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if0if::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0if`] module"]
#[doc(alias = "IF0IF")]
pub type If0if = crate::Reg<if0if::If0ifSpec>;
#[doc = "Message Object Interrupt Flag Register"]
pub mod if0if;
#[doc = "IF0IFS (w) register accessor: Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if0ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0ifs`] module"]
#[doc(alias = "IF0IFS")]
pub type If0ifs = crate::Reg<if0ifs::If0ifsSpec>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if0ifs;
#[doc = "IF0IFC (w) register accessor: Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if0ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0ifc`] module"]
#[doc(alias = "IF0IFC")]
pub type If0ifc = crate::Reg<if0ifc::If0ifcSpec>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if0ifc;
#[doc = "IF0IEN (rw) register accessor: Message Object Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if0ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if0ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0ien`] module"]
#[doc(alias = "IF0IEN")]
pub type If0ien = crate::Reg<if0ien::If0ienSpec>;
#[doc = "Message Object Interrupt Enable Register"]
pub mod if0ien;
#[doc = "IF1IF (r) register accessor: Status Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if1if::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1if`] module"]
#[doc(alias = "IF1IF")]
pub type If1if = crate::Reg<if1if::If1ifSpec>;
#[doc = "Status Interrupt Flag Register"]
pub mod if1if;
#[doc = "IF1IFS (w) register accessor: Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if1ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1ifs`] module"]
#[doc(alias = "IF1IFS")]
pub type If1ifs = crate::Reg<if1ifs::If1ifsSpec>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if1ifs;
#[doc = "IF1IFC (w) register accessor: Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if1ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1ifc`] module"]
#[doc(alias = "IF1IFC")]
pub type If1ifc = crate::Reg<if1ifc::If1ifcSpec>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if1ifc;
#[doc = "IF1IEN (rw) register accessor: Status Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if1ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if1ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1ien`] module"]
#[doc(alias = "IF1IEN")]
pub type If1ien = crate::Reg<if1ien::If1ienSpec>;
#[doc = "Status Interrupt Enable Register"]
pub mod if1ien;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`] module"]
#[doc(alias = "ROUTE")]
pub type Route = crate::Reg<route::RouteSpec>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "MIR0_CMDMASK (rw) register accessor: Interface Command Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_cmdmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_cmdmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_cmdmask`] module"]
#[doc(alias = "MIR0_CMDMASK")]
pub type Mir0Cmdmask = crate::Reg<mir0_cmdmask::Mir0CmdmaskSpec>;
#[doc = "Interface Command Mask Register"]
pub mod mir0_cmdmask;
#[doc = "MIR0_MASK (rw) register accessor: Interface Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_mask`] module"]
#[doc(alias = "MIR0_MASK")]
pub type Mir0Mask = crate::Reg<mir0_mask::Mir0MaskSpec>;
#[doc = "Interface Mask Register"]
pub mod mir0_mask;
#[doc = "MIR0_ARB (rw) register accessor: Interface Arbitration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_arb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_arb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_arb`] module"]
#[doc(alias = "MIR0_ARB")]
pub type Mir0Arb = crate::Reg<mir0_arb::Mir0ArbSpec>;
#[doc = "Interface Arbitration Register"]
pub mod mir0_arb;
#[doc = "MIR0_CTRL (rw) register accessor: Interface Message Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_ctrl`] module"]
#[doc(alias = "MIR0_CTRL")]
pub type Mir0Ctrl = crate::Reg<mir0_ctrl::Mir0CtrlSpec>;
#[doc = "Interface Message Control Register"]
pub mod mir0_ctrl;
#[doc = "MIR0_DATAL (rw) register accessor: Interface Data a Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_datal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_datal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_datal`] module"]
#[doc(alias = "MIR0_DATAL")]
pub type Mir0Datal = crate::Reg<mir0_datal::Mir0DatalSpec>;
#[doc = "Interface Data a Register"]
pub mod mir0_datal;
#[doc = "MIR0_DATAH (rw) register accessor: Interface Data B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_datah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_datah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_datah`] module"]
#[doc(alias = "MIR0_DATAH")]
pub type Mir0Datah = crate::Reg<mir0_datah::Mir0DatahSpec>;
#[doc = "Interface Data B Register"]
pub mod mir0_datah;
#[doc = "MIR0_CMDREQ (rw) register accessor: Interface Command Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_cmdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_cmdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_cmdreq`] module"]
#[doc(alias = "MIR0_CMDREQ")]
pub type Mir0Cmdreq = crate::Reg<mir0_cmdreq::Mir0CmdreqSpec>;
#[doc = "Interface Command Request Register"]
pub mod mir0_cmdreq;
#[doc = "MIR1_CMDMASK (rw) register accessor: Interface Command Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_cmdmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_cmdmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_cmdmask`] module"]
#[doc(alias = "MIR1_CMDMASK")]
pub type Mir1Cmdmask = crate::Reg<mir1_cmdmask::Mir1CmdmaskSpec>;
#[doc = "Interface Command Mask Register"]
pub mod mir1_cmdmask;
#[doc = "MIR1_MASK (rw) register accessor: Interface Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_mask`] module"]
#[doc(alias = "MIR1_MASK")]
pub type Mir1Mask = crate::Reg<mir1_mask::Mir1MaskSpec>;
#[doc = "Interface Mask Register"]
pub mod mir1_mask;
#[doc = "MIR1_ARB (rw) register accessor: Interface Arbitration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_arb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_arb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_arb`] module"]
#[doc(alias = "MIR1_ARB")]
pub type Mir1Arb = crate::Reg<mir1_arb::Mir1ArbSpec>;
#[doc = "Interface Arbitration Register"]
pub mod mir1_arb;
#[doc = "MIR1_CTRL (rw) register accessor: Interface Message Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_ctrl`] module"]
#[doc(alias = "MIR1_CTRL")]
pub type Mir1Ctrl = crate::Reg<mir1_ctrl::Mir1CtrlSpec>;
#[doc = "Interface Message Control Register"]
pub mod mir1_ctrl;
#[doc = "MIR1_DATAL (rw) register accessor: Interface Data a Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_datal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_datal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_datal`] module"]
#[doc(alias = "MIR1_DATAL")]
pub type Mir1Datal = crate::Reg<mir1_datal::Mir1DatalSpec>;
#[doc = "Interface Data a Register"]
pub mod mir1_datal;
#[doc = "MIR1_DATAH (rw) register accessor: Interface Data B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_datah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_datah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_datah`] module"]
#[doc(alias = "MIR1_DATAH")]
pub type Mir1Datah = crate::Reg<mir1_datah::Mir1DatahSpec>;
#[doc = "Interface Data B Register"]
pub mod mir1_datah;
#[doc = "MIR1_CMDREQ (rw) register accessor: Interface Command Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_cmdreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_cmdreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_cmdreq`] module"]
#[doc(alias = "MIR1_CMDREQ")]
pub type Mir1Cmdreq = crate::Reg<mir1_cmdreq::Mir1CmdreqSpec>;
#[doc = "Interface Command Request Register"]
pub mod mir1_cmdreq;
