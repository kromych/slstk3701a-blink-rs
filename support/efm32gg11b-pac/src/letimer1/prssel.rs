#[doc = "Register `PRSSEL` reader"]
pub type R = crate::R<PRSSEL_SPEC>;
#[doc = "Register `PRSSEL` writer"]
pub type W = crate::W<PRSSEL_SPEC>;
#[doc = "Field `PRSSTARTSEL` reader - PRS Start Select"]
pub type PRSSTARTSEL_R = crate::FieldReader<PRSSTARTSEL_A>;
#[doc = "PRS Start Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTSEL_A {
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
impl From<PRSSTARTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTARTSEL_A {
    type Ux = u8;
}
impl PRSSTARTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSTARTSEL_A> {
        match self.bits {
            0 => Some(PRSSTARTSEL_A::PRSCH0),
            1 => Some(PRSSTARTSEL_A::PRSCH1),
            2 => Some(PRSSTARTSEL_A::PRSCH2),
            3 => Some(PRSSTARTSEL_A::PRSCH3),
            4 => Some(PRSSTARTSEL_A::PRSCH4),
            5 => Some(PRSSTARTSEL_A::PRSCH5),
            6 => Some(PRSSTARTSEL_A::PRSCH6),
            7 => Some(PRSSTARTSEL_A::PRSCH7),
            8 => Some(PRSSTARTSEL_A::PRSCH8),
            9 => Some(PRSSTARTSEL_A::PRSCH9),
            10 => Some(PRSSTARTSEL_A::PRSCH10),
            11 => Some(PRSSTARTSEL_A::PRSCH11),
            12 => Some(PRSSTARTSEL_A::PRSCH12),
            13 => Some(PRSSTARTSEL_A::PRSCH13),
            14 => Some(PRSSTARTSEL_A::PRSCH14),
            15 => Some(PRSSTARTSEL_A::PRSCH15),
            16 => Some(PRSSTARTSEL_A::PRSCH16),
            17 => Some(PRSSTARTSEL_A::PRSCH17),
            18 => Some(PRSSTARTSEL_A::PRSCH18),
            19 => Some(PRSSTARTSEL_A::PRSCH19),
            20 => Some(PRSSTARTSEL_A::PRSCH20),
            21 => Some(PRSSTARTSEL_A::PRSCH21),
            22 => Some(PRSSTARTSEL_A::PRSCH22),
            23 => Some(PRSSTARTSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSTARTSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSSTARTSEL` writer - PRS Start Select"]
pub type PRSSTARTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSTARTSEL_A>;
impl<'a, REG> PRSSTARTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTSEL_A::PRSCH23)
    }
}
#[doc = "Field `PRSSTOPSEL` reader - PRS Stop Select"]
pub type PRSSTOPSEL_R = crate::FieldReader<PRSSTOPSEL_A>;
#[doc = "PRS Stop Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPSEL_A {
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
impl From<PRSSTOPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTOPSEL_A {
    type Ux = u8;
}
impl PRSSTOPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSTOPSEL_A> {
        match self.bits {
            0 => Some(PRSSTOPSEL_A::PRSCH0),
            1 => Some(PRSSTOPSEL_A::PRSCH1),
            2 => Some(PRSSTOPSEL_A::PRSCH2),
            3 => Some(PRSSTOPSEL_A::PRSCH3),
            4 => Some(PRSSTOPSEL_A::PRSCH4),
            5 => Some(PRSSTOPSEL_A::PRSCH5),
            6 => Some(PRSSTOPSEL_A::PRSCH6),
            7 => Some(PRSSTOPSEL_A::PRSCH7),
            8 => Some(PRSSTOPSEL_A::PRSCH8),
            9 => Some(PRSSTOPSEL_A::PRSCH9),
            10 => Some(PRSSTOPSEL_A::PRSCH10),
            11 => Some(PRSSTOPSEL_A::PRSCH11),
            12 => Some(PRSSTOPSEL_A::PRSCH12),
            13 => Some(PRSSTOPSEL_A::PRSCH13),
            14 => Some(PRSSTOPSEL_A::PRSCH14),
            15 => Some(PRSSTOPSEL_A::PRSCH15),
            16 => Some(PRSSTOPSEL_A::PRSCH16),
            17 => Some(PRSSTOPSEL_A::PRSCH17),
            18 => Some(PRSSTOPSEL_A::PRSCH18),
            19 => Some(PRSSTOPSEL_A::PRSCH19),
            20 => Some(PRSSTOPSEL_A::PRSCH20),
            21 => Some(PRSSTOPSEL_A::PRSCH21),
            22 => Some(PRSSTOPSEL_A::PRSCH22),
            23 => Some(PRSSTOPSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSTOPSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSSTOPSEL` writer - PRS Stop Select"]
pub type PRSSTOPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSTOPSEL_A>;
impl<'a, REG> PRSSTOPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPSEL_A::PRSCH23)
    }
}
#[doc = "Field `PRSCLEARSEL` reader - PRS Clear Select"]
pub type PRSCLEARSEL_R = crate::FieldReader<PRSCLEARSEL_A>;
#[doc = "PRS Clear Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARSEL_A {
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
impl From<PRSCLEARSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSCLEARSEL_A {
    type Ux = u8;
}
impl PRSCLEARSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSCLEARSEL_A> {
        match self.bits {
            0 => Some(PRSCLEARSEL_A::PRSCH0),
            1 => Some(PRSCLEARSEL_A::PRSCH1),
            2 => Some(PRSCLEARSEL_A::PRSCH2),
            3 => Some(PRSCLEARSEL_A::PRSCH3),
            4 => Some(PRSCLEARSEL_A::PRSCH4),
            5 => Some(PRSCLEARSEL_A::PRSCH5),
            6 => Some(PRSCLEARSEL_A::PRSCH6),
            7 => Some(PRSCLEARSEL_A::PRSCH7),
            8 => Some(PRSCLEARSEL_A::PRSCH8),
            9 => Some(PRSCLEARSEL_A::PRSCH9),
            10 => Some(PRSCLEARSEL_A::PRSCH10),
            11 => Some(PRSCLEARSEL_A::PRSCH11),
            12 => Some(PRSCLEARSEL_A::PRSCH12),
            13 => Some(PRSCLEARSEL_A::PRSCH13),
            14 => Some(PRSCLEARSEL_A::PRSCH14),
            15 => Some(PRSCLEARSEL_A::PRSCH15),
            16 => Some(PRSCLEARSEL_A::PRSCH16),
            17 => Some(PRSCLEARSEL_A::PRSCH17),
            18 => Some(PRSCLEARSEL_A::PRSCH18),
            19 => Some(PRSCLEARSEL_A::PRSCH19),
            20 => Some(PRSCLEARSEL_A::PRSCH20),
            21 => Some(PRSCLEARSEL_A::PRSCH21),
            22 => Some(PRSCLEARSEL_A::PRSCH22),
            23 => Some(PRSCLEARSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSCLEARSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSCLEARSEL` writer - PRS Clear Select"]
pub type PRSCLEARSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSCLEARSEL_A>;
impl<'a, REG> PRSCLEARSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARSEL_A::PRSCH23)
    }
}
#[doc = "Field `PRSSTARTMODE` reader - PRS Start Mode"]
pub type PRSSTARTMODE_R = crate::FieldReader<PRSSTARTMODE_A>;
#[doc = "PRS Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTARTMODE_A {
    #[doc = "0: PRS cannot start the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can start the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can start the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    BOTH = 3,
}
impl From<PRSSTARTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTARTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTARTMODE_A {
    type Ux = u8;
}
impl PRSSTARTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRSSTARTMODE_A {
        match self.bits {
            0 => PRSSTARTMODE_A::NONE,
            1 => PRSSTARTMODE_A::RISING,
            2 => PRSSTARTMODE_A::FALLING,
            3 => PRSSTARTMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTARTMODE_A::NONE
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTARTMODE_A::RISING
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTARTMODE_A::FALLING
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTARTMODE_A::BOTH
    }
}
#[doc = "Field `PRSSTARTMODE` writer - PRS Start Mode"]
pub type PRSSTARTMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRSSTARTMODE_A>;
impl<'a, REG> PRSSTARTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTARTMODE_A::BOTH)
    }
}
#[doc = "Field `PRSSTOPMODE` reader - PRS Stop Mode"]
pub type PRSSTOPMODE_R = crate::FieldReader<PRSSTOPMODE_A>;
#[doc = "PRS Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSTOPMODE_A {
    #[doc = "0: PRS cannot stop the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can stop the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can stop the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    BOTH = 3,
}
impl From<PRSSTOPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSTOPMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSTOPMODE_A {
    type Ux = u8;
}
impl PRSSTOPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRSSTOPMODE_A {
        match self.bits {
            0 => PRSSTOPMODE_A::NONE,
            1 => PRSSTOPMODE_A::RISING,
            2 => PRSSTOPMODE_A::FALLING,
            3 => PRSSTOPMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSSTOPMODE_A::NONE
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSSTOPMODE_A::RISING
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSSTOPMODE_A::FALLING
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSSTOPMODE_A::BOTH
    }
}
#[doc = "Field `PRSSTOPMODE` writer - PRS Stop Mode"]
pub type PRSSTOPMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRSSTOPMODE_A>;
impl<'a, REG> PRSSTOPMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSTOPMODE_A::BOTH)
    }
}
#[doc = "Field `PRSCLEARMODE` reader - PRS Clear Mode"]
pub type PRSCLEARMODE_R = crate::FieldReader<PRSCLEARMODE_A>;
#[doc = "PRS Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSCLEARMODE_A {
    #[doc = "0: PRS cannot clear the LETIMER"]
    NONE = 0,
    #[doc = "1: Rising edge of selected PRS input can clear the LETIMER"]
    RISING = 1,
    #[doc = "2: Falling edge of selected PRS input can clear the LETIMER"]
    FALLING = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    BOTH = 3,
}
impl From<PRSCLEARMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSCLEARMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSCLEARMODE_A {
    type Ux = u8;
}
impl PRSCLEARMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRSCLEARMODE_A {
        match self.bits {
            0 => PRSCLEARMODE_A::NONE,
            1 => PRSCLEARMODE_A::RISING,
            2 => PRSCLEARMODE_A::FALLING,
            3 => PRSCLEARMODE_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSCLEARMODE_A::NONE
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == PRSCLEARMODE_A::RISING
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == PRSCLEARMODE_A::FALLING
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == PRSCLEARMODE_A::BOTH
    }
}
#[doc = "Field `PRSCLEARMODE` writer - PRS Clear Mode"]
pub type PRSCLEARMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRSCLEARMODE_A>;
impl<'a, REG> PRSCLEARMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE_A::NONE)
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE_A::RISING)
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE_A::FALLING)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(PRSCLEARMODE_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:4 - PRS Start Select"]
    #[inline(always)]
    pub fn prsstartsel(&self) -> PRSSTARTSEL_R {
        PRSSTARTSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - PRS Stop Select"]
    #[inline(always)]
    pub fn prsstopsel(&self) -> PRSSTOPSEL_R {
        PRSSTOPSEL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - PRS Clear Select"]
    #[inline(always)]
    pub fn prsclearsel(&self) -> PRSCLEARSEL_R {
        PRSCLEARSEL_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    pub fn prsstartmode(&self) -> PRSSTARTMODE_R {
        PRSSTARTMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    pub fn prsstopmode(&self) -> PRSSTOPMODE_R {
        PRSSTOPMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    pub fn prsclearmode(&self) -> PRSCLEARMODE_R {
        PRSCLEARMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PRS Start Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartsel(&mut self) -> PRSSTARTSEL_W<PRSSEL_SPEC> {
        PRSSTARTSEL_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - PRS Stop Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopsel(&mut self) -> PRSSTOPSEL_W<PRSSEL_SPEC> {
        PRSSTOPSEL_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - PRS Clear Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearsel(&mut self) -> PRSCLEARSEL_W<PRSSEL_SPEC> {
        PRSCLEARSEL_W::new(self, 12)
    }
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstartmode(&mut self) -> PRSSTARTMODE_W<PRSSEL_SPEC> {
        PRSSTARTMODE_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsstopmode(&mut self) -> PRSSTOPMODE_W<PRSSEL_SPEC> {
        PRSSTOPMODE_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsclearmode(&mut self) -> PRSCLEARMODE_W<PRSSEL_SPEC> {
        PRSCLEARMODE_W::new(self, 26)
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
#[doc = "PRS Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSSEL_SPEC;
impl crate::RegisterSpec for PRSSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prssel::R`](R) reader structure"]
impl crate::Readable for PRSSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prssel::W`](W) writer structure"]
impl crate::Writable for PRSSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRSSEL to value 0"]
impl crate::Resettable for PRSSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
