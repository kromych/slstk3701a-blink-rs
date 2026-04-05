#[doc = "Register `DECSTATE` reader"]
pub type R = crate::R<DecstateSpec>;
#[doc = "Register `DECSTATE` writer"]
pub type W = crate::W<DecstateSpec>;
#[doc = "Field `DECSTATE` reader - Current Decoder State"]
pub type DecstateR = crate::FieldReader;
#[doc = "Field `DECSTATE` writer - Current Decoder State"]
pub type DecstateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Current Decoder State"]
    #[inline(always)]
    pub fn decstate(&self) -> DecstateR {
        DecstateR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Current Decoder State"]
    #[inline(always)]
    pub fn decstate(&mut self) -> DecstateW<'_, DecstateSpec> {
        DecstateW::new(self, 0)
    }
}
#[doc = "Current Decoder State\n\nYou can [`read`](crate::Reg::read) this register and get [`decstate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decstate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecstateSpec;
impl crate::RegisterSpec for DecstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decstate::R`](R) reader structure"]
impl crate::Readable for DecstateSpec {}
#[doc = "`write(|w| ..)` method takes [`decstate::W`](W) writer structure"]
impl crate::Writable for DecstateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECSTATE to value 0"]
impl crate::Resettable for DecstateSpec {}
