#[doc = "Register `TSUPTPTXSEC` reader"]
pub type R = crate::R<TSUPTPTXSEC_SPEC>;
#[doc = "Field `TIMER` reader - PTP Event Frame Transmitted Seconds"]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Transmitted Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsuptptxsec::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUPTPTXSEC_SPEC;
impl crate::RegisterSpec for TSUPTPTXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsuptptxsec::R`](R) reader structure"]
impl crate::Readable for TSUPTPTXSEC_SPEC {}
#[doc = "`reset()` method sets TSUPTPTXSEC to value 0"]
impl crate::Resettable for TSUPTPTXSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
