#[doc = "Register `EMACTRL` reader"]
pub type R = crate::R<EMACTRL_SPEC>;
#[doc = "Register `EMACTRL` writer"]
pub type W = crate::W<EMACTRL_SPEC>;
#[doc = "Field `EMASAMPLE` reader - EMA Sample Weight"]
pub type EMASAMPLE_R = crate::FieldReader<EMASAMPLE_A>;
#[doc = "EMA Sample Weight\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMASAMPLE_A {
    #[doc = "0: EMA weight (N) is 1."]
    W1 = 0,
    #[doc = "1: EMA weight (N) is 2."]
    W2 = 1,
    #[doc = "2: EMA weight (N) is 4."]
    W4 = 2,
    #[doc = "3: EMA weight (N) is 8."]
    W8 = 3,
    #[doc = "4: EMA weight (N) is 16."]
    W16 = 4,
    #[doc = "5: EMA weight (N) is 32."]
    W32 = 5,
    #[doc = "6: EMA weight (N) is 64."]
    W64 = 6,
}
impl From<EMASAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMASAMPLE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EMASAMPLE_A {
    type Ux = u8;
}
impl EMASAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EMASAMPLE_A> {
        match self.bits {
            0 => Some(EMASAMPLE_A::W1),
            1 => Some(EMASAMPLE_A::W2),
            2 => Some(EMASAMPLE_A::W4),
            3 => Some(EMASAMPLE_A::W8),
            4 => Some(EMASAMPLE_A::W16),
            5 => Some(EMASAMPLE_A::W32),
            6 => Some(EMASAMPLE_A::W64),
            _ => None,
        }
    }
    #[doc = "EMA weight (N) is 1."]
    #[inline(always)]
    pub fn is_w1(&self) -> bool {
        *self == EMASAMPLE_A::W1
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline(always)]
    pub fn is_w2(&self) -> bool {
        *self == EMASAMPLE_A::W2
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline(always)]
    pub fn is_w4(&self) -> bool {
        *self == EMASAMPLE_A::W4
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline(always)]
    pub fn is_w8(&self) -> bool {
        *self == EMASAMPLE_A::W8
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline(always)]
    pub fn is_w16(&self) -> bool {
        *self == EMASAMPLE_A::W16
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline(always)]
    pub fn is_w32(&self) -> bool {
        *self == EMASAMPLE_A::W32
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline(always)]
    pub fn is_w64(&self) -> bool {
        *self == EMASAMPLE_A::W64
    }
}
#[doc = "Field `EMASAMPLE` writer - EMA Sample Weight"]
pub type EMASAMPLE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EMASAMPLE_A>;
impl<'a, REG> EMASAMPLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EMA weight (N) is 1."]
    #[inline(always)]
    pub fn w1(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W1)
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline(always)]
    pub fn w2(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W2)
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline(always)]
    pub fn w4(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W4)
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline(always)]
    pub fn w8(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W8)
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline(always)]
    pub fn w16(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W16)
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline(always)]
    pub fn w32(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W32)
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline(always)]
    pub fn w64(self) -> &'a mut crate::W<REG> {
        self.variant(EMASAMPLE_A::W64)
    }
}
impl R {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&self) -> EMASAMPLE_R {
        EMASAMPLE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    #[must_use]
    pub fn emasample(&mut self) -> EMASAMPLE_W<EMACTRL_SPEC> {
        EMASAMPLE_W::new(self, 0)
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
#[doc = "Exponential Moving Average Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACTRL_SPEC;
impl crate::RegisterSpec for EMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emactrl::R`](R) reader structure"]
impl crate::Readable for EMACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emactrl::W`](W) writer structure"]
impl crate::Writable for EMACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMACTRL to value 0"]
impl crate::Resettable for EMACTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
