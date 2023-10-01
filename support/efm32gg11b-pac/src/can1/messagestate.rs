#[doc = "Register `MESSAGESTATE` reader"]
pub type R = crate::R<MESSAGESTATE_SPEC>;
#[doc = "Field `VALID` reader - Message Valid Bits (of All Message Objects)"]
pub type VALID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Message Valid Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(self.bits)
    }
}
#[doc = "Message Valid Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`messagestate::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESSAGESTATE_SPEC;
impl crate::RegisterSpec for MESSAGESTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`messagestate::R`](R) reader structure"]
impl crate::Readable for MESSAGESTATE_SPEC {}
#[doc = "`reset()` method sets MESSAGESTATE to value 0"]
impl crate::Resettable for MESSAGESTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
