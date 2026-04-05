#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `HFRCOENS` reader - HFRCO Enable Status"]
pub type HfrcoensR = crate::BitReader;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready"]
pub type HfrcordyR = crate::BitReader;
#[doc = "Field `HFXOENS` reader - HFXO Enable Status"]
pub type HfxoensR = crate::BitReader;
#[doc = "Field `HFXORDY` reader - HFXO Ready"]
pub type HfxordyR = crate::BitReader;
#[doc = "Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status"]
pub type AuxhfrcoensR = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready"]
pub type AuxhfrcordyR = crate::BitReader;
#[doc = "Field `LFRCOENS` reader - LFRCO Enable Status"]
pub type LfrcoensR = crate::BitReader;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready"]
pub type LfrcordyR = crate::BitReader;
#[doc = "Field `LFXOENS` reader - LFXO Enable Status"]
pub type LfxoensR = crate::BitReader;
#[doc = "Field `LFXORDY` reader - LFXO Ready"]
pub type LfxordyR = crate::BitReader;
#[doc = "Field `USHFRCOENS` reader - USHFRCO Enable Status"]
pub type UshfrcoensR = crate::BitReader;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready"]
pub type UshfrcordyR = crate::BitReader;
#[doc = "Field `DPLLENS` reader - DPLL Enable Status"]
pub type DpllensR = crate::BitReader;
#[doc = "Field `DPLLRDY` reader - DPLL Ready"]
pub type DpllrdyR = crate::BitReader;
#[doc = "Field `CALRDY` reader - Calibration Ready"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `SDIOCLKENS` reader - SDIO Clock Enabled Status"]
pub type SdioclkensR = crate::BitReader;
#[doc = "Field `QSPI0CLKENS` reader - QSPI0 Clock Enabled Status"]
pub type Qspi0clkensR = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXO Peak Detection Ready"]
pub type HfxopeakdetrdyR = crate::BitReader;
#[doc = "Field `HFXOAMPLOW` reader - HFXO Amplitude Tuning Value Too Low"]
pub type HfxoamplowR = crate::BitReader;
#[doc = "Field `LFXOPHASE` reader - LFXO Clock Phase"]
pub type LfxophaseR = crate::BitReader;
#[doc = "Field `LFRCOPHASE` reader - LFRCO Clock Phase"]
pub type LfrcophaseR = crate::BitReader;
#[doc = "Field `ULFRCOPHASE` reader - ULFRCO Clock Phase"]
pub type UlfrcophaseR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HfrcoensR {
        HfrcoensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HfxoensR {
        HfxoensR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AuxhfrcoensR {
        AuxhfrcoensR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LfrcoensR {
        LfrcoensR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LfxoensR {
        LfxoensR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USHFRCO Enable Status"]
    #[inline(always)]
    pub fn ushfrcoens(&self) -> UshfrcoensR {
        UshfrcoensR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USHFRCO Ready"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> UshfrcordyR {
        UshfrcordyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DPLL Enable Status"]
    #[inline(always)]
    pub fn dpllens(&self) -> DpllensR {
        DpllensR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DPLL Ready"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DpllrdyR {
        DpllrdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SDIO Clock Enabled Status"]
    #[inline(always)]
    pub fn sdioclkens(&self) -> SdioclkensR {
        SdioclkensR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - QSPI0 Clock Enabled Status"]
    #[inline(always)]
    pub fn qspi0clkens(&self) -> Qspi0clkensR {
        Qspi0clkensR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - HFXO Peak Detection Ready"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HfxopeakdetrdyR {
        HfxopeakdetrdyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - HFXO Amplitude Tuning Value Too Low"]
    #[inline(always)]
    pub fn hfxoamplow(&self) -> HfxoamplowR {
        HfxoamplowR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXO Clock Phase"]
    #[inline(always)]
    pub fn lfxophase(&self) -> LfxophaseR {
        LfxophaseR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCO Clock Phase"]
    #[inline(always)]
    pub fn lfrcophase(&self) -> LfrcophaseR {
        LfrcophaseR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCO Clock Phase"]
    #[inline(always)]
    pub fn ulfrcophase(&self) -> UlfrcophaseR {
        UlfrcophaseR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x0001_0003"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0001_0003;
}
