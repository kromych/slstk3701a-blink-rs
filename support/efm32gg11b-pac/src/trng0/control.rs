#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `ENABLE` reader - TRNG Module Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - TRNG Module Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTEN` reader - Test Enable"]
pub type TestenR = crate::BitReader;
#[doc = "Field `TESTEN` writer - Test Enable"]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONDBYPASS` reader - Conditioning Bypass"]
pub type CondbypassR = crate::BitReader;
#[doc = "Field `CONDBYPASS` writer - Conditioning Bypass"]
pub type CondbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPCOUNTIEN` reader - Interrupt Enable for Repetition Count Test Failure"]
pub type RepcountienR = crate::BitReader;
#[doc = "Field `REPCOUNTIEN` writer - Interrupt Enable for Repetition Count Test Failure"]
pub type RepcountienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT64IEN` reader - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
pub type Apt64ienR = crate::BitReader;
#[doc = "Field `APT64IEN` writer - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
pub type Apt64ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT4096IEN` reader - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
pub type Apt4096ienR = crate::BitReader;
#[doc = "Field `APT4096IEN` writer - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
pub type Apt4096ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLIEN` reader - Interrupt Enable for FIFO Full"]
pub type FullienR = crate::BitReader;
#[doc = "Field `FULLIEN` writer - Interrupt Enable for FIFO Full"]
pub type FullienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRESET` reader - Software Reset"]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - Software Reset"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREIEN` reader - Interrupt enable for AIS31 preliminary noise alarm"]
pub type PreienR = crate::BitReader;
#[doc = "Field `PREIEN` writer - Interrupt enable for AIS31 preliminary noise alarm"]
pub type PreienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIEN` reader - Interrupt enable for AIS31 noise alarm"]
pub type AlmienR = crate::BitReader;
#[doc = "Field `ALMIEN` writer - Interrupt enable for AIS31 noise alarm"]
pub type AlmienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCERUN` reader - Oscillator Force Run"]
pub type ForcerunR = crate::BitReader;
#[doc = "Field `FORCERUN` writer - Oscillator Force Run"]
pub type ForcerunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPNIST` reader - NIST Start-up Test Bypass."]
pub type BypnistR = crate::BitReader;
#[doc = "Field `BYPNIST` writer - NIST Start-up Test Bypass."]
pub type BypnistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPAIS31` reader - AIS31 Start-up Test Bypass."]
pub type Bypais31R = crate::BitReader;
#[doc = "Field `BYPAIS31` writer - AIS31 Start-up Test Bypass."]
pub type Bypais31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&self) -> CondbypassR {
        CondbypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    pub fn repcountien(&self) -> RepcountienR {
        RepcountienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    pub fn apt64ien(&self) -> Apt64ienR {
        Apt64ienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    pub fn apt4096ien(&self) -> Apt4096ienR {
        Apt4096ienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    pub fn fullien(&self) -> FullienR {
        FullienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    pub fn preien(&self) -> PreienR {
        PreienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&self) -> AlmienR {
        AlmienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&self) -> ForcerunR {
        ForcerunR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&self) -> BypnistR {
        BypnistR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&self) -> Bypais31R {
        Bypais31R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&mut self) -> TestenW<'_, ControlSpec> {
        TestenW::new(self, 2)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&mut self) -> CondbypassW<'_, ControlSpec> {
        CondbypassW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    pub fn repcountien(&mut self) -> RepcountienW<'_, ControlSpec> {
        RepcountienW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    pub fn apt64ien(&mut self) -> Apt64ienW<'_, ControlSpec> {
        Apt64ienW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    pub fn apt4096ien(&mut self) -> Apt4096ienW<'_, ControlSpec> {
        Apt4096ienW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    pub fn fullien(&mut self) -> FullienW<'_, ControlSpec> {
        FullienW::new(self, 7)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&mut self) -> SoftresetW<'_, ControlSpec> {
        SoftresetW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    pub fn preien(&mut self) -> PreienW<'_, ControlSpec> {
        PreienW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&mut self) -> AlmienW<'_, ControlSpec> {
        AlmienW::new(self, 10)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&mut self) -> ForcerunW<'_, ControlSpec> {
        ForcerunW::new(self, 11)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&mut self) -> BypnistW<'_, ControlSpec> {
        BypnistW::new(self, 12)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&mut self) -> Bypais31W<'_, ControlSpec> {
        Bypais31W::new(self, 13)
    }
}
#[doc = "Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
