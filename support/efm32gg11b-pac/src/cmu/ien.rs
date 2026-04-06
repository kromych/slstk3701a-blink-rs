#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `HFRCORDY` reader - HFRCORDY Interrupt Enable"]
pub type HfrcordyR = crate::BitReader;
#[doc = "Field `HFRCORDY` writer - HFRCORDY Interrupt Enable"]
pub type HfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` reader - HFXORDY Interrupt Enable"]
pub type HfxordyR = crate::BitReader;
#[doc = "Field `HFXORDY` writer - HFXORDY Interrupt Enable"]
pub type HfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` reader - LFRCORDY Interrupt Enable"]
pub type LfrcordyR = crate::BitReader;
#[doc = "Field `LFRCORDY` writer - LFRCORDY Interrupt Enable"]
pub type LfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` reader - LFXORDY Interrupt Enable"]
pub type LfxordyR = crate::BitReader;
#[doc = "Field `LFXORDY` writer - LFXORDY Interrupt Enable"]
pub type LfxordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCORDY Interrupt Enable"]
pub type AuxhfrcordyR = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCORDY Interrupt Enable"]
pub type AuxhfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` reader - CALRDY Interrupt Enable"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `CALRDY` writer - CALRDY Interrupt Enable"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` reader - CALOF Interrupt Enable"]
pub type CalofR = crate::BitReader;
#[doc = "Field `CALOF` writer - CALOF Interrupt Enable"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` reader - USHFRCORDY Interrupt Enable"]
pub type UshfrcordyR = crate::BitReader;
#[doc = "Field `USHFRCORDY` writer - USHFRCORDY Interrupt Enable"]
pub type UshfrcordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` reader - HFXODISERR Interrupt Enable"]
pub type HfxodiserrR = crate::BitReader;
#[doc = "Field `HFXODISERR` writer - HFXODISERR Interrupt Enable"]
pub type HfxodiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` reader - HFXOAUTOSW Interrupt Enable"]
pub type HfxoautoswR = crate::BitReader;
#[doc = "Field `HFXOAUTOSW` writer - HFXOAUTOSW Interrupt Enable"]
pub type HfxoautoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXOPEAKDETRDY Interrupt Enable"]
pub type HfxopeakdetrdyR = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` writer - HFXOPEAKDETRDY Interrupt Enable"]
pub type HfxopeakdetrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` reader - HFRCODIS Interrupt Enable"]
pub type HfrcodisR = crate::BitReader;
#[doc = "Field `HFRCODIS` writer - HFRCODIS Interrupt Enable"]
pub type HfrcodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` reader - LFTIMEOUTERR Interrupt Enable"]
pub type LftimeouterrR = crate::BitReader;
#[doc = "Field `LFTIMEOUTERR` writer - LFTIMEOUTERR Interrupt Enable"]
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLRDY` reader - DPLLRDY Interrupt Enable"]
pub type DpllrdyR = crate::BitReader;
#[doc = "Field `DPLLRDY` writer - DPLLRDY Interrupt Enable"]
pub type DpllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILLOW` reader - DPLLLOCKFAILLOW Interrupt Enable"]
pub type DplllockfaillowR = crate::BitReader;
#[doc = "Field `DPLLLOCKFAILLOW` writer - DPLLLOCKFAILLOW Interrupt Enable"]
pub type DplllockfaillowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILHIGH` reader - DPLLLOCKFAILHIGH Interrupt Enable"]
pub type DplllockfailhighR = crate::BitReader;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - DPLLLOCKFAILHIGH Interrupt Enable"]
pub type DplllockfailhighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEDGE` reader - LFXOEDGE Interrupt Enable"]
pub type LfxoedgeR = crate::BitReader;
#[doc = "Field `LFXOEDGE` writer - LFXOEDGE Interrupt Enable"]
pub type LfxoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEDGE` reader - LFRCOEDGE Interrupt Enable"]
pub type LfrcoedgeR = crate::BitReader;
#[doc = "Field `LFRCOEDGE` writer - LFRCOEDGE Interrupt Enable"]
pub type LfrcoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCOEDGE` reader - ULFRCOEDGE Interrupt Enable"]
pub type UlfrcoedgeR = crate::BitReader;
#[doc = "Field `ULFRCOEDGE` writer - ULFRCOEDGE Interrupt Enable"]
pub type UlfrcoedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` reader - CMUERR Interrupt Enable"]
pub type CmuerrR = crate::BitReader;
#[doc = "Field `CMUERR` writer - CMUERR Interrupt Enable"]
pub type CmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> UshfrcordyR {
        UshfrcordyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HfxodiserrR {
        HfxodiserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HfxoautoswR {
        HfxoautoswR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HfxopeakdetrdyR {
        HfxopeakdetrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HfrcodisR {
        HfrcodisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LftimeouterrR {
        LftimeouterrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DpllrdyR {
        DpllrdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DplllockfaillowR {
        DplllockfaillowR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DplllockfailhighR {
        DplllockfailhighR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LfxoedgeR {
        LfxoedgeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LfrcoedgeR {
        LfrcoedgeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> UlfrcoedgeR {
        UlfrcoedgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CmuerrR {
        CmuerrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HfrcordyW<'_, IenSpec> {
        HfrcordyW::new(self, 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HfxordyW<'_, IenSpec> {
        HfxordyW::new(self, 1)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LfrcordyW<'_, IenSpec> {
        LfrcordyW::new(self, 2)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LfxordyW<'_, IenSpec> {
        LfxordyW::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AuxhfrcordyW<'_, IenSpec> {
        AuxhfrcordyW::new(self, 4)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<'_, IenSpec> {
        CalrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<'_, IenSpec> {
        CalofW::new(self, 6)
    }
    #[doc = "Bit 7 - USHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&mut self) -> UshfrcordyW<'_, IenSpec> {
        UshfrcordyW::new(self, 7)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&mut self) -> HfxodiserrW<'_, IenSpec> {
        HfxodiserrW::new(self, 8)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&mut self) -> HfxoautoswW<'_, IenSpec> {
        HfxoautoswW::new(self, 9)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&mut self) -> HfxopeakdetrdyW<'_, IenSpec> {
        HfxopeakdetrdyW::new(self, 11)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HfrcodisW<'_, IenSpec> {
        HfrcodisW::new(self, 13)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<'_, IenSpec> {
        LftimeouterrW::new(self, 14)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&mut self) -> DpllrdyW<'_, IenSpec> {
        DpllrdyW::new(self, 15)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&mut self) -> DplllockfaillowW<'_, IenSpec> {
        DplllockfaillowW::new(self, 16)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&mut self) -> DplllockfailhighW<'_, IenSpec> {
        DplllockfailhighW::new(self, 17)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&mut self) -> LfxoedgeW<'_, IenSpec> {
        LfxoedgeW::new(self, 27)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&mut self) -> LfrcoedgeW<'_, IenSpec> {
        LfrcoedgeW::new(self, 28)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&mut self) -> UlfrcoedgeW<'_, IenSpec> {
        UlfrcoedgeW::new(self, 29)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&mut self) -> CmuerrW<'_, IenSpec> {
        CmuerrW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
