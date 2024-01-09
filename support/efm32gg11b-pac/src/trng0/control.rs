#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `ENABLE` reader - TRNG Module Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - TRNG Module Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTEN` reader - Test Enable"]
pub type TESTEN_R = crate::BitReader;
#[doc = "Field `TESTEN` writer - Test Enable"]
pub type TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONDBYPASS` reader - Conditioning Bypass"]
pub type CONDBYPASS_R = crate::BitReader;
#[doc = "Field `CONDBYPASS` writer - Conditioning Bypass"]
pub type CONDBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPCOUNTIEN` reader - Interrupt Enable for Repetition Count Test Failure"]
pub type REPCOUNTIEN_R = crate::BitReader;
#[doc = "Field `REPCOUNTIEN` writer - Interrupt Enable for Repetition Count Test Failure"]
pub type REPCOUNTIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT64IEN` reader - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
pub type APT64IEN_R = crate::BitReader;
#[doc = "Field `APT64IEN` writer - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
pub type APT64IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT4096IEN` reader - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
pub type APT4096IEN_R = crate::BitReader;
#[doc = "Field `APT4096IEN` writer - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
pub type APT4096IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLIEN` reader - Interrupt Enable for FIFO Full"]
pub type FULLIEN_R = crate::BitReader;
#[doc = "Field `FULLIEN` writer - Interrupt Enable for FIFO Full"]
pub type FULLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRESET` reader - Software Reset"]
pub type SOFTRESET_R = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - Software Reset"]
pub type SOFTRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIEN` reader - Interrupt enable for AIS31 preliminary noise alarm"]
pub type PREIEN_R = crate::BitReader;
#[doc = "Field `PREIEN` writer - Interrupt enable for AIS31 preliminary noise alarm"]
pub type PREIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIEN` reader - Interrupt enable for AIS31 noise alarm"]
pub type ALMIEN_R = crate::BitReader;
#[doc = "Field `ALMIEN` writer - Interrupt enable for AIS31 noise alarm"]
pub type ALMIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCERUN` reader - Oscillator Force Run"]
pub type FORCERUN_R = crate::BitReader;
#[doc = "Field `FORCERUN` writer - Oscillator Force Run"]
pub type FORCERUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPNIST` reader - NIST Start-up Test Bypass."]
pub type BYPNIST_R = crate::BitReader;
#[doc = "Field `BYPNIST` writer - NIST Start-up Test Bypass."]
pub type BYPNIST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPAIS31` reader - AIS31 Start-up Test Bypass."]
pub type BYPAIS31_R = crate::BitReader;
#[doc = "Field `BYPAIS31` writer - AIS31 Start-up Test Bypass."]
pub type BYPAIS31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&self) -> CONDBYPASS_R {
        CONDBYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    pub fn repcountien(&self) -> REPCOUNTIEN_R {
        REPCOUNTIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    pub fn apt64ien(&self) -> APT64IEN_R {
        APT64IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    pub fn apt4096ien(&self) -> APT4096IEN_R {
        APT4096IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    pub fn fullien(&self) -> FULLIEN_R {
        FULLIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    pub fn preien(&self) -> PREIEN_R {
        PREIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&self) -> FORCERUN_R {
        FORCERUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&self) -> BYPNIST_R {
        BYPNIST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&self) -> BYPAIS31_R {
        BYPAIS31_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONTROL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TESTEN_W<CONTROL_SPEC> {
        TESTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn condbypass(&mut self) -> CONDBYPASS_W<CONTROL_SPEC> {
        CONDBYPASS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    #[must_use]
    pub fn repcountien(&mut self) -> REPCOUNTIEN_W<CONTROL_SPEC> {
        REPCOUNTIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    #[must_use]
    pub fn apt64ien(&mut self) -> APT64IEN_W<CONTROL_SPEC> {
        APT64IEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    #[must_use]
    pub fn apt4096ien(&mut self) -> APT4096IEN_W<CONTROL_SPEC> {
        APT4096IEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn fullien(&mut self) -> FULLIEN_W<CONTROL_SPEC> {
        FULLIEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SOFTRESET_W<CONTROL_SPEC> {
        SOFTRESET_W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    #[must_use]
    pub fn preien(&mut self) -> PREIEN_W<CONTROL_SPEC> {
        PREIEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    #[must_use]
    pub fn almien(&mut self) -> ALMIEN_W<CONTROL_SPEC> {
        ALMIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    #[must_use]
    pub fn forcerun(&mut self) -> FORCERUN_W<CONTROL_SPEC> {
        FORCERUN_W::new(self, 11)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypnist(&mut self) -> BYPNIST_W<CONTROL_SPEC> {
        BYPNIST_W::new(self, 12)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypais31(&mut self) -> BYPAIS31_W<CONTROL_SPEC> {
        BYPAIS31_W::new(self, 13)
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
#[doc = "Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
