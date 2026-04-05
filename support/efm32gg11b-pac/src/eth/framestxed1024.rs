#[doc = "Register `FRAMESTXED1024` reader"]
pub type R = crate::R<Framestxed1024Spec>;
#[doc = "Register `FRAMESTXED1024` writer"]
pub type W = crate::W<Framestxed1024Spec>;
#[doc = "Field `COUNT` reader - 1024 to 1518 byte frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 1024 to 1518 byte frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1024 to 1518 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1024 to 1518 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framestxed1024Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "1024 to 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed1024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed1024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framestxed1024Spec;
impl crate::RegisterSpec for Framestxed1024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed1024::R`](R) reader structure"]
impl crate::Readable for Framestxed1024Spec {}
#[doc = "`write(|w| ..)` method takes [`framestxed1024::W`](W) writer structure"]
impl crate::Writable for Framestxed1024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXED1024 to value 0"]
impl crate::Resettable for Framestxed1024Spec {}
