#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `HFRCORDY` writer - Clear HFRCORDY Interrupt Flag"]
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - Clear HFXORDY Interrupt Flag"]
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - Clear LFRCORDY Interrupt Flag"]
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - Clear LFXORDY Interrupt Flag"]
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - Clear AUXHFRCORDY Interrupt Flag"]
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Clear CALRDY Interrupt Flag"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Clear CALOF Interrupt Flag"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` writer - Clear USHFRCORDY Interrupt Flag"]
pub type UshfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` writer - Clear HFXODISERR Interrupt Flag"]
pub type HfxodiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` writer - Clear HFXOAUTOSW Interrupt Flag"]
pub type HfxoautoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Clear HFXOPEAKDETRDY Interrupt Flag"]
pub type HfxopeakdetrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - Clear HFRCODIS Interrupt Flag"]
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` writer - Clear LFTIMEOUTERR Interrupt Flag"]
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLRDY` writer - Clear DPLLRDY Interrupt Flag"]
pub type DpllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILLOW` writer - Clear DPLLLOCKFAILLOW Interrupt Flag"]
pub type DplllockfaillowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - Clear DPLLLOCKFAILHIGH Interrupt Flag"]
pub type DplllockfailhighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEDGE` writer - Clear LFXOEDGE Interrupt Flag"]
pub type LfxoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEDGE` writer - Clear LFRCOEDGE Interrupt Flag"]
pub type LfrcoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCOEDGE` writer - Clear ULFRCOEDGE Interrupt Flag"]
pub type UlfrcoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` writer - Clear CMUERR Interrupt Flag"]
pub type CmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear HFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HfrcordyW<'_, IfcSpec> {
        HfrcordyW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear HFXORDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HfxordyW<'_, IfcSpec> {
        HfxordyW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LfrcordyW<'_, IfcSpec> {
        LfrcordyW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear LFXORDY Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LfxordyW<'_, IfcSpec> {
        LfxordyW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<'_, IfcSpec> {
        AuxhfrcordyW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CALRDY Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<'_, IfcSpec> {
        CalrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear CALOF Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<'_, IfcSpec> {
        CalofW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear USHFRCORDY Interrupt Flag"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> UshfrcordyW<'_, IfcSpec> {
        UshfrcordyW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear HFXODISERR Interrupt Flag"]
    #[inline(always)]
    pub fn hfxodiserr(&mut self) -> HfxodiserrW<'_, IfcSpec> {
        HfxodiserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoautosw(&mut self) -> HfxoautoswW<'_, IfcSpec> {
        HfxoautoswW::new(self, 9)
    }
    #[doc = "Bit 11 - Clear HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&mut self) -> HfxopeakdetrdyW<'_, IfcSpec> {
        HfxopeakdetrdyW::new(self, 11)
    }
    #[doc = "Bit 13 - Clear HFRCODIS Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HfrcodisW<'_, IfcSpec> {
        HfrcodisW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<'_, IfcSpec> {
        LftimeouterrW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear DPLLRDY Interrupt Flag"]
    #[inline(always)]
    pub fn dpllrdy(&mut self) -> DpllrdyW<'_, IfcSpec> {
        DpllrdyW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfaillow(&mut self) -> DplllockfaillowW<'_, IfcSpec> {
        DplllockfaillowW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfailhigh(&mut self) -> DplllockfailhighW<'_, IfcSpec> {
        DplllockfailhighW::new(self, 17)
    }
    #[doc = "Bit 27 - Clear LFXOEDGE Interrupt Flag"]
    #[inline(always)]
    pub fn lfxoedge(&mut self) -> LfxoedgeW<'_, IfcSpec> {
        LfxoedgeW::new(self, 27)
    }
    #[doc = "Bit 28 - Clear LFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcoedge(&mut self) -> LfrcoedgeW<'_, IfcSpec> {
        LfrcoedgeW::new(self, 28)
    }
    #[doc = "Bit 29 - Clear ULFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    pub fn ulfrcoedge(&mut self) -> UlfrcoedgeW<'_, IfcSpec> {
        UlfrcoedgeW::new(self, 29)
    }
    #[doc = "Bit 31 - Clear CMUERR Interrupt Flag"]
    #[inline(always)]
    pub fn cmuerr(&mut self) -> CmuerrW<'_, IfcSpec> {
        CmuerrW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
