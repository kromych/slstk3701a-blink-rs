#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `BUMODETS` reader - Timestamp for Backup Mode Entry Stored"]
pub type BUMODETS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timestamp for Backup Mode Entry Stored"]
    #[inline(always)]
    pub fn bumodets(&self) -> BUMODETS_R {
        BUMODETS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
