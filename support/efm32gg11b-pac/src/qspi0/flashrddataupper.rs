#[doc = "Register `FLASHRDDATAUPPER` reader"]
pub type R = crate::R<FLASHRDDATAUPPER_SPEC>;
#[doc = "Field `DATA` reader - Read Data Upper"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data Upper"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Flash Command Read Data Register (Upper) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashrddataupper::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHRDDATAUPPER_SPEC;
impl crate::RegisterSpec for FLASHRDDATAUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrddataupper::R`](R) reader structure"]
impl crate::Readable for FLASHRDDATAUPPER_SPEC {}
#[doc = "`reset()` method sets FLASHRDDATAUPPER to value 0"]
impl crate::Resettable for FLASHRDDATAUPPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
