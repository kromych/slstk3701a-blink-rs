#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `ERASE` writer - Clear ERASE Interrupt Flag"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Clear WRITE Interrupt Flag"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Clear CHOF Interrupt Flag"]
pub type CHOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Clear CMOF Interrupt Flag"]
pub type CMOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Clear PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Clear ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` writer - Clear WDATAOV Interrupt Flag"]
pub type WDATAOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVEWRITE` writer - Clear LVEWRITE Interrupt Flag"]
pub type LVEWRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR1B` writer - Clear RAMERR1B Interrupt Flag"]
pub type RAMERR1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR2B` writer - Clear RAMERR2B Interrupt Flag"]
pub type RAMERR2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR1B` writer - Clear RAM1ERR1B Interrupt Flag"]
pub type RAM1ERR1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR2B` writer - Clear RAM1ERR2B Interrupt Flag"]
pub type RAM1ERR2B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<IFC_SPEC> {
        ERASE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<IFC_SPEC> {
        WRITE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> CHOF_W<IFC_SPEC> {
        CHOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CMOF_W<IFC_SPEC> {
        CMOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<IFC_SPEC> {
        PWRUPF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> ICACHERR_W<IFC_SPEC> {
        ICACHERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear WDATAOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdataov(&mut self) -> WDATAOV_W<IFC_SPEC> {
        WDATAOV_W::new(self, 6)
    }
    #[doc = "Bit 8 - Clear LVEWRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lvewrite(&mut self) -> LVEWRITE_W<IFC_SPEC> {
        LVEWRITE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Clear RAMERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W<IFC_SPEC> {
        RAMERR1B_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear RAMERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W<IFC_SPEC> {
        RAMERR2B_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram1err1b(&mut self) -> RAM1ERR1B_W<IFC_SPEC> {
        RAM1ERR1B_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram1err2b(&mut self) -> RAM1ERR2B_W<IFC_SPEC> {
        RAM1ERR2B_W::new(self, 19)
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
