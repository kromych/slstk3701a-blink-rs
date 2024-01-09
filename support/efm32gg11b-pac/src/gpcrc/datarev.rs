#[doc = "Register `DATAREV` reader"]
pub type R = crate::R<DATAREV_SPEC>;
#[doc = "Field `DATAREV` reader - Data Reverse Value"]
pub type DATAREV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Reverse Value"]
    #[inline(always)]
    pub fn datarev(&self) -> DATAREV_R {
        DATAREV_R::new(self.bits)
    }
}
#[doc = "CRC Data Reverse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datarev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAREV_SPEC;
impl crate::RegisterSpec for DATAREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datarev::R`](R) reader structure"]
impl crate::Readable for DATAREV_SPEC {}
#[doc = "`reset()` method sets DATAREV to value 0"]
impl crate::Resettable for DATAREV_SPEC {
    const RESET_VALUE: u32 = 0;
}
