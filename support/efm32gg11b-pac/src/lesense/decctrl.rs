#[doc = "Register `DECCTRL` reader"]
pub type R = crate::R<DecctrlSpec>;
#[doc = "Register `DECCTRL` writer"]
pub type W = crate::W<DecctrlSpec>;
#[doc = "Field `DISABLE` reader - Disable the Decoder"]
pub type DisableR = crate::BitReader;
#[doc = "Field `DISABLE` writer - Disable the Decoder"]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRCHK` reader - Enable Check of Current State"]
pub type ErrchkR = crate::BitReader;
#[doc = "Field `ERRCHK` writer - Enable Check of Current State"]
pub type ErrchkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMAP` reader - Enable Decoder to Channel Interrupt Mapping"]
pub type IntmapR = crate::BitReader;
#[doc = "Field `INTMAP` writer - Enable Decoder to Channel Interrupt Mapping"]
pub type IntmapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS0` reader - Enable Decoder Hysteresis on PRS0 Output"]
pub type Hystprs0R = crate::BitReader;
#[doc = "Field `HYSTPRS0` writer - Enable Decoder Hysteresis on PRS0 Output"]
pub type Hystprs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS1` reader - Enable Decoder Hysteresis on PRS1 Output"]
pub type Hystprs1R = crate::BitReader;
#[doc = "Field `HYSTPRS1` writer - Enable Decoder Hysteresis on PRS1 Output"]
pub type Hystprs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS2` reader - Enable Decoder Hysteresis on PRS2 Output"]
pub type Hystprs2R = crate::BitReader;
#[doc = "Field `HYSTPRS2` writer - Enable Decoder Hysteresis on PRS2 Output"]
pub type Hystprs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTIRQ` reader - Enable Decoder Hysteresis on Interrupt Requests"]
pub type HystirqR = crate::BitReader;
#[doc = "Field `HYSTIRQ` writer - Enable Decoder Hysteresis on Interrupt Requests"]
pub type HystirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSCNT` reader - Enable Count Mode on Decoder PRS Channels 0 and 1"]
pub type PrscntR = crate::BitReader;
#[doc = "Field `PRSCNT` writer - Enable Count Mode on Decoder PRS Channels 0 and 1"]
pub type PrscntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT` reader - LESENSE Decoder Input Configuration"]
pub type InputR = crate::BitReader;
#[doc = "Field `INPUT` writer - LESENSE Decoder Input Configuration"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LESENSE Decoder PRS Input 0 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel0 {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    Prsch23 = 23,
}
impl From<Prssel0> for u8 {
    #[inline(always)]
    fn from(variant: Prssel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel0 {
    type Ux = u8;
}
impl crate::IsEnum for Prssel0 {}
#[doc = "Field `PRSSEL0` reader - LESENSE Decoder PRS Input 0 Configuration"]
pub type Prssel0R = crate::FieldReader<Prssel0>;
impl Prssel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel0> {
        match self.bits {
            0 => Some(Prssel0::Prsch0),
            1 => Some(Prssel0::Prsch1),
            2 => Some(Prssel0::Prsch2),
            3 => Some(Prssel0::Prsch3),
            4 => Some(Prssel0::Prsch4),
            5 => Some(Prssel0::Prsch5),
            6 => Some(Prssel0::Prsch6),
            7 => Some(Prssel0::Prsch7),
            8 => Some(Prssel0::Prsch8),
            9 => Some(Prssel0::Prsch9),
            10 => Some(Prssel0::Prsch10),
            11 => Some(Prssel0::Prsch11),
            12 => Some(Prssel0::Prsch12),
            13 => Some(Prssel0::Prsch13),
            14 => Some(Prssel0::Prsch14),
            15 => Some(Prssel0::Prsch15),
            16 => Some(Prssel0::Prsch16),
            17 => Some(Prssel0::Prsch17),
            18 => Some(Prssel0::Prsch18),
            19 => Some(Prssel0::Prsch19),
            20 => Some(Prssel0::Prsch20),
            21 => Some(Prssel0::Prsch21),
            22 => Some(Prssel0::Prsch22),
            23 => Some(Prssel0::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel0::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel0::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel0::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel0::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel0::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel0::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel0::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel0::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel0::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel0::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel0::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel0::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel0::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel0::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel0::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel0::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel0::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel0::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel0::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel0::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel0::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel0::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel0::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel0::Prsch23
    }
}
#[doc = "Field `PRSSEL0` writer - LESENSE Decoder PRS Input 0 Configuration"]
pub type Prssel0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel0>;
impl<'a, REG> Prssel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel0::Prsch23)
    }
}
#[doc = "LESENSE Decoder PRS Input 1 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel1 {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    Prsch23 = 23,
}
impl From<Prssel1> for u8 {
    #[inline(always)]
    fn from(variant: Prssel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel1 {
    type Ux = u8;
}
impl crate::IsEnum for Prssel1 {}
#[doc = "Field `PRSSEL1` reader - LESENSE Decoder PRS Input 1 Configuration"]
pub type Prssel1R = crate::FieldReader<Prssel1>;
impl Prssel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel1> {
        match self.bits {
            0 => Some(Prssel1::Prsch0),
            1 => Some(Prssel1::Prsch1),
            2 => Some(Prssel1::Prsch2),
            3 => Some(Prssel1::Prsch3),
            4 => Some(Prssel1::Prsch4),
            5 => Some(Prssel1::Prsch5),
            6 => Some(Prssel1::Prsch6),
            7 => Some(Prssel1::Prsch7),
            8 => Some(Prssel1::Prsch8),
            9 => Some(Prssel1::Prsch9),
            10 => Some(Prssel1::Prsch10),
            11 => Some(Prssel1::Prsch11),
            12 => Some(Prssel1::Prsch12),
            13 => Some(Prssel1::Prsch13),
            14 => Some(Prssel1::Prsch14),
            15 => Some(Prssel1::Prsch15),
            16 => Some(Prssel1::Prsch16),
            17 => Some(Prssel1::Prsch17),
            18 => Some(Prssel1::Prsch18),
            19 => Some(Prssel1::Prsch19),
            20 => Some(Prssel1::Prsch20),
            21 => Some(Prssel1::Prsch21),
            22 => Some(Prssel1::Prsch22),
            23 => Some(Prssel1::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel1::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel1::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel1::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel1::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel1::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel1::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel1::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel1::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel1::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel1::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel1::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel1::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel1::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel1::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel1::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel1::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel1::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel1::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel1::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel1::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel1::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel1::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel1::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel1::Prsch23
    }
}
#[doc = "Field `PRSSEL1` writer - LESENSE Decoder PRS Input 1 Configuration"]
pub type Prssel1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel1>;
impl<'a, REG> Prssel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel1::Prsch23)
    }
}
#[doc = "LESENSE Decoder PRS Input 2 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel2 {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    Prsch23 = 23,
}
impl From<Prssel2> for u8 {
    #[inline(always)]
    fn from(variant: Prssel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel2 {
    type Ux = u8;
}
impl crate::IsEnum for Prssel2 {}
#[doc = "Field `PRSSEL2` reader - LESENSE Decoder PRS Input 2 Configuration"]
pub type Prssel2R = crate::FieldReader<Prssel2>;
impl Prssel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel2> {
        match self.bits {
            0 => Some(Prssel2::Prsch0),
            1 => Some(Prssel2::Prsch1),
            2 => Some(Prssel2::Prsch2),
            3 => Some(Prssel2::Prsch3),
            4 => Some(Prssel2::Prsch4),
            5 => Some(Prssel2::Prsch5),
            6 => Some(Prssel2::Prsch6),
            7 => Some(Prssel2::Prsch7),
            8 => Some(Prssel2::Prsch8),
            9 => Some(Prssel2::Prsch9),
            10 => Some(Prssel2::Prsch10),
            11 => Some(Prssel2::Prsch11),
            12 => Some(Prssel2::Prsch12),
            13 => Some(Prssel2::Prsch13),
            14 => Some(Prssel2::Prsch14),
            15 => Some(Prssel2::Prsch15),
            16 => Some(Prssel2::Prsch16),
            17 => Some(Prssel2::Prsch17),
            18 => Some(Prssel2::Prsch18),
            19 => Some(Prssel2::Prsch19),
            20 => Some(Prssel2::Prsch20),
            21 => Some(Prssel2::Prsch21),
            22 => Some(Prssel2::Prsch22),
            23 => Some(Prssel2::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel2::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel2::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel2::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel2::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel2::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel2::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel2::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel2::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel2::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel2::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel2::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel2::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel2::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel2::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel2::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel2::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel2::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel2::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel2::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel2::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel2::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel2::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel2::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel2::Prsch23
    }
}
#[doc = "Field `PRSSEL2` writer - LESENSE Decoder PRS Input 2 Configuration"]
pub type Prssel2W<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel2>;
impl<'a, REG> Prssel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel2::Prsch23)
    }
}
#[doc = "LESENSE Decoder PRS Input 3 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel3 {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    Prsch23 = 23,
}
impl From<Prssel3> for u8 {
    #[inline(always)]
    fn from(variant: Prssel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel3 {
    type Ux = u8;
}
impl crate::IsEnum for Prssel3 {}
#[doc = "Field `PRSSEL3` reader - LESENSE Decoder PRS Input 3 Configuration"]
pub type Prssel3R = crate::FieldReader<Prssel3>;
impl Prssel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel3> {
        match self.bits {
            0 => Some(Prssel3::Prsch0),
            1 => Some(Prssel3::Prsch1),
            2 => Some(Prssel3::Prsch2),
            3 => Some(Prssel3::Prsch3),
            4 => Some(Prssel3::Prsch4),
            5 => Some(Prssel3::Prsch5),
            6 => Some(Prssel3::Prsch6),
            7 => Some(Prssel3::Prsch7),
            8 => Some(Prssel3::Prsch8),
            9 => Some(Prssel3::Prsch9),
            10 => Some(Prssel3::Prsch10),
            11 => Some(Prssel3::Prsch11),
            12 => Some(Prssel3::Prsch12),
            13 => Some(Prssel3::Prsch13),
            14 => Some(Prssel3::Prsch14),
            15 => Some(Prssel3::Prsch15),
            16 => Some(Prssel3::Prsch16),
            17 => Some(Prssel3::Prsch17),
            18 => Some(Prssel3::Prsch18),
            19 => Some(Prssel3::Prsch19),
            20 => Some(Prssel3::Prsch20),
            21 => Some(Prssel3::Prsch21),
            22 => Some(Prssel3::Prsch22),
            23 => Some(Prssel3::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel3::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel3::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel3::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel3::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel3::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel3::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel3::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel3::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel3::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel3::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel3::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel3::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel3::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel3::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel3::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel3::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel3::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel3::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel3::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel3::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel3::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel3::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel3::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel3::Prsch23
    }
}
#[doc = "Field `PRSSEL3` writer - LESENSE Decoder PRS Input 3 Configuration"]
pub type Prssel3W<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel3>;
impl<'a, REG> Prssel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel3::Prsch23)
    }
}
impl R {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&self) -> ErrchkR {
        ErrchkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&self) -> IntmapR {
        IntmapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> Hystprs0R {
        Hystprs0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> Hystprs1R {
        Hystprs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> Hystprs2R {
        Hystprs2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&self) -> HystirqR {
        HystirqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&self) -> PrscntR {
        PrscntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&self) -> InputR {
        InputR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&self) -> Prssel0R {
        Prssel0R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&self) -> Prssel1R {
        Prssel1R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&self) -> Prssel2R {
        Prssel2R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&self) -> Prssel3R {
        Prssel3R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, DecctrlSpec> {
        DisableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&mut self) -> ErrchkW<'_, DecctrlSpec> {
        ErrchkW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&mut self) -> IntmapW<'_, DecctrlSpec> {
        IntmapW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&mut self) -> Hystprs0W<'_, DecctrlSpec> {
        Hystprs0W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&mut self) -> Hystprs1W<'_, DecctrlSpec> {
        Hystprs1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&mut self) -> Hystprs2W<'_, DecctrlSpec> {
        Hystprs2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&mut self) -> HystirqW<'_, DecctrlSpec> {
        HystirqW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&mut self) -> PrscntW<'_, DecctrlSpec> {
        PrscntW::new(self, 7)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&mut self) -> InputW<'_, DecctrlSpec> {
        InputW::new(self, 8)
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&mut self) -> Prssel0W<'_, DecctrlSpec> {
        Prssel0W::new(self, 10)
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&mut self) -> Prssel1W<'_, DecctrlSpec> {
        Prssel1W::new(self, 15)
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&mut self) -> Prssel2W<'_, DecctrlSpec> {
        Prssel2W::new(self, 20)
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&mut self) -> Prssel3W<'_, DecctrlSpec> {
        Prssel3W::new(self, 25)
    }
}
#[doc = "Decoder Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`decctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecctrlSpec;
impl crate::RegisterSpec for DecctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decctrl::R`](R) reader structure"]
impl crate::Readable for DecctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`decctrl::W`](W) writer structure"]
impl crate::Writable for DecctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECCTRL to value 0"]
impl crate::Resettable for DecctrlSpec {}
