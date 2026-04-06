#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    wac: Wac,
    cmd: Cmd,
    _reserved3: [u8; 0x04],
    status: Status,
    dstatus: Dstatus,
    cstatus: Cstatus,
    _reserved6: [u8; 0x04],
    key: Key,
    keybuf: Keybuf,
    _reserved8: [u8; 0x08],
    seqctrl: Seqctrl,
    seqctrlb: Seqctrlb,
    _reserved10: [u8; 0x08],
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    seq0: Seq0,
    seq1: Seq1,
    seq2: Seq2,
    seq3: Seq3,
    seq4: Seq4,
    _reserved19: [u8; 0x1c],
    data0: Data0,
    data1: Data1,
    data2: Data2,
    data3: Data3,
    _reserved23: [u8; 0x10],
    data0xor: Data0xor,
    _reserved24: [u8; 0x0c],
    data0byte: Data0byte,
    data1byte: Data1byte,
    _reserved26: [u8; 0x04],
    data0xorbyte: Data0xorbyte,
    data0byte12: Data0byte12,
    data0byte13: Data0byte13,
    data0byte14: Data0byte14,
    data0byte15: Data0byte15,
    _reserved31: [u8; 0x30],
    ddata0: Ddata0,
    ddata1: Ddata1,
    ddata2: Ddata2,
    ddata3: Ddata3,
    ddata4: Ddata4,
    _reserved36: [u8; 0x1c],
    ddata0big: Ddata0big,
    _reserved37: [u8; 0x0c],
    ddata0byte: Ddata0byte,
    ddata1byte: Ddata1byte,
    ddata0byte32: Ddata0byte32,
    _reserved40: [u8; 0x34],
    qdata0: Qdata0,
    qdata1: Qdata1,
    _reserved42: [u8; 0x1c],
    qdata1big: Qdata1big,
    _reserved43: [u8; 0x18],
    qdata0byte: Qdata0byte,
    qdata1byte: Qdata1byte,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Wide Arithmetic Configuration"]
    #[inline(always)]
    pub const fn wac(&self) -> &Wac {
        &self.wac
    }
    #[doc = "0x08 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Data Status Register"]
    #[inline(always)]
    pub const fn dstatus(&self) -> &Dstatus {
        &self.dstatus
    }
    #[doc = "0x18 - Control Status Register"]
    #[inline(always)]
    pub const fn cstatus(&self) -> &Cstatus {
        &self.cstatus
    }
    #[doc = "0x20 - KEY Register Access"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x24 - KEY Buffer Register Access"]
    #[inline(always)]
    pub const fn keybuf(&self) -> &Keybuf {
        &self.keybuf
    }
    #[doc = "0x30 - Sequence Control"]
    #[inline(always)]
    pub const fn seqctrl(&self) -> &Seqctrl {
        &self.seqctrl
    }
    #[doc = "0x34 - Sequence Control B"]
    #[inline(always)]
    pub const fn seqctrlb(&self) -> &Seqctrlb {
        &self.seqctrlb
    }
    #[doc = "0x40 - AES Interrupt Flags"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x44 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x4c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x50 - Sequence Register 0"]
    #[inline(always)]
    pub const fn seq0(&self) -> &Seq0 {
        &self.seq0
    }
    #[doc = "0x54 - Sequence Register 1"]
    #[inline(always)]
    pub const fn seq1(&self) -> &Seq1 {
        &self.seq1
    }
    #[doc = "0x58 - Sequence Register 2"]
    #[inline(always)]
    pub const fn seq2(&self) -> &Seq2 {
        &self.seq2
    }
    #[doc = "0x5c - Sequence Register 3"]
    #[inline(always)]
    pub const fn seq3(&self) -> &Seq3 {
        &self.seq3
    }
    #[doc = "0x60 - Sequence Register 4"]
    #[inline(always)]
    pub const fn seq4(&self) -> &Seq4 {
        &self.seq4
    }
    #[doc = "0x80 - DATA0 Register Access"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x84 - DATA1 Register Access"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x88 - DATA2 Register Access"]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x8c - DATA3 Register Access"]
    #[inline(always)]
    pub const fn data3(&self) -> &Data3 {
        &self.data3
    }
    #[doc = "0xa0 - DATA0XOR Register Access"]
    #[inline(always)]
    pub const fn data0xor(&self) -> &Data0xor {
        &self.data0xor
    }
    #[doc = "0xb0 - DATA0 Register Byte Access"]
    #[inline(always)]
    pub const fn data0byte(&self) -> &Data0byte {
        &self.data0byte
    }
    #[doc = "0xb4 - DATA1 Register Byte Access"]
    #[inline(always)]
    pub const fn data1byte(&self) -> &Data1byte {
        &self.data1byte
    }
    #[doc = "0xbc - DATA0 Register Byte XOR Access"]
    #[inline(always)]
    pub const fn data0xorbyte(&self) -> &Data0xorbyte {
        &self.data0xorbyte
    }
    #[doc = "0xc0 - DATA0 Register Byte 12 Access"]
    #[inline(always)]
    pub const fn data0byte12(&self) -> &Data0byte12 {
        &self.data0byte12
    }
    #[doc = "0xc4 - DATA0 Register Byte 13 Access"]
    #[inline(always)]
    pub const fn data0byte13(&self) -> &Data0byte13 {
        &self.data0byte13
    }
    #[doc = "0xc8 - DATA0 Register Byte 14 Access"]
    #[inline(always)]
    pub const fn data0byte14(&self) -> &Data0byte14 {
        &self.data0byte14
    }
    #[doc = "0xcc - DATA0 Register Byte 15 Access"]
    #[inline(always)]
    pub const fn data0byte15(&self) -> &Data0byte15 {
        &self.data0byte15
    }
    #[doc = "0x100 - DDATA0 Register Access"]
    #[inline(always)]
    pub const fn ddata0(&self) -> &Ddata0 {
        &self.ddata0
    }
    #[doc = "0x104 - DDATA1 Register Access"]
    #[inline(always)]
    pub const fn ddata1(&self) -> &Ddata1 {
        &self.ddata1
    }
    #[doc = "0x108 - DDATA2 Register Access"]
    #[inline(always)]
    pub const fn ddata2(&self) -> &Ddata2 {
        &self.ddata2
    }
    #[doc = "0x10c - DDATA3 Register Access"]
    #[inline(always)]
    pub const fn ddata3(&self) -> &Ddata3 {
        &self.ddata3
    }
    #[doc = "0x110 - DDATA4 Register Access"]
    #[inline(always)]
    pub const fn ddata4(&self) -> &Ddata4 {
        &self.ddata4
    }
    #[doc = "0x130 - DDATA0 Register Big Endian Access"]
    #[inline(always)]
    pub const fn ddata0big(&self) -> &Ddata0big {
        &self.ddata0big
    }
    #[doc = "0x140 - DDATA0 Register Byte Access"]
    #[inline(always)]
    pub const fn ddata0byte(&self) -> &Ddata0byte {
        &self.ddata0byte
    }
    #[doc = "0x144 - DDATA1 Register Byte Access"]
    #[inline(always)]
    pub const fn ddata1byte(&self) -> &Ddata1byte {
        &self.ddata1byte
    }
    #[doc = "0x148 - DDATA0 Register Byte 32 Access"]
    #[inline(always)]
    pub const fn ddata0byte32(&self) -> &Ddata0byte32 {
        &self.ddata0byte32
    }
    #[doc = "0x180 - QDATA0 Register Access"]
    #[inline(always)]
    pub const fn qdata0(&self) -> &Qdata0 {
        &self.qdata0
    }
    #[doc = "0x184 - QDATA1 Register Access"]
    #[inline(always)]
    pub const fn qdata1(&self) -> &Qdata1 {
        &self.qdata1
    }
    #[doc = "0x1a4 - QDATA1 Register Big Endian Access"]
    #[inline(always)]
    pub const fn qdata1big(&self) -> &Qdata1big {
        &self.qdata1big
    }
    #[doc = "0x1c0 - QDATA0 Register Byte Access"]
    #[inline(always)]
    pub const fn qdata0byte(&self) -> &Qdata0byte {
        &self.qdata0byte
    }
    #[doc = "0x1c4 - QDATA1 Register Byte Access"]
    #[inline(always)]
    pub const fn qdata1byte(&self) -> &Qdata1byte {
        &self.qdata1byte
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "WAC (rw) register accessor: Wide Arithmetic Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wac`] module"]
#[doc(alias = "WAC")]
pub type Wac = crate::Reg<wac::WacSpec>;
#[doc = "Wide Arithmetic Configuration"]
pub mod wac;
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "DSTATUS (r) register accessor: Data Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstatus`] module"]
#[doc(alias = "DSTATUS")]
pub type Dstatus = crate::Reg<dstatus::DstatusSpec>;
#[doc = "Data Status Register"]
pub mod dstatus;
#[doc = "CSTATUS (r) register accessor: Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstatus`] module"]
#[doc(alias = "CSTATUS")]
pub type Cstatus = crate::Reg<cstatus::CstatusSpec>;
#[doc = "Control Status Register"]
pub mod cstatus;
#[doc = "KEY (rw) register accessor: KEY Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "KEY Register Access"]
pub mod key;
#[doc = "KEYBUF (rw) register accessor: KEY Buffer Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`keybuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keybuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@keybuf`] module"]
#[doc(alias = "KEYBUF")]
pub type Keybuf = crate::Reg<keybuf::KeybufSpec>;
#[doc = "KEY Buffer Register Access"]
pub mod keybuf;
#[doc = "SEQCTRL (rw) register accessor: Sequence Control\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl`] module"]
#[doc(alias = "SEQCTRL")]
pub type Seqctrl = crate::Reg<seqctrl::SeqctrlSpec>;
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "SEQCTRLB (rw) register accessor: Sequence Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrlb`] module"]
#[doc(alias = "SEQCTRLB")]
pub type Seqctrlb = crate::Reg<seqctrlb::SeqctrlbSpec>;
#[doc = "Sequence Control B"]
pub mod seqctrlb;
#[doc = "IF (r) register accessor: AES Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "AES Interrupt Flags"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`] module"]
#[doc(alias = "IFS")]
pub type Ifs = crate::Reg<ifs::IfsSpec>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`] module"]
#[doc(alias = "IFC")]
pub type Ifc = crate::Reg<ifc::IfcSpec>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "SEQ0 (rw) register accessor: Sequence Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`seq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq0`] module"]
#[doc(alias = "SEQ0")]
pub type Seq0 = crate::Reg<seq0::Seq0Spec>;
#[doc = "Sequence Register 0"]
pub mod seq0;
#[doc = "SEQ1 (rw) register accessor: Sequence Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`seq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq1`] module"]
#[doc(alias = "SEQ1")]
pub type Seq1 = crate::Reg<seq1::Seq1Spec>;
#[doc = "Sequence Register 1"]
pub mod seq1;
#[doc = "SEQ2 (rw) register accessor: Sequence Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`seq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq2`] module"]
#[doc(alias = "SEQ2")]
pub type Seq2 = crate::Reg<seq2::Seq2Spec>;
#[doc = "Sequence Register 2"]
pub mod seq2;
#[doc = "SEQ3 (rw) register accessor: Sequence Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`seq3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq3`] module"]
#[doc(alias = "SEQ3")]
pub type Seq3 = crate::Reg<seq3::Seq3Spec>;
#[doc = "Sequence Register 3"]
pub mod seq3;
#[doc = "SEQ4 (rw) register accessor: Sequence Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`seq4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq4`] module"]
#[doc(alias = "SEQ4")]
pub type Seq4 = crate::Reg<seq4::Seq4Spec>;
#[doc = "Sequence Register 4"]
pub mod seq4;
#[doc = "DATA0 (rw) register accessor: DATA0 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data0`] module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "DATA0 Register Access"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: DATA1 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data1`] module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "DATA1 Register Access"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: DATA2 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data2`] module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "DATA2 Register Access"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: DATA3 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data3`] module"]
#[doc(alias = "DATA3")]
pub type Data3 = crate::Reg<data3::Data3Spec>;
#[doc = "DATA3 Register Access"]
pub mod data3;
#[doc = "DATA0XOR (rw) register accessor: DATA0XOR Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0xor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0xor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data0xor`] module"]
#[doc(alias = "DATA0XOR")]
pub type Data0xor = crate::Reg<data0xor::Data0xorSpec>;
#[doc = "DATA0XOR Register Access"]
pub mod data0xor;
#[doc = "DATA0BYTE (rw) register accessor: DATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data0byte`] module"]
#[doc(alias = "DATA0BYTE")]
pub type Data0byte = crate::Reg<data0byte::Data0byteSpec>;
#[doc = "DATA0 Register Byte Access"]
pub mod data0byte;
#[doc = "DATA1BYTE (rw) register accessor: DATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data1byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data1byte`] module"]
#[doc(alias = "DATA1BYTE")]
pub type Data1byte = crate::Reg<data1byte::Data1byteSpec>;
#[doc = "DATA1 Register Byte Access"]
pub mod data1byte;
#[doc = "DATA0XORBYTE (rw) register accessor: DATA0 Register Byte XOR Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0xorbyte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0xorbyte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@data0xorbyte`] module"]
#[doc(alias = "DATA0XORBYTE")]
pub type Data0xorbyte = crate::Reg<data0xorbyte::Data0xorbyteSpec>;
#[doc = "DATA0 Register Byte XOR Access"]
pub mod data0xorbyte;
#[doc = "DATA0BYTE12 (rw) register accessor: DATA0 Register Byte 12 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0byte12`] module"]
#[doc(alias = "DATA0BYTE12")]
pub type Data0byte12 = crate::Reg<data0byte12::Data0byte12Spec>;
#[doc = "DATA0 Register Byte 12 Access"]
pub mod data0byte12;
#[doc = "DATA0BYTE13 (rw) register accessor: DATA0 Register Byte 13 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0byte13`] module"]
#[doc(alias = "DATA0BYTE13")]
pub type Data0byte13 = crate::Reg<data0byte13::Data0byte13Spec>;
#[doc = "DATA0 Register Byte 13 Access"]
pub mod data0byte13;
#[doc = "DATA0BYTE14 (rw) register accessor: DATA0 Register Byte 14 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0byte14`] module"]
#[doc(alias = "DATA0BYTE14")]
pub type Data0byte14 = crate::Reg<data0byte14::Data0byte14Spec>;
#[doc = "DATA0 Register Byte 14 Access"]
pub mod data0byte14;
#[doc = "DATA0BYTE15 (rw) register accessor: DATA0 Register Byte 15 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`data0byte15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0byte15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0byte15`] module"]
#[doc(alias = "DATA0BYTE15")]
pub type Data0byte15 = crate::Reg<data0byte15::Data0byte15Spec>;
#[doc = "DATA0 Register Byte 15 Access"]
pub mod data0byte15;
#[doc = "DDATA0 (rw) register accessor: DDATA0 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata0`] module"]
#[doc(alias = "DDATA0")]
pub type Ddata0 = crate::Reg<ddata0::Ddata0Spec>;
#[doc = "DDATA0 Register Access"]
pub mod ddata0;
#[doc = "DDATA1 (rw) register accessor: DDATA1 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata1`] module"]
#[doc(alias = "DDATA1")]
pub type Ddata1 = crate::Reg<ddata1::Ddata1Spec>;
#[doc = "DDATA1 Register Access"]
pub mod ddata1;
#[doc = "DDATA2 (rw) register accessor: DDATA2 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata2`] module"]
#[doc(alias = "DDATA2")]
pub type Ddata2 = crate::Reg<ddata2::Ddata2Spec>;
#[doc = "DDATA2 Register Access"]
pub mod ddata2;
#[doc = "DDATA3 (rw) register accessor: DDATA3 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata3`] module"]
#[doc(alias = "DDATA3")]
pub type Ddata3 = crate::Reg<ddata3::Ddata3Spec>;
#[doc = "DDATA3 Register Access"]
pub mod ddata3;
#[doc = "DDATA4 (rw) register accessor: DDATA4 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata4`] module"]
#[doc(alias = "DDATA4")]
pub type Ddata4 = crate::Reg<ddata4::Ddata4Spec>;
#[doc = "DDATA4 Register Access"]
pub mod ddata4;
#[doc = "DDATA0BIG (rw) register accessor: DDATA0 Register Big Endian Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0big::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0big::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata0big`] module"]
#[doc(alias = "DDATA0BIG")]
pub type Ddata0big = crate::Reg<ddata0big::Ddata0bigSpec>;
#[doc = "DDATA0 Register Big Endian Access"]
pub mod ddata0big;
#[doc = "DDATA0BYTE (rw) register accessor: DDATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata0byte`] module"]
#[doc(alias = "DDATA0BYTE")]
pub type Ddata0byte = crate::Reg<ddata0byte::Ddata0byteSpec>;
#[doc = "DDATA0 Register Byte Access"]
pub mod ddata0byte;
#[doc = "DDATA1BYTE (rw) register accessor: DDATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata1byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata1byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@ddata1byte`] module"]
#[doc(alias = "DDATA1BYTE")]
pub type Ddata1byte = crate::Reg<ddata1byte::Ddata1byteSpec>;
#[doc = "DDATA1 Register Byte Access"]
pub mod ddata1byte;
#[doc = "DDATA0BYTE32 (rw) register accessor: DDATA0 Register Byte 32 Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0byte32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0byte32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddata0byte32`] module"]
#[doc(alias = "DDATA0BYTE32")]
pub type Ddata0byte32 = crate::Reg<ddata0byte32::Ddata0byte32Spec>;
#[doc = "DDATA0 Register Byte 32 Access"]
pub mod ddata0byte32;
#[doc = "QDATA0 (rw) register accessor: QDATA0 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@qdata0`] module"]
#[doc(alias = "QDATA0")]
pub type Qdata0 = crate::Reg<qdata0::Qdata0Spec>;
#[doc = "QDATA0 Register Access"]
pub mod qdata0;
#[doc = "QDATA1 (rw) register accessor: QDATA1 Register Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@qdata1`] module"]
#[doc(alias = "QDATA1")]
pub type Qdata1 = crate::Reg<qdata1::Qdata1Spec>;
#[doc = "QDATA1 Register Access"]
pub mod qdata1;
#[doc = "QDATA1BIG (rw) register accessor: QDATA1 Register Big Endian Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata1big::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1big::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@qdata1big`] module"]
#[doc(alias = "QDATA1BIG")]
pub type Qdata1big = crate::Reg<qdata1big::Qdata1bigSpec>;
#[doc = "QDATA1 Register Big Endian Access"]
pub mod qdata1big;
#[doc = "QDATA0BYTE (rw) register accessor: QDATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata0byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata0byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@qdata0byte`] module"]
#[doc(alias = "QDATA0BYTE")]
pub type Qdata0byte = crate::Reg<qdata0byte::Qdata0byteSpec>;
#[doc = "QDATA0 Register Byte Access"]
pub mod qdata0byte;
#[doc = "QDATA1BYTE (rw) register accessor: QDATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`qdata1byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdata1byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>\n\nFor information about available fields see [`mod@qdata1byte`] module"]
#[doc(alias = "QDATA1BYTE")]
pub type Qdata1byte = crate::Reg<qdata1byte::Qdata1byteSpec>;
#[doc = "QDATA1 Register Byte Access"]
pub mod qdata1byte;
