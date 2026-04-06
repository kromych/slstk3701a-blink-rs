#[doc = "Register `OCTETSTXEDTOP` reader"]
pub type R = crate::R<OctetstxedtopSpec>;
#[doc = "Register `OCTETSTXEDTOP` writer"]
pub type W = crate::W<OctetstxedtopSpec>;
#[doc = "Field `COUNT` reader - Transmitted octets in frame without errors \\[47:32\\]"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Transmitted octets in frame without errors \\[47:32\\]"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, OctetstxedtopSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Octets Transmitted 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`octetstxedtop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octetstxedtop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OctetstxedtopSpec;
impl crate::RegisterSpec for OctetstxedtopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octetstxedtop::R`](R) reader structure"]
impl crate::Readable for OctetstxedtopSpec {}
#[doc = "`write(|w| ..)` method takes [`octetstxedtop::W`](W) writer structure"]
impl crate::Writable for OctetstxedtopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCTETSTXEDTOP to value 0"]
impl crate::Resettable for OctetstxedtopSpec {}
