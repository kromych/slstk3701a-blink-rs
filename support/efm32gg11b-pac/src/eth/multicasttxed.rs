#[doc = "Register `MULTICASTTXED` reader"]
pub type R = crate::R<MulticasttxedSpec>;
#[doc = "Register `MULTICASTTXED` writer"]
pub type W = crate::W<MulticasttxedSpec>;
#[doc = "Field `COUNT` reader - Multicast frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Multicast frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, MulticasttxedSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Multicast Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`multicasttxed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multicasttxed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MulticasttxedSpec;
impl crate::RegisterSpec for MulticasttxedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`multicasttxed::R`](R) reader structure"]
impl crate::Readable for MulticasttxedSpec {}
#[doc = "`write(|w| ..)` method takes [`multicasttxed::W`](W) writer structure"]
impl crate::Writable for MulticasttxedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULTICASTTXED to value 0"]
impl crate::Resettable for MulticasttxedSpec {}
