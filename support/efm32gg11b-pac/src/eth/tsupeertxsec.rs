#[doc = "Register `TSUPEERTXSEC` reader"]
pub type R = crate::R<TSUPEERTXSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Seconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Peer Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeertxsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPEERTXSEC_SPEC;
impl crate::RegisterSpec for TSUPEERTXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeertxsec::R`](R) reader structure"]
impl crate::Readable for TSUPEERTXSEC_SPEC {}
#[doc = "`reset()` method sets TSUPEERTXSEC to value 0"]
impl crate::Resettable for TSUPEERTXSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
