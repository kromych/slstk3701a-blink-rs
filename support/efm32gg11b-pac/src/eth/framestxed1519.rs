#[doc = "Register `FRAMESTXED1519` reader"]
pub type R = crate::R<Framestxed1519Spec>;
#[doc = "Register `FRAMESTXED1519` writer"]
pub type W = crate::W<Framestxed1519Spec>;
#[doc = "Field `COUNT` reader - Greater than 1518 byte frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Greater than 1518 byte frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Greater than 1518 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framestxed1519Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "Greater Than 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed1519::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed1519::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framestxed1519Spec;
impl crate::RegisterSpec for Framestxed1519Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed1519::R`](R) reader structure"]
impl crate::Readable for Framestxed1519Spec {}
#[doc = "`write(|w| ..)` method takes [`framestxed1519::W`](W) writer structure"]
impl crate::Writable for Framestxed1519Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXED1519 to value 0"]
impl crate::Resettable for Framestxed1519Spec {}
