#[doc = "Register `TXTHRESH` reader"]
pub type R = crate::R<TXTHRESH_SPEC>;
#[doc = "Register `TXTHRESH` writer"]
pub type W = crate::W<TXTHRESH_SPEC>;
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
    pub fn level(&mut self) -> LEVEL_W<TXTHRESH_SPEC, 0> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TX Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txthresh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txthresh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXTHRESH_SPEC;
impl crate::RegisterSpec for TXTHRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txthresh::R`](R) reader structure"]
impl crate::Readable for TXTHRESH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txthresh::W`](W) writer structure"]
impl crate::Writable for TXTHRESH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXTHRESH to value 0x01"]
impl crate::Resettable for TXTHRESH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
