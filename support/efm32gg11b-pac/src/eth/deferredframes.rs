#[doc = "Register `DEFERREDFRAMES` reader"]
pub type R = crate::R<DeferredframesSpec>;
#[doc = "Register `DEFERREDFRAMES` writer"]
pub type W = crate::W<DeferredframesSpec>;
#[doc = "Field `COUNT` reader - Deferred transmission frames"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Deferred transmission frames"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred transmission frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Deferred transmission frames"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, DeferredframesSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Deferred Transmission Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`deferredframes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deferredframes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeferredframesSpec;
impl crate::RegisterSpec for DeferredframesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deferredframes::R`](R) reader structure"]
impl crate::Readable for DeferredframesSpec {}
#[doc = "`write(|w| ..)` method takes [`deferredframes::W`](W) writer structure"]
impl crate::Writable for DeferredframesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEFERREDFRAMES to value 0"]
impl crate::Resettable for DeferredframesSpec {}
