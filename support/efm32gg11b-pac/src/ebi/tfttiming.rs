#[doc = "Register `TFTTIMING` reader"]
pub type R = crate::R<TFTTIMING_SPEC>;
#[doc = "Register `TFTTIMING` writer"]
pub type W = crate::W<TFTTIMING_SPEC>;
#[doc = "Field `DCLKPERIOD` reader - TFT Direct Drive Transaction (EBI_DCLK) Period"]
pub type DCLKPERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `DCLKPERIOD` writer - TFT Direct Drive Transaction (EBI_DCLK) Period"]
pub type DCLKPERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `TFTSTART` reader - TFT Direct Drive Transaction Start"]
pub type TFTSTART_R = crate::FieldReader<u16>;
#[doc = "Field `TFTSTART` writer - TFT Direct Drive Transaction Start"]
pub type TFTSTART_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `TFTSETUP` reader - TFT Setup Time"]
pub type TFTSETUP_R = crate::FieldReader;
#[doc = "Field `TFTSETUP` writer - TFT Setup Time"]
pub type TFTSETUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TFTHOLD` reader - TFT Hold Time"]
pub type TFTHOLD_R = crate::FieldReader;
#[doc = "Field `TFTHOLD` writer - TFT Hold Time"]
pub type TFTHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:11 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    pub fn dclkperiod(&self) -> DCLKPERIOD_R {
        DCLKPERIOD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    pub fn tftstart(&self) -> TFTSTART_R {
        TFTSTART_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:26 - TFT Setup Time"]
    #[inline(always)]
    pub fn tftsetup(&self) -> TFTSETUP_R {
        TFTSETUP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - TFT Hold Time"]
    #[inline(always)]
    pub fn tfthold(&self) -> TFTHOLD_R {
        TFTHOLD_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    #[must_use]
    pub fn dclkperiod(&mut self) -> DCLKPERIOD_W<TFTTIMING_SPEC, 0> {
        DCLKPERIOD_W::new(self)
    }
    #[doc = "Bits 12:23 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    #[must_use]
    pub fn tftstart(&mut self) -> TFTSTART_W<TFTTIMING_SPEC, 12> {
        TFTSTART_W::new(self)
    }
    #[doc = "Bits 24:26 - TFT Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn tftsetup(&mut self) -> TFTSETUP_W<TFTTIMING_SPEC, 24> {
        TFTSETUP_W::new(self)
    }
    #[doc = "Bits 28:30 - TFT Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn tfthold(&mut self) -> TFTHOLD_W<TFTTIMING_SPEC, 28> {
        TFTHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfttiming::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfttiming::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTTIMING_SPEC;
impl crate::RegisterSpec for TFTTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfttiming::R`](R) reader structure"]
impl crate::Readable for TFTTIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfttiming::W`](W) writer structure"]
impl crate::Writable for TFTTIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTTIMING to value 0"]
impl crate::Resettable for TFTTIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
