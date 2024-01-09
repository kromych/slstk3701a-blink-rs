#[doc = "Register `DEFERREDFRAMES` reader"]
pub type R = crate::R<DEFERREDFRAMES_SPEC>;
#[doc = "Register `DEFERREDFRAMES` writer"]
pub type W = crate::W<DEFERREDFRAMES_SPEC>;
#[doc = "Field `COUNT` reader - Deferred transmission frames"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Deferred transmission frames"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred transmission frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Deferred transmission frames"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<DEFERREDFRAMES_SPEC> {
        COUNT_W::new(self, 0)
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
#[doc = "Deferred Transmission Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deferredframes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deferredframes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEFERREDFRAMES_SPEC;
impl crate::RegisterSpec for DEFERREDFRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deferredframes::R`](R) reader structure"]
impl crate::Readable for DEFERREDFRAMES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deferredframes::W`](W) writer structure"]
impl crate::Writable for DEFERREDFRAMES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEFERREDFRAMES to value 0"]
impl crate::Resettable for DEFERREDFRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
