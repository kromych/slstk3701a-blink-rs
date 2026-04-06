#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Flag"]
pub type HfrcordyR = crate::BitReader;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Flag"]
pub type HfxordyR = crate::BitReader;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Flag"]
pub type LfrcordyR = crate::BitReader;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Flag"]
pub type LfxordyR = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Flag"]
pub type AuxhfrcordyR = crate::BitReader;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Flag"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Flag"]
pub type CalofR = crate::BitReader;
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready Interrupt Flag"]
pub type UshfrcordyR = crate::BitReader;
#[doc = "Field `HFXODISERR` reader - HFXO Disable Error Interrupt Flag"]
pub type HfxodiserrR = crate::BitReader;
#[doc = "Field `HFXOAUTOSW` reader - HFXO Automatic Switch Interrupt Flag"]
pub type HfxoautoswR = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXO Automatic Peak Detection Ready Interrupt Flag"]
pub type HfxopeakdetrdyR = crate::BitReader;
#[doc = "Field `HFRCODIS` reader - HFRCO Disable Interrupt Flag"]
pub type HfrcodisR = crate::BitReader;
#[doc = "Field `LFTIMEOUTERR` reader - Low Frequency Timeout Error Interrupt Flag"]
pub type LftimeouterrR = crate::BitReader;
#[doc = "Field `DPLLRDY` reader - DPLL Lock Interrupt Flag"]
pub type DpllrdyR = crate::BitReader;
#[doc = "Field `DPLLLOCKFAILLOW` reader - DPLL Lock Failure Low Interrupt Flag"]
pub type DplllockfaillowR = crate::BitReader;
#[doc = "Field `DPLLLOCKFAILHIGH` reader - DPLL Lock Failure Low Interrupt Flag"]
pub type DplllockfailhighR = crate::BitReader;
#[doc = "Field `LFXOEDGE` reader - LFXO Clock Edge Detected Interrupt Flag"]
pub type LfxoedgeR = crate::BitReader;
#[doc = "Field `LFRCOEDGE` reader - LFRCO Clock Edge Detected Interrupt Flag"]
pub type LfrcoedgeR = crate::BitReader;
#[doc = "Field `ULFRCOEDGE` reader - ULFRCO Clock Edge Detected Interrupt Flag"]
pub type UlfrcoedgeR = crate::BitReader;
#[doc = "Field `CMUERR` reader - CMU Error Interrupt Flag"]
pub type CmuerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> UshfrcordyR {
        UshfrcordyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HFXO Disable Error Interrupt Flag"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HfxodiserrR {
        HfxodiserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HFXO Automatic Switch Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HfxoautoswR {
        HfxoautoswR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXO Automatic Peak Detection Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HfxopeakdetrdyR {
        HfxopeakdetrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - HFRCO Disable Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HfrcodisR {
        HfrcodisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low Frequency Timeout Error Interrupt Flag"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LftimeouterrR {
        LftimeouterrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DpllrdyR {
        DpllrdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DplllockfaillowR {
        DplllockfaillowR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DplllockfailhighR {
        DplllockfailhighR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LfxoedgeR {
        LfxoedgeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LfrcoedgeR {
        LfrcoedgeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> UlfrcoedgeR {
        UlfrcoedgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CMU Error Interrupt Flag"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CmuerrR {
        CmuerrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0x01"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0x01;
}
