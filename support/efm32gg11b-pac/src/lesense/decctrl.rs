#[doc = "Register `DECCTRL` reader"]
pub type R = crate::R<DECCTRL_SPEC>;
#[doc = "Register `DECCTRL` writer"]
pub type W = crate::W<DECCTRL_SPEC>;
#[doc = "Field `DISABLE` reader - Disable the Decoder"]
pub type DISABLE_R = crate::BitReader;
#[doc = "Field `DISABLE` writer - Disable the Decoder"]
pub type DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRCHK` reader - Enable Check of Current State"]
pub type ERRCHK_R = crate::BitReader;
#[doc = "Field `ERRCHK` writer - Enable Check of Current State"]
pub type ERRCHK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMAP` reader - Enable Decoder to Channel Interrupt Mapping"]
pub type INTMAP_R = crate::BitReader;
#[doc = "Field `INTMAP` writer - Enable Decoder to Channel Interrupt Mapping"]
pub type INTMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS0` reader - Enable Decoder Hysteresis on PRS0 Output"]
pub type HYSTPRS0_R = crate::BitReader;
#[doc = "Field `HYSTPRS0` writer - Enable Decoder Hysteresis on PRS0 Output"]
pub type HYSTPRS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS1` reader - Enable Decoder Hysteresis on PRS1 Output"]
pub type HYSTPRS1_R = crate::BitReader;
#[doc = "Field `HYSTPRS1` writer - Enable Decoder Hysteresis on PRS1 Output"]
pub type HYSTPRS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS2` reader - Enable Decoder Hysteresis on PRS2 Output"]
pub type HYSTPRS2_R = crate::BitReader;
#[doc = "Field `HYSTPRS2` writer - Enable Decoder Hysteresis on PRS2 Output"]
pub type HYSTPRS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTIRQ` reader - Enable Decoder Hysteresis on Interrupt Requests"]
pub type HYSTIRQ_R = crate::BitReader;
#[doc = "Field `HYSTIRQ` writer - Enable Decoder Hysteresis on Interrupt Requests"]
pub type HYSTIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSCNT` reader - Enable Count Mode on Decoder PRS Channels 0 and 1"]
pub type PRSCNT_R = crate::BitReader;
#[doc = "Field `PRSCNT` writer - Enable Count Mode on Decoder PRS Channels 0 and 1"]
pub type PRSCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT` reader - LESENSE Decoder Input Configuration"]
pub type INPUT_R = crate::BitReader;
#[doc = "Field `INPUT` writer - LESENSE Decoder Input Configuration"]
pub type INPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSSEL0` reader - LESENSE Decoder PRS Input 0 Configuration"]
pub type PRSSEL0_R = crate::FieldReader<PRSSEL0_A>;
#[doc = "LESENSE Decoder PRS Input 0 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL0_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    PRSCH23 = 23,
}
impl From<PRSSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL0_A {
    type Ux = u8;
}
impl PRSSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL0_A> {
        match self.bits {
            0 => Some(PRSSEL0_A::PRSCH0),
            1 => Some(PRSSEL0_A::PRSCH1),
            2 => Some(PRSSEL0_A::PRSCH2),
            3 => Some(PRSSEL0_A::PRSCH3),
            4 => Some(PRSSEL0_A::PRSCH4),
            5 => Some(PRSSEL0_A::PRSCH5),
            6 => Some(PRSSEL0_A::PRSCH6),
            7 => Some(PRSSEL0_A::PRSCH7),
            8 => Some(PRSSEL0_A::PRSCH8),
            9 => Some(PRSSEL0_A::PRSCH9),
            10 => Some(PRSSEL0_A::PRSCH10),
            11 => Some(PRSSEL0_A::PRSCH11),
            12 => Some(PRSSEL0_A::PRSCH12),
            13 => Some(PRSSEL0_A::PRSCH13),
            14 => Some(PRSSEL0_A::PRSCH14),
            15 => Some(PRSSEL0_A::PRSCH15),
            16 => Some(PRSSEL0_A::PRSCH16),
            17 => Some(PRSSEL0_A::PRSCH17),
            18 => Some(PRSSEL0_A::PRSCH18),
            19 => Some(PRSSEL0_A::PRSCH19),
            20 => Some(PRSSEL0_A::PRSCH20),
            21 => Some(PRSSEL0_A::PRSCH21),
            22 => Some(PRSSEL0_A::PRSCH22),
            23 => Some(PRSSEL0_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL0_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL0_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL0_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL0_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL0_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL0_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL0_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL0_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL0_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL0_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL0_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL0_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL0_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL0_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL0_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL0_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL0_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL0_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL0_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL0_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL0_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL0_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL0_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL0_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL0` writer - LESENSE Decoder PRS Input 0 Configuration"]
