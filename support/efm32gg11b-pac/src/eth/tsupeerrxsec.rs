#[doc = "Register `TSUPEERRXSEC` reader"]
pub type R = crate::R<TsupeerrxsecSpec>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Seconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Peer Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeerrxsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsupeerrxsecSpec;
impl crate::RegisterSpec for TsupeerrxsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeerrxsec::R`](R) reader structure"]
impl crate::Readable for TsupeerrxsecSpec {}
#[doc = "`reset()` method sets TSUPEERRXSEC to value 0"]
impl crate::Resettable for TsupeerrxsecSpec {}
