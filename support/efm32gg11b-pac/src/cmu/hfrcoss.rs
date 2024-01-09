#[doc = "Register `HFRCOSS` reader"]
pub type R = crate::R<HFRCOSS_SPEC>;
#[doc = "Register `HFRCOSS` writer"]
pub type W = crate::W<HFRCOSS_SPEC>;
#[doc = "Field `SSAMP` reader - Spread Spectrum Amplitude"]
pub type SSAMP_R = crate::FieldReader;
#[doc = "Field `SSAMP` writer - Spread Spectrum Amplitude"]
pub type SSAMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SSINV` reader - Spread Spectrum Update Interval"]
pub type SSINV_R = crate::FieldReader;
#[doc = "Field `SSINV` writer - Spread Spectrum Update Interval"]
pub type SSINV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&self) -> SSAMP_R {
        SSAMP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&self) -> SSINV_R {
        SSINV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    #[must_use]
    pub fn ssamp(&mut self) -> SSAMP_W<HFRCOSS_SPEC> {
        SSAMP_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    #[must_use]
    pub fn ssinv(&mut self) -> SSINV_W<HFRCOSS_SPEC> {
        SSINV_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HFRCO Spread Spectrum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfrcoss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfrcoss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFRCOSS_SPEC;
impl crate::RegisterSpec for HFRCOSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcoss::R`](R) reader structure"]
impl crate::Readable for HFRCOSS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfrcoss::W`](W) writer structure"]
impl crate::Writable for HFRCOSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFRCOSS to value 0"]
impl crate::Resettable for HFRCOSS_SPEC {
    const RESET_VALUE: u32 = 0;
}
