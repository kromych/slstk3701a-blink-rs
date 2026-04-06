#[doc = "Register `TFTTIMING` reader"]
pub type R = crate::R<TfttimingSpec>;
#[doc = "Register `TFTTIMING` writer"]
pub type W = crate::W<TfttimingSpec>;
#[doc = "Field `DCLKPERIOD` reader - TFT Direct Drive Transaction (EBI_DCLK) Period"]
pub type DclkperiodR = crate::FieldReader<u16>;
#[doc = "Field `DCLKPERIOD` writer - TFT Direct Drive Transaction (EBI_DCLK) Period"]
pub type DclkperiodW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TFTSTART` reader - TFT Direct Drive Transaction Start"]
pub type TftstartR = crate::FieldReader<u16>;
#[doc = "Field `TFTSTART` writer - TFT Direct Drive Transaction Start"]
pub type TftstartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TFTSETUP` reader - TFT Setup Time"]
pub type TftsetupR = crate::FieldReader;
#[doc = "Field `TFTSETUP` writer - TFT Setup Time"]
pub type TftsetupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TFTHOLD` reader - TFT Hold Time"]
pub type TftholdR = crate::FieldReader;
#[doc = "Field `TFTHOLD` writer - TFT Hold Time"]
pub type TftholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    pub fn dclkperiod(&self) -> DclkperiodR {
        DclkperiodR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    pub fn tftstart(&self) -> TftstartR {
        TftstartR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:26 - TFT Setup Time"]
    #[inline(always)]
    pub fn tftsetup(&self) -> TftsetupR {
        TftsetupR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - TFT Hold Time"]
    #[inline(always)]
    pub fn tfthold(&self) -> TftholdR {
        TftholdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    pub fn dclkperiod(&mut self) -> DclkperiodW<'_, TfttimingSpec> {
        DclkperiodW::new(self, 0)
    }
    #[doc = "Bits 12:23 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    pub fn tftstart(&mut self) -> TftstartW<'_, TfttimingSpec> {
        TftstartW::new(self, 12)
    }
    #[doc = "Bits 24:26 - TFT Setup Time"]
    #[inline(always)]
    pub fn tftsetup(&mut self) -> TftsetupW<'_, TfttimingSpec> {
        TftsetupW::new(self, 24)
    }
    #[doc = "Bits 28:30 - TFT Hold Time"]
    #[inline(always)]
    pub fn tfthold(&mut self) -> TftholdW<'_, TfttimingSpec> {
        TftholdW::new(self, 28)
    }
}
#[doc = "TFT Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfttiming::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfttiming::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfttimingSpec;
impl crate::RegisterSpec for TfttimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfttiming::R`](R) reader structure"]
impl crate::Readable for TfttimingSpec {}
#[doc = "`write(|w| ..)` method takes [`tfttiming::W`](W) writer structure"]
impl crate::Writable for TfttimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTTIMING to value 0"]
impl crate::Resettable for TfttimingSpec {}
