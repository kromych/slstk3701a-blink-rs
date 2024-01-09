#[doc = "Register `HFXOSTARTUPCTRL` reader"]
pub type R = crate::R<HFXOSTARTUPCTRL_SPEC>;
#[doc = "Register `HFXOSTARTUPCTRL` writer"]
pub type W = crate::W<HFXOSTARTUPCTRL_SPEC>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_R = crate::FieldReader<u16>;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Startup Oscillator Core Bias Current"]
pub type IBTRIMXOCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:10 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<HFXOSTARTUPCTRL_SPEC> {
        IBTRIMXOCORE_W::new(self, 0)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<HFXOSTARTUPCTRL_SPEC> {
        CTUNE_W::new(self, 11)
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
#[doc = "HFXO Startup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxostartupctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxostartupctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOSTARTUPCTRL_SPEC;
impl crate::RegisterSpec for HFXOSTARTUPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxostartupctrl::R`](R) reader structure"]
impl crate::Readable for HFXOSTARTUPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxostartupctrl::W`](W) writer structure"]
impl crate::Writable for HFXOSTARTUPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOSTARTUPCTRL to value 0x0600"]
impl crate::Resettable for HFXOSTARTUPCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0600;
}
