#[doc = "Register `TSUPEERRXNSEC` reader"]
pub type R = crate::R<TSUPEERRXNSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeerrxnsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPEERRXNSEC_SPEC;
impl crate::RegisterSpec for TSUPEERRXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeerrxnsec::R`](R) reader structure"]
impl crate::Readable for TSUPEERRXNSEC_SPEC {}
#[doc = "`reset()` method sets TSUPEERRXNSEC to value 0"]
impl crate::Resettable for TSUPEERRXNSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
