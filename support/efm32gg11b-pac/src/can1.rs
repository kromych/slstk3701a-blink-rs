#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    status: STATUS,
    errcnt: ERRCNT,
    bittiming: BITTIMING,
    intid: INTID,
    test: TEST,
    brpe: BRPE,
    transreq: TRANSREQ,
    messagedata: MESSAGEDATA,
    _reserved9: [u8; 0x04],
    messagestate: MESSAGESTATE,
    config: CONFIG,
    if0if: IF0IF,
    if0ifs: IF0IFS,
    if0ifc: IF0IFC,
    if0ien: IF0IEN,
    if1if: IF1IF,
    if1ifs: IF1IFS,
    if1ifc: IF1IFC,
    if1ien: IF1IEN,
    route: ROUTE,
    _reserved20: [u8; 0x0c],
    mir0_cmdmask: MIR0_CMDMASK,
    mir0_mask: MIR0_MASK,
    mir0_arb: MIR0_ARB,
    mir0_ctrl: MIR0_CTRL,
    mir0_datal: MIR0_DATAL,
    mir0_datah: MIR0_DATAH,
    mir0_cmdreq: MIR0_CMDREQ,
    _reserved27: [u8; 0x04],
    mir1_cmdmask: MIR1_CMDMASK,
    mir1_mask: MIR1_MASK,
    mir1_arb: MIR1_ARB,
    mir1_ctrl: MIR1_CTRL,
    mir1_datal: MIR1_DATAL,
    mir1_datah: MIR1_DATAH,
    mir1_cmdreq: MIR1_CMDREQ,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Error Count Register"]
    #[inline(always)]
    pub const fn errcnt(&self) -> &ERRCNT {
        &self.errcnt
    }
    #[doc = "0x0c - Bit Timing Register"]
    #[inline(always)]
    pub const fn bittiming(&self) -> &BITTIMING {
        &self.bittiming
    }
    #[doc = "0x10 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn intid(&self) -> &INTID {
        &self.intid
    }
    #[doc = "0x14 - Test Register"]
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    #[doc = "0x18 - BRP Extension Register"]
    #[inline(always)]
    pub const fn brpe(&self) -> &BRPE {
        &self.brpe
    }
    #[doc = "0x1c - Transmission Request Register"]
    #[inline(always)]
    pub const fn transreq(&self) -> &TRANSREQ {
        &self.transreq
    }
    #[doc = "0x20 - New Data Register"]
    #[inline(always)]
    pub const fn messagedata(&self) -> &MESSAGEDATA {
        &self.messagedata
    }
    #[doc = "0x28 - Message Valid Register"]
    #[inline(always)]
    pub const fn messagestate(&self) -> &MESSAGESTATE {
        &self.messagestate
    }
    #[doc = "0x2c - Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x30 - Message Object Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if0if(&self) -> &IF0IF {
        &self.if0if
    }
    #[doc = "0x34 - Message Object Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn if0ifs(&self) -> &IF0IFS {
        &self.if0ifs
    }
    #[doc = "0x38 - Message Object Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn if0ifc(&self) -> &IF0IFC {
        &self.if0ifc
    }
    #[doc = "0x3c - Message Object Interrupt Enable Register"]
    #[inline(always)]
    pub const fn if0ien(&self) -> &IF0IEN {
        &self.if0ien
    }
    #[doc = "0x40 - Status Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if1if(&self) -> &IF1IF {
        &self.if1if
    }
    #[doc = "0x44 - Message Object Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn if1ifs(&self) -> &IF1IFS {
        &self.if1ifs
    }
    #[doc = "0x48 - Message Object Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn if1ifc(&self) -> &IF1IFC {
        &self.if1ifc
    }
    #[doc = "0x4c - Status Interrupt Enable Register"]
    #[inline(always)]
    pub const fn if1ien(&self) -> &IF1IEN {
        &self.if1ien
    }
    #[doc = "0x50 - I/O Routing Register"]
    #[inline(always)]
    pub const fn route(&self) -> &ROUTE {
        &self.route
    }
    #[doc = "0x60 - Interface Command Mask Register"]
    #[inline(always)]
    pub const fn mir0_cmdmask(&self) -> &MIR0_CMDMASK {
        &self.mir0_cmdmask
    }
    #[doc = "0x64 - Interface Mask Register"]
    #[inline(always)]
    pub const fn mir0_mask(&self) -> &MIR0_MASK {
        &self.mir0_mask
    }
    #[doc = "0x68 - Interface Arbitration Register"]
    #[inline(always)]
    pub const fn mir0_arb(&self) -> &MIR0_ARB {
        &self.mir0_arb
    }
    #[doc = "0x6c - Interface Message Control Register"]
    #[inline(always)]
    pub const fn mir0_ctrl(&self) -> &MIR0_CTRL {
        &self.mir0_ctrl
    }
    #[doc = "0x70 - Interface Data a Register"]
    #[inline(always)]
    pub const fn mir0_datal(&self) -> &MIR0_DATAL {
        &self.mir0_datal
    }
    #[doc = "0x74 - Interface Data B Register"]
    #[inline(always)]
    pub const fn mir0_datah(&self) -> &MIR0_DATAH {
        &self.mir0_datah
    }
    #[doc = "0x78 - Interface Command Request Register"]
    #[inline(always)]
    pub const fn mir0_cmdreq(&self) -> &MIR0_CMDREQ {
        &self.mir0_cmdreq
    }
    #[doc = "0x80 - Interface Command Mask Register"]
    #[inline(always)]
    pub const fn mir1_cmdmask(&self) -> &MIR1_CMDMASK {
        &self.mir1_cmdmask
    }
    #[doc = "0x84 - Interface Mask Register"]
    #[inline(always)]
    pub const fn mir1_mask(&self) -> &MIR1_MASK {
        &self.mir1_mask
    }
    #[doc = "0x88 - Interface Arbitration Register"]
    #[inline(always)]
    pub const fn mir1_arb(&self) -> &MIR1_ARB {
        &self.mir1_arb
    }
    #[doc = "0x8c - Interface Message Control Register"]
    #[inline(always)]
    pub const fn mir1_ctrl(&self) -> &MIR1_CTRL {
        &self.mir1_ctrl
    }
    #[doc = "0x90 - Interface Data a Register"]
    #[inline(always)]
    pub const fn mir1_datal(&self) -> &MIR1_DATAL {
        &self.mir1_datal
    }
    #[doc = "0x94 - Interface Data B Register"]
    #[inline(always)]
    pub const fn mir1_datah(&self) -> &MIR1_DATAH {
        &self.mir1_datah
    }
    #[doc = "0x98 - Interface Command Request Register"]
    #[inline(always)]
    pub const fn mir1_cmdreq(&self) -> &MIR1_CMDREQ {
        &self.mir1_cmdreq
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ERRCNT (r) register accessor: Error Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errcnt`]
module"]
pub type ERRCNT = crate::Reg<errcnt::ERRCNT_SPEC>;
#[doc = "Error Count Register"]
pub mod errcnt;
#[doc = "BITTIMING (rw) register accessor: Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bittiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bittiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bittiming`]
module"]
pub type BITTIMING = crate::Reg<bittiming::BITTIMING_SPEC>;
#[doc = "Bit Timing Register"]
pub mod bittiming;
#[doc = "INTID (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intid`]
module"]
pub type INTID = crate::Reg<intid::INTID_SPEC>;
#[doc = "Interrupt Identification Register"]
pub mod intid;
#[doc = "TEST (rw) register accessor: Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "BRPE (rw) register accessor: BRP Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brpe`]
module"]
pub type BRPE = crate::Reg<brpe::BRPE_SPEC>;
#[doc = "BRP Extension Register"]
pub mod brpe;
#[doc = "TRANSREQ (r) register accessor: Transmission Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transreq`]
module"]
pub type TRANSREQ = crate::Reg<transreq::TRANSREQ_SPEC>;
#[doc = "Transmission Request Register"]
pub mod transreq;
#[doc = "MESSAGEDATA (r) register accessor: New Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`messagedata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@messagedata`]
module"]
pub type MESSAGEDATA = crate::Reg<messagedata::MESSAGEDATA_SPEC>;
#[doc = "New Data Register"]
pub mod messagedata;
#[doc = "MESSAGESTATE (r) register accessor: Message Valid Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`messagestate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@messagestate`]
module"]
pub type MESSAGESTATE = crate::Reg<messagestate::MESSAGESTATE_SPEC>;
#[doc = "Message Valid Register"]
pub mod messagestate;
#[doc = "CONFIG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "IF0IF (r) register accessor: Message Object Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if0if::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0if`]
module"]
pub type IF0IF = crate::Reg<if0if::IF0IF_SPEC>;
#[doc = "Message Object Interrupt Flag Register"]
pub mod if0if;
#[doc = "IF0IFS (w) register accessor: Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0ifs`]
module"]
pub type IF0IFS = crate::Reg<if0ifs::IF0IFS_SPEC>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if0ifs;
#[doc = "IF0IFC (w) register accessor: Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0ifc`]
module"]
pub type IF0IFC = crate::Reg<if0ifc::IF0IFC_SPEC>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if0ifc;
#[doc = "IF0IEN (rw) register accessor: Message Object Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if0ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if0ien`]
module"]
pub type IF0IEN = crate::Reg<if0ien::IF0IEN_SPEC>;
#[doc = "Message Object Interrupt Enable Register"]
pub mod if0ien;
#[doc = "IF1IF (r) register accessor: Status Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1if::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1if`]
module"]
pub type IF1IF = crate::Reg<if1if::IF1IF_SPEC>;
#[doc = "Status Interrupt Flag Register"]
pub mod if1if;
#[doc = "IF1IFS (w) register accessor: Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1ifs`]
module"]
pub type IF1IFS = crate::Reg<if1ifs::IF1IFS_SPEC>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if1ifs;
#[doc = "IF1IFC (w) register accessor: Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1ifc`]
module"]
pub type IF1IFC = crate::Reg<if1ifc::IF1IFC_SPEC>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if1ifc;
#[doc = "IF1IEN (rw) register accessor: Status Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if1ien`]
module"]
pub type IF1IEN = crate::Reg<if1ien::IF1IEN_SPEC>;
#[doc = "Status Interrupt Enable Register"]
pub mod if1ien;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@route`]
module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "MIR0_CMDMASK (rw) register accessor: Interface Command Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_cmdmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_cmdmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_cmdmask`]
module"]
pub type MIR0_CMDMASK = crate::Reg<mir0_cmdmask::MIR0_CMDMASK_SPEC>;
#[doc = "Interface Command Mask Register"]
pub mod mir0_cmdmask;
#[doc = "MIR0_MASK (rw) register accessor: Interface Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_mask`]
module"]
pub type MIR0_MASK = crate::Reg<mir0_mask::MIR0_MASK_SPEC>;
#[doc = "Interface Mask Register"]
pub mod mir0_mask;
#[doc = "MIR0_ARB (rw) register accessor: Interface Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_arb`]
module"]
pub type MIR0_ARB = crate::Reg<mir0_arb::MIR0_ARB_SPEC>;
#[doc = "Interface Arbitration Register"]
pub mod mir0_arb;
#[doc = "MIR0_CTRL (rw) register accessor: Interface Message Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_ctrl`]
module"]
pub type MIR0_CTRL = crate::Reg<mir0_ctrl::MIR0_CTRL_SPEC>;
#[doc = "Interface Message Control Register"]
pub mod mir0_ctrl;
#[doc = "MIR0_DATAL (rw) register accessor: Interface Data a Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_datal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_datal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_datal`]
module"]
pub type MIR0_DATAL = crate::Reg<mir0_datal::MIR0_DATAL_SPEC>;
#[doc = "Interface Data a Register"]
pub mod mir0_datal;
#[doc = "MIR0_DATAH (rw) register accessor: Interface Data B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_datah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_datah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_datah`]
module"]
pub type MIR0_DATAH = crate::Reg<mir0_datah::MIR0_DATAH_SPEC>;
#[doc = "Interface Data B Register"]
pub mod mir0_datah;
#[doc = "MIR0_CMDREQ (rw) register accessor: Interface Command Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_cmdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_cmdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir0_cmdreq`]
module"]
pub type MIR0_CMDREQ = crate::Reg<mir0_cmdreq::MIR0_CMDREQ_SPEC>;
#[doc = "Interface Command Request Register"]
pub mod mir0_cmdreq;
#[doc = "MIR1_CMDMASK (rw) register accessor: Interface Command Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_cmdmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_cmdmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_cmdmask`]
module"]
pub type MIR1_CMDMASK = crate::Reg<mir1_cmdmask::MIR1_CMDMASK_SPEC>;
#[doc = "Interface Command Mask Register"]
pub mod mir1_cmdmask;
#[doc = "MIR1_MASK (rw) register accessor: Interface Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_mask`]
module"]
pub type MIR1_MASK = crate::Reg<mir1_mask::MIR1_MASK_SPEC>;
#[doc = "Interface Mask Register"]
pub mod mir1_mask;
#[doc = "MIR1_ARB (rw) register accessor: Interface Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_arb`]
module"]
pub type MIR1_ARB = crate::Reg<mir1_arb::MIR1_ARB_SPEC>;
#[doc = "Interface Arbitration Register"]
pub mod mir1_arb;
#[doc = "MIR1_CTRL (rw) register accessor: Interface Message Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_ctrl`]
module"]
pub type MIR1_CTRL = crate::Reg<mir1_ctrl::MIR1_CTRL_SPEC>;
#[doc = "Interface Message Control Register"]
pub mod mir1_ctrl;
#[doc = "MIR1_DATAL (rw) register accessor: Interface Data a Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_datal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_datal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_datal`]
module"]
pub type MIR1_DATAL = crate::Reg<mir1_datal::MIR1_DATAL_SPEC>;
#[doc = "Interface Data a Register"]
pub mod mir1_datal;
#[doc = "MIR1_DATAH (rw) register accessor: Interface Data B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_datah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_datah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_datah`]
module"]
pub type MIR1_DATAH = crate::Reg<mir1_datah::MIR1_DATAH_SPEC>;
#[doc = "Interface Data B Register"]
pub mod mir1_datah;
#[doc = "MIR1_CMDREQ (rw) register accessor: Interface Command Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_cmdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_cmdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mir1_cmdreq`]
module"]
pub type MIR1_CMDREQ = crate::Reg<mir1_cmdreq::MIR1_CMDREQ_SPEC>;
#[doc = "Interface Command Request Register"]
pub mod mir1_cmdreq;
