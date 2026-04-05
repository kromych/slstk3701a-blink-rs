#[doc = "Register `DVBUSPULSE` reader"]
pub type R = crate::R<DvbuspulseSpec>;
#[doc = "Register `DVBUSPULSE` writer"]
pub type W = crate::W<DvbuspulseSpec>;
#[doc = "Field `DVBUSPULSE` reader - Device VBUS Pulsing Time"]
pub type DvbuspulseR = crate::FieldReader<u16>;
#[doc = "Field `DVBUSPULSE` writer - Device VBUS Pulsing Time"]
pub type DvbuspulseW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Device VBUS Pulsing Time"]
    #[inline(always)]
    pub fn dvbuspulse(&self) -> DvbuspulseR {
        DvbuspulseR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS Pulsing Time"]
    #[inline(always)]
    pub fn dvbuspulse(&mut self) -> DvbuspulseW<'_, DvbuspulseSpec> {
        DvbuspulseW::new(self, 0)
    }
}
#[doc = "Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvbuspulseSpec;
impl crate::RegisterSpec for DvbuspulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbuspulse::R`](R) reader structure"]
impl crate::Readable for DvbuspulseSpec {}
#[doc = "`write(|w| ..)` method takes [`dvbuspulse::W`](W) writer structure"]
impl crate::Writable for DvbuspulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVBUSPULSE to value 0x05b8"]
impl crate::Resettable for DvbuspulseSpec {
    const RESET_VALUE: u32 = 0x05b8;
}
