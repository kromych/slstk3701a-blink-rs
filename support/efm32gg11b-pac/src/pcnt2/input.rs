#[doc = "Register `INPUT` reader"]
pub type R = crate::R<InputSpec>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<InputSpec>;
#[doc = "S0IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0prssel {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected."]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected."]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected."]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected."]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected."]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected."]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected."]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected."]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected."]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected."]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected."]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected."]
    Prsch23 = 23,
}
impl From<S0prssel> for u8 {
    #[inline(always)]
    fn from(variant: S0prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0prssel {
    type Ux = u8;
}
impl crate::IsEnum for S0prssel {}
#[doc = "Field `S0PRSSEL` reader - S0IN PRS Channel Select"]
pub type S0prsselR = crate::FieldReader<S0prssel>;
impl S0prsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0prssel> {
        match self.bits {
            0 => Some(S0prssel::Prsch0),
            1 => Some(S0prssel::Prsch1),
            2 => Some(S0prssel::Prsch2),
            3 => Some(S0prssel::Prsch3),
            4 => Some(S0prssel::Prsch4),
            5 => Some(S0prssel::Prsch5),
            6 => Some(S0prssel::Prsch6),
            7 => Some(S0prssel::Prsch7),
            8 => Some(S0prssel::Prsch8),
            9 => Some(S0prssel::Prsch9),
            10 => Some(S0prssel::Prsch10),
            11 => Some(S0prssel::Prsch11),
            12 => Some(S0prssel::Prsch12),
            13 => Some(S0prssel::Prsch13),
            14 => Some(S0prssel::Prsch14),
            15 => Some(S0prssel::Prsch15),
            16 => Some(S0prssel::Prsch16),
            17 => Some(S0prssel::Prsch17),
            18 => Some(S0prssel::Prsch18),
            19 => Some(S0prssel::Prsch19),
            20 => Some(S0prssel::Prsch20),
            21 => Some(S0prssel::Prsch21),
            22 => Some(S0prssel::Prsch22),
            23 => Some(S0prssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S0prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S0prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S0prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S0prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S0prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S0prssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S0prssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S0prssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S0prssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S0prssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S0prssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S0prssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == S0prssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == S0prssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == S0prssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == S0prssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == S0prssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == S0prssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == S0prssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == S0prssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == S0prssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == S0prssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == S0prssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == S0prssel::Prsch23
    }
}
#[doc = "Field `S0PRSSEL` writer - S0IN PRS Channel Select"]
pub type S0prsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, S0prssel>;
impl<'a, REG> S0prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(S0prssel::Prsch23)
    }
}
#[doc = "Field `S0PRSEN` reader - S0IN PRS Enable"]
pub type S0prsenR = crate::BitReader;
#[doc = "Field `S0PRSEN` writer - S0IN PRS Enable"]
pub type S0prsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "S1IN PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1prssel {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected."]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected."]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected."]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected."]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected."]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected."]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected."]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected."]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected."]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected."]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected."]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected."]
    Prsch23 = 23,
}
impl From<S1prssel> for u8 {
    #[inline(always)]
    fn from(variant: S1prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1prssel {
    type Ux = u8;
}
impl crate::IsEnum for S1prssel {}
#[doc = "Field `S1PRSSEL` reader - S1IN PRS Channel Select"]
pub type S1prsselR = crate::FieldReader<S1prssel>;
impl S1prsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1prssel> {
        match self.bits {
            0 => Some(S1prssel::Prsch0),
            1 => Some(S1prssel::Prsch1),
            2 => Some(S1prssel::Prsch2),
            3 => Some(S1prssel::Prsch3),
            4 => Some(S1prssel::Prsch4),
            5 => Some(S1prssel::Prsch5),
            6 => Some(S1prssel::Prsch6),
            7 => Some(S1prssel::Prsch7),
            8 => Some(S1prssel::Prsch8),
            9 => Some(S1prssel::Prsch9),
            10 => Some(S1prssel::Prsch10),
            11 => Some(S1prssel::Prsch11),
            12 => Some(S1prssel::Prsch12),
            13 => Some(S1prssel::Prsch13),
            14 => Some(S1prssel::Prsch14),
            15 => Some(S1prssel::Prsch15),
            16 => Some(S1prssel::Prsch16),
            17 => Some(S1prssel::Prsch17),
            18 => Some(S1prssel::Prsch18),
            19 => Some(S1prssel::Prsch19),
            20 => Some(S1prssel::Prsch20),
            21 => Some(S1prssel::Prsch21),
            22 => Some(S1prssel::Prsch22),
            23 => Some(S1prssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == S1prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == S1prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == S1prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == S1prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == S1prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == S1prssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == S1prssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == S1prssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == S1prssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == S1prssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == S1prssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == S1prssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == S1prssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == S1prssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == S1prssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == S1prssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == S1prssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == S1prssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == S1prssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == S1prssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == S1prssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == S1prssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == S1prssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == S1prssel::Prsch23
    }
}
#[doc = "Field `S1PRSSEL` writer - S1IN PRS Channel Select"]
pub type S1prsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, S1prssel>;
impl<'a, REG> S1prsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(S1prssel::Prsch23)
    }
}
#[doc = "Field `S1PRSEN` reader - S1IN PRS Enable"]
pub type S1prsenR = crate::BitReader;
#[doc = "Field `S1PRSEN` writer - S1IN PRS Enable"]
pub type S1prsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&self) -> S0prsselR {
        S0prsselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0prsenR {
        S0prsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&self) -> S1prsselR {
        S1prsselR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1prsenR {
        S1prsenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - S0IN PRS Channel Select"]
    #[inline(always)]
    pub fn s0prssel(&mut self) -> S0prsselW<'_, InputSpec> {
        S0prsselW::new(self, 0)
    }
    #[doc = "Bit 5 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&mut self) -> S0prsenW<'_, InputSpec> {
        S0prsenW::new(self, 5)
    }
    #[doc = "Bits 6:10 - S1IN PRS Channel Select"]
    #[inline(always)]
    pub fn s1prssel(&mut self) -> S1prsselW<'_, InputSpec> {
        S1prsselW::new(self, 6)
    }
    #[doc = "Bit 11 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&mut self) -> S1prsenW<'_, InputSpec> {
        S1prsenW::new(self, 11)
    }
}
#[doc = "PCNT Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputSpec;
impl crate::RegisterSpec for InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for InputSpec {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for InputSpec {}
