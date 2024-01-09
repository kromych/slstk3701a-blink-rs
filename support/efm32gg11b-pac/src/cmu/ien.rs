#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `HFRCORDY` reader - HFRCORDY Interrupt Enable"]
pub type HFRCORDY_R = crate::BitReader;
#[doc = "Field `HFRCORDY` writer - HFRCORDY Interrupt Enable"]
pub type HFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` reader - HFXORDY Interrupt Enable"]
pub type HFXORDY_R = crate::BitReader;
#[doc = "Field `HFXORDY` writer - HFXORDY Interrupt Enable"]
pub type HFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` reader - LFRCORDY Interrupt Enable"]
pub type LFRCORDY_R = crate::BitReader;
#[doc = "Field `LFRCORDY` writer - LFRCORDY Interrupt Enable"]
pub type LFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` reader - LFXORDY Interrupt Enable"]
pub type LFXORDY_R = crate::BitReader;
#[doc = "Field `LFXORDY` writer - LFXORDY Interrupt Enable"]
pub type LFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCORDY Interrupt Enable"]
pub type AUXHFRCORDY_R = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCORDY Interrupt Enable"]
pub type AUXHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` reader - CALRDY Interrupt Enable"]
pub type CALRDY_R = crate::BitReader;
#[doc = "Field `CALRDY` writer - CALRDY Interrupt Enable"]
pub type CALRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` reader - CALOF Interrupt Enable"]
pub type CALOF_R = crate::BitReader;
#[doc = "Field `CALOF` writer - CALOF Interrupt Enable"]
pub type CALOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USHFRCORDY` reader - USHFRCORDY Interrupt Enable"]
pub type USHFRCORDY_R = crate::BitReader;
#[doc = "Field `USHFRCORDY` writer - USHFRCORDY Interrupt Enable"]
pub type USHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` reader - HFXODISERR Interrupt Enable"]
pub type HFXODISERR_R = crate::BitReader;
#[doc = "Field `HFXODISERR` writer - HFXODISERR Interrupt Enable"]
pub type HFXODISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` reader - HFXOAUTOSW Interrupt Enable"]
pub type HFXOAUTOSW_R = crate::BitReader;
#[doc = "Field `HFXOAUTOSW` writer - HFXOAUTOSW Interrupt Enable"]
pub type HFXOAUTOSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXOPEAKDETRDY Interrupt Enable"]
pub type HFXOPEAKDETRDY_R = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` writer - HFXOPEAKDETRDY Interrupt Enable"]
pub type HFXOPEAKDETRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` reader - HFRCODIS Interrupt Enable"]
pub type HFRCODIS_R = crate::BitReader;
#[doc = "Field `HFRCODIS` writer - HFRCODIS Interrupt Enable"]
pub type HFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` reader - LFTIMEOUTERR Interrupt Enable"]
pub type LFTIMEOUTERR_R = crate::BitReader;
#[doc = "Field `LFTIMEOUTERR` writer - LFTIMEOUTERR Interrupt Enable"]
pub type LFTIMEOUTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLRDY` reader - DPLLRDY Interrupt Enable"]
pub type DPLLRDY_R = crate::BitReader;
#[doc = "Field `DPLLRDY` writer - DPLLRDY Interrupt Enable"]
pub type DPLLRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILLOW` reader - DPLLLOCKFAILLOW Interrupt Enable"]
pub type DPLLLOCKFAILLOW_R = crate::BitReader;
#[doc = "Field `DPLLLOCKFAILLOW` writer - DPLLLOCKFAILLOW Interrupt Enable"]
pub type DPLLLOCKFAILLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLOCKFAILHIGH` reader - DPLLLOCKFAILHIGH Interrupt Enable"]
pub type DPLLLOCKFAILHIGH_R = crate::BitReader;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - DPLLLOCKFAILHIGH Interrupt Enable"]
pub type DPLLLOCKFAILHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXOEDGE` reader - LFXOEDGE Interrupt Enable"]
pub type LFXOEDGE_R = crate::BitReader;
#[doc = "Field `LFXOEDGE` writer - LFXOEDGE Interrupt Enable"]
pub type LFXOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCOEDGE` reader - LFRCOEDGE Interrupt Enable"]
pub type LFRCOEDGE_R = crate::BitReader;
#[doc = "Field `LFRCOEDGE` writer - LFRCOEDGE Interrupt Enable"]
pub type LFRCOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCOEDGE` reader - ULFRCOEDGE Interrupt Enable"]
pub type ULFRCOEDGE_R = crate::BitReader;
#[doc = "Field `ULFRCOEDGE` writer - ULFRCOEDGE Interrupt Enable"]
pub type ULFRCOEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` reader - CMUERR Interrupt Enable"]
pub type CMUERR_R = crate::BitReader;
#[doc = "Field `CMUERR` writer - CMUERR Interrupt Enable"]
pub type CMUERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HFXODISERR_R {
        HFXODISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HFXOAUTOSW_R {
        HFXOAUTOSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HFRCODIS_R {
        HFRCODIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DPLLLOCKFAILLOW_R {
        DPLLLOCKFAILLOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DPLLLOCKFAILHIGH_R {
        DPLLLOCKFAILHIGH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LFXOEDGE_R {
        LFXOEDGE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LFRCOEDGE_R {
        LFRCOEDGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> ULFRCOEDGE_R {
        ULFRCOEDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CMUERR_R {
        CMUERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IEN_SPEC> {
        HFRCORDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IEN_SPEC> {
        HFXORDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IEN_SPEC> {
        LFRCORDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IEN_SPEC> {
        LFXORDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IEN_SPEC> {
        AUXHFRCORDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IEN_SPEC> {
        CALRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IEN_SPEC> {
        CALOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - USHFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ushfrcordy(&mut self) -> USHFRCORDY_W<IEN_SPEC> {
        USHFRCORDY_W::new(self, 7)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W<IEN_SPEC> {
        HFXODISERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W<IEN_SPEC> {
        HFXOAUTOSW_W::new(self, 9)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W<IEN_SPEC> {
        HFXOPEAKDETRDY_W::new(self, 11)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<IEN_SPEC> {
        HFRCODIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W<IEN_SPEC> {
        LFTIMEOUTERR_W::new(self, 14)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpllrdy(&mut self) -> DPLLRDY_W<IEN_SPEC> {
        DPLLRDY_W::new(self, 15)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfaillow(&mut self) -> DPLLLOCKFAILLOW_W<IEN_SPEC> {
        DPLLLOCKFAILLOW_W::new(self, 16)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfailhigh(&mut self) -> DPLLLOCKFAILHIGH_W<IEN_SPEC> {
        DPLLLOCKFAILHIGH_W::new(self, 17)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoedge(&mut self) -> LFXOEDGE_W<IEN_SPEC> {
        LFXOEDGE_W::new(self, 27)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoedge(&mut self) -> LFRCOEDGE_W<IEN_SPEC> {
        LFRCOEDGE_W::new(self, 28)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulfrcoedge(&mut self) -> ULFRCOEDGE_W<IEN_SPEC> {
        ULFRCOEDGE_W::new(self, 29)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CMUERR_W<IEN_SPEC> {
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
