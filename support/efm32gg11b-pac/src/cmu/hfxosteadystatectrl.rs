#[doc = "Register `HFXOSTEADYSTATECTRL` reader"]
pub type R = crate::R<HFXOSTEADYSTATECTRL_SPEC>;
#[doc = "Register `HFXOSTEADYSTATECTRL` writer"]
pub type W = crate::W<HFXOSTEADYSTATECTRL_SPEC>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_R = crate::FieldReader<u16>;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current."]
pub type IBTRIMXOCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_R = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CTUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_R = crate::BitReader;
#[doc = "Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors"]
pub type PEAKDETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAKMONEN` reader - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
pub type PEAKMONEN_R = crate::BitReader;
#[doc = "Field `PEAKMONEN` writer - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
pub type PEAKMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PEAKDETEN_R {
        PEAKDETEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&self) -> PEAKMONEN_R {
        PEAKMONEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W<HFXOSTEADYSTATECTRL_SPEC> {
        IBTRIMXOCORE_W::new(self, 0)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CTUNE_W<HFXOSTEADYSTATECTRL_SPEC> {
        CTUNE_W::new(self, 11)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    #[must_use]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W<HFXOSTEADYSTATECTRL_SPEC> {
        PEAKDETEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    #[must_use]
    pub fn peakmonen(&mut self) -> PEAKMONEN_W<HFXOSTEADYSTATECTRL_SPEC> {
        PEAKMONEN_W::new(self, 27)
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
#[doc = "HFXO Steady State Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosteadystatectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosteadystatectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOSTEADYSTATECTRL_SPEC;
impl crate::RegisterSpec for HFXOSTEADYSTATECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosteadystatectrl::R`](R) reader structure"]
impl crate::Readable for HFXOSTEADYSTATECTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfxosteadystatectrl::W`](W) writer structure"]
impl crate::Writable for HFXOSTEADYSTATECTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOSTEADYSTATECTRL to value 0x0800_0100"]
impl crate::Resettable for HFXOSTEADYSTATECTRL_SPEC {
    const RESET_VALUE: u32 = 0x0800_0100;
}
