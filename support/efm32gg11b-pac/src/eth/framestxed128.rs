#[doc = "Register `FRAMESTXED128` reader"]
pub type R = crate::R<Framestxed128Spec>;
#[doc = "Register `FRAMESTXED128` writer"]
pub type W = crate::W<Framestxed128Spec>;
#[doc = "Field `COUNT` reader - 128 to 255 byte frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 128 to 255 byte frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 128 to 255 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framestxed128Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "128 to 255 Byte Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxed128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxed128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framestxed128Spec;
impl crate::RegisterSpec for Framestxed128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed128::R`](R) reader structure"]
impl crate::Readable for Framestxed128Spec {}
#[doc = "`write(|w| ..)` method takes [`framestxed128::W`](W) writer structure"]
impl crate::Writable for Framestxed128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXED128 to value 0"]
impl crate::Resettable for Framestxed128Spec {}
