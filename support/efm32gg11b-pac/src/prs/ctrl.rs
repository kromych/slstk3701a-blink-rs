#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SEVONPRS` reader - Set Event on PRS"]
pub type SEVONPRS_R = crate::BitReader;
#[doc = "Field `SEVONPRS` writer - Set Event on PRS"]
pub type SEVONPRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEVONPRSSEL` reader - SEVONPRS PRS Channel Select"]
pub type SEVONPRSSEL_R = crate::FieldReader<SEVONPRSSEL_A>;
#[doc = "SEVONPRS PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEVONPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected"]
    PRSCH23 = 23,
}
impl From<SEVONPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEVONPRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEVONPRSSEL_A {
    type Ux = u8;
}
impl SEVONPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEVONPRSSEL_A> {
        match self.bits {
            0 => Some(SEVONPRSSEL_A::PRSCH0),
            1 => Some(SEVONPRSSEL_A::PRSCH1),
            2 => Some(SEVONPRSSEL_A::PRSCH2),
            3 => Some(SEVONPRSSEL_A::PRSCH3),
            4 => Some(SEVONPRSSEL_A::PRSCH4),
            5 => Some(SEVONPRSSEL_A::PRSCH5),
            6 => Some(SEVONPRSSEL_A::PRSCH6),
            7 => Some(SEVONPRSSEL_A::PRSCH7),
            8 => Some(SEVONPRSSEL_A::PRSCH8),
            9 => Some(SEVONPRSSEL_A::PRSCH9),
            10 => Some(SEVONPRSSEL_A::PRSCH10),
            11 => Some(SEVONPRSSEL_A::PRSCH11),
            12 => Some(SEVONPRSSEL_A::PRSCH12),
            13 => Some(SEVONPRSSEL_A::PRSCH13),
            14 => Some(SEVONPRSSEL_A::PRSCH14),
            15 => Some(SEVONPRSSEL_A::PRSCH15),
            16 => Some(SEVONPRSSEL_A::PRSCH16),
            17 => Some(SEVONPRSSEL_A::PRSCH17),
            18 => Some(SEVONPRSSEL_A::PRSCH18),
            19 => Some(SEVONPRSSEL_A::PRSCH19),
            20 => Some(SEVONPRSSEL_A::PRSCH20),
            21 => Some(SEVONPRSSEL_A::PRSCH21),
            22 => Some(SEVONPRSSEL_A::PRSCH22),
            23 => Some(SEVONPRSSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == SEVONPRSSEL_A::PRSCH23
    }
}
#[doc = "Field `SEVONPRSSEL` writer - SEVONPRS PRS Channel Select"]
pub type SEVONPRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SEVONPRSSEL_A>;
impl<'a, REG> SEVONPRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL_A::PRSCH23)
    }
}
impl R {
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline(always)]
    pub fn sevonprs(&self) -> SEVONPRS_R {
        SEVONPRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - SEVONPRS PRS Channel Select"]
    #[inline(always)]
    pub fn sevonprssel(&self) -> SEVONPRSSEL_R {
        SEVONPRSSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline(always)]
    #[must_use]
    pub fn sevonprs(&mut self) -> SEVONPRS_W<CTRL_SPEC> {
        SEVONPRS_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - SEVONPRS PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn sevonprssel(&mut self) -> SEVONPRSSEL_W<CTRL_SPEC> {
        SEVONPRSSEL_W::new(self, 1)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
