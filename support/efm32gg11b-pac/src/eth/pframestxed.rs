#[doc = "Register `PFRAMESTXED` reader"]
pub type R = crate::R<PframestxedSpec>;
#[doc = "Register `PFRAMESTXED` writer"]
pub type W = crate::W<PframestxedSpec>;
#[doc = "Field `COUNT` reader - Transmitted pause frames"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Transmitted pause frames"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted pause frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitted pause frames"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, PframestxedSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Pause Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`pframestxed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pframestxed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PframestxedSpec;
impl crate::RegisterSpec for PframestxedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pframestxed::R`](R) reader structure"]
impl crate::Readable for PframestxedSpec {}
#[doc = "`write(|w| ..)` method takes [`pframestxed::W`](W) writer structure"]
impl crate::Writable for PframestxedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFRAMESTXED to value 0"]
impl crate::Resettable for PframestxedSpec {}
