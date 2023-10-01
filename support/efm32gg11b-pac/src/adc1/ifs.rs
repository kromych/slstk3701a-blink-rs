#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `SINGLEOF` writer - Set SINGLEOF Interrupt Flag"]
pub type SINGLEOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANOF` writer - Set SCANOF Interrupt Flag"]
pub type SCANOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SINGLEUF` writer - Set SINGLEUF Interrupt Flag"]
pub type SINGLEUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANUF` writer - Set SCANUF Interrupt Flag"]
pub type SCANUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SINGLECMP` writer - Set SINGLECMP Interrupt Flag"]
pub type SINGLECMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANCMP` writer - Set SCANCMP Interrupt Flag"]
pub type SCANCMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFOV` writer - Set VREFOV Interrupt Flag"]
pub type VREFOV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROGERR` writer - Set PROGERR Interrupt Flag"]
pub type PROGERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANEXTPEND` writer - Set SCANEXTPEND Interrupt Flag"]
pub type SCANEXTPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCANPEND` writer - Set SCANPEND Interrupt Flag"]
pub type SCANPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSTIMEDERR` writer - Set PRSTIMEDERR Interrupt Flag"]
pub type PRSTIMEDERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM23ERR` writer - Set EM23ERR Interrupt Flag"]
pub type EM23ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 8 - Set SINGLEOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IFS_SPEC, 8> {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - Set SCANOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IFS_SPEC, 9> {
        SCANOF_W::new(self)
    }
    #[doc = "Bit 10 - Set SINGLEUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SINGLEUF_W<IFS_SPEC, 10> {
        SINGLEUF_W::new(self)
    }
    #[doc = "Bit 11 - Set SCANUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> SCANUF_W<IFS_SPEC, 11> {
        SCANUF_W::new(self)
    }
    #[doc = "Bit 16 - Set SINGLECMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<IFS_SPEC, 16> {
        SINGLECMP_W::new(self)
    }
    #[doc = "Bit 17 - Set SCANCMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<IFS_SPEC, 17> {
        SCANCMP_W::new(self)
    }
    #[doc = "Bit 24 - Set VREFOV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VREFOV_W<IFS_SPEC, 24> {
        VREFOV_W::new(self)
    }
    #[doc = "Bit 25 - Set PROGERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<IFS_SPEC, 25> {
        PROGERR_W::new(self)
    }
    #[doc = "Bit 26 - Set SCANEXTPEND Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W<IFS_SPEC, 26> {
        SCANEXTPEND_W::new(self)
    }
    #[doc = "Bit 27 - Set SCANPEND Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scanpend(&mut self) -> SCANPEND_W<IFS_SPEC, 27> {
        SCANPEND_W::new(self)
    }
    #[doc = "Bit 28 - Set PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W<IFS_SPEC, 28> {
        PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 29 - Set EM23ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<IFS_SPEC, 29> {
        EM23ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
