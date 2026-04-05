#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    fifolevel: Fifolevel,
    _reserved2: [u8; 0x04],
    fifodepth: Fifodepth,
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
    testdata: Testdata,
    _reserved8: [u8; 0x0c],
    status: Status,
    initwaitval: Initwaitval,
    _reserved10: [u8; 0xc8],
    fifo: Fifo,
}
impl RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - FIFO Level Register"]
    #[inline(always)]
    pub const fn fifolevel(&self) -> &Fifolevel {
        &self.fifolevel
    }
    #[doc = "0x0c - FIFO Depth Register"]
    #[inline(always)]
    pub const fn fifodepth(&self) -> &Fifodepth {
        &self.fifodepth
    }
    #[doc = "0x10 - Key Register 0"]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x14 - Key Register 1"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x18 - Key Register 2"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x1c - Key Register 3"]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
    #[doc = "0x20 - Test Data Register"]
    #[inline(always)]
    pub const fn testdata(&self) -> &Testdata {
        &self.testdata
    }
    #[doc = "0x30 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Initial Wait Counter"]
    #[inline(always)]
    pub const fn initwaitval(&self) -> &Initwaitval {
        &self.initwaitval
    }
    #[doc = "0x100 - FIFO Data"]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
}
#[doc = "CONTROL (rw) register accessor: Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Main Control Register"]
pub mod control;
#[doc = "FIFOLEVEL (r) register accessor: FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolevel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@fifolevel`] module"]
#[doc(alias = "FIFOLEVEL")]
pub type Fifolevel = crate::Reg<fifolevel::FifolevelSpec>;
#[doc = "FIFO Level Register"]
pub mod fifolevel;
#[doc = "FIFODEPTH (r) register accessor: FIFO Depth Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodepth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodepth`] module"]
#[doc(alias = "FIFODEPTH")]
pub type Fifodepth = crate::Reg<fifodepth::FifodepthSpec>;
#[doc = "FIFO Depth Register"]
pub mod fifodepth;
#[doc = "KEY0 (rw) register accessor: Key Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`] module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "Key Register 0"]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: Key Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`] module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "Key Register 1"]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: Key Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`] module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "Key Register 2"]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: Key Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`] module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "Key Register 3"]
pub mod key3;
#[doc = "TESTDATA (rw) register accessor: Test Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`testdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testdata`] module"]
#[doc(alias = "TESTDATA")]
pub type Testdata = crate::Reg<testdata::TestdataSpec>;
#[doc = "Test Data Register"]
pub mod testdata;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INITWAITVAL (rw) register accessor: Initial Wait Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`initwaitval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initwaitval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initwaitval`] module"]
#[doc(alias = "INITWAITVAL")]
pub type Initwaitval = crate::Reg<initwaitval::InitwaitvalSpec>;
#[doc = "Initial Wait Counter"]
pub mod initwaitval;
#[doc = "FIFO (r) register accessor: FIFO Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@fifo`] module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "FIFO Data"]
pub mod fifo;
