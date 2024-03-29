#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `SINGLE` reader - SINGLE Interrupt Enable"]
pub type SINGLE_R = crate::BitReader;
#[doc = "Field `SINGLE` writer - SINGLE Interrupt Enable"]
pub type SINGLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` reader - SCAN Interrupt Enable"]
pub type SCAN_R = crate::BitReader;
#[doc = "Field `SCAN` writer - SCAN Interrupt Enable"]
pub type SCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` reader - SINGLEOF Interrupt Enable"]
pub type SINGLEOF_R = crate::BitReader;
#[doc = "Field `SINGLEOF` writer - SINGLEOF Interrupt Enable"]
pub type SINGLEOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` reader - SCANOF Interrupt Enable"]
pub type SCANOF_R = crate::BitReader;
#[doc = "Field `SCANOF` writer - SCANOF Interrupt Enable"]
pub type SCANOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` reader - SINGLEUF Interrupt Enable"]
pub type SINGLEUF_R = crate::BitReader;
#[doc = "Field `SINGLEUF` writer - SINGLEUF Interrupt Enable"]
pub type SINGLEUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` reader - SCANUF Interrupt Enable"]
pub type SCANUF_R = crate::BitReader;
#[doc = "Field `SCANUF` writer - SCANUF Interrupt Enable"]
pub type SCANUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` reader - SINGLECMP Interrupt Enable"]
pub type SINGLECMP_R = crate::BitReader;
#[doc = "Field `SINGLECMP` writer - SINGLECMP Interrupt Enable"]
pub type SINGLECMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` reader - SCANCMP Interrupt Enable"]
pub type SCANCMP_R = crate::BitReader;
#[doc = "Field `SCANCMP` writer - SCANCMP Interrupt Enable"]
pub type SCANCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` reader - VREFOV Interrupt Enable"]
pub type VREFOV_R = crate::BitReader;
#[doc = "Field `VREFOV` writer - VREFOV Interrupt Enable"]
pub type VREFOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` reader - PROGERR Interrupt Enable"]
pub type PROGERR_R = crate::BitReader;
#[doc = "Field `PROGERR` writer - PROGERR Interrupt Enable"]
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANEXTPEND` reader - SCANEXTPEND Interrupt Enable"]
pub type SCANEXTPEND_R = crate::BitReader;
#[doc = "Field `SCANEXTPEND` writer - SCANEXTPEND Interrupt Enable"]
pub type SCANEXTPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANPEND` reader - SCANPEND Interrupt Enable"]
pub type SCANPEND_R = crate::BitReader;
#[doc = "Field `SCANPEND` writer - SCANPEND Interrupt Enable"]
pub type SCANPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTIMEDERR` reader - PRSTIMEDERR Interrupt Enable"]
pub type PRSTIMEDERR_R = crate::BitReader;
#[doc = "Field `PRSTIMEDERR` writer - PRSTIMEDERR Interrupt Enable"]
pub type PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` reader - EM23ERR Interrupt Enable"]
pub type EM23ERR_R = crate::BitReader;
#[doc = "Field `EM23ERR` writer - EM23ERR Interrupt Enable"]
pub type EM23ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&self) -> SINGLEUF_R {
        SINGLEUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&self) -> SCANUF_R {
        SCANUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&self) -> VREFOV_R {
        VREFOV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&self) -> SCANEXTPEND_R {
        SCANEXTPEND_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&self) -> SCANPEND_R {
        SCANPEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PRSTIMEDERR_R {
        PRSTIMEDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<IEN_SPEC> {
        SINGLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<IEN_SPEC> {
        SCAN_W::new(self, 1)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SINGLEOF_W<IEN_SPEC> {
        SINGLEOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> SCANOF_W<IEN_SPEC> {
        SCANOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SINGLEUF_W<IEN_SPEC> {
        SINGLEUF_W::new(self, 10)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> SCANUF_W<IEN_SPEC> {
        SCANUF_W::new(self, 11)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SINGLECMP_W<IEN_SPEC> {
        SINGLECMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> SCANCMP_W<IEN_SPEC> {
        SCANCMP_W::new(self, 17)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VREFOV_W<IEN_SPEC> {
        VREFOV_W::new(self, 24)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<IEN_SPEC> {
        PROGERR_W::new(self, 25)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W<IEN_SPEC> {
        SCANEXTPEND_W::new(self, 26)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scanpend(&mut self) -> SCANPEND_W<IEN_SPEC> {
        SCANPEND_W::new(self, 27)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W<IEN_SPEC> {
        PRSTIMEDERR_W::new(self, 28)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<IEN_SPEC> {
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
