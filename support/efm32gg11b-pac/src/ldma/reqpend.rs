#[doc = "Register `REQPEND` reader"]
pub type R = crate::R<REQPEND_SPEC>;
#[doc = "Field `REQPEND` reader - DMA Requests Pending"]
pub type REQPEND_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> REQPEND_R {
        REQPEND_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "DMA Channel Requests Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqpend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQPEND_SPEC;
impl crate::RegisterSpec for REQPEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqpend::R`](R) reader structure"]
impl crate::Readable for REQPEND_SPEC {}
#[doc = "`reset()` method sets REQPEND to value 0"]
impl crate::Resettable for REQPEND_SPEC {
    const RESET_VALUE: u32 = 0;
}
