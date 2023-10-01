#[doc = "Register `RXTHRESH` reader"]
pub type R = crate::R<RXTHRESH_SPEC>;
#[doc = "Register `RXTHRESH` writer"]
pub type W = crate::W<RXTHRESH_SPEC>;
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LEVEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<RXTHRESH_SPEC, 0> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RX Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxthresh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxthresh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXTHRESH_SPEC;
impl crate::RegisterSpec for RXTHRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxthresh::R`](R) reader structure"]
impl crate::Readable for RXTHRESH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxthresh::W`](W) writer structure"]
impl crate::Writable for RXTHRESH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXTHRESH to value 0x01"]
impl crate::Resettable for RXTHRESH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
