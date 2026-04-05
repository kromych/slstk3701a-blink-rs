#[doc = "Register `HFRCOSS` reader"]
pub type R = crate::R<HfrcossSpec>;
#[doc = "Register `HFRCOSS` writer"]
pub type W = crate::W<HfrcossSpec>;
#[doc = "Field `SSAMP` reader - Spread Spectrum Amplitude"]
pub type SsampR = crate::FieldReader;
#[doc = "Field `SSAMP` writer - Spread Spectrum Amplitude"]
pub type SsampW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SSINV` reader - Spread Spectrum Update Interval"]
pub type SsinvR = crate::FieldReader;
#[doc = "Field `SSINV` writer - Spread Spectrum Update Interval"]
pub type SsinvW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&self) -> SsampR {
        SsampR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&self) -> SsinvR {
        SsinvR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&mut self) -> SsampW<'_, HfrcossSpec> {
        SsampW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&mut self) -> SsinvW<'_, HfrcossSpec> {
        SsinvW::new(self, 8)
    }
}
#[doc = "HFRCO Spread Spectrum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrcoss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfrcossSpec;
impl crate::RegisterSpec for HfrcossSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcoss::R`](R) reader structure"]
impl crate::Readable for HfrcossSpec {}
#[doc = "`write(|w| ..)` method takes [`hfrcoss::W`](W) writer structure"]
impl crate::Writable for HfrcossSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFRCOSS to value 0"]
impl crate::Resettable for HfrcossSpec {}
