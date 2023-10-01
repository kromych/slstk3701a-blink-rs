#[doc = "Register `MESSAGEDATA` reader"]
pub type R = crate::R<MESSAGEDATA_SPEC>;
#[doc = "Field `VALID` reader - DATAVALID Bits (of All Message Objects)"]
pub type VALID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DATAVALID Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(self.bits)
    }
}
#[doc = "New Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`messagedata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESSAGEDATA_SPEC;
impl crate::RegisterSpec for MESSAGEDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`messagedata::R`](R) reader structure"]
impl crate::Readable for MESSAGEDATA_SPEC {}
#[doc = "`reset()` method sets MESSAGEDATA to value 0"]
impl crate::Resettable for MESSAGEDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
