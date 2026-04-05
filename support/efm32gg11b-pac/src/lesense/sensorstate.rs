#[doc = "Register `SENSORSTATE` reader"]
pub type R = crate::R<SensorstateSpec>;
#[doc = "Register `SENSORSTATE` writer"]
pub type W = crate::W<SensorstateSpec>;
#[doc = "Field `SENSORSTATE` reader - Decoder Input Register"]
pub type SensorstateR = crate::FieldReader;
#[doc = "Field `SENSORSTATE` writer - Decoder Input Register"]
pub type SensorstateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Decoder Input Register"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SensorstateR {
        SensorstateR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decoder Input Register"]
    #[inline(always)]
    pub fn sensorstate(&mut self) -> SensorstateW<'_, SensorstateSpec> {
        SensorstateW::new(self, 0)
    }
}
#[doc = "Decoder Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sensorstate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensorstate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SensorstateSpec;
impl crate::RegisterSpec for SensorstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensorstate::R`](R) reader structure"]
impl crate::Readable for SensorstateSpec {}
#[doc = "`write(|w| ..)` method takes [`sensorstate::W`](W) writer structure"]
impl crate::Writable for SensorstateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SENSORSTATE to value 0"]
impl crate::Resettable for SensorstateSpec {}
