#[doc = "Register `TSUPEERRXNSEC` reader"]
pub type R = crate::R<TsupeerrxnsecSpec>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Nanoseconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeerrxnsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsupeerrxnsecSpec;
impl crate::RegisterSpec for TsupeerrxnsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeerrxnsec::R`](R) reader structure"]
impl crate::Readable for TsupeerrxnsecSpec {}
#[doc = "`reset()` method sets TSUPEERRXNSEC to value 0"]
impl crate::Resettable for TsupeerrxnsecSpec {}
