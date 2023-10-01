#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub type CALSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub type CALSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXOPEAKDETSTART` writer - HFXO Peak Detection Start"]
pub type HFXOPEAKDETSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn calstart(&mut self) -> CALSTART_W<CMD_SPEC, 0> {
        CALSTART_W::new(self)
    }
    #[doc = "Bit 1 - Calibration Stop"]
    #[inline(always)]
    #[must_use]
    pub fn calstop(&mut self) -> CALSTOP_W<CMD_SPEC, 1> {
        CALSTOP_W::new(self)
    }
    #[doc = "Bit 4 - HFXO Peak Detection Start"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetstart(&mut self) -> HFXOPEAKDETSTART_W<CMD_SPEC, 4> {
        HFXOPEAKDETSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
