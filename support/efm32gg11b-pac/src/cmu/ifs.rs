#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `HFRCORDY` writer - Set HFRCORDY Interrupt Flag"]
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - Set HFXORDY Interrupt Flag"]
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - Set LFRCORDY Interrupt Flag"]
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - Set LFXORDY Interrupt Flag"]
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - Set AUXHFRCORDY Interrupt Flag"]
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Set CALRDY Interrupt Flag"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Set CALOF Interrupt Flag"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` writer - Set USHFRCORDY Interrupt Flag"]
pub type UshfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` writer - Set HFXODISERR Interrupt Flag"]
pub type HfxodiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` writer - Set HFXOAUTOSW Interrupt Flag"]
pub type HfxoautoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Set HFXOPEAKDETRDY Interrupt Flag"]
pub type HfxopeakdetrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - Set HFRCODIS Interrupt Flag"]
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` writer - Set LFTIMEOUTERR Interrupt Flag"]
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLRDY` writer - Set DPLLRDY Interrupt Flag"]
pub type DpllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILLOW` writer - Set DPLLLOCKFAILLOW Interrupt Flag"]
pub type DplllockfaillowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - Set DPLLLOCKFAILHIGH Interrupt Flag"]
pub type DplllockfailhighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEDGE` writer - Set LFXOEDGE Interrupt Flag"]
pub type LfxoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEDGE` writer - Set LFRCOEDGE Interrupt Flag"]
pub type LfrcoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCOEDGE` writer - Set ULFRCOEDGE Interrupt Flag"]
pub type UlfrcoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` writer - Set CMUERR Interrupt Flag"]
pub type CmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set HFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HfrcordyW<'_, IfsSpec> {
        HfrcordyW::new(self, 0)
    }
    #[doc = "Bit 1 - Set HFXORDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HfxordyW<'_, IfsSpec> {
        HfxordyW::new(self, 1)
    }
    #[doc = "Bit 2 - Set LFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LfrcordyW<'_, IfsSpec> {
        LfrcordyW::new(self, 2)
    }
    #[doc = "Bit 3 - Set LFXORDY Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LfxordyW<'_, IfsSpec> {
        LfxordyW::new(self, 3)
    }
    #[doc = "Bit 4 - Set AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<'_, IfsSpec> {
        AuxhfrcordyW::new(self, 4)
    }
    #[doc = "Bit 5 - Set CALRDY Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<'_, IfsSpec> {
        CalrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Set CALOF Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<'_, IfsSpec> {
        CalofW::new(self, 6)
    }
    #[doc = "Bit 7 - Set USHFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> UshfrcordyW<'_, IfsSpec> {
        UshfrcordyW::new(self, 7)
    }
    #[doc = "Bit 8 - Set HFXODISERR Interrupt Flag"]
    #[inline(always)]
    pub fn hfxodiserr(&mut self) -> HfxodiserrW<'_, IfsSpec> {
        HfxodiserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Set HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoautosw(&mut self) -> HfxoautoswW<'_, IfsSpec> {
        HfxoautoswW::new(self, 9)
    }
    #[doc = "Bit 11 - Set HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&mut self) -> HfxopeakdetrdyW<'_, IfsSpec> {
        HfxopeakdetrdyW::new(self, 11)
    }
    #[doc = "Bit 13 - Set HFRCODIS Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HfrcodisW<'_, IfsSpec> {
        HfrcodisW::new(self, 13)
    }
    #[doc = "Bit 14 - Set LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<'_, IfsSpec> {
        LftimeouterrW::new(self, 14)
    }
    #[doc = "Bit 15 - Set DPLLRDY Interrupt Flag"]
    #[inline(always)]
    pub fn dpllrdy(&mut self) -> DpllrdyW<'_, IfsSpec> {
        DpllrdyW::new(self, 15)
    }
    #[doc = "Bit 16 - Set DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfaillow(&mut self) -> DplllockfaillowW<'_, IfsSpec> {
        DplllockfaillowW::new(self, 16)
    }
    #[doc = "Bit 17 - Set DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfailhigh(&mut self) -> DplllockfailhighW<'_, IfsSpec> {
        DplllockfailhighW::new(self, 17)
    }
    #[doc = "Bit 27 - Set LFXOEDGE Interrupt Flag"]
    #[inline(always)]
    pub fn lfxoedge(&mut self) -> LfxoedgeW<'_, IfsSpec> {
        LfxoedgeW::new(self, 27)
    }
    #[doc = "Bit 28 - Set LFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcoedge(&mut self) -> LfrcoedgeW<'_, IfsSpec> {
        LfrcoedgeW::new(self, 28)
    }
    #[doc = "Bit 29 - Set ULFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    pub fn ulfrcoedge(&mut self) -> UlfrcoedgeW<'_, IfsSpec> {
        UlfrcoedgeW::new(self, 29)
    }
    #[doc = "Bit 31 - Set CMUERR Interrupt Flag"]
    #[inline(always)]
    pub fn cmuerr(&mut self) -> CmuerrW<'_, IfsSpec> {
        CmuerrW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
