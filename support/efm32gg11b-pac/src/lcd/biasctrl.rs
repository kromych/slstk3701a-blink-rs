#[doc = "Register `BIASCTRL` reader"]
pub type R = crate::R<BIASCTRL_SPEC>;
#[doc = "Register `BIASCTRL` writer"]
pub type W = crate::W<BIASCTRL_SPEC>;
#[doc = "Field `SPEED` reader - SPEED Adjustment"]
pub type SPEED_R = crate::FieldReader;
#[doc = "Field `SPEED` writer - SPEED Adjustment"]
pub type SPEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BUFDRV` reader - Buffer Drive Strength"]
pub type BUFDRV_R = crate::FieldReader;
#[doc = "Field `BUFDRV` writer - Buffer Drive Strength"]
pub type BUFDRV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BUFBIAS` reader - Buffer Bias Setting"]
pub type BUFBIAS_R = crate::FieldReader;
#[doc = "Field `BUFBIAS` writer - Buffer Bias Setting"]
pub type BUFBIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&self) -> BUFDRV_R {
        BUFDRV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&self) -> BUFBIAS_R {
        BUFBIAS_R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<BIASCTRL_SPEC, 0> {
        SPEED_W::new(self)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn bufdrv(&mut self) -> BUFDRV_W<BIASCTRL_SPEC, 4> {
        BUFDRV_W::new(self)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    #[must_use]
    pub fn bufbias(&mut self) -> BUFBIAS_W<BIASCTRL_SPEC, 10> {
        BUFBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Analog BIAS Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASCTRL_SPEC;
impl crate::RegisterSpec for BIASCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasctrl::R`](R) reader structure"]
impl crate::Readable for BIASCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`biasctrl::W`](W) writer structure"]
impl crate::Writable for BIASCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASCTRL to value 0"]
impl crate::Resettable for BIASCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
