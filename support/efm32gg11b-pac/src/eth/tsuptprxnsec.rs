#[doc = "Register `TSUPTPRXNSEC` reader"]
pub type R = crate::R<TsuptprxnsecSpec>;
#[doc = "Field `TIMER` reader - PTP Event Frame Received Nanoseconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptprxnsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuptprxnsecSpec;
impl crate::RegisterSpec for TsuptprxnsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptprxnsec::R`](R) reader structure"]
impl crate::Readable for TsuptprxnsecSpec {}
#[doc = "`reset()` method sets TSUPTPRXNSEC to value 0"]
impl crate::Resettable for TsuptprxnsecSpec {}
