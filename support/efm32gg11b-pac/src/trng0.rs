#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    control: CONTROL,
    fifolevel: FIFOLEVEL,
    _reserved2: [u8; 0x04],
    fifodepth: FIFODEPTH,
    key0: KEY0,
    key1: KEY1,
    key2: KEY2,
    key3: KEY3,
    testdata: TESTDATA,
    _reserved8: [u8; 0x0c],
    status: STATUS,
    initwaitval: INITWAITVAL,
    _reserved10: [u8; 0xc8],
    fifo: FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - FIFO Level Register"]
    #[inline(always)]
    pub const fn fifolevel(&self) -> &FIFOLEVEL {
        &self.fifolevel
    }
    #[doc = "0x0c - FIFO Depth Register"]
    #[inline(always)]
    pub const fn fifodepth(&self) -> &FIFODEPTH {
        &self.fifodepth
    }
    #[doc = "0x10 - Key Register 0"]
    #[inline(always)]
    pub const fn key0(&self) -> &KEY0 {
        &self.key0
    }
    #[doc = "0x14 - Key Register 1"]
    #[inline(always)]
    pub const fn key1(&self) -> &KEY1 {
        &self.key1
    }
    #[doc = "0x18 - Key Register 2"]
    #[inline(always)]
    pub const fn key2(&self) -> &KEY2 {
        &self.key2
    }
    #[doc = "0x1c - Key Register 3"]
    #[inline(always)]
    pub const fn key3(&self) -> &KEY3 {
        &self.key3
    }
    #[doc = "0x20 - Test Data Register"]
    #[inline(always)]
    pub const fn testdata(&self) -> &TESTDATA {
        &self.testdata
    }
    #[doc = "0x30 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x34 - Initial Wait Counter"]
    #[inline(always)]
    pub const fn initwaitval(&self) -> &INITWAITVAL {
        &self.initwaitval
    }
    #[doc = "0x100 - FIFO Data"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
}
#[doc = "CONTROL (rw) register accessor: Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Main Control Register"]
pub mod control;
#[doc = "FIFOLEVEL (r) register accessor: FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifolevel::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifolevel`]
module"]
pub type FIFOLEVEL = crate::Reg<fifolevel::FIFOLEVEL_SPEC>;
#[doc = "FIFO Level Register"]
pub mod fifolevel;
#[doc = "FIFODEPTH (r) register accessor: FIFO Depth Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifodepth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodepth`]
module"]
pub type FIFODEPTH = crate::Reg<fifodepth::FIFODEPTH_SPEC>;
#[doc = "FIFO Depth Register"]
pub mod fifodepth;
#[doc = "KEY0 (rw) register accessor: Key Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`]
module"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Key Register 0"]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: Key Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Key Register 1"]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: Key Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "Key Register 2"]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: Key Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`]
module"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "Key Register 3"]
pub mod key3;
#[doc = "TESTDATA (rw) register accessor: Test Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testdata`]
module"]
pub type TESTDATA = crate::Reg<testdata::TESTDATA_SPEC>;
#[doc = "Test Data Register"]
pub mod testdata;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INITWAITVAL (rw) register accessor: Initial Wait Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`initwaitval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`initwaitval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initwaitval`]
module"]
pub type INITWAITVAL = crate::Reg<initwaitval::INITWAITVAL_SPEC>;
#[doc = "Initial Wait Counter"]
pub mod initwaitval;
#[doc = "FIFO (r) register accessor: FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Data"]
pub mod fifo;
