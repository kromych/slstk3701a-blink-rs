#[doc = "Register `CAL` reader"]
pub type R = crate::R<CAL_SPEC>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CAL_SPEC>;
#[doc = "Field `OFFSETTRIM` reader - Input Buffer Offset Calibration Value"]
pub type OFFSETTRIM_R = crate::FieldReader;
#[doc = "Field `OFFSETTRIM` writer - Input Buffer Offset Calibration Value"]
pub type OFFSETTRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GAINERRTRIM` reader - Gain Error Trim Value"]
pub type GAINERRTRIM_R = crate::FieldReader;
#[doc = "Field `GAINERRTRIM` writer - Gain Error Trim Value"]
pub type GAINERRTRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `GAINERRTRIMCH1` reader - Gain Error Trim Value for CH1"]
pub type GAINERRTRIMCH1_R = crate::FieldReader;
#[doc = "Field `GAINERRTRIMCH1` writer - Gain Error Trim Value for CH1"]
pub type GAINERRTRIMCH1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&self) -> OFFSETTRIM_R {
        OFFSETTRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&self) -> GAINERRTRIM_R {
        GAINERRTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&self) -> GAINERRTRIMCH1_R {
        GAINERRTRIMCH1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsettrim(&mut self) -> OFFSETTRIM_W<CAL_SPEC, 0> {
        OFFSETTRIM_W::new(self)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn gainerrtrim(&mut self) -> GAINERRTRIM_W<CAL_SPEC, 8> {
        GAINERRTRIM_W::new(self)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    #[must_use]
    pub fn gainerrtrimch1(&mut self) -> GAINERRTRIMCH1_W<CAL_SPEC, 16> {
        GAINERRTRIMCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL to value 0x0008_2004"]
impl crate::Resettable for CAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_2004;
}
