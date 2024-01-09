#[doc = "Register `LFECLKSEL` reader"]
pub type R = crate::R<LFECLKSEL_SPEC>;
#[doc = "Register `LFECLKSEL` writer"]
pub type W = crate::W<LFECLKSEL_SPEC>;
#[doc = "Field `LFE` reader - Clock Select for LFE"]
pub type LFE_R = crate::FieldReader<LFE_A>;
#[doc = "Clock Select for LFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFE_A {
    #[doc = "0: LFECLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFECLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFECLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFECLK"]
    ULFRCO = 4,
}
impl From<LFE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFE_A {
    type Ux = u8;
}
impl LFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFE_A> {
        match self.bits {
            0 => Some(LFE_A::DISABLED),
            1 => Some(LFE_A::LFRCO),
            2 => Some(LFE_A::LFXO),
            4 => Some(LFE_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFE_A::DISABLED
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFE_A::LFRCO
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFE_A::LFXO
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFE_A::ULFRCO
    }
}
#[doc = "Field `LFE` writer - Clock Select for LFE"]
pub type LFE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LFE_A>;
impl<'a, REG> LFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFE_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFE_A::LFRCO)
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFE_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFE_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&self) -> LFE_R {
        LFE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    #[must_use]
    pub fn lfe(&mut self) -> LFE_W<LFECLKSEL_SPEC> {
        LFE_W::new(self, 0)
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
#[doc = "Low Frequency E Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFECLKSEL_SPEC;
impl crate::RegisterSpec for LFECLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfeclksel::R`](R) reader structure"]
impl crate::Readable for LFECLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfeclksel::W`](W) writer structure"]
impl crate::Writable for LFECLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFECLKSEL to value 0"]
impl crate::Resettable for LFECLKSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
