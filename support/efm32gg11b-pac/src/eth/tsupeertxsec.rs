#[doc = "Register `TSUPEERTXSEC` reader"]
pub type R = crate::R<TsupeertxsecSpec>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Seconds"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Peer Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeertxsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsupeertxsecSpec;
impl crate::RegisterSpec for TsupeertxsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeertxsec::R`](R) reader structure"]
impl crate::Readable for TsupeertxsecSpec {}
#[doc = "`reset()` method sets TSUPEERTXSEC to value 0"]
impl crate::Resettable for TsupeertxsecSpec {}
