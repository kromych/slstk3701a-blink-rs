#[doc = "Register `ITTRIGOUT` reader"]
pub type R = crate::R<IttrigoutSpec>;
#[doc = "Register `ITTRIGOUT` writer"]
pub type W = crate::W<IttrigoutSpec>;
#[doc = "Field `TRIGGEROUT` reader - Trigger output value"]
pub type TriggeroutR = crate::BitReader;
#[doc = "Field `TRIGGEROUT` writer - Trigger output value"]
pub type TriggeroutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&self) -> TriggeroutR {
        TriggeroutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&mut self) -> TriggeroutW<'_, IttrigoutSpec> {
        TriggeroutW::new(self, 0)
    }
}
#[doc = "Integration Test Trigger Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrigout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrigout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IttrigoutSpec;
impl crate::RegisterSpec for IttrigoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ittrigout::R`](R) reader structure"]
impl crate::Readable for IttrigoutSpec {}
#[doc = "`write(|w| ..)` method takes [`ittrigout::W`](W) writer structure"]
impl crate::Writable for IttrigoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for IttrigoutSpec {}
