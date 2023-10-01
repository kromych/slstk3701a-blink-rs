#[doc = "Register `EMA` reader"]
pub type R = crate::R<EMA_SPEC>;
#[doc = "Register `EMA` writer"]
pub type W = crate::W<EMA_SPEC>;
#[doc = "Field `EMA` reader - Calculated Exponential Moving Average"]
pub type EMA_R = crate::FieldReader<u32>;
#[doc = "Field `EMA` writer - Calculated Exponential Moving Average"]
pub type EMA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&self) -> EMA_R {
        EMA_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    #[must_use]
    pub fn ema(&mut self) -> EMA_W<EMA_SPEC, 0> {
        EMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Exponential Moving Average\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ema::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ema::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMA_SPEC;
impl crate::RegisterSpec for EMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ema::R`](R) reader structure"]
impl crate::Readable for EMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ema::W`](W) writer structure"]
impl crate::Writable for EMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMA to value 0"]
impl crate::Resettable for EMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
