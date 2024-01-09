#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `HFRCORDY` writer - Clear HFRCORDY Interrupt Flag"]
pub type HFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - Clear HFXORDY Interrupt Flag"]
pub type HFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - Clear LFRCORDY Interrupt Flag"]
pub type LFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - Clear LFXORDY Interrupt Flag"]
pub type LFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - Clear AUXHFRCORDY Interrupt Flag"]
pub type AUXHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Clear CALRDY Interrupt Flag"]
pub type CALRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Clear CALOF Interrupt Flag"]
pub type CALOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` writer - Clear USHFRCORDY Interrupt Flag"]
pub type USHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` writer - Clear HFXODISERR Interrupt Flag"]
pub type HFXODISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` writer - Clear HFXOAUTOSW Interrupt Flag"]
pub type HFXOAUTOSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Clear HFXOPEAKDETRDY Interrupt Flag"]
pub type HFXOPEAKDETRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - Clear HFRCODIS Interrupt Flag"]
pub type HFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` writer - Clear LFTIMEOUTERR Interrupt Flag"]
pub type LFTIMEOUTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLRDY` writer - Clear DPLLRDY Interrupt Flag"]
pub type DPLLRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILLOW` writer - Clear DPLLLOCKFAILLOW Interrupt Flag"]
pub type DPLLLOCKFAILLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - Clear DPLLLOCKFAILHIGH Interrupt Flag"]
pub type DPLLLOCKFAILHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEDGE` writer - Clear LFXOEDGE Interrupt Flag"]
pub type LFXOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEDGE` writer - Clear LFRCOEDGE Interrupt Flag"]
pub type LFRCOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCOEDGE` writer - Clear ULFRCOEDGE Interrupt Flag"]
pub type ULFRCOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` writer - Clear CMUERR Interrupt Flag"]
pub type CMUERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear HFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IFC_SPEC> {
        HFRCORDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear HFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IFC_SPEC> {
        HFXORDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IFC_SPEC> {
        LFRCORDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear LFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IFC_SPEC> {
        LFXORDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IFC_SPEC> {
        AUXHFRCORDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CALRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IFC_SPEC> {
        CALRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear CALOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IFC_SPEC> {
        CALOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear USHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W<IFC_SPEC> {
        USHFRCORDY_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear HFXODISERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W<IFC_SPEC> {
        HFXODISERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W<IFC_SPEC> {
        HFXOAUTOSW_W::new(self, 9)
    }
    #[doc = "Bit 11 - Clear HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W<IFC_SPEC> {
        HFXOPEAKDETRDY_W::new(self, 11)
    }
    #[doc = "Bit 13 - Clear HFRCODIS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<IFC_SPEC> {
        HFRCODIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W<IFC_SPEC> {
        LFTIMEOUTERR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear DPLLRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpllrdy(&mut self) -> DPLLRDY_W<IFC_SPEC> {
        DPLLRDY_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfaillow(&mut self) -> DPLLLOCKFAILLOW_W<IFC_SPEC> {
        DPLLLOCKFAILLOW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfailhigh(&mut self) -> DPLLLOCKFAILHIGH_W<IFC_SPEC> {
        DPLLLOCKFAILHIGH_W::new(self, 17)
    }
    #[doc = "Bit 27 - Clear LFXOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoedge(&mut self) -> LFXOEDGE_W<IFC_SPEC> {
        LFXOEDGE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear LFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoedge(&mut self) -> LFRCOEDGE_W<IFC_SPEC> {
        LFRCOEDGE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear ULFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ulfrcoedge(&mut self) -> ULFRCOEDGE_W<IFC_SPEC> {
        ULFRCOEDGE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Clear CMUERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CMUERR_W<IFC_SPEC> {
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
