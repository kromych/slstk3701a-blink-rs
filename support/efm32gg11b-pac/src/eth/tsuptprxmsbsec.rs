#[doc = "Register `TSUPTPRXMSBSEC` reader"]
pub type R = crate::R<TsuptprxmsbsecSpec>;
#[doc = "Field `TIMERSEC` reader - PTP Event Frame TX Seconds"]
pub type TimersecR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TimersecR {
        TimersecR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Received Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsuptprxmsbsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsuptprxmsbsecSpec;
impl crate::RegisterSpec for TsuptprxmsbsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptprxmsbsec::R`](R) reader structure"]
impl crate::Readable for TsuptprxmsbsecSpec {}
#[doc = "`reset()` method sets TSUPTPRXMSBSEC to value 0"]
impl crate::Resettable for TsuptprxmsbsecSpec {}
