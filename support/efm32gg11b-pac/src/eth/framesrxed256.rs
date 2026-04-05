#[doc = "Register `FRAMESRXED256` reader"]
pub type R = crate::R<Framesrxed256Spec>;
#[doc = "Register `FRAMESRXED256` writer"]
pub type W = crate::W<Framesrxed256Spec>;
#[doc = "Field `COUNT` reader - 256 to 511 byte frames received without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 256 to 511 byte frames received without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 256 to 511 byte frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 256 to 511 byte frames received without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framesrxed256Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "256 to 511 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed256::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed256::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framesrxed256Spec;
impl crate::RegisterSpec for Framesrxed256Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxed256::R`](R) reader structure"]
impl crate::Readable for Framesrxed256Spec {}
#[doc = "`write(|w| ..)` method takes [`framesrxed256::W`](W) writer structure"]
impl crate::Writable for Framesrxed256Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESRXED256 to value 0"]
impl crate::Resettable for Framesrxed256Spec {}
