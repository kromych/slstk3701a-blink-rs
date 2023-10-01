#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPEN_SPEC>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPEN_SPEC>;
#[doc = "Field `MDIOPEN` reader - MDIO I/O Enable"]
pub type MDIOPEN_R = crate::BitReader;
#[doc = "Field `MDIOPEN` writer - MDIO I/O Enable"]
pub type MDIOPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MIITXERPEN` reader - MII TX ER I/O Enable"]
pub type MIITXERPEN_R = crate::BitReader;
#[doc = "Field `MIITXERPEN` writer - MII TX ER I/O Enable"]
pub type MIITXERPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MIIRXERPEN` reader - MII TX ER I/O Enable"]
pub type MIIRXERPEN_R = crate::BitReader;
#[doc = "Field `MIIRXERPEN` writer - MII TX ER I/O Enable"]
pub type MIIRXERPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MIIPEN` reader - MII I/O Enable"]
pub type MIIPEN_R = crate::BitReader;
#[doc = "Field `MIIPEN` writer - MII I/O Enable"]
pub type MIIPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RMIIPEN` reader - RMII I/O Enable"]
pub type RMIIPEN_R = crate::BitReader;
#[doc = "Field `RMIIPEN` writer - RMII I/O Enable"]
pub type RMIIPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSUTMRTOGPEN` reader - TSU_TMR_CNT_SEC Output Enable"]
pub type TSUTMRTOGPEN_R = crate::BitReader;
#[doc = "Field `TSUTMRTOGPEN` writer - TSU_TMR_CNT_SEC Output Enable"]
pub type TSUTMRTOGPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&self) -> MDIOPEN_R {
        MDIOPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&self) -> MIITXERPEN_R {
        MIITXERPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&self) -> MIIRXERPEN_R {
        MIIRXERPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&self) -> MIIPEN_R {
        MIIPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&self) -> RMIIPEN_R {
        RMIIPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&self) -> TSUTMRTOGPEN_R {
        TSUTMRTOGPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdiopen(&mut self) -> MDIOPEN_W<ROUTEPEN_SPEC, 0> {
        MDIOPEN_W::new(self)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn miitxerpen(&mut self) -> MIITXERPEN_W<ROUTEPEN_SPEC, 1> {
        MIITXERPEN_W::new(self)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn miirxerpen(&mut self) -> MIIRXERPEN_W<ROUTEPEN_SPEC, 2> {
        MIIRXERPEN_W::new(self)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn miipen(&mut self) -> MIIPEN_W<ROUTEPEN_SPEC, 3> {
        MIIPEN_W::new(self)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmiipen(&mut self) -> RMIIPEN_W<ROUTEPEN_SPEC, 4> {
        RMIIPEN_W::new(self)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsutmrtogpen(&mut self) -> TSUTMRTOGPEN_W<ROUTEPEN_SPEC, 5> {
        TSUTMRTOGPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Route Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
