#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - CRC Init Value"]
    pub init: INIT,
    #[doc = "0x0c - CRC Polynomial Value"]
    pub poly: POLY,
    #[doc = "0x10 - Input 32-bit Data Register"]
    pub inputdata: INPUTDATA,
    #[doc = "0x14 - Input 16-bit Data Register"]
    pub inputdatahword: INPUTDATAHWORD,
    #[doc = "0x18 - Input 8-bit Data Register"]
    pub inputdatabyte: INPUTDATABYTE,
    #[doc = "0x1c - CRC Data Register"]
    pub data: DATA,
    #[doc = "0x20 - CRC Data Reverse Register"]
    pub datarev: DATAREV,
    #[doc = "0x24 - CRC Data Byte Reverse Register"]
    pub databyterev: DATABYTEREV,
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "INIT (rw) register accessor: CRC Init Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`init`] module"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "CRC Init Value"]
pub mod init;
#[doc = "POLY (rw) register accessor: CRC Polynomial Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`poly`] module"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "CRC Polynomial Value"]
pub mod poly;
#[doc = "INPUTDATA (rw) register accessor: Input 32-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inputdata`] module"]
pub type INPUTDATA = crate::Reg<inputdata::INPUTDATA_SPEC>;
#[doc = "Input 32-bit Data Register"]
pub mod inputdata;
#[doc = "INPUTDATAHWORD (rw) register accessor: Input 16-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdatahword::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdatahword::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inputdatahword`] module"]
pub type INPUTDATAHWORD = crate::Reg<inputdatahword::INPUTDATAHWORD_SPEC>;
#[doc = "Input 16-bit Data Register"]
pub mod inputdatahword;
#[doc = "INPUTDATABYTE (rw) register accessor: Input 8-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdatabyte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdatabyte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inputdatabyte`] module"]
pub type INPUTDATABYTE = crate::Reg<inputdatabyte::INPUTDATABYTE_SPEC>;
#[doc = "Input 8-bit Data Register"]
pub mod inputdatabyte;
#[doc = "DATA (r) register accessor: CRC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "CRC Data Register"]
pub mod data;
#[doc = "DATAREV (r) register accessor: CRC Data Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datarev::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datarev`] module"]
pub type DATAREV = crate::Reg<datarev::DATAREV_SPEC>;
#[doc = "CRC Data Reverse Register"]
pub mod datarev;
#[doc = "DATABYTEREV (r) register accessor: CRC Data Byte Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`databyterev::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`databyterev`] module"]
pub type DATABYTEREV = crate::Reg<databyterev::DATABYTEREV_SPEC>;
#[doc = "CRC Data Byte Reverse Register"]
pub mod databyterev;
