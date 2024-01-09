#[doc = "Register `TSUPTPTXMSBSEC` reader"]
pub type R = crate::R<TSUPTPTXMSBSEC_SPEC>;
#[doc = "Field `TIMERSEC` reader - PTP Event Frame TX Seconds"]
pub type TIMERSEC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TIMERSEC_R {
        TIMERSEC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptptxmsbsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPTPTXMSBSEC_SPEC;
impl crate::RegisterSpec for TSUPTPTXMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptptxmsbsec::R`](R) reader structure"]
impl crate::Readable for TSUPTPTXMSBSEC_SPEC {}
#[doc = "`reset()` method sets TSUPTPTXMSBSEC to value 0"]
impl crate::Resettable for TSUPTPTXMSBSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
