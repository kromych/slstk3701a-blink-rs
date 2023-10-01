#[doc = "Register `TSUPEERTXNSEC` reader"]
pub type R = crate::R<TSUPEERTXNSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Transmitted Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsupeertxnsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPEERTXNSEC_SPEC;
impl crate::RegisterSpec for TSUPEERTXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsupeertxnsec::R`](R) reader structure"]
impl crate::Readable for TSUPEERTXNSEC_SPEC {}
#[doc = "`reset()` method sets TSUPEERTXNSEC to value 0"]
impl crate::Resettable for TSUPEERTXNSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
