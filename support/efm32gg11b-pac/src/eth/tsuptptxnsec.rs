#[doc = "Register `TSUPTPTXNSEC` reader"]
pub type R = crate::R<TSUPTPTXNSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Event Frame Transmitted Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptptxnsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPTPTXNSEC_SPEC;
impl crate::RegisterSpec for TSUPTPTXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptptxnsec::R`](R) reader structure"]
impl crate::Readable for TSUPTPTXNSEC_SPEC {}
#[doc = "`reset()` method sets TSUPTPTXNSEC to value 0"]
impl crate::Resettable for TSUPTPTXNSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
