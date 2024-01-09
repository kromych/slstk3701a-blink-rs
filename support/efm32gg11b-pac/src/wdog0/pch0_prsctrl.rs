#[doc = "Register `PCH0_PRSCTRL` reader"]
pub type R = crate::R<PCH0_PRSCTRL_SPEC>;
#[doc = "Register `PCH0_PRSCTRL` writer"]
pub type W = crate::W<PCH0_PRSCTRL_SPEC>;
#[doc = "Field `PRSSEL` reader - PRS Channel PRS Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "PRS Channel PRS Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
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
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL` writer - PRS Channel PRS Select"]
pub type PRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRSSEL_A>;
impl<'a, REG> PRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH23)
    }
}
#[doc = "Field `PRSMISSRSTEN` reader - PRS Missing Event Will Trigger a Watchdog Reset"]
pub type PRSMISSRSTEN_R = crate::BitReader;
#[doc = "Field `PRSMISSRSTEN` writer - PRS Missing Event Will Trigger a Watchdog Reset"]
pub type PRSMISSRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - PRS Channel PRS Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - PRS Missing Event Will Trigger a Watchdog Reset"]
    #[inline(always)]
    pub fn prsmissrsten(&self) -> PRSMISSRSTEN_R {
        PRSMISSRSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PRS Channel PRS Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<PCH0_PRSCTRL_SPEC> {
        PRSSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - PRS Missing Event Will Trigger a Watchdog Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prsmissrsten(&mut self) -> PRSMISSRSTEN_W<PCH0_PRSCTRL_SPEC> {
        PRSMISSRSTEN_W::new(self, 8)
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
#[doc = "PRS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch0_prsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch0_prsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCH0_PRSCTRL_SPEC;
impl crate::RegisterSpec for PCH0_PRSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pch0_prsctrl::R`](R) reader structure"]
impl crate::Readable for PCH0_PRSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pch0_prsctrl::W`](W) writer structure"]
impl crate::Writable for PCH0_PRSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCH0_PRSCTRL to value 0"]
impl crate::Resettable for PCH0_PRSCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
