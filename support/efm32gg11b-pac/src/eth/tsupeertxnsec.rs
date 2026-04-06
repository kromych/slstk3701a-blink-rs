#[doc = "Register `TSUPEERTXNSEC` reader"]
pub type R = crate::R<TsupeertxnsecSpec>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Transmitted Nanoseconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeertxnsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsupeertxnsecSpec;
impl crate::RegisterSpec for TsupeertxnsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeertxnsec::R`](R) reader structure"]
impl crate::Readable for TsupeertxnsecSpec {}
#[doc = "`reset()` method sets TSUPEERTXNSEC to value 0"]
impl crate::Resettable for TsupeertxnsecSpec {}
