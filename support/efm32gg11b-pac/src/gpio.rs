#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pa_ctrl: PA_CTRL,
    pa_model: PA_MODEL,
    pa_modeh: PA_MODEH,
    pa_dout: PA_DOUT,
    _reserved4: [u8; 0x08],
    pa_douttgl: PA_DOUTTGL,
    pa_din: PA_DIN,
    pa_pinlockn: PA_PINLOCKN,
    _reserved7: [u8; 0x04],
    pa_ovtdis: PA_OVTDIS,
    _reserved8: [u8; 0x04],
    pb_ctrl: PB_CTRL,
    pb_model: PB_MODEL,
    pb_modeh: PB_MODEH,
    pb_dout: PB_DOUT,
    _reserved12: [u8; 0x08],
    pb_douttgl: PB_DOUTTGL,
    pb_din: PB_DIN,
    pb_pinlockn: PB_PINLOCKN,
    _reserved15: [u8; 0x04],
    pb_ovtdis: PB_OVTDIS,
    _reserved16: [u8; 0x04],
    pc_ctrl: PC_CTRL,
    pc_model: PC_MODEL,
    pc_modeh: PC_MODEH,
    pc_dout: PC_DOUT,
    _reserved20: [u8; 0x08],
    pc_douttgl: PC_DOUTTGL,
    pc_din: PC_DIN,
    pc_pinlockn: PC_PINLOCKN,
    _reserved23: [u8; 0x04],
    pc_ovtdis: PC_OVTDIS,
    _reserved24: [u8; 0x04],
    pd_ctrl: PD_CTRL,
    pd_model: PD_MODEL,
    pd_modeh: PD_MODEH,
    pd_dout: PD_DOUT,
    _reserved28: [u8; 0x08],
    pd_douttgl: PD_DOUTTGL,
    pd_din: PD_DIN,
    pd_pinlockn: PD_PINLOCKN,
    _reserved31: [u8; 0x04],
    pd_ovtdis: PD_OVTDIS,
    _reserved32: [u8; 0x04],
    pe_ctrl: PE_CTRL,
    pe_model: PE_MODEL,
    pe_modeh: PE_MODEH,
    pe_dout: PE_DOUT,
    _reserved36: [u8; 0x08],
    pe_douttgl: PE_DOUTTGL,
    pe_din: PE_DIN,
    pe_pinlockn: PE_PINLOCKN,
    _reserved39: [u8; 0x04],
    pe_ovtdis: PE_OVTDIS,
    _reserved40: [u8; 0x04],
    pf_ctrl: PF_CTRL,
    pf_model: PF_MODEL,
    pf_modeh: PF_MODEH,
    pf_dout: PF_DOUT,
    _reserved44: [u8; 0x08],
    pf_douttgl: PF_DOUTTGL,
    pf_din: PF_DIN,
    pf_pinlockn: PF_PINLOCKN,
    _reserved47: [u8; 0x04],
    pf_ovtdis: PF_OVTDIS,
    _reserved48: [u8; 0x04],
    pg_ctrl: PG_CTRL,
    pg_model: PG_MODEL,
    pg_modeh: PG_MODEH,
    pg_dout: PG_DOUT,
    _reserved52: [u8; 0x08],
    pg_douttgl: PG_DOUTTGL,
    pg_din: PG_DIN,
    pg_pinlockn: PG_PINLOCKN,
    _reserved55: [u8; 0x04],
    pg_ovtdis: PG_OVTDIS,
    _reserved56: [u8; 0x04],
    ph_ctrl: PH_CTRL,
    ph_model: PH_MODEL,
    ph_modeh: PH_MODEH,
    ph_dout: PH_DOUT,
    _reserved60: [u8; 0x08],
    ph_douttgl: PH_DOUTTGL,
    ph_din: PH_DIN,
    ph_pinlockn: PH_PINLOCKN,
    _reserved63: [u8; 0x04],
    ph_ovtdis: PH_OVTDIS,
    _reserved64: [u8; 0x04],
    pi_ctrl: PI_CTRL,
    pi_model: PI_MODEL,
    pi_modeh: PI_MODEH,
    pi_dout: PI_DOUT,
    _reserved68: [u8; 0x08],
    pi_douttgl: PI_DOUTTGL,
    pi_din: PI_DIN,
    pi_pinlockn: PI_PINLOCKN,
    _reserved71: [u8; 0x04],
    pi_ovtdis: PI_OVTDIS,
    _reserved72: [u8; 0x04],
    pj_ctrl: PJ_CTRL,
    pj_model: PJ_MODEL,
    pj_modeh: PJ_MODEH,
    pj_dout: PJ_DOUT,
    _reserved76: [u8; 0x08],
    pj_douttgl: PJ_DOUTTGL,
    pj_din: PJ_DIN,
    pj_pinlockn: PJ_PINLOCKN,
    _reserved79: [u8; 0x04],
    pj_ovtdis: PJ_OVTDIS,
    _reserved80: [u8; 0x04],
    pk_ctrl: PK_CTRL,
    pk_model: PK_MODEL,
    pk_modeh: PK_MODEH,
    pk_dout: PK_DOUT,
    _reserved84: [u8; 0x08],
    pk_douttgl: PK_DOUTTGL,
    pk_din: PK_DIN,
    pk_pinlockn: PK_PINLOCKN,
    _reserved87: [u8; 0x04],
    pk_ovtdis: PK_OVTDIS,
    _reserved88: [u8; 0x04],
    pl_ctrl: PL_CTRL,
    pl_model: PL_MODEL,
    pl_modeh: PL_MODEH,
    pl_dout: PL_DOUT,
    _reserved92: [u8; 0x08],
    pl_douttgl: PL_DOUTTGL,
    pl_din: PL_DIN,
    pl_pinlockn: PL_PINLOCKN,
    _reserved95: [u8; 0x04],
    pl_ovtdis: PL_OVTDIS,
    _reserved96: [u8; 0x01c4],
    extipsell: EXTIPSELL,
    extipselh: EXTIPSELH,
    extipinsell: EXTIPINSELL,
    extipinselh: EXTIPINSELH,
    extirise: EXTIRISE,
    extifall: EXTIFALL,
    extilevel: EXTILEVEL,
    if_: IF,
    ifs: IFS,
    ifc: IFC,
    ien: IEN,
    em4wuen: EM4WUEN,
    _reserved108: [u8; 0x10],
    routepen: ROUTEPEN,
    routeloc0: ROUTELOC0,
    _reserved110: [u8; 0x08],
    insense: INSENSE,
    lock: LOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    #[inline(always)]
    pub const fn pa_ctrl(&self) -> &PA_CTRL {
        &self.pa_ctrl
    }
    #[doc = "0x04 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pa_model(&self) -> &PA_MODEL {
        &self.pa_model
    }
    #[doc = "0x08 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pa_modeh(&self) -> &PA_MODEH {
        &self.pa_modeh
    }
    #[doc = "0x0c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pa_dout(&self) -> &PA_DOUT {
        &self.pa_dout
    }
    #[doc = "0x18 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pa_douttgl(&self) -> &PA_DOUTTGL {
        &self.pa_douttgl
    }
    #[doc = "0x1c - Port Data in Register"]
    #[inline(always)]
    pub const fn pa_din(&self) -> &PA_DIN {
        &self.pa_din
    }
    #[doc = "0x20 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pa_pinlockn(&self) -> &PA_PINLOCKN {
        &self.pa_pinlockn
    }
    #[doc = "0x28 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pa_ovtdis(&self) -> &PA_OVTDIS {
        &self.pa_ovtdis
    }
    #[doc = "0x30 - Port Control Register"]
    #[inline(always)]
    pub const fn pb_ctrl(&self) -> &PB_CTRL {
        &self.pb_ctrl
    }
    #[doc = "0x34 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pb_model(&self) -> &PB_MODEL {
        &self.pb_model
    }
    #[doc = "0x38 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pb_modeh(&self) -> &PB_MODEH {
        &self.pb_modeh
    }
    #[doc = "0x3c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pb_dout(&self) -> &PB_DOUT {
        &self.pb_dout
    }
    #[doc = "0x48 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pb_douttgl(&self) -> &PB_DOUTTGL {
        &self.pb_douttgl
    }
    #[doc = "0x4c - Port Data in Register"]
    #[inline(always)]
    pub const fn pb_din(&self) -> &PB_DIN {
        &self.pb_din
    }
    #[doc = "0x50 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pb_pinlockn(&self) -> &PB_PINLOCKN {
        &self.pb_pinlockn
    }
    #[doc = "0x58 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pb_ovtdis(&self) -> &PB_OVTDIS {
        &self.pb_ovtdis
    }
    #[doc = "0x60 - Port Control Register"]
    #[inline(always)]
    pub const fn pc_ctrl(&self) -> &PC_CTRL {
        &self.pc_ctrl
    }
    #[doc = "0x64 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pc_model(&self) -> &PC_MODEL {
        &self.pc_model
    }
    #[doc = "0x68 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pc_modeh(&self) -> &PC_MODEH {
        &self.pc_modeh
    }
    #[doc = "0x6c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pc_dout(&self) -> &PC_DOUT {
        &self.pc_dout
    }
    #[doc = "0x78 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pc_douttgl(&self) -> &PC_DOUTTGL {
        &self.pc_douttgl
    }
    #[doc = "0x7c - Port Data in Register"]
    #[inline(always)]
    pub const fn pc_din(&self) -> &PC_DIN {
        &self.pc_din
    }
    #[doc = "0x80 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pc_pinlockn(&self) -> &PC_PINLOCKN {
        &self.pc_pinlockn
    }
    #[doc = "0x88 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pc_ovtdis(&self) -> &PC_OVTDIS {
        &self.pc_ovtdis
    }
    #[doc = "0x90 - Port Control Register"]
    #[inline(always)]
    pub const fn pd_ctrl(&self) -> &PD_CTRL {
        &self.pd_ctrl
    }
    #[doc = "0x94 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pd_model(&self) -> &PD_MODEL {
        &self.pd_model
    }
    #[doc = "0x98 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pd_modeh(&self) -> &PD_MODEH {
        &self.pd_modeh
    }
    #[doc = "0x9c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pd_dout(&self) -> &PD_DOUT {
        &self.pd_dout
    }
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pd_douttgl(&self) -> &PD_DOUTTGL {
        &self.pd_douttgl
    }
    #[doc = "0xac - Port Data in Register"]
    #[inline(always)]
    pub const fn pd_din(&self) -> &PD_DIN {
        &self.pd_din
    }
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pd_pinlockn(&self) -> &PD_PINLOCKN {
        &self.pd_pinlockn
    }
    #[doc = "0xb8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pd_ovtdis(&self) -> &PD_OVTDIS {
        &self.pd_ovtdis
    }
    #[doc = "0xc0 - Port Control Register"]
    #[inline(always)]
    pub const fn pe_ctrl(&self) -> &PE_CTRL {
        &self.pe_ctrl
    }
    #[doc = "0xc4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pe_model(&self) -> &PE_MODEL {
        &self.pe_model
    }
    #[doc = "0xc8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pe_modeh(&self) -> &PE_MODEH {
        &self.pe_modeh
    }
    #[doc = "0xcc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pe_dout(&self) -> &PE_DOUT {
        &self.pe_dout
    }
    #[doc = "0xd8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pe_douttgl(&self) -> &PE_DOUTTGL {
        &self.pe_douttgl
    }
    #[doc = "0xdc - Port Data in Register"]
    #[inline(always)]
    pub const fn pe_din(&self) -> &PE_DIN {
        &self.pe_din
    }
    #[doc = "0xe0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pe_pinlockn(&self) -> &PE_PINLOCKN {
        &self.pe_pinlockn
    }
    #[doc = "0xe8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pe_ovtdis(&self) -> &PE_OVTDIS {
        &self.pe_ovtdis
    }
    #[doc = "0xf0 - Port Control Register"]
    #[inline(always)]
    pub const fn pf_ctrl(&self) -> &PF_CTRL {
        &self.pf_ctrl
    }
    #[doc = "0xf4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pf_model(&self) -> &PF_MODEL {
        &self.pf_model
    }
    #[doc = "0xf8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pf_modeh(&self) -> &PF_MODEH {
        &self.pf_modeh
    }
    #[doc = "0xfc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pf_dout(&self) -> &PF_DOUT {
        &self.pf_dout
    }
    #[doc = "0x108 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pf_douttgl(&self) -> &PF_DOUTTGL {
        &self.pf_douttgl
    }
    #[doc = "0x10c - Port Data in Register"]
    #[inline(always)]
    pub const fn pf_din(&self) -> &PF_DIN {
        &self.pf_din
    }
    #[doc = "0x110 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pf_pinlockn(&self) -> &PF_PINLOCKN {
        &self.pf_pinlockn
    }
    #[doc = "0x118 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pf_ovtdis(&self) -> &PF_OVTDIS {
        &self.pf_ovtdis
    }
    #[doc = "0x120 - Port Control Register"]
    #[inline(always)]
    pub const fn pg_ctrl(&self) -> &PG_CTRL {
        &self.pg_ctrl
    }
    #[doc = "0x124 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pg_model(&self) -> &PG_MODEL {
        &self.pg_model
    }
    #[doc = "0x128 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pg_modeh(&self) -> &PG_MODEH {
        &self.pg_modeh
    }
    #[doc = "0x12c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pg_dout(&self) -> &PG_DOUT {
        &self.pg_dout
    }
    #[doc = "0x138 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pg_douttgl(&self) -> &PG_DOUTTGL {
        &self.pg_douttgl
    }
    #[doc = "0x13c - Port Data in Register"]
    #[inline(always)]
    pub const fn pg_din(&self) -> &PG_DIN {
        &self.pg_din
    }
    #[doc = "0x140 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pg_pinlockn(&self) -> &PG_PINLOCKN {
        &self.pg_pinlockn
    }
    #[doc = "0x148 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pg_ovtdis(&self) -> &PG_OVTDIS {
        &self.pg_ovtdis
    }
    #[doc = "0x150 - Port Control Register"]
    #[inline(always)]
    pub const fn ph_ctrl(&self) -> &PH_CTRL {
        &self.ph_ctrl
    }
    #[doc = "0x154 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn ph_model(&self) -> &PH_MODEL {
        &self.ph_model
    }
    #[doc = "0x158 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn ph_modeh(&self) -> &PH_MODEH {
        &self.ph_modeh
    }
    #[doc = "0x15c - Port Data Out Register"]
    #[inline(always)]
    pub const fn ph_dout(&self) -> &PH_DOUT {
        &self.ph_dout
    }
    #[doc = "0x168 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn ph_douttgl(&self) -> &PH_DOUTTGL {
        &self.ph_douttgl
    }
    #[doc = "0x16c - Port Data in Register"]
    #[inline(always)]
    pub const fn ph_din(&self) -> &PH_DIN {
        &self.ph_din
    }
    #[doc = "0x170 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn ph_pinlockn(&self) -> &PH_PINLOCKN {
        &self.ph_pinlockn
    }
    #[doc = "0x178 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn ph_ovtdis(&self) -> &PH_OVTDIS {
        &self.ph_ovtdis
    }
    #[doc = "0x180 - Port Control Register"]
    #[inline(always)]
    pub const fn pi_ctrl(&self) -> &PI_CTRL {
        &self.pi_ctrl
    }
    #[doc = "0x184 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pi_model(&self) -> &PI_MODEL {
        &self.pi_model
    }
    #[doc = "0x188 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pi_modeh(&self) -> &PI_MODEH {
        &self.pi_modeh
    }
    #[doc = "0x18c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pi_dout(&self) -> &PI_DOUT {
        &self.pi_dout
    }
    #[doc = "0x198 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pi_douttgl(&self) -> &PI_DOUTTGL {
        &self.pi_douttgl
    }
    #[doc = "0x19c - Port Data in Register"]
    #[inline(always)]
    pub const fn pi_din(&self) -> &PI_DIN {
        &self.pi_din
    }
    #[doc = "0x1a0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pi_pinlockn(&self) -> &PI_PINLOCKN {
        &self.pi_pinlockn
    }
    #[doc = "0x1a8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pi_ovtdis(&self) -> &PI_OVTDIS {
        &self.pi_ovtdis
    }
    #[doc = "0x1b0 - Port Control Register"]
    #[inline(always)]
    pub const fn pj_ctrl(&self) -> &PJ_CTRL {
        &self.pj_ctrl
    }
    #[doc = "0x1b4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pj_model(&self) -> &PJ_MODEL {
        &self.pj_model
    }
    #[doc = "0x1b8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pj_modeh(&self) -> &PJ_MODEH {
        &self.pj_modeh
    }
    #[doc = "0x1bc - Port Data Out Register"]
    #[inline(always)]
    pub const fn pj_dout(&self) -> &PJ_DOUT {
        &self.pj_dout
    }
    #[doc = "0x1c8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pj_douttgl(&self) -> &PJ_DOUTTGL {
        &self.pj_douttgl
    }
    #[doc = "0x1cc - Port Data in Register"]
    #[inline(always)]
    pub const fn pj_din(&self) -> &PJ_DIN {
        &self.pj_din
    }
    #[doc = "0x1d0 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pj_pinlockn(&self) -> &PJ_PINLOCKN {
        &self.pj_pinlockn
    }
    #[doc = "0x1d8 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pj_ovtdis(&self) -> &PJ_OVTDIS {
        &self.pj_ovtdis
    }
    #[doc = "0x1e0 - Port Control Register"]
    #[inline(always)]
    pub const fn pk_ctrl(&self) -> &PK_CTRL {
        &self.pk_ctrl
    }
    #[doc = "0x1e4 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pk_model(&self) -> &PK_MODEL {
        &self.pk_model
    }
    #[doc = "0x1e8 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pk_modeh(&self) -> &PK_MODEH {
        &self.pk_modeh
    }
    #[doc = "0x1ec - Port Data Out Register"]
    #[inline(always)]
    pub const fn pk_dout(&self) -> &PK_DOUT {
        &self.pk_dout
    }
    #[doc = "0x1f8 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pk_douttgl(&self) -> &PK_DOUTTGL {
        &self.pk_douttgl
    }
    #[doc = "0x1fc - Port Data in Register"]
    #[inline(always)]
    pub const fn pk_din(&self) -> &PK_DIN {
        &self.pk_din
    }
    #[doc = "0x200 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pk_pinlockn(&self) -> &PK_PINLOCKN {
        &self.pk_pinlockn
    }
    #[doc = "0x208 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pk_ovtdis(&self) -> &PK_OVTDIS {
        &self.pk_ovtdis
    }
    #[doc = "0x210 - Port Control Register"]
    #[inline(always)]
    pub const fn pl_ctrl(&self) -> &PL_CTRL {
        &self.pl_ctrl
    }
    #[doc = "0x214 - Port Pin Mode Low Register"]
    #[inline(always)]
    pub const fn pl_model(&self) -> &PL_MODEL {
        &self.pl_model
    }
    #[doc = "0x218 - Port Pin Mode High Register"]
    #[inline(always)]
    pub const fn pl_modeh(&self) -> &PL_MODEH {
        &self.pl_modeh
    }
    #[doc = "0x21c - Port Data Out Register"]
    #[inline(always)]
    pub const fn pl_dout(&self) -> &PL_DOUT {
        &self.pl_dout
    }
    #[doc = "0x228 - Port Data Out Toggle Register"]
    #[inline(always)]
    pub const fn pl_douttgl(&self) -> &PL_DOUTTGL {
        &self.pl_douttgl
    }
    #[doc = "0x22c - Port Data in Register"]
    #[inline(always)]
    pub const fn pl_din(&self) -> &PL_DIN {
        &self.pl_din
    }
    #[doc = "0x230 - Port Unlocked Pins Register"]
    #[inline(always)]
    pub const fn pl_pinlockn(&self) -> &PL_PINLOCKN {
        &self.pl_pinlockn
    }
    #[doc = "0x238 - Over Voltage Disable for All Modes"]
    #[inline(always)]
    pub const fn pl_ovtdis(&self) -> &PL_OVTDIS {
        &self.pl_ovtdis
    }
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &EXTIPSELL {
        &self.extipsell
    }
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &EXTIPSELH {
        &self.extipselh
    }
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    #[inline(always)]
    pub const fn extipinsell(&self) -> &EXTIPINSELL {
        &self.extipinsell
    }
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    #[inline(always)]
    pub const fn extipinselh(&self) -> &EXTIPINSELH {
        &self.extipinselh
    }
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    #[inline(always)]
    pub const fn extirise(&self) -> &EXTIRISE {
        &self.extirise
    }
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    #[inline(always)]
    pub const fn extifall(&self) -> &EXTIFALL {
        &self.extifall
    }
    #[doc = "0x418 - External Interrupt Level Register"]
    #[inline(always)]
    pub const fn extilevel(&self) -> &EXTILEVEL {
        &self.extilevel
    }
    #[doc = "0x41c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &IF {
        &self.if_
    }
    #[doc = "0x420 - Interrupt Flag Set Register"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
    #[doc = "0x428 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &IEN {
        &self.ien
    }
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &EM4WUEN {
        &self.em4wuen
    }
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    #[inline(always)]
    pub const fn routepen(&self) -> &ROUTEPEN {
        &self.routepen
    }
    #[doc = "0x444 - I/O Routing Location Register"]
    #[inline(always)]
    pub const fn routeloc0(&self) -> &ROUTELOC0 {
        &self.routeloc0
    }
    #[doc = "0x450 - Input Sense Register"]
    #[inline(always)]
    pub const fn insense(&self) -> &INSENSE {
        &self.insense
    }
    #[doc = "0x454 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
}
#[doc = "PA_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ctrl`]
module"]
pub type PA_CTRL = crate::Reg<pa_ctrl::PA_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "PA_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_model`]
module"]
pub type PA_MODEL = crate::Reg<pa_model::PA_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "PA_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_modeh`]
module"]
pub type PA_MODEH = crate::Reg<pa_modeh::PA_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "PA_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_dout`]
module"]
pub type PA_DOUT = crate::Reg<pa_dout::PA_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "PA_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_douttgl`]
module"]
pub type PA_DOUTTGL = crate::Reg<pa_douttgl::PA_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "PA_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_din`]
module"]
pub type PA_DIN = crate::Reg<pa_din::PA_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pa_din;
#[doc = "PA_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pinlockn`]
module"]
pub type PA_PINLOCKN = crate::Reg<pa_pinlockn::PA_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "PA_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ovtdis`]
module"]
pub type PA_OVTDIS = crate::Reg<pa_ovtdis::PA_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pa_ovtdis;
#[doc = "PB_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ctrl`]
module"]
pub type PB_CTRL = crate::Reg<pb_ctrl::PB_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "PB_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_model`]
module"]
pub type PB_MODEL = crate::Reg<pb_model::PB_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "PB_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_modeh`]
module"]
pub type PB_MODEH = crate::Reg<pb_modeh::PB_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "PB_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_dout`]
module"]
pub type PB_DOUT = crate::Reg<pb_dout::PB_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "PB_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_douttgl`]
module"]
pub type PB_DOUTTGL = crate::Reg<pb_douttgl::PB_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "PB_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_din`]
module"]
pub type PB_DIN = crate::Reg<pb_din::PB_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pb_din;
#[doc = "PB_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pinlockn`]
module"]
pub type PB_PINLOCKN = crate::Reg<pb_pinlockn::PB_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "PB_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ovtdis`]
module"]
pub type PB_OVTDIS = crate::Reg<pb_ovtdis::PB_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pb_ovtdis;
#[doc = "PC_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ctrl`]
module"]
pub type PC_CTRL = crate::Reg<pc_ctrl::PC_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "PC_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_model`]
module"]
pub type PC_MODEL = crate::Reg<pc_model::PC_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "PC_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_modeh`]
module"]
pub type PC_MODEH = crate::Reg<pc_modeh::PC_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "PC_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_dout`]
module"]
pub type PC_DOUT = crate::Reg<pc_dout::PC_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "PC_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_douttgl`]
module"]
pub type PC_DOUTTGL = crate::Reg<pc_douttgl::PC_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "PC_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_din`]
module"]
pub type PC_DIN = crate::Reg<pc_din::PC_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pc_din;
#[doc = "PC_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pinlockn`]
module"]
pub type PC_PINLOCKN = crate::Reg<pc_pinlockn::PC_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "PC_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ovtdis`]
module"]
pub type PC_OVTDIS = crate::Reg<pc_ovtdis::PC_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pc_ovtdis;
#[doc = "PD_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ctrl`]
module"]
pub type PD_CTRL = crate::Reg<pd_ctrl::PD_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "PD_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_model`]
module"]
pub type PD_MODEL = crate::Reg<pd_model::PD_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "PD_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_modeh`]
module"]
pub type PD_MODEH = crate::Reg<pd_modeh::PD_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "PD_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_dout`]
module"]
pub type PD_DOUT = crate::Reg<pd_dout::PD_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "PD_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_douttgl`]
module"]
pub type PD_DOUTTGL = crate::Reg<pd_douttgl::PD_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "PD_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_din`]
module"]
pub type PD_DIN = crate::Reg<pd_din::PD_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pd_din;
#[doc = "PD_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pinlockn`]
module"]
pub type PD_PINLOCKN = crate::Reg<pd_pinlockn::PD_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "PD_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ovtdis`]
module"]
pub type PD_OVTDIS = crate::Reg<pd_ovtdis::PD_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pd_ovtdis;
#[doc = "PE_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ctrl`]
module"]
pub type PE_CTRL = crate::Reg<pe_ctrl::PE_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "PE_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_model`]
module"]
pub type PE_MODEL = crate::Reg<pe_model::PE_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "PE_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_modeh`]
module"]
pub type PE_MODEH = crate::Reg<pe_modeh::PE_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "PE_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_dout`]
module"]
pub type PE_DOUT = crate::Reg<pe_dout::PE_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "PE_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_douttgl`]
module"]
pub type PE_DOUTTGL = crate::Reg<pe_douttgl::PE_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "PE_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_din`]
module"]
pub type PE_DIN = crate::Reg<pe_din::PE_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pe_din;
#[doc = "PE_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pinlockn`]
module"]
pub type PE_PINLOCKN = crate::Reg<pe_pinlockn::PE_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "PE_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ovtdis`]
module"]
pub type PE_OVTDIS = crate::Reg<pe_ovtdis::PE_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pe_ovtdis;
#[doc = "PF_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ctrl`]
module"]
pub type PF_CTRL = crate::Reg<pf_ctrl::PF_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "PF_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_model`]
module"]
pub type PF_MODEL = crate::Reg<pf_model::PF_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "PF_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_modeh`]
module"]
pub type PF_MODEH = crate::Reg<pf_modeh::PF_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "PF_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_dout`]
module"]
pub type PF_DOUT = crate::Reg<pf_dout::PF_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "PF_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_douttgl`]
module"]
pub type PF_DOUTTGL = crate::Reg<pf_douttgl::PF_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "PF_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_din`]
module"]
pub type PF_DIN = crate::Reg<pf_din::PF_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pf_din;
#[doc = "PF_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pinlockn`]
module"]
pub type PF_PINLOCKN = crate::Reg<pf_pinlockn::PF_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "PF_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ovtdis`]
module"]
pub type PF_OVTDIS = crate::Reg<pf_ovtdis::PF_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pf_ovtdis;
#[doc = "PG_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_ctrl`]
module"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pg_ctrl;
#[doc = "PG_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_model`]
module"]
pub type PG_MODEL = crate::Reg<pg_model::PG_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pg_model;
#[doc = "PG_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_modeh`]
module"]
pub type PG_MODEH = crate::Reg<pg_modeh::PG_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pg_modeh;
#[doc = "PG_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_dout`]
module"]
pub type PG_DOUT = crate::Reg<pg_dout::PG_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pg_dout;
#[doc = "PG_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_douttgl`]
module"]
pub type PG_DOUTTGL = crate::Reg<pg_douttgl::PG_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pg_douttgl;
#[doc = "PG_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_din`]
module"]
pub type PG_DIN = crate::Reg<pg_din::PG_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pg_din;
#[doc = "PG_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_pinlockn`]
module"]
pub type PG_PINLOCKN = crate::Reg<pg_pinlockn::PG_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pg_pinlockn;
#[doc = "PG_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_ovtdis`]
module"]
pub type PG_OVTDIS = crate::Reg<pg_ovtdis::PG_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pg_ovtdis;
#[doc = "PH_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_ctrl`]
module"]
pub type PH_CTRL = crate::Reg<ph_ctrl::PH_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod ph_ctrl;
#[doc = "PH_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_model`]
module"]
pub type PH_MODEL = crate::Reg<ph_model::PH_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod ph_model;
#[doc = "PH_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_modeh`]
module"]
pub type PH_MODEH = crate::Reg<ph_modeh::PH_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod ph_modeh;
#[doc = "PH_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_dout`]
module"]
pub type PH_DOUT = crate::Reg<ph_dout::PH_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod ph_dout;
#[doc = "PH_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_douttgl`]
module"]
pub type PH_DOUTTGL = crate::Reg<ph_douttgl::PH_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod ph_douttgl;
#[doc = "PH_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_din`]
module"]
pub type PH_DIN = crate::Reg<ph_din::PH_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod ph_din;
#[doc = "PH_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_pinlockn`]
module"]
pub type PH_PINLOCKN = crate::Reg<ph_pinlockn::PH_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod ph_pinlockn;
#[doc = "PH_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph_ovtdis`]
module"]
pub type PH_OVTDIS = crate::Reg<ph_ovtdis::PH_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod ph_ovtdis;
#[doc = "PI_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_ctrl`]
module"]
pub type PI_CTRL = crate::Reg<pi_ctrl::PI_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pi_ctrl;
#[doc = "PI_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_model`]
module"]
pub type PI_MODEL = crate::Reg<pi_model::PI_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pi_model;
#[doc = "PI_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_modeh`]
module"]
pub type PI_MODEH = crate::Reg<pi_modeh::PI_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pi_modeh;
#[doc = "PI_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_dout`]
module"]
pub type PI_DOUT = crate::Reg<pi_dout::PI_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pi_dout;
#[doc = "PI_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_douttgl`]
module"]
pub type PI_DOUTTGL = crate::Reg<pi_douttgl::PI_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pi_douttgl;
#[doc = "PI_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_din`]
module"]
pub type PI_DIN = crate::Reg<pi_din::PI_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pi_din;
#[doc = "PI_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_pinlockn`]
module"]
pub type PI_PINLOCKN = crate::Reg<pi_pinlockn::PI_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pi_pinlockn;
#[doc = "PI_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_ovtdis`]
module"]
pub type PI_OVTDIS = crate::Reg<pi_ovtdis::PI_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pi_ovtdis;
#[doc = "PJ_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_ctrl`]
module"]
pub type PJ_CTRL = crate::Reg<pj_ctrl::PJ_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pj_ctrl;
#[doc = "PJ_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_model`]
module"]
pub type PJ_MODEL = crate::Reg<pj_model::PJ_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pj_model;
#[doc = "PJ_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_modeh`]
module"]
pub type PJ_MODEH = crate::Reg<pj_modeh::PJ_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pj_modeh;
#[doc = "PJ_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_dout`]
module"]
pub type PJ_DOUT = crate::Reg<pj_dout::PJ_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pj_dout;
#[doc = "PJ_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_douttgl`]
module"]
pub type PJ_DOUTTGL = crate::Reg<pj_douttgl::PJ_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pj_douttgl;
#[doc = "PJ_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_din`]
module"]
pub type PJ_DIN = crate::Reg<pj_din::PJ_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pj_din;
#[doc = "PJ_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_pinlockn`]
module"]
pub type PJ_PINLOCKN = crate::Reg<pj_pinlockn::PJ_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pj_pinlockn;
#[doc = "PJ_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pj_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pj_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pj_ovtdis`]
module"]
pub type PJ_OVTDIS = crate::Reg<pj_ovtdis::PJ_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pj_ovtdis;
#[doc = "PK_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_ctrl`]
module"]
pub type PK_CTRL = crate::Reg<pk_ctrl::PK_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pk_ctrl;
#[doc = "PK_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_model`]
module"]
pub type PK_MODEL = crate::Reg<pk_model::PK_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pk_model;
#[doc = "PK_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_modeh`]
module"]
pub type PK_MODEH = crate::Reg<pk_modeh::PK_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pk_modeh;
#[doc = "PK_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_dout`]
module"]
pub type PK_DOUT = crate::Reg<pk_dout::PK_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pk_dout;
#[doc = "PK_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_douttgl`]
module"]
pub type PK_DOUTTGL = crate::Reg<pk_douttgl::PK_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pk_douttgl;
#[doc = "PK_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_din`]
module"]
pub type PK_DIN = crate::Reg<pk_din::PK_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pk_din;
#[doc = "PK_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_pinlockn`]
module"]
pub type PK_PINLOCKN = crate::Reg<pk_pinlockn::PK_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pk_pinlockn;
#[doc = "PK_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pk_ovtdis`]
module"]
pub type PK_OVTDIS = crate::Reg<pk_ovtdis::PK_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pk_ovtdis;
#[doc = "PL_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_ctrl`]
module"]
pub type PL_CTRL = crate::Reg<pl_ctrl::PL_CTRL_SPEC>;
#[doc = "Port Control Register"]
pub mod pl_ctrl;
#[doc = "PL_MODEL (rw) register accessor: Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_model::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_model::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_model`]
module"]
pub type PL_MODEL = crate::Reg<pl_model::PL_MODEL_SPEC>;
#[doc = "Port Pin Mode Low Register"]
pub mod pl_model;
#[doc = "PL_MODEH (rw) register accessor: Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_modeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_modeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_modeh`]
module"]
pub type PL_MODEH = crate::Reg<pl_modeh::PL_MODEH_SPEC>;
#[doc = "Port Pin Mode High Register"]
pub mod pl_modeh;
#[doc = "PL_DOUT (rw) register accessor: Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_dout`]
module"]
pub type PL_DOUT = crate::Reg<pl_dout::PL_DOUT_SPEC>;
#[doc = "Port Data Out Register"]
pub mod pl_dout;
#[doc = "PL_DOUTTGL (w) register accessor: Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_douttgl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_douttgl`]
module"]
pub type PL_DOUTTGL = crate::Reg<pl_douttgl::PL_DOUTTGL_SPEC>;
#[doc = "Port Data Out Toggle Register"]
pub mod pl_douttgl;
#[doc = "PL_DIN (r) register accessor: Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_din::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_din`]
module"]
pub type PL_DIN = crate::Reg<pl_din::PL_DIN_SPEC>;
#[doc = "Port Data in Register"]
pub mod pl_din;
#[doc = "PL_PINLOCKN (rw) register accessor: Port Unlocked Pins Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_pinlockn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_pinlockn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_pinlockn`]
module"]
pub type PL_PINLOCKN = crate::Reg<pl_pinlockn::PL_PINLOCKN_SPEC>;
#[doc = "Port Unlocked Pins Register"]
pub mod pl_pinlockn;
#[doc = "PL_OVTDIS (rw) register accessor: Over Voltage Disable for All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl_ovtdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl_ovtdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl_ovtdis`]
module"]
pub type PL_OVTDIS = crate::Reg<pl_ovtdis::PL_OVTDIS_SPEC>;
#[doc = "Over Voltage Disable for All Modes"]
pub mod pl_ovtdis;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipsell::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`]
module"]
pub type EXTIPSELL = crate::Reg<extipsell::EXTIPSELL_SPEC>;
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External Interrupt Port Select High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipselh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`]
module"]
pub type EXTIPSELH = crate::Reg<extipselh::EXTIPSELH_SPEC>;
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: External Interrupt Pin Select Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipinsell::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipinsell::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinsell`]
module"]
pub type EXTIPINSELL = crate::Reg<extipinsell::EXTIPINSELL_SPEC>;
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: External Interrupt Pin Select High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extipinselh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extipinselh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinselh`]
module"]
pub type EXTIPINSELH = crate::Reg<extipinselh::EXTIPINSELH_SPEC>;
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extirise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`]
module"]
pub type EXTIRISE = crate::Reg<extirise::EXTIRISE_SPEC>;
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extifall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`]
module"]
pub type EXTIFALL = crate::Reg<extifall::EXTIFALL_SPEC>;
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "EXTILEVEL (rw) register accessor: External Interrupt Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extilevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extilevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extilevel`]
module"]
pub type EXTILEVEL = crate::Reg<extilevel::EXTILEVEL_SPEC>;
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
#[doc = "IF (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`]
module"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`]
module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`]
module"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "EM4WUEN (rw) register accessor: EM4 Wake Up Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em4wuen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`]
module"]
pub type EM4WUEN = crate::Reg<em4wuen::EM4WUEN_SPEC>;
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
#[doc = "ROUTEPEN (rw) register accessor: I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routepen`]
module"]
pub type ROUTEPEN = crate::Reg<routepen::ROUTEPEN_SPEC>;
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "ROUTELOC0 (rw) register accessor: I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@routeloc0`]
module"]
pub type ROUTELOC0 = crate::Reg<routeloc0::ROUTELOC0_SPEC>;
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "INSENSE (rw) register accessor: Input Sense Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`insense::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`insense::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insense`]
module"]
pub type INSENSE = crate::Reg<insense::INSENSE_SPEC>;
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
