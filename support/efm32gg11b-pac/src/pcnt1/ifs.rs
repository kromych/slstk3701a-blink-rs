#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `UF` writer - Set UF Interrupt Flag"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` writer - Set DIRCNG Interrupt Flag"]
pub type DIRCNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` writer - Set AUXOF Interrupt Flag"]
pub type AUXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Set TCC Interrupt Flag"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OQSTERR` writer - Set OQSTERR Interrupt Flag"]
pub type OQSTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IFS_SPEC> {
        UF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFS_SPEC> {
        OF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set DIRCNG Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DIRCNG_W<IFS_SPEC> {
        DIRCNG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set AUXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AUXOF_W<IFS_SPEC> {
        AUXOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set TCC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<IFS_SPEC> {
        TCC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set OQSTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OQSTERR_W<IFS_SPEC> {
        OQSTERR_W::new(self, 5)
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
