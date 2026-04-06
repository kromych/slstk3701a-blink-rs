#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `TESTDATABUSY` reader - Test Data Busy"]
pub type TestdatabusyR = crate::BitReader;
#[doc = "Field `REPCOUNTIF` reader - Repetition Count Test Interrupt Status"]
pub type RepcountifR = crate::BitReader;
#[doc = "Field `APT64IF` reader - Adaptive Proportion test failure (64-sample window) interrupt status"]
pub type Apt64ifR = crate::BitReader;
#[doc = "Field `APT4096IF` reader - Adaptive Proportion test failure (4096-sample window) interrupt status"]
pub type Apt4096ifR = crate::BitReader;
#[doc = "Field `FULLIF` reader - FIFO Full Interrupt Status"]
pub type FullifR = crate::BitReader;
#[doc = "Field `PREIF` reader - AIS31 Preliminary Noise Alarm interrupt status"]
pub type PreifR = crate::BitReader;
#[doc = "Field `PREIF` writer - AIS31 Preliminary Noise Alarm interrupt status"]
pub type PreifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIF` reader - AIS31 Noise Alarm interrupt status"]
pub type AlmifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Test Data Busy"]
    #[inline(always)]
    pub fn testdatabusy(&self) -> TestdatabusyR {
        TestdatabusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Repetition Count Test Interrupt Status"]
    #[inline(always)]
    pub fn repcountif(&self) -> RepcountifR {
        RepcountifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Adaptive Proportion test failure (64-sample window) interrupt status"]
    #[inline(always)]
    pub fn apt64if(&self) -> Apt64ifR {
        Apt64ifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Adaptive Proportion test failure (4096-sample window) interrupt status"]
    #[inline(always)]
    pub fn apt4096if(&self) -> Apt4096ifR {
        Apt4096ifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn fullif(&self) -> FullifR {
        FullifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn preif(&self) -> PreifR {
        PreifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AIS31 Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn almif(&self) -> AlmifR {
        AlmifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn preif(&mut self) -> PreifW<'_, StatusSpec> {
        PreifW::new(self, 8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
