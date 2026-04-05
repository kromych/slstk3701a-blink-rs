#[doc = "Register `BROADCASTRXED` reader"]
pub type R = crate::R<BroadcastrxedSpec>;
#[doc = "Register `BROADCASTRXED` writer"]
pub type W = crate::W<BroadcastrxedSpec>;
#[doc = "Field `COUNT` reader - Broadcast frames received without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Broadcast frames received without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Broadcast frames received without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, BroadcastrxedSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Broadcast Frames Received\n\nYou can [`read`](crate::Reg::read) this register and get [`broadcastrxed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`broadcastrxed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BroadcastrxedSpec;
impl crate::RegisterSpec for BroadcastrxedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`broadcastrxed::R`](R) reader structure"]
impl crate::Readable for BroadcastrxedSpec {}
#[doc = "`write(|w| ..)` method takes [`broadcastrxed::W`](W) writer structure"]
impl crate::Writable for BroadcastrxedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BROADCASTRXED to value 0"]
impl crate::Resettable for BroadcastrxedSpec {}
