#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `SINGLEOF` writer - Clear SINGLEOF Interrupt Flag"]
pub type SINGLEOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Clear SCANOF Interrupt Flag"]
pub type SCANOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` writer - Clear SINGLEUF Interrupt Flag"]
pub type SINGLEUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` writer - Clear SCANUF Interrupt Flag"]
pub type SCANUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` writer - Clear SINGLECMP Interrupt Flag"]
pub type SINGLECMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` writer - Clear SCANCMP Interrupt Flag"]
pub type SCANCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` writer - Clear VREFOV Interrupt Flag"]
pub type VREFOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` writer - Clear PROGERR Interrupt Flag"]
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANEXTPEND` writer - Clear SCANEXTPEND Interrupt Flag"]
pub type SCANEXTPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANPEND` writer - Clear SCANPEND Interrupt Flag"]
pub type SCANPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTIMEDERR` writer - Clear PRSTIMEDERR Interrupt Flag"]
pub type PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` writer - Clear EM23ERR Interrupt Flag"]
pub type EM23ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Clear SINGLEOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IFC_SPEC> {
        SINGLEOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear SCANOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IFC_SPEC> {
        SCANOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear SINGLEUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SINGLEUF_W<IFC_SPEC> {
        SINGLEUF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear SCANUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> SCANUF_W<IFC_SPEC> {
        SCANUF_W::new(self, 11)
    }
    #[doc = "Bit 16 - Clear SINGLECMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<IFC_SPEC> {
        SINGLECMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear SCANCMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<IFC_SPEC> {
        SCANCMP_W::new(self, 17)
    }
    #[doc = "Bit 24 - Clear VREFOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VREFOV_W<IFC_SPEC> {
        VREFOV_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear PROGERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<IFC_SPEC> {
        PROGERR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear SCANEXTPEND Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W<IFC_SPEC> {
        SCANEXTPEND_W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear SCANPEND Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanpend(&mut self) -> SCANPEND_W<IFC_SPEC> {
        SCANPEND_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W<IFC_SPEC> {
        PRSTIMEDERR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear EM23ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<IFC_SPEC> {
        EM23ERR_W::new(self, 29)
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
