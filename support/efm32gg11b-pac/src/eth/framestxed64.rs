#[doc = "Register `FRAMESTXED64` reader"]
pub type R = crate::R<Framestxed64Spec>;
#[doc = "Register `FRAMESTXED64` writer"]
pub type W = crate::W<Framestxed64Spec>;
#[doc = "Field `COUNT` reader - 64 byte frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 64 byte frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 64 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 64 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framestxed64Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "64 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framestxed64Spec;
impl crate::RegisterSpec for Framestxed64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed64::R`](R) reader structure"]
impl crate::Readable for Framestxed64Spec {}
#[doc = "`write(|w| ..)` method takes [`framestxed64::W`](W) writer structure"]
impl crate::Writable for Framestxed64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXED64 to value 0"]
impl crate::Resettable for Framestxed64Spec {}
