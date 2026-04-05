#[doc = "Register `TSUPEERTXMSBSEC` reader"]
pub type R = crate::R<TsupeertxmsbsecSpec>;
#[doc = "Field `TIMERSEC` reader - PTP Peer Event Frame TX Seconds"]
pub type TimersecR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Peer Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TimersecR {
        TimersecR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsupeertxmsbsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsupeertxmsbsecSpec;
impl crate::RegisterSpec for TsupeertxmsbsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeertxmsbsec::R`](R) reader structure"]
impl crate::Readable for TsupeertxmsbsecSpec {}
#[doc = "`reset()` method sets TSUPEERTXMSBSEC to value 0"]
impl crate::Resettable for TsupeertxmsbsecSpec {}
