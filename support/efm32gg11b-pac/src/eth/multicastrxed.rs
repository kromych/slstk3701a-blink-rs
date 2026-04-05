#[doc = "Register `MULTICASTRXED` reader"]
pub type R = crate::R<MulticastrxedSpec>;
#[doc = "Register `MULTICASTRXED` writer"]
pub type W = crate::W<MulticastrxedSpec>;
#[doc = "Field `COUNT` reader - Multicast frames received without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Multicast frames received without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast frames received without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, MulticastrxedSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Multicast Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`multicastrxed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multicastrxed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MulticastrxedSpec;
impl crate::RegisterSpec for MulticastrxedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`multicastrxed::R`](R) reader structure"]
impl crate::Readable for MulticastrxedSpec {}
#[doc = "`write(|w| ..)` method takes [`multicastrxed::W`](W) writer structure"]
impl crate::Writable for MulticastrxedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULTICASTRXED to value 0"]
impl crate::Resettable for MulticastrxedSpec {}
