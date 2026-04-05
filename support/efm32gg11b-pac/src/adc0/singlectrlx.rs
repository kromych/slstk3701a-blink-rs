#[doc = "Register `SINGLECTRLX` reader"]
pub type R = crate::R<SinglectrlxSpec>;
#[doc = "Register `SINGLECTRLX` writer"]
pub type W = crate::W<SinglectrlxSpec>;
#[doc = "Single Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vrefsel {
    #[doc = "0: Internal 0.83V Bandgap reference"]
    Vbgr = 0,
    #[doc = "1: Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    Vddxwatt = 1,
    #[doc = "2: Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    Vrefpwatt = 2,
    #[doc = "3: Raw single ended external Vref: ADCn_EXTP"]
    Vrefp = 3,
    #[doc = "4: Special mode used to generate ENTROPY."]
    Ventropy = 4,
    #[doc = "5: Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    Vrefpnwatt = 5,
    #[doc = "6: Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    Vrefpn = 6,
    #[doc = "7: Internal Bandgap reference at low setting 0.78V"]
    Vbgrlow = 7,
}
impl From<Vrefsel> for u8 {
    #[inline(always)]
    fn from(variant: Vrefsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vrefsel {
    type Ux = u8;
}
impl crate::IsEnum for Vrefsel {}
#[doc = "Field `VREFSEL` reader - Single Channel Reference Selection"]
pub type VrefselR = crate::FieldReader<Vrefsel>;
impl VrefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefsel {
        match self.bits {
            0 => Vrefsel::Vbgr,
            1 => Vrefsel::Vddxwatt,
            2 => Vrefsel::Vrefpwatt,
            3 => Vrefsel::Vrefp,
            4 => Vrefsel::Ventropy,
            5 => Vrefsel::Vrefpnwatt,
            6 => Vrefsel::Vrefpn,
            7 => Vrefsel::Vbgrlow,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == Vrefsel::Vbgr
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vddxwatt(&self) -> bool {
        *self == Vrefsel::Vddxwatt
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpwatt(&self) -> bool {
        *self == Vrefsel::Vrefpwatt
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == Vrefsel::Vrefp
    }
    #[doc = "Special mode used to generate ENTROPY."]
    #[inline(always)]
    pub fn is_ventropy(&self) -> bool {
        *self == Vrefsel::Ventropy
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn is_vrefpnwatt(&self) -> bool {
        *self == Vrefsel::Vrefpnwatt
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn is_vrefpn(&self) -> bool {
        *self == Vrefsel::Vrefpn
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn is_vbgrlow(&self) -> bool {
        *self == Vrefsel::Vbgrlow
    }
}
#[doc = "Field `VREFSEL` writer - Single Channel Reference Selection"]
pub type VrefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vrefsel, crate::Safe>;
impl<'a, REG> VrefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 0.83V Bandgap reference"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vbgr)
    }
    #[doc = "Scaled AVDD: AVDD*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vddxwatt(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vddxwatt)
    }
    #[doc = "Scaled singled ended external Vref: ADCn_EXTP*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpwatt(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vrefpwatt)
    }
    #[doc = "Raw single ended external Vref: ADCn_EXTP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vrefp)
    }
    #[doc = "Special mode used to generate ENTROPY."]
    #[inline(always)]
    pub fn ventropy(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Ventropy)
    }
    #[doc = "Scaled differential external Vref from : (ADCn_EXTP-ADCn_EXTN)*(the VREF attenuation factor)"]
    #[inline(always)]
    pub fn vrefpnwatt(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vrefpnwatt)
    }
    #[doc = "Raw differential external Vref from : (ADCn_EXTP-ADCn_EXTN)"]
    #[inline(always)]
    pub fn vrefpn(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vrefpn)
    }
    #[doc = "Internal Bandgap reference at low setting 0.78V"]
    #[inline(always)]
    pub fn vbgrlow(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefsel::Vbgrlow)
    }
}
#[doc = "Field `VREFATTFIX` reader - Enable Fixed Scaling on VREF"]
pub type VrefattfixR = crate::BitReader;
#[doc = "Field `VREFATTFIX` writer - Enable Fixed Scaling on VREF"]
pub type VrefattfixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFATT` reader - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VrefattR = crate::FieldReader;
#[doc = "Field `VREFATT` writer - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
pub type VrefattW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VINATT` reader - Code for VIN Attenuation Factor"]
pub type VinattR = crate::FieldReader;
#[doc = "Field `VINATT` writer - Code for VIN Attenuation Factor"]
pub type VinattW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DVL` reader - Single Channel DV Level Select"]
pub type DvlR = crate::FieldReader;
#[doc = "Field `DVL` writer - Single Channel DV Level Select"]
pub type DvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIFOOFACT` reader - Single Channel FIFO Overflow Action"]
pub type FifoofactR = crate::BitReader;
#[doc = "Field `FIFOOFACT` writer - Single Channel FIFO Overflow Action"]
pub type FifoofactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSMODE` reader - Single Channel PRS Trigger Mode"]
pub type PrsmodeR = crate::BitReader;
#[doc = "Field `PRSMODE` writer - Single Channel PRS Trigger Mode"]
pub type PrsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Single Channel PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS ch 0 triggers single channel"]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 triggers single channel"]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 triggers single channel"]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 triggers single channel"]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 triggers single channel"]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 triggers single channel"]
    Prsch5 = 5,
    #[doc = "6: PRS ch 6 triggers single channel"]
    Prsch6 = 6,
    #[doc = "7: PRS ch 7 triggers single channel"]
    Prsch7 = 7,
    #[doc = "8: PRS ch 8 triggers single channel"]
    Prsch8 = 8,
    #[doc = "9: PRS ch 9 triggers single channel"]
    Prsch9 = 9,
    #[doc = "10: PRS ch 10 triggers single channel"]
    Prsch10 = 10,
    #[doc = "11: PRS ch 11 triggers single channel"]
    Prsch11 = 11,
    #[doc = "12: PRS ch 12 triggers single channel"]
    Prsch12 = 12,
    #[doc = "13: PRS ch 13 triggers single channel"]
    Prsch13 = 13,
    #[doc = "14: PRS ch 14 triggers single channel"]
    Prsch14 = 14,
    #[doc = "15: PRS ch 15 triggers single channel"]
    Prsch15 = 15,
    #[doc = "16: PRS ch 16 triggers single channel"]
    Prsch16 = 16,
    #[doc = "17: PRS ch 17 triggers single channel"]
    Prsch17 = 17,
    #[doc = "18: PRS ch 18 triggers single channel"]
    Prsch18 = 18,
    #[doc = "19: PRS ch 19 triggers single channel"]
    Prsch19 = 19,
    #[doc = "20: PRS ch 20 triggers single channel"]
    Prsch20 = 20,
    #[doc = "21: PRS ch 21 triggers single channel"]
    Prsch21 = 21,
    #[doc = "22: PRS ch 22 triggers single channel"]
    Prsch22 = 22,
    #[doc = "23: PRS ch 23 triggers single channel"]
    Prsch23 = 23,
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(variant: Prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel {
    type Ux = u8;
}
impl crate::IsEnum for Prssel {}
#[doc = "Field `PRSSEL` reader - Single Channel PRS Trigger Select"]
pub type PrsselR = crate::FieldReader<Prssel>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel> {
        match self.bits {
            0 => Some(Prssel::Prsch0),
            1 => Some(Prssel::Prsch1),
            2 => Some(Prssel::Prsch2),
            3 => Some(Prssel::Prsch3),
            4 => Some(Prssel::Prsch4),
            5 => Some(Prssel::Prsch5),
            6 => Some(Prssel::Prsch6),
            7 => Some(Prssel::Prsch7),
            8 => Some(Prssel::Prsch8),
            9 => Some(Prssel::Prsch9),
            10 => Some(Prssel::Prsch10),
            11 => Some(Prssel::Prsch11),
            12 => Some(Prssel::Prsch12),
            13 => Some(Prssel::Prsch13),
            14 => Some(Prssel::Prsch14),
            15 => Some(Prssel::Prsch15),
            16 => Some(Prssel::Prsch16),
            17 => Some(Prssel::Prsch17),
            18 => Some(Prssel::Prsch18),
            19 => Some(Prssel::Prsch19),
            20 => Some(Prssel::Prsch20),
            21 => Some(Prssel::Prsch21),
            22 => Some(Prssel::Prsch22),
            23 => Some(Prssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS ch 1 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS ch 2 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS ch 3 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS ch 4 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS ch 5 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
    #[doc = "PRS ch 6 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel::Prsch6
    }
    #[doc = "PRS ch 7 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel::Prsch7
    }
    #[doc = "PRS ch 8 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel::Prsch8
    }
    #[doc = "PRS ch 9 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel::Prsch9
    }
    #[doc = "PRS ch 10 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel::Prsch10
    }
    #[doc = "PRS ch 11 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel::Prsch11
    }
    #[doc = "PRS ch 12 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel::Prsch12
    }
    #[doc = "PRS ch 13 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel::Prsch13
    }
    #[doc = "PRS ch 14 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel::Prsch14
    }
    #[doc = "PRS ch 15 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel::Prsch15
    }
    #[doc = "PRS ch 16 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel::Prsch16
    }
    #[doc = "PRS ch 17 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel::Prsch17
    }
    #[doc = "PRS ch 18 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel::Prsch18
    }
    #[doc = "PRS ch 19 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel::Prsch19
    }
    #[doc = "PRS ch 20 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel::Prsch20
    }
    #[doc = "PRS ch 21 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel::Prsch21
    }
    #[doc = "PRS ch 22 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel::Prsch22
    }
    #[doc = "PRS ch 23 triggers single channel"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel::Prsch23
    }
}
#[doc = "Field `PRSSEL` writer - Single Channel PRS Trigger Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers single channel"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS ch 1 triggers single channel"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS ch 2 triggers single channel"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS ch 3 triggers single channel"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS ch 4 triggers single channel"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS ch 5 triggers single channel"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
    #[doc = "PRS ch 6 triggers single channel"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch6)
    }
    #[doc = "PRS ch 7 triggers single channel"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch7)
    }
    #[doc = "PRS ch 8 triggers single channel"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch8)
    }
    #[doc = "PRS ch 9 triggers single channel"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch9)
    }
    #[doc = "PRS ch 10 triggers single channel"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch10)
    }
    #[doc = "PRS ch 11 triggers single channel"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch11)
    }
    #[doc = "PRS ch 12 triggers single channel"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch12)
    }
    #[doc = "PRS ch 13 triggers single channel"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch13)
    }
    #[doc = "PRS ch 14 triggers single channel"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch14)
    }
    #[doc = "PRS ch 15 triggers single channel"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch15)
    }
    #[doc = "PRS ch 16 triggers single channel"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch16)
    }
    #[doc = "PRS ch 17 triggers single channel"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch17)
    }
    #[doc = "PRS ch 18 triggers single channel"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch18)
    }
    #[doc = "PRS ch 19 triggers single channel"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch19)
    }
    #[doc = "PRS ch 20 triggers single channel"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch20)
    }
    #[doc = "PRS ch 21 triggers single channel"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch21)
    }
    #[doc = "PRS ch 22 triggers single channel"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch22)
    }
    #[doc = "PRS ch 23 triggers single channel"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch23)
    }
}
#[doc = "Field `CONVSTARTDELAY` reader - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type ConvstartdelayR = crate::FieldReader;
#[doc = "Field `CONVSTARTDELAY` writer - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
pub type ConvstartdelayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CONVSTARTDELAYEN` reader - Enable Delaying Next Conversion Start"]
pub type ConvstartdelayenR = crate::BitReader;
#[doc = "Field `CONVSTARTDELAYEN` writer - Enable Delaying Next Conversion Start"]
pub type ConvstartdelayenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "REPDELAY Select for SINGLE REP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Repdelay {
    #[doc = "0: No delay"]
    Nodelay = 0,
    #[doc = "1: 4 conversion clock cycles"]
    _4cycles = 1,
    #[doc = "2: 8 conversion clock cycles"]
    _8cycles = 2,
    #[doc = "3: 16 conversion clock cycles"]
    _16cycles = 3,
    #[doc = "4: 32 conversion clock cycles"]
    _32cycles = 4,
    #[doc = "5: 64 conversion clock cycles"]
    _64cycles = 5,
    #[doc = "6: 128 conversion clock cycles"]
    _128cycles = 6,
    #[doc = "7: 256 conversion clock cycles"]
    _256cycles = 7,
}
impl From<Repdelay> for u8 {
    #[inline(always)]
    fn from(variant: Repdelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Repdelay {
    type Ux = u8;
}
impl crate::IsEnum for Repdelay {}
#[doc = "Field `REPDELAY` reader - REPDELAY Select for SINGLE REP Mode"]
pub type RepdelayR = crate::FieldReader<Repdelay>;
impl RepdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Repdelay {
        match self.bits {
            0 => Repdelay::Nodelay,
            1 => Repdelay::_4cycles,
            2 => Repdelay::_8cycles,
            3 => Repdelay::_16cycles,
            4 => Repdelay::_32cycles,
            5 => Repdelay::_64cycles,
            6 => Repdelay::_128cycles,
            7 => Repdelay::_256cycles,
            _ => unreachable!(),
        }
    }
    #[doc = "No delay"]
    #[inline(always)]
    pub fn is_nodelay(&self) -> bool {
        *self == Repdelay::Nodelay
    }
    #[doc = "4 conversion clock cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Repdelay::_4cycles
    }
    #[doc = "8 conversion clock cycles"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == Repdelay::_8cycles
    }
    #[doc = "16 conversion clock cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Repdelay::_16cycles
    }
    #[doc = "32 conversion clock cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Repdelay::_32cycles
    }
    #[doc = "64 conversion clock cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Repdelay::_64cycles
    }
    #[doc = "128 conversion clock cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Repdelay::_128cycles
    }
    #[doc = "256 conversion clock cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Repdelay::_256cycles
    }
}
#[doc = "Field `REPDELAY` writer - REPDELAY Select for SINGLE REP Mode"]
pub type RepdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3, Repdelay, crate::Safe>;
impl<'a, REG> RepdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No delay"]
    #[inline(always)]
    pub fn nodelay(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::Nodelay)
    }
    #[doc = "4 conversion clock cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_4cycles)
    }
    #[doc = "8 conversion clock cycles"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_8cycles)
    }
    #[doc = "16 conversion clock cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_16cycles)
    }
    #[doc = "32 conversion clock cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_32cycles)
    }
    #[doc = "64 conversion clock cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_64cycles)
    }
    #[doc = "128 conversion clock cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_128cycles)
    }
    #[doc = "256 conversion clock cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Repdelay::_256cycles)
    }
}
impl R {
    #[doc = "Bits 0:2 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VrefselR {
        VrefselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&self) -> VrefattfixR {
        VrefattfixR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&self) -> VrefattR {
        VrefattR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&self) -> VinattR {
        VinattR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Single Channel DV Level Select"]
    #[inline(always)]
    pub fn dvl(&self) -> DvlR {
        DvlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Single Channel FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&self) -> FifoofactR {
        FifoofactR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Channel PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PrsmodeR {
        PrsmodeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Single Channel PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&self) -> ConvstartdelayR {
        ConvstartdelayR::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&self) -> ConvstartdelayenR {
        ConvstartdelayenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SINGLE REP Mode"]
    #[inline(always)]
    pub fn repdelay(&self) -> RepdelayR {
        RepdelayR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn vrefsel(&mut self) -> VrefselW<'_, SinglectrlxSpec> {
        VrefselW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Fixed Scaling on VREF"]
    #[inline(always)]
    pub fn vrefattfix(&mut self) -> VrefattfixW<'_, SinglectrlxSpec> {
        VrefattfixW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Code for VREF Attenuation Factor When VREFSEL is 1, 2 or 5"]
    #[inline(always)]
    pub fn vrefatt(&mut self) -> VrefattW<'_, SinglectrlxSpec> {
        VrefattW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Code for VIN Attenuation Factor"]
    #[inline(always)]
    pub fn vinatt(&mut self) -> VinattW<'_, SinglectrlxSpec> {
        VinattW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Single Channel DV Level Select"]
    #[inline(always)]
    pub fn dvl(&mut self) -> DvlW<'_, SinglectrlxSpec> {
        DvlW::new(self, 12)
    }
    #[doc = "Bit 14 - Single Channel FIFO Overflow Action"]
    #[inline(always)]
    pub fn fifoofact(&mut self) -> FifoofactW<'_, SinglectrlxSpec> {
        FifoofactW::new(self, 14)
    }
    #[doc = "Bit 16 - Single Channel PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&mut self) -> PrsmodeW<'_, SinglectrlxSpec> {
        PrsmodeW::new(self, 16)
    }
    #[doc = "Bits 17:21 - Single Channel PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, SinglectrlxSpec> {
        PrsselW::new(self, 17)
    }
    #[doc = "Bits 22:26 - Delay Value for Next Conversion Start If CONVSTARTDELAYEN is Set"]
    #[inline(always)]
    pub fn convstartdelay(&mut self) -> ConvstartdelayW<'_, SinglectrlxSpec> {
        ConvstartdelayW::new(self, 22)
    }
    #[doc = "Bit 27 - Enable Delaying Next Conversion Start"]
    #[inline(always)]
    pub fn convstartdelayen(&mut self) -> ConvstartdelayenW<'_, SinglectrlxSpec> {
        ConvstartdelayenW::new(self, 27)
    }
    #[doc = "Bits 29:31 - REPDELAY Select for SINGLE REP Mode"]
    #[inline(always)]
    pub fn repdelay(&mut self) -> RepdelayW<'_, SinglectrlxSpec> {
        RepdelayW::new(self, 29)
    }
}
#[doc = "Single Channel Control Register Continued\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrlx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrlx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglectrlxSpec;
impl crate::RegisterSpec for SinglectrlxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlectrlx::R`](R) reader structure"]
impl crate::Readable for SinglectrlxSpec {}
#[doc = "`write(|w| ..)` method takes [`singlectrlx::W`](W) writer structure"]
impl crate::Writable for SinglectrlxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLECTRLX to value 0"]
impl crate::Resettable for SinglectrlxSpec {}
