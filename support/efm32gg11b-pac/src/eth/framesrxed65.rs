#[doc = "Register `FRAMESRXED65` reader"]
pub type R = crate::R<Framesrxed65Spec>;
#[doc = "Register `FRAMESRXED65` writer"]
pub type W = crate::W<Framesrxed65Spec>;
#[doc = "Field `COUNT` reader - 65 to 127 byte frames received without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 65 to 127 byte frames received without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 65 to 127 byte frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 65 to 127 byte frames received without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Framesrxed65Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "65 to 127 Byte Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`framesrxed65::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framesrxed65::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Framesrxed65Spec;
impl crate::RegisterSpec for Framesrxed65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxed65::R`](R) reader structure"]
impl crate::Readable for Framesrxed65Spec {}
#[doc = "`write(|w| ..)` method takes [`framesrxed65::W`](W) writer structure"]
impl crate::Writable for Framesrxed65Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESRXED65 to value 0"]
impl crate::Resettable for Framesrxed65Spec {}
