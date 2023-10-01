#[doc = "Register `TSUPEERRXSEC` reader"]
pub type R = crate::R<TSUPEERRXSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Seconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Peer Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeerrxsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPEERRXSEC_SPEC;
impl crate::RegisterSpec for TSUPEERRXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeerrxsec::R`](R) reader structure"]
impl crate::Readable for TSUPEERRXSEC_SPEC {}
#[doc = "`reset()` method sets TSUPEERRXSEC to value 0"]
impl crate::Resettable for TSUPEERRXSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
