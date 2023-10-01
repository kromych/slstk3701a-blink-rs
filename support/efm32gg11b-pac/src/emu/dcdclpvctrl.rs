#[doc = "Register `DCDCLPVCTRL` reader"]
pub type R = crate::R<DCDCLPVCTRL_SPEC>;
#[doc = "Register `DCDCLPVCTRL` writer"]
pub type W = crate::W<DCDCLPVCTRL_SPEC>;
#[doc = "Field `LPATT` reader - Low Power Feedback Attenuation"]
pub type LPATT_R = crate::BitReader;
#[doc = "Field `LPATT` writer - Low Power Feedback Attenuation"]
pub type LPATT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPVREF` reader - LP Mode Reference Selection for EM23 and EM4H"]
pub type LPVREF_R = crate::FieldReader;
#[doc = "Field `LPVREF` writer - LP Mode Reference Selection for EM23 and EM4H"]
pub type LPVREF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Low Power Feedback Attenuation"]
    #[inline(always)]
    pub fn lpatt(&self) -> LPATT_R {
        LPATT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpvref(&self) -> LPVREF_R {
        LPVREF_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Feedback Attenuation"]
    #[inline(always)]
    #[must_use]
    pub fn lpatt(&mut self) -> LPATT_W<DCDCLPVCTRL_SPEC, 0> {
        LPATT_W::new(self)
    }
    #[doc = "Bits 1:8 - LP Mode Reference Selection for EM23 and EM4H"]
    #[inline(always)]
    #[must_use]
    pub fn lpvref(&mut self) -> LPVREF_W<DCDCLPVCTRL_SPEC, 1> {
        LPVREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DCDC Low Power Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclpvctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclpvctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLPVCTRL_SPEC;
impl crate::RegisterSpec for DCDCLPVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclpvctrl::R`](R) reader structure"]
impl crate::Readable for DCDCLPVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdclpvctrl::W`](W) writer structure"]
impl crate::Writable for DCDCLPVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCLPVCTRL to value 0x0168"]
impl crate::Resettable for DCDCLPVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0168;
}
