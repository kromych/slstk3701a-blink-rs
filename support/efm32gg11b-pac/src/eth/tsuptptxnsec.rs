#[doc = "Register `TSUPTPTXNSEC` reader"]
pub type R = crate::R<TsuptptxnsecSpec>;
#[doc = "Field `TIMER` reader - PTP Event Frame Transmitted Nanoseconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptptxnsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuptptxnsecSpec;
impl crate::RegisterSpec for TsuptptxnsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptptxnsec::R`](R) reader structure"]
impl crate::Readable for TsuptptxnsecSpec {}
#[doc = "`reset()` method sets TSUPTPTXNSEC to value 0"]
impl crate::Resettable for TsuptptxnsecSpec {}
