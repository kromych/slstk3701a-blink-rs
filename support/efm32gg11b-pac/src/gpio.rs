#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa_ctrl: PaCtrl,
    pa_model: PaModel,
    pa_modeh: PaModeh,
    pa_dout: PaDout,
    _reserved4: [u8; 0x08],
    pa_douttgl: PaDouttgl,
    pa_din: PaDin,
    pa_pinlockn: PaPinlockn,
    _reserved7: [u8; 0x04],
    pa_ovtdis: PaOvtdis,
    _reserved8: [u8; 0x04],
    pb_ctrl: PbCtrl,
    pb_model: PbModel,
    pb_modeh: PbModeh,
    pb_dout: PbDout,
    _reserved12: [u8; 0x08],
    pb_douttgl: PbDouttgl,
    pb_din: PbDin,
    pb_pinlockn: PbPinlockn,
    _reserved15: [u8; 0x04],
    pb_ovtdis: PbOvtdis,
    _reserved16: [u8; 0x04],
    pc_ctrl: PcCtrl,
    pc_model: PcModel,
    pc_modeh: PcModeh,
    pc_dout: PcDout,
    _reserved20: [u8; 0x08],
    pc_douttgl: PcDouttgl,
    pc_din: PcDin,
    pc_pinlockn: PcPinlockn,
    _reserved23: [u8; 0x04],
    pc_ovtdis: PcOvtdis,
    _reserved24: [u8; 0x04],
    pd_ctrl: PdCtrl,
    pd_model: PdModel,
    pd_modeh: PdModeh,
    pd_dout: PdDout,
    _reserved28: [u8; 0x08],
    pd_douttgl: PdDouttgl,
    pd_din: PdDin,
    pd_pinlockn: PdPinlockn,
    _reserved31: [u8; 0x04],
    pd_ovtdis: PdOvtdis,
    _reserved32: [u8; 0x04],
    pe_ctrl: PeCtrl,
    pe_model: PeModel,
    pe_modeh: PeModeh,
    pe_dout: PeDout,
    _reserved36: [u8; 0x08],
    pe_douttgl: PeDouttgl,
    pe_din: PeDin,
    pe_pinlockn: PePinlockn,
    _reserved39: [u8; 0x04],
    pe_ovtdis: PeOvtdis,
    _reserved40: [u8; 0x04],
    pf_ctrl: PfCtrl,
    pf_model: PfModel,
    pf_modeh: PfModeh,
    pf_dout: PfDout,
    _reserved44: [u8; 0x08],
    pf_douttgl: PfDouttgl,
    pf_din: PfDin,
    pf_pinlockn: PfPinlockn,
    _reserved47: [u8; 0x04],
    pf_ovtdis: PfOvtdis,
    _reserved48: [u8; 0x04],
    pg_ctrl: PgCtrl,
    pg_model: PgModel,
    pg_modeh: PgModeh,
    pg_dout: PgDout,
    _reserved52: [u8; 0x08],
    pg_douttgl: PgDouttgl,
    pg_din: PgDin,
    pg_pinlockn: PgPinlockn,
    _reserved55: [u8; 0x04],
    pg_ovtdis: PgOvtdis,
    _reserved56: [u8; 0x04],
    ph_ctrl: PhCtrl,
    ph_model: PhModel,
    ph_modeh: PhModeh,
    ph_dout: PhDout,
    _reserved60: [u8; 0x08],
    ph_douttgl: PhDouttgl,
    ph_din: PhDin,
    ph_pinlockn: PhPinlockn,
    _reserved63: [u8; 0x04],
    ph_ovtdis: PhOvtdis,
    _reserved64: [u8; 0x04],
    pi_ctrl: PiCtrl,
    pi_model: PiModel,
    pi_modeh: PiModeh,
    pi_dout: PiDout,
    _reserved68: [u8; 0x08],
    pi_douttgl: PiDouttgl,
    pi_din: PiDin,
    pi_pinlockn: PiPinlockn,
    _reserved71: [u8; 0x04],
    pi_ovtdis: PiOvtdis,
    _reserved72: [u8; 0x04],
    pj_ctrl: PjCtrl,
    pj_model: PjModel,
    pj_modeh: PjModeh,
    pj_dout: PjDout,
    _reserved76: [u8; 0x08],
    pj_douttgl: PjDouttgl,
    pj_din: PjDin,
    pj_pinlockn: PjPinlockn,
    _reserved79: [u8; 0x04],
    pj_ovtdis: PjOvtdis,
    _reserved80: [u8; 0x04],
    pk_ctrl: PkCtrl,
    pk_model: PkModel,
    pk_modeh: PkModeh,
    pk_dout: PkDout,
    _reserved84: [u8; 0x08],
    pk_douttgl: PkDouttgl,
    pk_din: PkDin,
    pk_pinlockn: PkPinlockn,
    _reserved87: [u8; 0x04],
    pk_ovtdis: PkOvtdis,
    _reserved88: [u8; 0x04],
    pl_ctrl: PlCtrl,
    pl_model: PlModel,
    pl_modeh: PlModeh,
    pl_dout: PlDout,
    _reserved92: [u8; 0x08],
    pl_douttgl: PlDouttgl,
    pl_din: PlDin,
    pl_pinlockn: PlPinlockn,
    _reserved95: [u8; 0x04],
    pl_ovtdis: PlOvtdis,
    _reserved96: [u8; 0x01c4],
    extipsell: Extipsell,
    extipselh: Extipselh,
    extipinsell: Extipinsell,
    extipinselh: Extipinselh,
    extirise: Extirise,
    extifall: Extifall,
    extilevel: Extilevel,
    if_: If,
    ifs: Ifs,
    ifc: Ifc,
    ien: Ien,
    em4wuen: Em4wuen,
    _reserved108: [u8; 0x10],
    routepen: Routepen,
    routeloc0: Routeloc0,
    _reserved110: [u8; 0x08],
    insense: Insense,
    lock: Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    #[inline(always)]
    pub const fn pa_ctrl(&self) -> &PaCtrl {
        &self.pa_ctrl
    }
    #[doc = "0x04 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pa_model(&self) -> &PaModel {
        &self.pa_model
    }
    #[doc = "0x08 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pa_modeh(&self) -> &PaModeh {
        &self.pa_modeh
    }
    #[doc = "0x0c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pa_dout(&self) -> &PaDout {
        &self.pa_dout
    }
    #[doc = "0x18 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pa_douttgl(&self) -> &PaDouttgl {
        &self.pa_douttgl
    }
    #[doc = "0x1c - Port Data in Register"]
    #[inline(always)]
    pub const fn pa_din(&self) -> &PaDin {
        &self.pa_din
    }
    #[doc = "0x20 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pa_pinlockn(&self) -> &PaPinlockn {
        &self.pa_pinlockn
    }
    #[doc = "0x28 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pa_ovtdis(&self) -> &PaOvtdis {
        &self.pa_ovtdis
    }
    #[doc = "0x30 - Port Control Register"]
    #[inline(always)]
    pub const fn pb_ctrl(&self) -> &PbCtrl {
        &self.pb_ctrl
    }
    #[doc = "0x34 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pb_model(&self) -> &PbModel {
        &self.pb_model
    }
    #[doc = "0x38 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pb_modeh(&self) -> &PbModeh {
        &self.pb_modeh
    }
    #[doc = "0x3c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pb_dout(&self) -> &PbDout {
        &self.pb_dout
    }
    #[doc = "0x48 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pb_douttgl(&self) -> &PbDouttgl {
        &self.pb_douttgl
    }
    #[doc = "0x4c - Port Data in Register"]
    #[inline(always)]
    pub const fn pb_din(&self) -> &PbDin {
        &self.pb_din
    }
    #[doc = "0x50 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pb_pinlockn(&self) -> &PbPinlockn {
        &self.pb_pinlockn
    }
    #[doc = "0x58 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pb_ovtdis(&self) -> &PbOvtdis {
        &self.pb_ovtdis
    }
    #[doc = "0x60 - Port Control Register"]
    #[inline(always)]
    pub const fn pc_ctrl(&self) -> &PcCtrl {
        &self.pc_ctrl
    }
    #[doc = "0x64 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pc_model(&self) -> &PcModel {
        &self.pc_model
    }
    #[doc = "0x68 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pc_modeh(&self) -> &PcModeh {
        &self.pc_modeh
    }
    #[doc = "0x6c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pc_dout(&self) -> &PcDout {
        &self.pc_dout
    }
    #[doc = "0x78 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pc_douttgl(&self) -> &PcDouttgl {
        &self.pc_douttgl
    }
    #[doc = "0x7c - Port Data in Register"]
    #[inline(always)]
    pub const fn pc_din(&self) -> &PcDin {
        &self.pc_din
    }
    #[doc = "0x80 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pc_pinlockn(&self) -> &PcPinlockn {
        &self.pc_pinlockn
    }
    #[doc = "0x88 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pc_ovtdis(&self) -> &PcOvtdis {
        &self.pc_ovtdis
    }
    #[doc = "0x90 - Port Control Register"]
    #[inline(always)]
    pub const fn pd_ctrl(&self) -> &PdCtrl {
        &self.pd_ctrl
    }
    #[doc = "0x94 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pd_model(&self) -> &PdModel {
        &self.pd_model
    }
    #[doc = "0x98 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pd_modeh(&self) -> &PdModeh {
        &self.pd_modeh
    }
    #[doc = "0x9c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pd_dout(&self) -> &PdDout {
        &self.pd_dout
    }
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pd_douttgl(&self) -> &PdDouttgl {
        &self.pd_douttgl
    }
    #[doc = "0xac - Port Data in Register"]
    #[inline(always)]
    pub const fn pd_din(&self) -> &PdDin {
        &self.pd_din
    }
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pd_pinlockn(&self) -> &PdPinlockn {
        &self.pd_pinlockn
    }
    #[doc = "0xb8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pd_ovtdis(&self) -> &PdOvtdis {
        &self.pd_ovtdis
    }
    #[doc = "0xc0 - Port Control Register"]
    #[inline(always)]
    pub const fn pe_ctrl(&self) -> &PeCtrl {
        &self.pe_ctrl
    }
    #[doc = "0xc4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pe_model(&self) -> &PeModel {
        &self.pe_model
    }
    #[doc = "0xc8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pe_modeh(&self) -> &PeModeh {
        &self.pe_modeh
    }
    #[doc = "0xcc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pe_dout(&self) -> &PeDout {
        &self.pe_dout
    }
    #[doc = "0xd8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pe_douttgl(&self) -> &PeDouttgl {
        &self.pe_douttgl
    }
    #[doc = "0xdc - Port Data in Register"]
    #[inline(always)]
    pub const fn pe_din(&self) -> &PeDin {
        &self.pe_din
    }
    #[doc = "0xe0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pe_pinlockn(&self) -> &PePinlockn {
        &self.pe_pinlockn
    }
    #[doc = "0xe8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pe_ovtdis(&self) -> &PeOvtdis {
        &self.pe_ovtdis
    }
    #[doc = "0xf0 - Port Control Register"]
    #[inline(always)]
    pub const fn pf_ctrl(&self) -> &PfCtrl {
        &self.pf_ctrl
    }
    #[doc = "0xf4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pf_model(&self) -> &PfModel {
        &self.pf_model
    }
    #[doc = "0xf8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pf_modeh(&self) -> &PfModeh {
        &self.pf_modeh
    }
    #[doc = "0xfc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pf_dout(&self) -> &PfDout {
        &self.pf_dout
    }
    #[doc = "0x108 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pf_douttgl(&self) -> &PfDouttgl {
        &self.pf_douttgl
    }
    #[doc = "0x10c - Port Data in Register"]
    #[inline(always)]
    pub const fn pf_din(&self) -> &PfDin {
        &self.pf_din
    }
    #[doc = "0x110 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pf_pinlockn(&self) -> &PfPinlockn {
        &self.pf_pinlockn
    }
    #[doc = "0x118 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pf_ovtdis(&self) -> &PfOvtdis {
        &self.pf_ovtdis
    }
    #[doc = "0x120 - Port Control Register"]
    #[inline(always)]
    pub const fn pg_ctrl(&self) -> &PgCtrl {
        &self.pg_ctrl
    }
    #[doc = "0x124 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pg_model(&self) -> &PgModel {
        &self.pg_model
    }
    #[doc = "0x128 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pg_modeh(&self) -> &PgModeh {
        &self.pg_modeh
    }
    #[doc = "0x12c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pg_dout(&self) -> &PgDout {
        &self.pg_dout
    }
    #[doc = "0x138 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pg_douttgl(&self) -> &PgDouttgl {
        &self.pg_douttgl
    }
    #[doc = "0x13c - Port Data in Register"]
    #[inline(always)]
    pub const fn pg_din(&self) -> &PgDin {
        &self.pg_din
    }
    #[doc = "0x140 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pg_pinlockn(&self) -> &PgPinlockn {
        &self.pg_pinlockn
    }
    #[doc = "0x148 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pg_ovtdis(&self) -> &PgOvtdis {
        &self.pg_ovtdis
    }
    #[doc = "0x150 - Port Control Register"]
    #[inline(always)]
    pub const fn ph_ctrl(&self) -> &PhCtrl {
        &self.ph_ctrl
    }
    #[doc = "0x154 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn ph_model(&self) -> &PhModel {
        &self.ph_model
    }
    #[doc = "0x158 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn ph_modeh(&self) -> &PhModeh {
        &self.ph_modeh
    }
    #[doc = "0x15c - Port Data Out Register"]
    #[inline(always)]
    pub const fn ph_dout(&self) -> &PhDout {
        &self.ph_dout
    }
    #[doc = "0x168 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn ph_douttgl(&self) -> &PhDouttgl {
        &self.ph_douttgl
    }
    #[doc = "0x16c - Port Data in Register"]
    #[inline(always)]
    pub const fn ph_din(&self) -> &PhDin {
        &self.ph_din
    }
    #[doc = "0x170 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn ph_pinlockn(&self) -> &PhPinlockn {
        &self.ph_pinlockn
    }
    #[doc = "0x178 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn ph_ovtdis(&self) -> &PhOvtdis {
        &self.ph_ovtdis
    }
    #[doc = "0x180 - Port Control Register"]
    #[inline(always)]
    pub const fn pi_ctrl(&self) -> &PiCtrl {
        &self.pi_ctrl
    }
    #[doc = "0x184 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pi_model(&self) -> &PiModel {
        &self.pi_model
    }
    #[doc = "0x188 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pi_modeh(&self) -> &PiModeh {
        &self.pi_modeh
    }
    #[doc = "0x18c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pi_dout(&self) -> &PiDout {
        &self.pi_dout
    }
    #[doc = "0x198 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pi_douttgl(&self) -> &PiDouttgl {
        &self.pi_douttgl
    }
    #[doc = "0x19c - Port Data in Register"]
    #[inline(always)]
    pub const fn pi_din(&self) -> &PiDin {
        &self.pi_din
    }
    #[doc = "0x1a0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pi_pinlockn(&self) -> &PiPinlockn {
        &self.pi_pinlockn
    }
    #[doc = "0x1a8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pi_ovtdis(&self) -> &PiOvtdis {
        &self.pi_ovtdis
    }
    #[doc = "0x1b0 - Port Control Register"]
    #[inline(always)]
    pub const fn pj_ctrl(&self) -> &PjCtrl {
        &self.pj_ctrl
    }
    #[doc = "0x1b4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pj_model(&self) -> &PjModel {
        &self.pj_model
    }
    #[doc = "0x1b8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pj_modeh(&self) -> &PjModeh {
        &self.pj_modeh
    }
    #[doc = "0x1bc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pj_dout(&self) -> &PjDout {
        &self.pj_dout
    }
    #[doc = "0x1c8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pj_douttgl(&self) -> &PjDouttgl {
        &self.pj_douttgl
    }
    #[doc = "0x1cc - Port Data in Register"]
    #[inline(always)]
    pub const fn pj_din(&self) -> &PjDin {
        &self.pj_din
    }
    #[doc = "0x1d0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pj_pinlockn(&self) -> &PjPinlockn {
        &self.pj_pinlockn
    }
    #[doc = "0x1d8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pj_ovtdis(&self) -> &PjOvtdis {
        &self.pj_ovtdis
    }
    #[doc = "0x1e0 - Port Control Register"]
    #[inline(always)]
    pub const fn pk_ctrl(&self) -> &PkCtrl {
        &self.pk_ctrl
    }
    #[doc = "0x1e4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pk_model(&self) -> &PkModel {
        &self.pk_model
    }
    #[doc = "0x1e8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pk_modeh(&self) -> &PkModeh {
        &self.pk_modeh
    }
    #[doc = "0x1ec - Port Data Out Register"]
    #[inline(always)]
    pub const fn pk_dout(&self) -> &PkDout {
        &self.pk_dout
    }
    #[doc = "0x1f8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pk_douttgl(&self) -> &PkDouttgl {
        &self.pk_douttgl
    }
    #[doc = "0x1fc - Port Data in Register"]
    #[inline(always)]
    pub const fn pk_din(&self) -> &PkDin {
        &self.pk_din
    }
    #[doc = "0x200 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pk_pinlockn(&self) -> &PkPinlockn {
        &self.pk_pinlockn
    }
    #[doc = "0x208 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pk_ovtdis(&self) -> &PkOvtdis {
        &self.pk_ovtdis
    }
    #[doc = "0x210 - Port Control Register"]
    #[inline(always)]
    pub const fn pl_ctrl(&self) -> &PlCtrl {
        &self.pl_ctrl
    }
    #[doc = "0x214 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pl_model(&self) -> &PlModel {
        &self.pl_model
    }
    #[doc = "0x218 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pl_modeh(&self) -> &PlModeh {
        &self.pl_modeh
    }
    #[doc = "0x21c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pl_dout(&self) -> &PlDout {
        &self.pl_dout
    }
    #[doc = "0x228 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pl_douttgl(&self) -> &PlDouttgl {
        &self.pl_douttgl
    }
    #[doc = "0x22c - Port Data in Register"]
    #[inline(always)]
    pub const fn pl_din(&self) -> &PlDin {
        &self.pl_din
    }
    #[doc = "0x230 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pl_pinlockn(&self) -> &PlPinlockn {
        &self.pl_pinlockn
    }
    #[doc = "0x238 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pl_ovtdis(&self) -> &PlOvtdis {
        &self.pl_ovtdis
    }
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &Extipsell {
        &self.extipsell
    }
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &Extipselh {
        &self.extipselh
    }
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    #[inline(always)]
    pub const fn extipinsell(&self) -> &Extipinsell {
        &self.extipinsell
    }
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    #[inline(always)]
    pub const fn extipinselh(&self) -> &Extipinselh {
        &self.extipinselh
    }
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    #[inline(always)]
    pub const fn extirise(&self) -> &Extirise {
        &self.extirise
    }
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    #[inline(always)]
    pub const fn extifall(&self) -> &Extifall {
        &self.extifall
    }
    #[doc = "0x418 - External Interrupt Level Register"]
    #[inline(always)]
    pub const fn extilevel(&self) -> &Extilevel {
        &self.extilevel
    }
    #[doc = "0x41c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x420 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &Ifs {
        &self.ifs
    }
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &Ifc {
        &self.ifc
    }
    #[doc = "0x428 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &Routepen {
        &self.routepen
    }
    #[doc = "0x444 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &Routeloc0 {
        &self.routeloc0
    }
    #[doc = "0x450 - Input Sense Register"]
    #[inline(always)]
    pub const fn insense(&self) -> &Insense {
        &self.insense
    }
    #[doc = "0x454 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "PA_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ctrl`] module"]
