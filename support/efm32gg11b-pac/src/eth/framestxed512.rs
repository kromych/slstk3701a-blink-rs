#[doc = "Register `FRAMESTXED512` reader"]
pub type R = crate::R<Framestxed512Spec>;
#[doc = "Register `FRAMESTXED512` writer"]
pub type W = crate::W<Framestxed512Spec>;
#[doc = "Field `COUNT` reader - 512 to 1023 byte frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 512 to 1023 byte frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 512 to 1023 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 512 to 1023 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framestxed512Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "512 to 1023 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed512::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed512::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framestxed512Spec;
impl crate::RegisterSpec for Framestxed512Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed512::R`](R) reader structure"]
impl crate::Readable for Framestxed512Spec {}
#[doc = "`write(|w| ..)` method takes [`framestxed512::W`](W) writer structure"]
impl crate::Writable for Framestxed512Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXED512 to value 0"]
impl crate::Resettable for Framestxed512Spec {}
