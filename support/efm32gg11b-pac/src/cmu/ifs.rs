#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `HFRCORDY` writer - Set HFRCORDY Interrupt Flag"]
pub type HFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - Set HFXORDY Interrupt Flag"]
pub type HFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - Set LFRCORDY Interrupt Flag"]
pub type LFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - Set LFXORDY Interrupt Flag"]
pub type LFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - Set AUXHFRCORDY Interrupt Flag"]
pub type AUXHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Set CALRDY Interrupt Flag"]
pub type CALRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Set CALOF Interrupt Flag"]
pub type CALOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` writer - Set USHFRCORDY Interrupt Flag"]
pub type USHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` writer - Set HFXODISERR Interrupt Flag"]
pub type HFXODISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` writer - Set HFXOAUTOSW Interrupt Flag"]
pub type HFXOAUTOSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Set HFXOPEAKDETRDY Interrupt Flag"]
pub type HFXOPEAKDETRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - Set HFRCODIS Interrupt Flag"]
pub type HFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` writer - Set LFTIMEOUTERR Interrupt Flag"]
pub type LFTIMEOUTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLRDY` writer - Set DPLLRDY Interrupt Flag"]
pub type DPLLRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILLOW` writer - Set DPLLLOCKFAILLOW Interrupt Flag"]
pub type DPLLLOCKFAILLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - Set DPLLLOCKFAILHIGH Interrupt Flag"]
pub type DPLLLOCKFAILHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEDGE` writer - Set LFXOEDGE Interrupt Flag"]
pub type LFXOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEDGE` writer - Set LFRCOEDGE Interrupt Flag"]
pub type LFRCOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCOEDGE` writer - Set ULFRCOEDGE Interrupt Flag"]
pub type ULFRCOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` writer - Set CMUERR Interrupt Flag"]
pub type CMUERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set HFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IFS_SPEC> {
        HFRCORDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set HFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IFS_SPEC> {
        HFXORDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set LFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IFS_SPEC> {
        LFRCORDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set LFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IFS_SPEC> {
        LFXORDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IFS_SPEC> {
        AUXHFRCORDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set CALRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IFS_SPEC> {
        CALRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set CALOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IFS_SPEC> {
        CALOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set USHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W<IFS_SPEC> {
        USHFRCORDY_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set HFXODISERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W<IFS_SPEC> {
        HFXODISERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W<IFS_SPEC> {
        HFXOAUTOSW_W::new(self, 9)
    }
    #[doc = "Bit 11 - Set HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W<IFS_SPEC> {
        HFXOPEAKDETRDY_W::new(self, 11)
    }
    #[doc = "Bit 13 - Set HFRCODIS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<IFS_SPEC> {
        HFRCODIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W<IFS_SPEC> {
        LFTIMEOUTERR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set DPLLRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpllrdy(&mut self) -> DPLLRDY_W<IFS_SPEC> {
        DPLLRDY_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfaillow(&mut self) -> DPLLLOCKFAILLOW_W<IFS_SPEC> {
        DPLLLOCKFAILLOW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfailhigh(&mut self) -> DPLLLOCKFAILHIGH_W<IFS_SPEC> {
        DPLLLOCKFAILHIGH_W::new(self, 17)
    }
    #[doc = "Bit 27 - Set LFXOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoedge(&mut self) -> LFXOEDGE_W<IFS_SPEC> {
        LFXOEDGE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set LFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoedge(&mut self) -> LFRCOEDGE_W<IFS_SPEC> {
        LFRCOEDGE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set ULFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ulfrcoedge(&mut self) -> ULFRCOEDGE_W<IFS_SPEC> {
        ULFRCOEDGE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Set CMUERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CMUERR_W<IFS_SPEC> {
        CMUERR_W::new(self, 31)
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
