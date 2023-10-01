#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub control: CONTROL,
    #[doc = "0x04 - FIFO Level Register"]
    pub fifolevel: FIFOLEVEL,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FIFO Depth Register"]
    pub fifodepth: FIFODEPTH,
    #[doc = "0x10 - Key Register 0"]
    pub key0: KEY0,
    #[doc = "0x14 - Key Register 1"]
    pub key1: KEY1,
    #[doc = "0x18 - Key Register 2"]
    pub key2: KEY2,
    #[doc = "0x1c - Key Register 3"]
    pub key3: KEY3,
    #[doc = "0x20 - Test Data Register"]
    pub testdata: TESTDATA,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Status Register"]
    pub status: STATUS,
    #[doc = "0x34 - Initial Wait Counter"]
    pub initwaitval: INITWAITVAL,
    _reserved10: [u8; 0xc8],
    #[doc = "0x100 - FIFO Data"]
    pub fifo: FIFO,
}
#[doc = "CONTROL (rw) register accessor: Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`control`] module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Main Control Register"]
pub mod control;
#[doc = "FIFOLEVEL (r) register accessor: FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifolevel::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifolevel`] module"]
pub type FIFOLEVEL = crate::Reg<fifolevel::FIFOLEVEL_SPEC>;
#[doc = "FIFO Level Register"]
pub mod fifolevel;
#[doc = "FIFODEPTH (r) register accessor: FIFO Depth Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifodepth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifodepth`] module"]
pub type FIFODEPTH = crate::Reg<fifodepth::FIFODEPTH_SPEC>;
#[doc = "FIFO Depth Register"]
pub mod fifodepth;
#[doc = "KEY0 (rw) register accessor: Key Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key0`] module"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Key Register 0"]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: Key Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key1`] module"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Key Register 1"]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: Key Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key2`] module"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "Key Register 2"]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: Key Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key3`] module"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "Key Register 3"]
pub mod key3;
#[doc = "TESTDATA (rw) register accessor: Test Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`testdata`] module"]
pub type TESTDATA = crate::Reg<testdata::TESTDATA_SPEC>;
#[doc = "Test Data Register"]
pub mod testdata;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INITWAITVAL (rw) register accessor: Initial Wait Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`initwaitval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`initwaitval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`initwaitval`] module"]
pub type INITWAITVAL = crate::Reg<initwaitval::INITWAITVAL_SPEC>;
#[doc = "Initial Wait Counter"]
pub mod initwaitval;
#[doc = "FIFO (r) register accessor: FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`] module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Data"]
pub mod fifo;
