#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `HFRCOENS` reader - HFRCO Enable Status"]
pub type HFRCOENS_R = crate::BitReader;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready"]
pub type HFRCORDY_R = crate::BitReader;
#[doc = "Field `HFXOENS` reader - HFXO Enable Status"]
pub type HFXOENS_R = crate::BitReader;
#[doc = "Field `HFXORDY` reader - HFXO Ready"]
pub type HFXORDY_R = crate::BitReader;
#[doc = "Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status"]
pub type AUXHFRCOENS_R = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready"]
pub type AUXHFRCORDY_R = crate::BitReader;
#[doc = "Field `LFRCOENS` reader - LFRCO Enable Status"]
pub type LFRCOENS_R = crate::BitReader;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready"]
pub type LFRCORDY_R = crate::BitReader;
#[doc = "Field `LFXOENS` reader - LFXO Enable Status"]
pub type LFXOENS_R = crate::BitReader;
#[doc = "Field `LFXORDY` reader - LFXO Ready"]
pub type LFXORDY_R = crate::BitReader;
#[doc = "Field `USHFRCOENS` reader - USHFRCO Enable Status"]
pub type USHFRCOENS_R = crate::BitReader;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready"]
pub type USHFRCORDY_R = crate::BitReader;
#[doc = "Field `DPLLENS` reader - DPLL Enable Status"]
pub type DPLLENS_R = crate::BitReader;
#[doc = "Field `DPLLRDY` reader - DPLL Ready"]
pub type DPLLRDY_R = crate::BitReader;
#[doc = "Field `CALRDY` reader - Calibration Ready"]
pub type CALRDY_R = crate::BitReader;
#[doc = "Field `SDIOCLKENS` reader - SDIO Clock Enabled Status"]
pub type SDIOCLKENS_R = crate::BitReader;
#[doc = "Field `QSPI0CLKENS` reader - QSPI0 Clock Enabled Status"]
pub type QSPI0CLKENS_R = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXO Peak Detection Ready"]
pub type HFXOPEAKDETRDY_R = crate::BitReader;
#[doc = "Field `HFXOAMPLOW` reader - HFXO Amplitude Tuning Value Too Low"]
pub type HFXOAMPLOW_R = crate::BitReader;
#[doc = "Field `LFXOPHASE` reader - LFXO Clock Phase"]
pub type LFXOPHASE_R = crate::BitReader;
#[doc = "Field `LFRCOPHASE` reader - LFRCO Clock Phase"]
pub type LFRCOPHASE_R = crate::BitReader;
#[doc = "Field `ULFRCOPHASE` reader - ULFRCO Clock Phase"]
pub type ULFRCOPHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USHFRCO Enable Status"]
    #[inline(always)]
    pub fn ushfrcoens(&self) -> USHFRCOENS_R {
        USHFRCOENS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USHFRCO Ready"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DPLL Enable Status"]
    #[inline(always)]
    pub fn dpllens(&self) -> DPLLENS_R {
        DPLLENS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DPLL Ready"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SDIO Clock Enabled Status"]
    #[inline(always)]
    pub fn sdioclkens(&self) -> SDIOCLKENS_R {
        SDIOCLKENS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - QSPI0 Clock Enabled Status"]
    #[inline(always)]
    pub fn qspi0clkens(&self) -> QSPI0CLKENS_R {
        QSPI0CLKENS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - HFXO Peak Detection Ready"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - HFXO Amplitude Tuning Value Too Low"]
    #[inline(always)]
    pub fn hfxoamplow(&self) -> HFXOAMPLOW_R {
        HFXOAMPLOW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXO Clock Phase"]
    #[inline(always)]
    pub fn lfxophase(&self) -> LFXOPHASE_R {
        LFXOPHASE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCO Clock Phase"]
    #[inline(always)]
    pub fn lfrcophase(&self) -> LFRCOPHASE_R {
        LFRCOPHASE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCO Clock Phase"]
    #[inline(always)]
    pub fn ulfrcophase(&self) -> ULFRCOPHASE_R {
        ULFRCOPHASE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x0001_0003"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0003;
}