#[doc(alias = "PA_CTRL")]
pub type PaCtrl = crate::Reg<pa_ctrl::PaCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_model`] module"]
#[doc(alias = "PA_MODEL")]
pub type PaModel = crate::Reg<pa_model::PaModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_modeh`] module"]
#[doc(alias = "PA_MODEH")]
pub type PaModeh = crate::Reg<pa_modeh::PaModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_dout`] module"]
#[doc(alias = "PA_DOUT")]
pub type PaDout = crate::Reg<pa_dout::PaDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_douttgl`] module"]
#[doc(alias = "PA_DOUTTGL")]
pub type PaDouttgl = crate::Reg<pa_douttgl::PaDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_din`] module"]
#[doc(alias = "PA_DIN")]
pub type PaDin = crate::Reg<pa_din::PaDinSpec>;
#[doc = "Port Data in Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pinlockn`] module"]
#[doc(alias = "PA_PINLOCKN")]
pub type PaPinlockn = crate::Reg<pa_pinlockn::PaPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PA_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ovtdis`] module"]
#[doc(alias = "PA_OVTDIS")]
pub type PaOvtdis = crate::Reg<pa_ovtdis::PaOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pa_ovtdis;
#[doc = "PB_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ctrl`] module"]
#[doc(alias = "PB_CTRL")]
pub type PbCtrl = crate::Reg<pb_ctrl::PbCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_model`] module"]
#[doc(alias = "PB_MODEL")]
pub type PbModel = crate::Reg<pb_model::PbModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_modeh`] module"]
#[doc(alias = "PB_MODEH")]
pub type PbModeh = crate::Reg<pb_modeh::PbModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_dout`] module"]
#[doc(alias = "PB_DOUT")]
pub type PbDout = crate::Reg<pb_dout::PbDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_douttgl`] module"]
#[doc(alias = "PB_DOUTTGL")]
pub type PbDouttgl = crate::Reg<pb_douttgl::PbDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_din`] module"]
#[doc(alias = "PB_DIN")]
pub type PbDin = crate::Reg<pb_din::PbDinSpec>;
#[doc = "Port Data in Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pinlockn`] module"]
#[doc(alias = "PB_PINLOCKN")]
pub type PbPinlockn = crate::Reg<pb_pinlockn::PbPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PB_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ovtdis`] module"]
#[doc(alias = "PB_OVTDIS")]
pub type PbOvtdis = crate::Reg<pb_ovtdis::PbOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pb_ovtdis;
#[doc = "PC_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ctrl`] module"]
#[doc(alias = "PC_CTRL")]
pub type PcCtrl = crate::Reg<pc_ctrl::PcCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_model`] module"]
#[doc(alias = "PC_MODEL")]
pub type PcModel = crate::Reg<pc_model::PcModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_modeh`] module"]
#[doc(alias = "PC_MODEH")]
pub type PcModeh = crate::Reg<pc_modeh::PcModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_dout`] module"]
#[doc(alias = "PC_DOUT")]
pub type PcDout = crate::Reg<pc_dout::PcDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_douttgl`] module"]
#[doc(alias = "PC_DOUTTGL")]
pub type PcDouttgl = crate::Reg<pc_douttgl::PcDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_din`] module"]
#[doc(alias = "PC_DIN")]
pub type PcDin = crate::Reg<pc_din::PcDinSpec>;
#[doc = "Port Data in Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pinlockn`] module"]
#[doc(alias = "PC_PINLOCKN")]
pub type PcPinlockn = crate::Reg<pc_pinlockn::PcPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PC_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ovtdis`] module"]
#[doc(alias = "PC_OVTDIS")]
pub type PcOvtdis = crate::Reg<pc_ovtdis::PcOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pc_ovtdis;
#[doc = "PD_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ctrl`] module"]
#[doc(alias = "PD_CTRL")]
pub type PdCtrl = crate::Reg<pd_ctrl::PdCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_model`] module"]
#[doc(alias = "PD_MODEL")]
pub type PdModel = crate::Reg<pd_model::PdModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_modeh`] module"]
#[doc(alias = "PD_MODEH")]
pub type PdModeh = crate::Reg<pd_modeh::PdModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_dout`] module"]
#[doc(alias = "PD_DOUT")]
pub type PdDout = crate::Reg<pd_dout::PdDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_douttgl`] module"]
#[doc(alias = "PD_DOUTTGL")]
pub type PdDouttgl = crate::Reg<pd_douttgl::PdDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_din`] module"]
#[doc(alias = "PD_DIN")]
pub type PdDin = crate::Reg<pd_din::PdDinSpec>;
#[doc = "Port Data in Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pinlockn`] module"]
#[doc(alias = "PD_PINLOCKN")]
pub type PdPinlockn = crate::Reg<pd_pinlockn::PdPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PD_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ovtdis`] module"]
#[doc(alias = "PD_OVTDIS")]
pub type PdOvtdis = crate::Reg<pd_ovtdis::PdOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pd_ovtdis;
#[doc = "PE_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ctrl`] module"]
#[doc(alias = "PE_CTRL")]
pub type PeCtrl = crate::Reg<pe_ctrl::PeCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_model`] module"]
#[doc(alias = "PE_MODEL")]
pub type PeModel = crate::Reg<pe_model::PeModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_modeh`] module"]
#[doc(alias = "PE_MODEH")]
pub type PeModeh = crate::Reg<pe_modeh::PeModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_dout`] module"]
#[doc(alias = "PE_DOUT")]
pub type PeDout = crate::Reg<pe_dout::PeDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_douttgl`] module"]
#[doc(alias = "PE_DOUTTGL")]
pub type PeDouttgl = crate::Reg<pe_douttgl::PeDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_din`] module"]
#[doc(alias = "PE_DIN")]
pub type PeDin = crate::Reg<pe_din::PeDinSpec>;
#[doc = "Port Data in Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pinlockn`] module"]
#[doc(alias = "PE_PINLOCKN")]
pub type PePinlockn = crate::Reg<pe_pinlockn::PePinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PE_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ovtdis`] module"]
#[doc(alias = "PE_OVTDIS")]
pub type PeOvtdis = crate::Reg<pe_ovtdis::PeOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pe_ovtdis;
#[doc = "PF_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ctrl`] module"]
#[doc(alias = "PF_CTRL")]
pub type PfCtrl = crate::Reg<pf_ctrl::PfCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_model`] module"]
#[doc(alias = "PF_MODEL")]
pub type PfModel = crate::Reg<pf_model::PfModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_modeh`] module"]
#[doc(alias = "PF_MODEH")]
pub type PfModeh = crate::Reg<pf_modeh::PfModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_dout`] module"]
#[doc(alias = "PF_DOUT")]
pub type PfDout = crate::Reg<pf_dout::PfDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_douttgl`] module"]
#[doc(alias = "PF_DOUTTGL")]
pub type PfDouttgl = crate::Reg<pf_douttgl::PfDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_din`] module"]
#[doc(alias = "PF_DIN")]
pub type PfDin = crate::Reg<pf_din::PfDinSpec>;
#[doc = "Port Data in Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pinlockn`] module"]
#[doc(alias = "PF_PINLOCKN")]
pub type PfPinlockn = crate::Reg<pf_pinlockn::PfPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "PF_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ovtdis`] module"]
#[doc(alias = "PF_OVTDIS")]
pub type PfOvtdis = crate::Reg<pf_ovtdis::PfOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pf_ovtdis;
#[doc = "PG_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_ctrl`] module"]
#[doc(alias = "PG_CTRL")]
pub type PgCtrl = crate::Reg<pg_ctrl::PgCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pg_ctrl;
#[doc = "PG_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_model`] module"]
#[doc(alias = "PG_MODEL")]
pub type PgModel = crate::Reg<pg_model::PgModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pg_model;
#[doc = "PG_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_modeh`] module"]
#[doc(alias = "PG_MODEH")]
pub type PgModeh = crate::Reg<pg_modeh::PgModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pg_modeh;
#[doc = "PG_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_dout`] module"]
#[doc(alias = "PG_DOUT")]
pub type PgDout = crate::Reg<pg_dout::PgDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pg_dout;
#[doc = "PG_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_douttgl`] module"]
#[doc(alias = "PG_DOUTTGL")]
pub type PgDouttgl = crate::Reg<pg_douttgl::PgDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pg_douttgl;
#[doc = "PG_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_din`] module"]
#[doc(alias = "PG_DIN")]
pub type PgDin = crate::Reg<pg_din::PgDinSpec>;
#[doc = "Port Data in Register"]
pub mod pg_din;
#[doc = "PG_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_pinlockn`] module"]
#[doc(alias = "PG_PINLOCKN")]
pub type PgPinlockn = crate::Reg<pg_pinlockn::PgPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pg_pinlockn;
#[doc = "PG_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_ovtdis`] module"]
#[doc(alias = "PG_OVTDIS")]
pub type PgOvtdis = crate::Reg<pg_ovtdis::PgOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pg_ovtdis;
#[doc = "PH_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_ctrl`] module"]
#[doc(alias = "PH_CTRL")]
pub type PhCtrl = crate::Reg<ph_ctrl::PhCtrlSpec>;
#[doc = "Port Control Register"]
pub mod ph_ctrl;
#[doc = "PH_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_model`] module"]
#[doc(alias = "PH_MODEL")]
pub type PhModel = crate::Reg<ph_model::PhModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod ph_model;
#[doc = "PH_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_modeh`] module"]
#[doc(alias = "PH_MODEH")]
pub type PhModeh = crate::Reg<ph_modeh::PhModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod ph_modeh;
#[doc = "PH_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_dout`] module"]
#[doc(alias = "PH_DOUT")]
pub type PhDout = crate::Reg<ph_dout::PhDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod ph_dout;
#[doc = "PH_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_douttgl`] module"]
#[doc(alias = "PH_DOUTTGL")]
pub type PhDouttgl = crate::Reg<ph_douttgl::PhDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod ph_douttgl;
#[doc = "PH_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_din`] module"]
#[doc(alias = "PH_DIN")]
pub type PhDin = crate::Reg<ph_din::PhDinSpec>;
#[doc = "Port Data in Register"]
pub mod ph_din;
#[doc = "PH_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_pinlockn`] module"]
#[doc(alias = "PH_PINLOCKN")]
pub type PhPinlockn = crate::Reg<ph_pinlockn::PhPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod ph_pinlockn;
#[doc = "PH_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ph_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_ovtdis`] module"]
#[doc(alias = "PH_OVTDIS")]
pub type PhOvtdis = crate::Reg<ph_ovtdis::PhOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod ph_ovtdis;
#[doc = "PI_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_ctrl`] module"]
#[doc(alias = "PI_CTRL")]
pub type PiCtrl = crate::Reg<pi_ctrl::PiCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pi_ctrl;
#[doc = "PI_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_model`] module"]
#[doc(alias = "PI_MODEL")]
pub type PiModel = crate::Reg<pi_model::PiModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pi_model;
#[doc = "PI_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_modeh`] module"]
#[doc(alias = "PI_MODEH")]
pub type PiModeh = crate::Reg<pi_modeh::PiModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pi_modeh;
#[doc = "PI_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_dout`] module"]
#[doc(alias = "PI_DOUT")]
pub type PiDout = crate::Reg<pi_dout::PiDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pi_dout;
#[doc = "PI_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_douttgl`] module"]
#[doc(alias = "PI_DOUTTGL")]
pub type PiDouttgl = crate::Reg<pi_douttgl::PiDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pi_douttgl;
#[doc = "PI_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_din`] module"]
#[doc(alias = "PI_DIN")]
pub type PiDin = crate::Reg<pi_din::PiDinSpec>;
#[doc = "Port Data in Register"]
pub mod pi_din;
#[doc = "PI_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_pinlockn`] module"]
#[doc(alias = "PI_PINLOCKN")]
pub type PiPinlockn = crate::Reg<pi_pinlockn::PiPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pi_pinlockn;
#[doc = "PI_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_ovtdis`] module"]
#[doc(alias = "PI_OVTDIS")]
pub type PiOvtdis = crate::Reg<pi_ovtdis::PiOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pi_ovtdis;
#[doc = "PJ_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_ctrl`] module"]
#[doc(alias = "PJ_CTRL")]
pub type PjCtrl = crate::Reg<pj_ctrl::PjCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pj_ctrl;
#[doc = "PJ_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_model`] module"]
#[doc(alias = "PJ_MODEL")]
pub type PjModel = crate::Reg<pj_model::PjModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pj_model;
#[doc = "PJ_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_modeh`] module"]
#[doc(alias = "PJ_MODEH")]
pub type PjModeh = crate::Reg<pj_modeh::PjModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pj_modeh;
#[doc = "PJ_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_dout`] module"]
#[doc(alias = "PJ_DOUT")]
pub type PjDout = crate::Reg<pj_dout::PjDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pj_dout;
#[doc = "PJ_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_douttgl`] module"]
#[doc(alias = "PJ_DOUTTGL")]
pub type PjDouttgl = crate::Reg<pj_douttgl::PjDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pj_douttgl;
#[doc = "PJ_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_din`] module"]
#[doc(alias = "PJ_DIN")]
pub type PjDin = crate::Reg<pj_din::PjDinSpec>;
#[doc = "Port Data in Register"]
pub mod pj_din;
#[doc = "PJ_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_pinlockn`] module"]
#[doc(alias = "PJ_PINLOCKN")]
pub type PjPinlockn = crate::Reg<pj_pinlockn::PjPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pj_pinlockn;
#[doc = "PJ_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_ovtdis`] module"]
#[doc(alias = "PJ_OVTDIS")]
pub type PjOvtdis = crate::Reg<pj_ovtdis::PjOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pj_ovtdis;
#[doc = "PK_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_ctrl`] module"]
#[doc(alias = "PK_CTRL")]
pub type PkCtrl = crate::Reg<pk_ctrl::PkCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pk_ctrl;
#[doc = "PK_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_model`] module"]
#[doc(alias = "PK_MODEL")]
pub type PkModel = crate::Reg<pk_model::PkModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pk_model;
#[doc = "PK_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_modeh`] module"]
#[doc(alias = "PK_MODEH")]
pub type PkModeh = crate::Reg<pk_modeh::PkModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pk_modeh;
#[doc = "PK_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_dout`] module"]
#[doc(alias = "PK_DOUT")]
pub type PkDout = crate::Reg<pk_dout::PkDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pk_dout;
#[doc = "PK_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_douttgl`] module"]
#[doc(alias = "PK_DOUTTGL")]
pub type PkDouttgl = crate::Reg<pk_douttgl::PkDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pk_douttgl;
#[doc = "PK_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_din`] module"]
#[doc(alias = "PK_DIN")]
pub type PkDin = crate::Reg<pk_din::PkDinSpec>;
#[doc = "Port Data in Register"]
pub mod pk_din;
#[doc = "PK_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_pinlockn`] module"]
#[doc(alias = "PK_PINLOCKN")]
pub type PkPinlockn = crate::Reg<pk_pinlockn::PkPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pk_pinlockn;
#[doc = "PK_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_ovtdis`] module"]
#[doc(alias = "PK_OVTDIS")]
pub type PkOvtdis = crate::Reg<pk_ovtdis::PkOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pk_ovtdis;
#[doc = "PL_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_ctrl`] module"]
#[doc(alias = "PL_CTRL")]
pub type PlCtrl = crate::Reg<pl_ctrl::PlCtrlSpec>;
#[doc = "Port Control Register"]
pub mod pl_ctrl;
#[doc = "PL_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_model`] module"]
#[doc(alias = "PL_MODEL")]
pub type PlModel = crate::Reg<pl_model::PlModelSpec>;
#[doc = "Port Pin Mode Low Register"]
pub mod pl_model;
#[doc = "PL_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_modeh`] module"]
#[doc(alias = "PL_MODEH")]
pub type PlModeh = crate::Reg<pl_modeh::PlModehSpec>;
#[doc = "Port Pin Mode High Register"]
pub mod pl_modeh;
#[doc = "PL_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_dout`] module"]
#[doc(alias = "PL_DOUT")]
pub type PlDout = crate::Reg<pl_dout::PlDoutSpec>;
#[doc = "Port Data Out Register"]
pub mod pl_dout;
#[doc = "PL_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_douttgl`] module"]
#[doc(alias = "PL_DOUTTGL")]
pub type PlDouttgl = crate::Reg<pl_douttgl::PlDouttglSpec>;
#[doc = "Port Data Out Toggle Register"]
pub mod pl_douttgl;
#[doc = "PL_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_din`] module"]
#[doc(alias = "PL_DIN")]
pub type PlDin = crate::Reg<pl_din::PlDinSpec>;
#[doc = "Port Data in Register"]
pub mod pl_din;
#[doc = "PL_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_pinlockn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_pinlockn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_pinlockn`] module"]
#[doc(alias = "PL_PINLOCKN")]
pub type PlPinlockn = crate::Reg<pl_pinlockn::PlPinlocknSpec>;
#[doc = "Port Unlocked Pins Register"]
pub mod pl_pinlockn;
#[doc = "PL_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_ovtdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl_ovtdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_ovtdis`] module"]
#[doc(alias = "PL_OVTDIS")]
pub type PlOvtdis = crate::Reg<pl_ovtdis::PlOvtdisSpec>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pl_ovtdis;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`] module"]
#[doc(alias = "EXTIPSELL")]
pub type Extipsell = crate::Reg<extipsell::ExtipsellSpec>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External Interrupt Port Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`] module"]
#[doc(alias = "EXTIPSELH")]
pub type Extipselh = crate::Reg<extipselh::ExtipselhSpec>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: External Interrupt Pin Select Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinsell`] module"]
#[doc(alias = "EXTIPINSELL")]
pub type Extipinsell = crate::Reg<extipinsell::ExtipinsellSpec>;
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: External Interrupt Pin Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinselh`] module"]
#[doc(alias = "EXTIPINSELH")]
pub type Extipinselh = crate::Reg<extipinselh::ExtipinselhSpec>;
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`] module"]
#[doc(alias = "EXTIRISE")]
pub type Extirise = crate::Reg<extirise::ExtiriseSpec>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`] module"]
#[doc(alias = "EXTIFALL")]
pub type Extifall = crate::Reg<extifall::ExtifallSpec>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "EXTILEVEL (rw) register accessor: External Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extilevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extilevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extilevel`] module"]
#[doc(alias = "EXTILEVEL")]
pub type Extilevel = crate::Reg<extilevel::ExtilevelSpec>;
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag Register"]
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
#[doc = "EM4WUEN (rw) register accessor: EM4 Wake Up Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`] module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::Em4wuenSpec>;
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`] module"]
#[doc(alias = "ROUTEPEN")]
pub type Routepen = crate::Reg<routepen::RoutepenSpec>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`] module"]
#[doc(alias = "ROUTELOC0")]
pub type Routeloc0 = crate::Reg<routeloc0::Routeloc0Spec>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "INSENSE (rw) register accessor: Input Sense Register\n\nYou can [`read`](crate::Reg::read) this register and get [`insense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insense`] module"]
#[doc(alias = "INSENSE")]
pub type Insense = crate::Reg<insense::InsenseSpec>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Configuration Lock Register"]
pub mod lock;
