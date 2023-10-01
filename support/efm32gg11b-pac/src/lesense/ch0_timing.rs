#[doc = "Register `CH0_TIMING` reader"]
pub type R = crate::R<CH0_TIMING_SPEC>;
#[doc = "Register `CH0_TIMING` writer"]
pub type W = crate::W<CH0_TIMING_SPEC>;
#[doc = "Field `EXTIME` reader - Set Excitation Time"]
pub type EXTIME_R = crate::FieldReader;
#[doc = "Field `EXTIME` writer - Set Excitation Time"]
pub type EXTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SAMPLEDLY` reader - Set Sample Delay"]
pub type SAMPLEDLY_R = crate::FieldReader;
#[doc = "Field `SAMPLEDLY` writer - Set Sample Delay"]
pub type SAMPLEDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MEASUREDLY` reader - Set Measure Delay"]
pub type MEASUREDLY_R = crate::FieldReader<u16>;
#[doc = "Field `MEASUREDLY` writer - Set Measure Delay"]
pub type MEASUREDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:5 - Set Excitation Time"]
    #[inline(always)]
    pub fn extime(&self) -> EXTIME_R {
        EXTIME_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Set Sample Delay"]
    #[inline(always)]
    pub fn sampledly(&self) -> SAMPLEDLY_R {
        SAMPLEDLY_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Set Measure Delay"]
    #[inline(always)]
    pub fn measuredly(&self) -> MEASUREDLY_R {
        MEASUREDLY_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set Excitation Time"]
    #[inline(always)]
    #[must_use]
    pub fn extime(&mut self) -> EXTIME_W<CH0_TIMING_SPEC, 0> {
        EXTIME_W::new(self)
    }
    #[doc = "Bits 6:13 - Set Sample Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sampledly(&mut self) -> SAMPLEDLY_W<CH0_TIMING_SPEC, 6> {
        SAMPLEDLY_W::new(self)
    }
    #[doc = "Bits 14:23 - Set Measure Delay"]
    #[inline(always)]
    #[must_use]
    pub fn measuredly(&mut self) -> MEASUREDLY_W<CH0_TIMING_SPEC, 14> {
        MEASUREDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scan Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0_timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0_TIMING_SPEC;
impl crate::RegisterSpec for CH0_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_timing::R`](R) reader structure"]
impl crate::Readable for CH0_TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch0_timing::W`](W) writer structure"]
impl crate::Writable for CH0_TIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0_TIMING to value 0"]
impl crate::Resettable for CH0_TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
