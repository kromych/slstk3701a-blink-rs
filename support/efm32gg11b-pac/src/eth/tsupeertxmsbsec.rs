#[doc = "Register `TSUPEERTXMSBSEC` reader"]
pub type R = crate::R<TSUPEERTXMSBSEC_SPEC>;
#[doc = "Field `TIMERSEC` reader - PTP Peer Event Frame TX Seconds"]
pub type TIMERSEC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Peer Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TIMERSEC_R {
        TIMERSEC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeertxmsbsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPEERTXMSBSEC_SPEC;
impl crate::RegisterSpec for TSUPEERTXMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeertxmsbsec::R`](R) reader structure"]
impl crate::Readable for TSUPEERTXMSBSEC_SPEC {}
#[doc = "`reset()` method sets TSUPEERTXMSBSEC to value 0"]
impl crate::Resettable for TSUPEERTXMSBSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
