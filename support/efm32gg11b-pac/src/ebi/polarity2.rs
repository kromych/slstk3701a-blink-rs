#[doc = "Register `POLARITY2` reader"]
pub type R = crate::R<POLARITY2_SPEC>;
#[doc = "Register `POLARITY2` writer"]
pub type W = crate::W<POLARITY2_SPEC>;
#[doc = "Field `CSPOL` reader - Chip Select Polarity"]
pub type CSPOL_R = crate::BitReader;
#[doc = "Field `CSPOL` writer - Chip Select Polarity"]
pub type CSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOL` reader - Read Enable Polarity"]
pub type REPOL_R = crate::BitReader;
#[doc = "Field `REPOL` writer - Read Enable Polarity"]
pub type REPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPOL` reader - Write Enable Polarity"]
pub type WEPOL_R = crate::BitReader;
#[doc = "Field `WEPOL` writer - Write Enable Polarity"]
pub type WEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALEPOL` reader - Address Latch Polarity"]
pub type ALEPOL_R = crate::BitReader;
#[doc = "Field `ALEPOL` writer - Address Latch Polarity"]
pub type ALEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYPOL` reader - ARDY Polarity"]
pub type ARDYPOL_R = crate::BitReader;
#[doc = "Field `ARDYPOL` writer - ARDY Polarity"]
pub type ARDYPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLPOL` reader - BL Polarity"]
pub type BLPOL_R = crate::BitReader;
#[doc = "Field `BLPOL` writer - BL Polarity"]
pub type BLPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    pub fn repol(&self) -> REPOL_R {
        REPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    pub fn wepol(&self) -> WEPOL_R {
        WEPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn alepol(&self) -> ALEPOL_R {
        ALEPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    pub fn ardypol(&self) -> ARDYPOL_R {
        ARDYPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    pub fn blpol(&self) -> BLPOL_R {
        BLPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<POLARITY2_SPEC> {
        CSPOL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn repol(&mut self) -> REPOL_W<POLARITY2_SPEC> {
        REPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wepol(&mut self) -> WEPOL_W<POLARITY2_SPEC> {
        WEPOL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn alepol(&mut self) -> ALEPOL_W<POLARITY2_SPEC> {
        ALEPOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ardypol(&mut self) -> ARDYPOL_W<POLARITY2_SPEC> {
        ARDYPOL_W::new(self, 4)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn blpol(&mut self) -> BLPOL_W<POLARITY2_SPEC> {
        BLPOL_W::new(self, 5)
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
#[doc = "Polarity Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polarity2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polarity2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLARITY2_SPEC;
impl crate::RegisterSpec for POLARITY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity2::R`](R) reader structure"]
impl crate::Readable for POLARITY2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`polarity2::W`](W) writer structure"]
impl crate::Writable for POLARITY2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLARITY2 to value 0"]
impl crate::Resettable for POLARITY2_SPEC {
    const RESET_VALUE: u32 = 0;
}
