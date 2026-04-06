#[doc = "Register `TSUPTPTXSEC` reader"]
pub type R = crate::R<TsuptptxsecSpec>;
#[doc = "Field `TIMER` reader - PTP Event Frame Transmitted Seconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Transmitted Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptptxsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuptptxsecSpec;
impl crate::RegisterSpec for TsuptptxsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptptxsec::R`](R) reader structure"]
impl crate::Readable for TsuptptxsecSpec {}
#[doc = "`reset()` method sets TSUPTPTXSEC to value 0"]
impl crate::Resettable for TsuptptxsecSpec {}
