#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `ERASE` writer - Set ERASE Interrupt Flag"]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Set WRITE Interrupt Flag"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Set CHOF Interrupt Flag"]
pub type CHOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Set CMOF Interrupt Flag"]
pub type CMOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Set PWRUPF Interrupt Flag"]
pub type PWRUPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Set ICACHERR Interrupt Flag"]
pub type ICACHERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` writer - Set WDATAOV Interrupt Flag"]
pub type WDATAOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVEWRITE` writer - Set LVEWRITE Interrupt Flag"]
pub type LVEWRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR1B` writer - Set RAMERR1B Interrupt Flag"]
pub type RAMERR1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR2B` writer - Set RAMERR2B Interrupt Flag"]
pub type RAMERR2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR1B` writer - Set RAM1ERR1B Interrupt Flag"]
pub type RAM1ERR1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR2B` writer - Set RAM1ERR2B Interrupt Flag"]
pub type RAM1ERR2B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set ERASE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<IFS_SPEC> {
        ERASE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set WRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<IFS_SPEC> {
        WRITE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set CHOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chof(&mut self) -> CHOF_W<IFS_SPEC> {
        CHOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set CMOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmof(&mut self) -> CMOF_W<IFS_SPEC> {
        CMOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set PWRUPF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<IFS_SPEC> {
        PWRUPF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set ICACHERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icacherr(&mut self) -> ICACHERR_W<IFS_SPEC> {
        ICACHERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set WDATAOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdataov(&mut self) -> WDATAOV_W<IFS_SPEC> {
        WDATAOV_W::new(self, 6)
    }
    #[doc = "Bit 8 - Set LVEWRITE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lvewrite(&mut self) -> LVEWRITE_W<IFS_SPEC> {
        LVEWRITE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Set RAMERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W<IFS_SPEC> {
        RAMERR1B_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set RAMERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W<IFS_SPEC> {
        RAMERR2B_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram1err1b(&mut self) -> RAM1ERR1B_W<IFS_SPEC> {
        RAM1ERR1B_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ram1err2b(&mut self) -> RAM1ERR2B_W<IFS_SPEC> {
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
