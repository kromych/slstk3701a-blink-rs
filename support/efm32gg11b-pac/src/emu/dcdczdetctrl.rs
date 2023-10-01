#[doc = "Register `DCDCZDETCTRL` reader"]
pub type R = crate::R<DCDCZDETCTRL_SPEC>;
#[doc = "Register `DCDCZDETCTRL` writer"]
pub type W = crate::W<DCDCZDETCTRL_SPEC>;
#[doc = "Field `ZDETILIMSEL` reader - Reverse Current Limit Level Selection for Zero Detector"]
pub type ZDETILIMSEL_R = crate::FieldReader;
#[doc = "Field `ZDETILIMSEL` writer - Reverse Current Limit Level Selection for Zero Detector"]
pub type ZDETILIMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ZDETBLANKDLY` reader - Reserved for internal use. Do not change."]
pub type ZDETBLANKDLY_R = crate::FieldReader;
#[doc = "Field `ZDETBLANKDLY` writer - Reserved for internal use. Do not change."]
pub type ZDETBLANKDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    pub fn zdetilimsel(&self) -> ZDETILIMSEL_R {
        ZDETILIMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn zdetblankdly(&self) -> ZDETBLANKDLY_R {
        ZDETBLANKDLY_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    #[must_use]
    pub fn zdetilimsel(&mut self) -> ZDETILIMSEL_W<DCDCZDETCTRL_SPEC, 4> {
        ZDETILIMSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn zdetblankdly(&mut self) -> ZDETBLANKDLY_W<DCDCZDETCTRL_SPEC, 8> {
        ZDETBLANKDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdczdetctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdczdetctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCZDETCTRL_SPEC;
impl crate::RegisterSpec for DCDCZDETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdczdetctrl::R`](R) reader structure"]
impl crate::Readable for DCDCZDETCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdczdetctrl::W`](W) writer structure"]
impl crate::Writable for DCDCZDETCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCZDETCTRL to value 0x0150"]
impl crate::Resettable for DCDCZDETCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0150;
}
