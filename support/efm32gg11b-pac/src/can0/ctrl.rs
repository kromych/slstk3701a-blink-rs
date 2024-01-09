#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `INIT` reader - Initialize"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialize"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Module Interrupt Enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Module Interrupt Enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIE` reader - Status Change Interrupt Enable"]
pub type SIE_R = crate::BitReader;
#[doc = "Field `SIE` writer - Status Change Interrupt Enable"]
pub type SIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error Interrupt Enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error Interrupt Enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DAR_R = crate::BitReader;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test Mode Enable Write"]
pub type TEST_R = crate::BitReader;
#[doc = "Field `TEST` writer - Test Mode Enable Write"]
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn sie(&self) -> SIE_R {
        SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable Write"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<CTRL_SPEC> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CTRL_SPEC> {
        IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sie(&mut self) -> SIE_W<CTRL_SPEC> {
        SIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<CTRL_SPEC> {
        EIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Disable Automatic Retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<CTRL_SPEC> {
        DAR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<CTRL_SPEC> {
        CCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable Write"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<CTRL_SPEC> {
        TEST_W::new(self, 7)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
