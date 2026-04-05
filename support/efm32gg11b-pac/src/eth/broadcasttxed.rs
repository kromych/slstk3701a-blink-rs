#[doc = "Register `BROADCASTTXED` reader"]
pub type R = crate::R<BroadcasttxedSpec>;
#[doc = "Register `BROADCASTTXED` writer"]
pub type W = crate::W<BroadcasttxedSpec>;
#[doc = "Field `COUNT` reader - Broadcast frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Broadcast frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Broadcast frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, BroadcasttxedSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Broadcast Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`broadcasttxed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`broadcasttxed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BroadcasttxedSpec;
impl crate::RegisterSpec for BroadcasttxedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`broadcasttxed::R`](R) reader structure"]
impl crate::Readable for BroadcasttxedSpec {}
#[doc = "`write(|w| ..)` method takes [`broadcasttxed::W`](W) writer structure"]
impl crate::Writable for BroadcasttxedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BROADCASTTXED to value 0"]
impl crate::Resettable for BroadcasttxedSpec {}
