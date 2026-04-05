#[doc = "Register `FRAMESRXED1519` reader"]
pub type R = crate::R<Framesrxed1519Spec>;
#[doc = "Register `FRAMESRXED1519` writer"]
pub type W = crate::W<Framesrxed1519Spec>;
#[doc = "Field `COUNT` reader - 1519 to maximum byte frames received without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 1519 to maximum byte frames received without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1519 to maximum byte frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1519 to maximum byte frames received without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framesrxed1519Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "1519 to maximum Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed1519::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed1519::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framesrxed1519Spec;
impl crate::RegisterSpec for Framesrxed1519Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxed1519::R`](R) reader structure"]
impl crate::Readable for Framesrxed1519Spec {}
#[doc = "`write(|w| ..)` method takes [`framesrxed1519::W`](W) writer structure"]
impl crate::Writable for Framesrxed1519Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESRXED1519 to value 0"]
impl crate::Resettable for Framesrxed1519Spec {}
