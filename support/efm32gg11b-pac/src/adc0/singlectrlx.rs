#[doc = "Register `SINGLECTRLX` reader"]
pub type R = crate::R<SINGLECTRLX_SPEC>;
#[doc = "Register `SINGLECTRLX` writer"]
pub type W = crate::W<SINGLECTRLX_SPEC>;
#[doc = "Field `VREFSEL` reader - Single Channel Reference Selection"]
pub type VREFSEL_R = crate::FieldReader<VREFSEL_A>;
#[doc = "Single Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREFSEL_A {
    #[doc = "0: Internal 0.83V Bandgap reference"]
    VBGR = 0,
    #[doc = "1: Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    VDDXWATT = 1,
    #[doc = "2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    VREFPWATT = 2,
    #[doc = "3: Raw single ended external Vref: ADCn_EXTP"]
    VREFP = 3,
    #[doc = "4: Special mode used to generate ENTROPY."]
    VENTROPY = 4,
    #[doc = "5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    VREFPNWATT = 5,
    #[doc = "6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    VREFPN = 6,
    #[doc = "7: Internal Bandgap reference at low setting 0.78V"]
    VBGRLOW = 7,
}
impl From<VREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREFSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VREFSEL_A {
    type Ux = u8;
}
impl VREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFSEL_A {
        match self.bits {
            0 => VREFSEL_A::VBGR,
            1 => VREFSEL_A::VDDXWATT,
            2 => VREFSEL_A::VREFPWATT,
            3 => VREFSEL_A::VREFP,
            4 => VREFSEL_A::VENTROPY,
            5 => VREFSEL_A::VREFPNWATT,
            6 => VREFSEL_A::VREFPN,
            7 => VREFSEL_A::VBGRLOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREFSEL_A::VBGR
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == VREFSEL_A::VDDXWATT
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPWATT
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == VREFSEL_A::VREFP
    }
    #[doc = "Special mode used to generate ENTROPY."]
    #[inline(always)]
    pub fn is_ventropy(&self) -> bool {
        *self == VREFSEL_A::VENTROPY
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == VREFSEL_A::VREFPNWATT
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == VREFSEL_A::VREFPN
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == VREFSEL_A::VBGRLOW
    }
}
#[doc = "Field `VREFSEL` writer - Single Channel Reference Selection"]
pub type VREFSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, VREFSEL_A>;
impl<'a, REG> VREFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VBGR)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VDDXWATT)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFPWATT)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFP)
    }
    #[doc = "Special mode used to generate ENTROPY."]
    #[inline(always)]
    pub fn ventropy(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VENTROPY)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFPNWATT)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VREFPN)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut crate::W<REG> {
        self.variant(VREFSEL_A::VBGRLOW)
    }
}
#[doc = "Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_R = crate::BitReader;
#[doc = "Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF"]
pub type VREFATTFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_R = crate::FieldReader;
#[doc = "Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VREFATT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VINATT` reader - Code for VIN Attenuation Factor"]
pub type VINATT_R = crate::FieldReader;
#[doc = "Field `VINATT` writer - Code for VIN Attenuation Factor"]
pub type VINATT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DVL` reader - Single Channel DV Level Select"]
pub type DVL_R = crate::FieldReader;
#[doc = "Field `DVL` writer - Single Channel DV Level Select"]
pub type DVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIFOOFACT` reader - Single Channel FIFO Overflow Action"]
pub type FIFOOFACT_R = crate::BitReader;
#[doc = "Field `FIFOOFACT` writer - Single Channel FIFO Overflow Action"]
pub type FIFOOFACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSMODE` reader - Single Channel PRS Trigger Mode"]
pub type PRSMODE_R = crate::BitReader;
#[doc = "Field `PRSMODE` writer - Single Channel PRS Trigger Mode"]
pub type PRSMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSSEL` reader - Single Channel PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "Single Channel PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers single channel"]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers single channel"]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers single channel"]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers single channel"]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers single channel"]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers single channel"]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers single channel"]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers single channel"]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers single channel"]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers single channel"]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers single channel"]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers single channel"]
    PRSCH11 = 11,
    #[doc = "12: PRS ch 12 triggers single channel"]
    PRSCH12 = 12,
    #[doc = "13: PRS ch 13 triggers single channel"]
    PRSCH13 = 13,
    #[doc = "14: PRS ch 14 triggers single channel"]
    PRSCH14 = 14,
    #[doc = "15: PRS ch 15 triggers single channel"]
    PRSCH15 = 15,
    #[doc = "16: PRS ch 16 triggers single channel"]
    PRSCH16 = 16,
    #[doc = "17: PRS ch 17 triggers single channel"]
    PRSCH17 = 17,
    #[doc = "18: PRS ch 18 triggers single channel"]
    PRSCH18 = 18,
    #[doc = "19: PRS ch 19 triggers single channel"]
    PRSCH19 = 19,
    #[doc = "20: PRS ch 20 triggers single channel"]
    PRSCH20 = 20,
    #[doc = "21: PRS ch 21 triggers single channel"]
    PRSCH21 = 21,
    #[doc = "22: PRS ch 22 triggers single channel"]
    PRSCH22 = 22,
    #[doc = "23: PRS ch 23 triggers single channel"]
    PRSCH23 = 23,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL_A {
    type Ux = u8;
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            12 => Some(PRSSEL_A::PRSCH12),
            13 => Some(PRSSEL_A::PRSCH13),
            14 => Some(PRSSEL_A::PRSCH14),
            15 => Some(PRSSEL_A::PRSCH15),
            16 => Some(PRSSEL_A::PRSCH16),
            17 => Some(PRSSEL_A::PRSCH17),
            18 => Some(PRSSEL_A::PRSCH18),
            19 => Some(PRSSEL_A::PRSCH19),
            20 => Some(PRSSEL_A::PRSCH20),
            21 => Some(PRSSEL_A::PRSCH21),
            22 => Some(PRSSEL_A::PRSCH22),
            23 => Some(PRSSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS ch 1 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS ch 2 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS ch 3 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS ch 4 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS ch 5 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "PRS ch 6 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "PRS ch 7 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "PRS ch 8 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "PRS ch 9 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "PRS ch 10 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "PRS ch 11 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
    #[doc = "PRS ch 12 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL_A::PRSCH12
    }
    #[doc = "PRS ch 13 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL_A::PRSCH13
    }
    #[doc = "PRS ch 14 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL_A::PRSCH14
    }
    #[doc = "PRS ch 15 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL_A::PRSCH15
    }
    #[doc = "PRS ch 16 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL_A::PRSCH16
    }
    #[doc = "PRS ch 17 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL_A::PRSCH17
    }
    #[doc = "PRS ch 18 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL_A::PRSCH18
    }
    #[doc = "PRS ch 19 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL_A::PRSCH19
    }
    #[doc = "PRS ch 20 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL_A::PRSCH20
    }
    #[doc = "PRS ch 21 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL_A::PRSCH21
    }
    #[doc = "PRS ch 22 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL_A::PRSCH22
    }
    #[doc = "PRS ch 23 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL` writer - Single Channel PRS Trigger Select"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL_A>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers single channel"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers single channel"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers single channel"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers single channel"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers single channel"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers single channel"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers single channel"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers single channel"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers single channel"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers single channel"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers single channel"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers single channel"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers single channel"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers single channel"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers single channel"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers single channel"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS ch 16 triggers single channel"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS ch 17 triggers single channel"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS ch 18 triggers single channel"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS ch 19 triggers single channel"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS ch 20 triggers single channel"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS ch 21 triggers single channel"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS ch 22 triggers single channel"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS ch 23 triggers single channel"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH23)
    }
}
#[doc = "Field `CONVSTARTDELAY` reader - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_R = crate::FieldReader;
#[doc = "Field `CONVSTARTDELAY` writer - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type CONVSTARTDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_R = crate::BitReader;
#[doc = "Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start"]
pub type CONVSTARTDELAYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPDELAY` reader - REPDELAY Select for SINGLE REP Mode"]
pub type REPDELAY_R = crate::FieldReader<REPDELAY_A>;
#[doc = "REPDELAY Select for SINGLE REP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REPDELAY_A {
    #[doc = "0: No delay"]
    NODELAY = 0,
    #[doc = "1: 4 conversion clock cycles"]
    _4CYCLES = 1,
    #[doc = "2: 8 conversion clock cycles"]
    _8CYCLES = 2,
    #[doc = "3: 16 conversion clock cycles"]
    _16CYCLES = 3,
    #[doc = "4: 32 conversion clock cycles"]
    _32CYCLES = 4,
    #[doc = "5: 64 conversion clock cycles"]
    _64CYCLES = 5,
    #[doc = "6: 128 conversion clock cycles"]
    _128CYCLES = 6,
    #[doc = "7: 256 conversion clock cycles"]
    _256CYCLES = 7,
}
impl From<REPDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: REPDELAY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REPDELAY_A {
    type Ux = u8;
}
impl REPDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REPDELAY_A {
        match self.bits {
            0 => REPDELAY_A::NODELAY,
            1 => REPDELAY_A::_4CYCLES,
            2 => REPDELAY_A::_8CYCLES,
            3 => REPDELAY_A::_16CYCLES,
            4 => REPDELAY_A::_32CYCLES,
            5 => REPDELAY_A::_64CYCLES,
            6 => REPDELAY_A::_128CYCLES,
            7 => REPDELAY_A::_256CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "No delay"]
    #[inline(always)]
    pub fn is_nodelay(&self) -> bool {
        *self == REPDELAY_A::NODELAY
    }
    #[doc = "4 conversion clock cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == REPDELAY_A::_4CYCLES
    }
    #[doc = "8 conversion clock cycles"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == REPDELAY_A::_8CYCLES
    }
    #[doc = "16 conversion clock cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == REPDELAY_A::_16CYCLES
    }
    #[doc = "32 conversion clock cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == REPDELAY_A::_32CYCLES
    }
    #[doc = "64 conversion clock cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == REPDELAY_A::_64CYCLES
    }
    #[doc = "128 conversion clock cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == REPDELAY_A::_128CYCLES
    }
    #[doc = "256 conversion clock cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == REPDELAY_A::_256CYCLES
    }
}
#[doc = "Field `REPDELAY` writer - REPDELAY Select for SINGLE REP Mode"]
pub type REPDELAY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, REPDELAY_A>;
impl<'a, REG> REPDELAY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No delay"]
    #[inline(always)]
    pub fn nodelay(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::NODELAY)
    }
    #[doc = "4 conversion clock cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(REPDELAY_A::_256CYCLES)
    }
}
impl R {
    #[doc = "Bits 0:2 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&self) -> VREFATTFIX_R {
        VREFATTFIX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&self) -> VREFATT_R {
        VREFATT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&self) -> VINATT_R {
        VINATT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Single Channel DV Level Select"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Single Channel FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&self) -> FIFOOFACT_R {
        FIFOOFACT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Channel PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Single Channel PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&self) -> CONVSTARTDELAY_R {
        CONVSTARTDELAY_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&self) -> CONVSTARTDELAYEN_R {
        CONVSTARTDELAYEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SINGLE REP Mode"]
    #[inline(always)]
    pub fn repdelay(&self) -> REPDELAY_R {
        REPDELAY_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Single Channel Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<SINGLECTRLX_SPEC> {
        VREFSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefattfix(&mut self) -> VREFATTFIX_W<SINGLECTRLX_SPEC> {
        VREFATTFIX_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    #[must_use]
    pub fn vrefatt(&mut self) -> VREFATT_W<SINGLECTRLX_SPEC> {
        VREFATT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn vinatt(&mut self) -> VINATT_W<SINGLECTRLX_SPEC> {
        VINATT_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Single Channel DV Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn dvl(&mut self) -> DVL_W<SINGLECTRLX_SPEC> {
        DVL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Single Channel FIFO Overflow Action"]
    #[inline(always)]
    #[must_use]
    pub fn fifoofact(&mut self) -> FIFOOFACT_W<SINGLECTRLX_SPEC> {
        FIFOOFACT_W::new(self, 14)
    }
    #[doc = "Bit 16 - Single Channel PRS Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PRSMODE_W<SINGLECTRLX_SPEC> {
        PRSMODE_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - Single Channel PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<SINGLECTRLX_SPEC> {
        PRSSEL_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelay(&mut self) -> CONVSTARTDELAY_W<SINGLECTRLX_SPEC> {
        CONVSTARTDELAY_W::new(self, 22)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn convstartdelayen(&mut self) -> CONVSTARTDELAYEN_W<SINGLECTRLX_SPEC> {
        CONVSTARTDELAYEN_W::new(self, 27)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SINGLE REP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn repdelay(&mut self) -> REPDELAY_W<SINGLECTRLX_SPEC> {
        REPDELAY_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Single Channel Control Register Continued\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlectrlx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlectrlx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLECTRLX_SPEC;
impl crate::RegisterSpec for SINGLECTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlectrlx::R`](R) reader structure"]
impl crate::Readable for SINGLECTRLX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`singlectrlx::W`](W) writer structure"]
impl crate::Writable for SINGLECTRLX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINGLECTRLX to value 0"]
impl crate::Resettable for SINGLECTRLX_SPEC {
    const RESET_VALUE: u32 = 0;
}
