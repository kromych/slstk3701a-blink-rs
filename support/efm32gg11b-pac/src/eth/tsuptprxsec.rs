#[doc = "Register `TSUPTPRXSEC` reader"]
pub type R = crate::R<TSUPTPRXSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Event Frame Received Seconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptprxsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPTPRXSEC_SPEC;
impl crate::RegisterSpec for TSUPTPRXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptprxsec::R`](R) reader structure"]
impl crate::Readable for TSUPTPRXSEC_SPEC {}
#[doc = "`reset()` method sets TSUPTPRXSEC to value 0"]
impl crate::Resettable for TSUPTPRXSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
