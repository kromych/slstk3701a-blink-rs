#[doc = "Register `TSUPTPRXSEC` reader"]
pub type R = crate::R<TsuptprxsecSpec>;
#[doc = "Field `TIMER` reader - PTP Event Frame Received Seconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
#[doc = "PTP Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptprxsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuptprxsecSpec;
impl crate::RegisterSpec for TsuptprxsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptprxsec::R`](R) reader structure"]
impl crate::Readable for TsuptprxsecSpec {}
#[doc = "`reset()` method sets TSUPTPRXSEC to value 0"]
impl crate::Resettable for TsuptprxsecSpec {}
