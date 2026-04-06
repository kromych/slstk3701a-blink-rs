#[doc = "Register `HFXOSTEADYSTATECTRL` reader"]
pub type R = crate::R<HfxosteadystatectrlSpec>;
#[doc = "Register `HFXOSTEADYSTATECTRL` writer"]
pub type W = crate::W<HfxosteadystatectrlSpec>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Steady State Oscillator Core Bias Current."]
pub type IbtrimxocoreR = crate::FieldReader<u16>;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Steady State Oscillator Core Bias Current."]
pub type IbtrimxocoreW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CtuneR = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CtuneW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PEAKDETEN` reader - Enables Oscillator Peak Detectors"]
pub type PeakdetenR = crate::BitReader;
#[doc = "Field `PEAKDETEN` writer - Enables Oscillator Peak Detectors"]
pub type PeakdetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAKMONEN` reader - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
pub type PeakmonenR = crate::BitReader;
#[doc = "Field `PEAKMONEN` writer - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
pub type PeakmonenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IbtrimxocoreR {
        IbtrimxocoreR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CtuneR {
        CtuneR::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PeakdetenR {
        PeakdetenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&self) -> PeakmonenR {
        PeakmonenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IbtrimxocoreW<'_, HfxosteadystatectrlSpec> {
        IbtrimxocoreW::new(self, 0)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CtuneW<'_, HfxosteadystatectrlSpec> {
        CtuneW::new(self, 11)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&mut self) -> PeakdetenW<'_, HfxosteadystatectrlSpec> {
        PeakdetenW::new(self, 26)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&mut self) -> PeakmonenW<'_, HfxosteadystatectrlSpec> {
        PeakmonenW::new(self, 27)
    }
}
#[doc = "HFXO Steady State Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxosteadystatectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxosteadystatectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxosteadystatectrlSpec;
impl crate::RegisterSpec for HfxosteadystatectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxosteadystatectrl::R`](R) reader structure"]
impl crate::Readable for HfxosteadystatectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxosteadystatectrl::W`](W) writer structure"]
impl crate::Writable for HfxosteadystatectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFXOSTEADYSTATECTRL to value 0x0800_0100"]
impl crate::Resettable for HfxosteadystatectrlSpec {
    const RESET_VALUE: u32 = 0x0800_0100;
}
