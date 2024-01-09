#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `CSENBUSY` reader - Busy Flag"]
pub type CSENBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy Flag"]
    #[inline(always)]
    pub fn csenbusy(&self) -> CSENBUSY_R {
        CSENBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
