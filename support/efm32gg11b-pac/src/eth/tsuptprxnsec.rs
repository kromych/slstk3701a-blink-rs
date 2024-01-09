#[doc = "Register `TSUPTPRXNSEC` reader"]
pub type R = crate::R<TSUPTPRXNSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Event Frame Received Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptprxnsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPTPRXNSEC_SPEC;
impl crate::RegisterSpec for TSUPTPRXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptprxnsec::R`](R) reader structure"]
impl crate::Readable for TSUPTPRXNSEC_SPEC {}
#[doc = "`reset()` method sets TSUPTPRXNSEC to value 0"]
impl crate::Resettable for TSUPTPRXNSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
