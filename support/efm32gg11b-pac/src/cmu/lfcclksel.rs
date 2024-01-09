#[doc = "Register `LFCCLKSEL` reader"]
pub type R = crate::R<LFCCLKSEL_SPEC>;
#[doc = "Register `LFCCLKSEL` writer"]
pub type W = crate::W<LFCCLKSEL_SPEC>;
#[doc = "Field `LFC` reader - Clock Select for LFC"]
pub type LFC_R = crate::FieldReader<LFC_A>;
#[doc = "Clock Select for LFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFC_A {
    #[doc = "0: LFCCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFCCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFCCLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFCCLK"]
    ULFRCO = 4,
}
impl From<LFC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFC_A {
    type Ux = u8;
}
impl LFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFC_A> {
        match self.bits {
            0 => Some(LFC_A::DISABLED),
            1 => Some(LFC_A::LFRCO),
            2 => Some(LFC_A::LFXO),
            4 => Some(LFC_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "LFCCLK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFC_A::DISABLED
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFC_A::LFRCO
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFC_A::LFXO
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFC_A::ULFRCO
    }
}
#[doc = "Field `LFC` writer - Clock Select for LFC"]
pub type LFC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LFC_A>;
impl<'a, REG> LFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFCCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFC_A::ULFRCO)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LFC_R {
        LFC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    #[must_use]
    pub fn lfc(&mut self) -> LFC_W<LFCCLKSEL_SPEC> {
        LFC_W::new(self, 0)
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
#[doc = "Low Frequency C Clock Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfcclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfcclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFCCLKSEL_SPEC;
impl crate::RegisterSpec for LFCCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfcclksel::R`](R) reader structure"]
impl crate::Readable for LFCCLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfcclksel::W`](W) writer structure"]
impl crate::Writable for LFCCLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFCCLKSEL to value 0"]
impl crate::Resettable for LFCCLKSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
