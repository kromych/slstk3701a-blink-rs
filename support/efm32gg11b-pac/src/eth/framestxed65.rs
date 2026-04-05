#[doc = "Register `FRAMESTXED65` reader"]
pub type R = crate::R<Framestxed65Spec>;
#[doc = "Register `FRAMESTXED65` writer"]
pub type W = crate::W<Framestxed65Spec>;
#[doc = "Field `COUNT` reader - 65 to127 byte frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 65 to127 byte frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to127 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 65 to127 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framestxed65Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "65 to 127 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed65::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed65::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framestxed65Spec;
impl crate::RegisterSpec for Framestxed65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed65::R`](R) reader structure"]
impl crate::Readable for Framestxed65Spec {}
#[doc = "`write(|w| ..)` method takes [`framestxed65::W`](W) writer structure"]
impl crate::Writable for Framestxed65Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXED65 to value 0"]
impl crate::Resettable for Framestxed65Spec {}
