#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Error Count Register"]
    pub errcnt: ERRCNT,
    #[doc = "0x0c - Bit Timing Register"]
    pub bittiming: BITTIMING,
    #[doc = "0x10 - Interrupt Identification Register"]
    pub intid: INTID,
    #[doc = "0x14 - Test Register"]
    pub test: TEST,
    #[doc = "0x18 - BRP Extension Register"]
    pub brpe: BRPE,
    #[doc = "0x1c - Transmission Request Register"]
    pub transreq: TRANSREQ,
    #[doc = "0x20 - New Data Register"]
    pub messagedata: MESSAGEDATA,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Message Valid Register"]
    pub messagestate: MESSAGESTATE,
    #[doc = "0x2c - Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x30 - Message Object Interrupt Flag Register"]
    pub if0if: IF0IF,
    #[doc = "0x34 - Message Object Interrupt Flag Set Register"]
    pub if0ifs: IF0IFS,
    #[doc = "0x38 - Message Object Interrupt Flag Clear Register"]
    pub if0ifc: IF0IFC,
    #[doc = "0x3c - Message Object Interrupt Enable Register"]
    pub if0ien: IF0IEN,
    #[doc = "0x40 - Status Interrupt Flag Register"]
    pub if1if: IF1IF,
    #[doc = "0x44 - Message Object Interrupt Flag Set Register"]
    pub if1ifs: IF1IFS,
    #[doc = "0x48 - Message Object Interrupt Flag Clear Register"]
    pub if1ifc: IF1IFC,
    #[doc = "0x4c - Status Interrupt Enable Register"]
    pub if1ien: IF1IEN,
    #[doc = "0x50 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved20: [u8; 0x0c],
    #[doc = "0x60 - Interface Command Mask Register"]
    pub mir0_cmdmask: MIR0_CMDMASK,
    #[doc = "0x64 - Interface Mask Register"]
    pub mir0_mask: MIR0_MASK,
    #[doc = "0x68 - Interface Arbitration Register"]
    pub mir0_arb: MIR0_ARB,
    #[doc = "0x6c - Interface Message Control Register"]
    pub mir0_ctrl: MIR0_CTRL,
    #[doc = "0x70 - Interface Data a Register"]
    pub mir0_datal: MIR0_DATAL,
    #[doc = "0x74 - Interface Data B Register"]
    pub mir0_datah: MIR0_DATAH,
    #[doc = "0x78 - Interface Command Request Register"]
    pub mir0_cmdreq: MIR0_CMDREQ,
    _reserved27: [u8; 0x04],
    #[doc = "0x80 - Interface Command Mask Register"]
    pub mir1_cmdmask: MIR1_CMDMASK,
    #[doc = "0x84 - Interface Mask Register"]
    pub mir1_mask: MIR1_MASK,
    #[doc = "0x88 - Interface Arbitration Register"]
    pub mir1_arb: MIR1_ARB,
    #[doc = "0x8c - Interface Message Control Register"]
    pub mir1_ctrl: MIR1_CTRL,
    #[doc = "0x90 - Interface Data a Register"]
    pub mir1_datal: MIR1_DATAL,
    #[doc = "0x94 - Interface Data B Register"]
    pub mir1_datah: MIR1_DATAH,
    #[doc = "0x98 - Interface Command Request Register"]
    pub mir1_cmdreq: MIR1_CMDREQ,
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ERRCNT (r) register accessor: Error Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`errcnt`] module"]
pub type ERRCNT = crate::Reg<errcnt::ERRCNT_SPEC>;
#[doc = "Error Count Register"]
pub mod errcnt;
#[doc = "BITTIMING (rw) register accessor: Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bittiming::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bittiming::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bittiming`] module"]
pub type BITTIMING = crate::Reg<bittiming::BITTIMING_SPEC>;
#[doc = "Bit Timing Register"]
pub mod bittiming;
#[doc = "INTID (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intid`] module"]
pub type INTID = crate::Reg<intid::INTID_SPEC>;
#[doc = "Interrupt Identification Register"]
pub mod intid;
#[doc = "TEST (rw) register accessor: Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`test`] module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "BRPE (rw) register accessor: BRP Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brpe`] module"]
pub type BRPE = crate::Reg<brpe::BRPE_SPEC>;
#[doc = "BRP Extension Register"]
pub mod brpe;
#[doc = "TRANSREQ (r) register accessor: Transmission Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transreq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`transreq`] module"]
pub type TRANSREQ = crate::Reg<transreq::TRANSREQ_SPEC>;
#[doc = "Transmission Request Register"]
pub mod transreq;
#[doc = "MESSAGEDATA (r) register accessor: New Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`messagedata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`messagedata`] module"]
pub type MESSAGEDATA = crate::Reg<messagedata::MESSAGEDATA_SPEC>;
#[doc = "New Data Register"]
pub mod messagedata;
#[doc = "MESSAGESTATE (r) register accessor: Message Valid Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`messagestate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`messagestate`] module"]
pub type MESSAGESTATE = crate::Reg<messagestate::MESSAGESTATE_SPEC>;
#[doc = "Message Valid Register"]
pub mod messagestate;
#[doc = "CONFIG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "IF0IF (r) register accessor: Message Object Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if0if::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if0if`] module"]
pub type IF0IF = crate::Reg<if0if::IF0IF_SPEC>;
#[doc = "Message Object Interrupt Flag Register"]
pub mod if0if;
#[doc = "IF0IFS (w) register accessor: Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if0ifs`] module"]
pub type IF0IFS = crate::Reg<if0ifs::IF0IFS_SPEC>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if0ifs;
#[doc = "IF0IFC (w) register accessor: Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if0ifc`] module"]
pub type IF0IFC = crate::Reg<if0ifc::IF0IFC_SPEC>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if0ifc;
#[doc = "IF0IEN (rw) register accessor: Message Object Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if0ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if0ien`] module"]
pub type IF0IEN = crate::Reg<if0ien::IF0IEN_SPEC>;
#[doc = "Message Object Interrupt Enable Register"]
pub mod if0ien;
#[doc = "IF1IF (r) register accessor: Status Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1if::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if1if`] module"]
pub type IF1IF = crate::Reg<if1if::IF1IF_SPEC>;
#[doc = "Status Interrupt Flag Register"]
pub mod if1if;
#[doc = "IF1IFS (w) register accessor: Message Object Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if1ifs`] module"]
pub type IF1IFS = crate::Reg<if1ifs::IF1IFS_SPEC>;
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if1ifs;
#[doc = "IF1IFC (w) register accessor: Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if1ifc`] module"]
pub type IF1IFC = crate::Reg<if1ifc::IF1IFC_SPEC>;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if1ifc;
#[doc = "IF1IEN (rw) register accessor: Status Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if1ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`if1ien`] module"]
pub type IF1IEN = crate::Reg<if1ien::IF1IEN_SPEC>;
#[doc = "Status Interrupt Enable Register"]
pub mod if1ien;
#[doc = "ROUTE (rw) register accessor: I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`route::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`route::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`route`] module"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "MIR0_CMDMASK (rw) register accessor: Interface Command Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_cmdmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_cmdmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_cmdmask`] module"]
pub type MIR0_CMDMASK = crate::Reg<mir0_cmdmask::MIR0_CMDMASK_SPEC>;
#[doc = "Interface Command Mask Register"]
pub mod mir0_cmdmask;
#[doc = "MIR0_MASK (rw) register accessor: Interface Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_mask`] module"]
pub type MIR0_MASK = crate::Reg<mir0_mask::MIR0_MASK_SPEC>;
#[doc = "Interface Mask Register"]
pub mod mir0_mask;
#[doc = "MIR0_ARB (rw) register accessor: Interface Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_arb`] module"]
pub type MIR0_ARB = crate::Reg<mir0_arb::MIR0_ARB_SPEC>;
#[doc = "Interface Arbitration Register"]
pub mod mir0_arb;
#[doc = "MIR0_CTRL (rw) register accessor: Interface Message Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_ctrl`] module"]
pub type MIR0_CTRL = crate::Reg<mir0_ctrl::MIR0_CTRL_SPEC>;
#[doc = "Interface Message Control Register"]
pub mod mir0_ctrl;
#[doc = "MIR0_DATAL (rw) register accessor: Interface Data a Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_datal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_datal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_datal`] module"]
pub type MIR0_DATAL = crate::Reg<mir0_datal::MIR0_DATAL_SPEC>;
#[doc = "Interface Data a Register"]
pub mod mir0_datal;
#[doc = "MIR0_DATAH (rw) register accessor: Interface Data B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_datah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_datah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_datah`] module"]
pub type MIR0_DATAH = crate::Reg<mir0_datah::MIR0_DATAH_SPEC>;
#[doc = "Interface Data B Register"]
pub mod mir0_datah;
#[doc = "MIR0_CMDREQ (rw) register accessor: Interface Command Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_cmdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_cmdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir0_cmdreq`] module"]
pub type MIR0_CMDREQ = crate::Reg<mir0_cmdreq::MIR0_CMDREQ_SPEC>;
#[doc = "Interface Command Request Register"]
pub mod mir0_cmdreq;
#[doc = "MIR1_CMDMASK (rw) register accessor: Interface Command Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_cmdmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_cmdmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_cmdmask`] module"]
pub type MIR1_CMDMASK = crate::Reg<mir1_cmdmask::MIR1_CMDMASK_SPEC>;
#[doc = "Interface Command Mask Register"]
pub mod mir1_cmdmask;
#[doc = "MIR1_MASK (rw) register accessor: Interface Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_mask`] module"]
pub type MIR1_MASK = crate::Reg<mir1_mask::MIR1_MASK_SPEC>;
#[doc = "Interface Mask Register"]
pub mod mir1_mask;
#[doc = "MIR1_ARB (rw) register accessor: Interface Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_arb`] module"]
pub type MIR1_ARB = crate::Reg<mir1_arb::MIR1_ARB_SPEC>;
#[doc = "Interface Arbitration Register"]
pub mod mir1_arb;
#[doc = "MIR1_CTRL (rw) register accessor: Interface Message Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_ctrl`] module"]
pub type MIR1_CTRL = crate::Reg<mir1_ctrl::MIR1_CTRL_SPEC>;
#[doc = "Interface Message Control Register"]
pub mod mir1_ctrl;
#[doc = "MIR1_DATAL (rw) register accessor: Interface Data a Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_datal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_datal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_datal`] module"]
pub type MIR1_DATAL = crate::Reg<mir1_datal::MIR1_DATAL_SPEC>;
#[doc = "Interface Data a Register"]
pub mod mir1_datal;
#[doc = "MIR1_DATAH (rw) register accessor: Interface Data B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_datah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_datah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_datah`] module"]
pub type MIR1_DATAH = crate::Reg<mir1_datah::MIR1_DATAH_SPEC>;
#[doc = "Interface Data B Register"]
pub mod mir1_datah;
#[doc = "MIR1_CMDREQ (rw) register accessor: Interface Command Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_cmdreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_cmdreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mir1_cmdreq`] module"]
pub type MIR1_CMDREQ = crate::Reg<mir1_cmdreq::MIR1_CMDREQ_SPEC>;
#[doc = "Interface Command Request Register"]
pub mod mir1_cmdreq;