pub type PRSSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL0_A>;
impl<'a, REG> PRSSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL0_A::PRSCH23)
    }
}
#[doc = "Field `PRSSEL1` reader - LESENSE Decoder PRS Input 1 Configuration"]
pub type PRSSEL1_R = crate::FieldReader<PRSSEL1_A>;
#[doc = "LESENSE Decoder PRS Input 1 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL1_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    PRSCH23 = 23,
}
impl From<PRSSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL1_A {
    type Ux = u8;
}
impl PRSSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL1_A> {
        match self.bits {
            0 => Some(PRSSEL1_A::PRSCH0),
            1 => Some(PRSSEL1_A::PRSCH1),
            2 => Some(PRSSEL1_A::PRSCH2),
            3 => Some(PRSSEL1_A::PRSCH3),
            4 => Some(PRSSEL1_A::PRSCH4),
            5 => Some(PRSSEL1_A::PRSCH5),
            6 => Some(PRSSEL1_A::PRSCH6),
            7 => Some(PRSSEL1_A::PRSCH7),
            8 => Some(PRSSEL1_A::PRSCH8),
            9 => Some(PRSSEL1_A::PRSCH9),
            10 => Some(PRSSEL1_A::PRSCH10),
            11 => Some(PRSSEL1_A::PRSCH11),
            12 => Some(PRSSEL1_A::PRSCH12),
            13 => Some(PRSSEL1_A::PRSCH13),
            14 => Some(PRSSEL1_A::PRSCH14),
            15 => Some(PRSSEL1_A::PRSCH15),
            16 => Some(PRSSEL1_A::PRSCH16),
            17 => Some(PRSSEL1_A::PRSCH17),
            18 => Some(PRSSEL1_A::PRSCH18),
            19 => Some(PRSSEL1_A::PRSCH19),
            20 => Some(PRSSEL1_A::PRSCH20),
            21 => Some(PRSSEL1_A::PRSCH21),
            22 => Some(PRSSEL1_A::PRSCH22),
            23 => Some(PRSSEL1_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL1_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL1_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL1_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL1_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL1_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL1_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL1_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL1_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL1_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL1_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL1_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL1_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL1_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL1_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL1_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL1_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL1_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL1_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL1_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL1_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL1_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL1_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL1_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL1_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL1` writer - LESENSE Decoder PRS Input 1 Configuration"]
pub type PRSSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL1_A>;
impl<'a, REG> PRSSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL1_A::PRSCH23)
    }
}
#[doc = "Field `PRSSEL2` reader - LESENSE Decoder PRS Input 2 Configuration"]
pub type PRSSEL2_R = crate::FieldReader<PRSSEL2_A>;
#[doc = "LESENSE Decoder PRS Input 2 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL2_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    PRSCH23 = 23,
}
impl From<PRSSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL2_A {
    type Ux = u8;
}
impl PRSSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL2_A> {
        match self.bits {
            0 => Some(PRSSEL2_A::PRSCH0),
            1 => Some(PRSSEL2_A::PRSCH1),
            2 => Some(PRSSEL2_A::PRSCH2),
            3 => Some(PRSSEL2_A::PRSCH3),
            4 => Some(PRSSEL2_A::PRSCH4),
            5 => Some(PRSSEL2_A::PRSCH5),
            6 => Some(PRSSEL2_A::PRSCH6),
            7 => Some(PRSSEL2_A::PRSCH7),
            8 => Some(PRSSEL2_A::PRSCH8),
            9 => Some(PRSSEL2_A::PRSCH9),
            10 => Some(PRSSEL2_A::PRSCH10),
            11 => Some(PRSSEL2_A::PRSCH11),
            12 => Some(PRSSEL2_A::PRSCH12),
            13 => Some(PRSSEL2_A::PRSCH13),
            14 => Some(PRSSEL2_A::PRSCH14),
            15 => Some(PRSSEL2_A::PRSCH15),
            16 => Some(PRSSEL2_A::PRSCH16),
            17 => Some(PRSSEL2_A::PRSCH17),
            18 => Some(PRSSEL2_A::PRSCH18),
            19 => Some(PRSSEL2_A::PRSCH19),
            20 => Some(PRSSEL2_A::PRSCH20),
            21 => Some(PRSSEL2_A::PRSCH21),
            22 => Some(PRSSEL2_A::PRSCH22),
            23 => Some(PRSSEL2_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL2_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL2_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL2_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL2_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL2_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL2_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL2_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL2_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL2_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL2_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL2_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL2_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL2_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL2_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL2_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL2_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL2_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL2_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL2_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL2_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL2_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL2_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL2_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL2_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL2` writer - LESENSE Decoder PRS Input 2 Configuration"]
pub type PRSSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL2_A>;
impl<'a, REG> PRSSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL2_A::PRSCH23)
    }
}
#[doc = "Field `PRSSEL3` reader - LESENSE Decoder PRS Input 3 Configuration"]
pub type PRSSEL3_R = crate::FieldReader<PRSSEL3_A>;
#[doc = "LESENSE Decoder PRS Input 3 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL3_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    PRSCH23 = 23,
}
impl From<PRSSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL3_A {
    type Ux = u8;
}
impl PRSSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL3_A> {
        match self.bits {
            0 => Some(PRSSEL3_A::PRSCH0),
            1 => Some(PRSSEL3_A::PRSCH1),
            2 => Some(PRSSEL3_A::PRSCH2),
            3 => Some(PRSSEL3_A::PRSCH3),
            4 => Some(PRSSEL3_A::PRSCH4),
            5 => Some(PRSSEL3_A::PRSCH5),
            6 => Some(PRSSEL3_A::PRSCH6),
            7 => Some(PRSSEL3_A::PRSCH7),
            8 => Some(PRSSEL3_A::PRSCH8),
            9 => Some(PRSSEL3_A::PRSCH9),
            10 => Some(PRSSEL3_A::PRSCH10),
            11 => Some(PRSSEL3_A::PRSCH11),
            12 => Some(PRSSEL3_A::PRSCH12),
            13 => Some(PRSSEL3_A::PRSCH13),
            14 => Some(PRSSEL3_A::PRSCH14),
            15 => Some(PRSSEL3_A::PRSCH15),
            16 => Some(PRSSEL3_A::PRSCH16),
            17 => Some(PRSSEL3_A::PRSCH17),
            18 => Some(PRSSEL3_A::PRSCH18),
            19 => Some(PRSSEL3_A::PRSCH19),
            20 => Some(PRSSEL3_A::PRSCH20),
            21 => Some(PRSSEL3_A::PRSCH21),
            22 => Some(PRSSEL3_A::PRSCH22),
            23 => Some(PRSSEL3_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL3_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL3_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL3_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL3_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL3_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL3_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL3_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL3_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL3_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL3_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL3_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL3_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL3_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL3_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL3_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL3_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL3_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL3_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL3_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL3_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL3_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL3_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL3_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL3_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL3` writer - LESENSE Decoder PRS Input 3 Configuration"]
pub type PRSSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL3_A>;
impl<'a, REG> PRSSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL3_A::PRSCH23)
    }
}
impl R {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    pub fn errchk(&self) -> ERRCHK_R {
        ERRCHK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    pub fn intmap(&self) -> INTMAP_R {
        INTMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> HYSTPRS0_R {
        HYSTPRS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> HYSTPRS1_R {
        HYSTPRS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> HYSTPRS2_R {
        HYSTPRS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    pub fn hystirq(&self) -> HYSTIRQ_R {
        HYSTIRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    pub fn prscnt(&self) -> PRSCNT_R {
        PRSCNT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    pub fn prssel0(&self) -> PRSSEL0_R {
        PRSSEL0_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    pub fn prssel1(&self) -> PRSSEL1_R {
        PRSSEL1_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    pub fn prssel2(&self) -> PRSSEL2_R {
        PRSSEL2_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    pub fn prssel3(&self) -> PRSSEL3_R {
        PRSSEL3_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the Decoder"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DISABLE_W<DECCTRL_SPEC> {
        DISABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Check of Current State"]
    #[inline(always)]
    #[must_use]
    pub fn errchk(&mut self) -> ERRCHK_W<DECCTRL_SPEC> {
        ERRCHK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Decoder to Channel Interrupt Mapping"]
    #[inline(always)]
    #[must_use]
    pub fn intmap(&mut self) -> INTMAP_W<DECCTRL_SPEC> {
        INTMAP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Decoder Hysteresis on PRS0 Output"]
    #[inline(always)]
    #[must_use]
    pub fn hystprs0(&mut self) -> HYSTPRS0_W<DECCTRL_SPEC> {
        HYSTPRS0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Decoder Hysteresis on PRS1 Output"]
    #[inline(always)]
    #[must_use]
    pub fn hystprs1(&mut self) -> HYSTPRS1_W<DECCTRL_SPEC> {
        HYSTPRS1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Decoder Hysteresis on PRS2 Output"]
    #[inline(always)]
    #[must_use]
    pub fn hystprs2(&mut self) -> HYSTPRS2_W<DECCTRL_SPEC> {
        HYSTPRS2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Decoder Hysteresis on Interrupt Requests"]
    #[inline(always)]
    #[must_use]
    pub fn hystirq(&mut self) -> HYSTIRQ_W<DECCTRL_SPEC> {
        HYSTIRQ_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Count Mode on Decoder PRS Channels 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn prscnt(&mut self) -> PRSCNT_W<DECCTRL_SPEC> {
        PRSCNT_W::new(self, 7)
    }
    #[doc = "Bit 8 - LESENSE Decoder Input Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<DECCTRL_SPEC> {
        INPUT_W::new(self, 8)
    }
    #[doc = "Bits 10:14 - LESENSE Decoder PRS Input 0 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prssel0(&mut self) -> PRSSEL0_W<DECCTRL_SPEC> {
        PRSSEL0_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - LESENSE Decoder PRS Input 1 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prssel1(&mut self) -> PRSSEL1_W<DECCTRL_SPEC> {
        PRSSEL1_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - LESENSE Decoder PRS Input 2 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prssel2(&mut self) -> PRSSEL2_W<DECCTRL_SPEC> {
        PRSSEL2_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - LESENSE Decoder PRS Input 3 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prssel3(&mut self) -> PRSSEL3_W<DECCTRL_SPEC> {
        PRSSEL3_W::new(self, 25)
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
#[doc = "Decoder Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`decctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DECCTRL_SPEC;
impl crate::RegisterSpec for DECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decctrl::R`](R) reader structure"]
impl crate::Readable for DECCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`decctrl::W`](W) writer structure"]
impl crate::Writable for DECCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DECCTRL to value 0"]
impl crate::Resettable for DECCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
