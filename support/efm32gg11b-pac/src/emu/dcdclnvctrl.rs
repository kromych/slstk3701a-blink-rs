#[doc = "Register `DCDCLNVCTRL` reader"]
pub type R = crate::R<DCDCLNVCTRL_SPEC>;
#[doc = "Register `DCDCLNVCTRL` writer"]
pub type W = crate::W<DCDCLNVCTRL_SPEC>;
#[doc = "Field `LNATT` reader - Low Noise Mode Feedback Attenuation"]
pub type LNATT_R = crate::BitReader;
#[doc = "Field `LNATT` writer - Low Noise Mode Feedback Attenuation"]
pub type LNATT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNVREF` reader - Low Noise Mode VREF Trim"]
pub type LNVREF_R = crate::FieldReader;
#[doc = "Field `LNVREF` writer - Low Noise Mode VREF Trim"]
pub type LNVREF_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    pub fn lnatt(&self) -> LNATT_R {
        LNATT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    pub fn lnvref(&self) -> LNVREF_R {
        LNVREF_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Low Noise Mode Feedback Attenuation"]
    #[inline(always)]
    #[must_use]
    pub fn lnatt(&mut self) -> LNATT_W<DCDCLNVCTRL_SPEC> {
        LNATT_W::new(self, 1)
    }
    #[doc = "Bits 8:14 - Low Noise Mode VREF Trim"]
    #[inline(always)]
    #[must_use]
    pub fn lnvref(&mut self) -> LNVREF_W<DCDCLNVCTRL_SPEC> {
        LNVREF_W::new(self, 8)
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
#[doc = "DCDC Low Noise Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclnvctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclnvctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLNVCTRL_SPEC;
impl crate::RegisterSpec for DCDCLNVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclnvctrl::R`](R) reader structure"]
impl crate::Readable for DCDCLNVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdclnvctrl::W`](W) writer structure"]
impl crate::Writable for DCDCLNVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCLNVCTRL to value 0x7100"]
impl crate::Resettable for DCDCLNVCTRL_SPEC {
    const RESET_VALUE: u32 = 0x7100;
}
