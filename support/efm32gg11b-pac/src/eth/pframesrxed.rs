#[doc = "Register `PFRAMESRXED` reader"]
pub type R = crate::R<PframesrxedSpec>;
#[doc = "Register `PFRAMESRXED` writer"]
pub type W = crate::W<PframesrxedSpec>;
#[doc = "Field `COUNT` reader - Received pause frames"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Received pause frames"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Received pause frames"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, PframesrxedSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Pause Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`pframesrxed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pframesrxed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PframesrxedSpec;
impl crate::RegisterSpec for PframesrxedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pframesrxed::R`](R) reader structure"]
impl crate::Readable for PframesrxedSpec {}
#[doc = "`write(|w| ..)` method takes [`pframesrxed::W`](W) writer structure"]
impl crate::Writable for PframesrxedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFRAMESRXED to value 0"]
impl crate::Resettable for PframesrxedSpec {}
