#[doc = "Register `FLASHRDDATALOWER` reader"]
pub type R = crate::R<FLASHRDDATALOWER_SPEC>;
#[doc = "Field `DATA` reader - Read Data Lower"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data Lower"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Flash Command Read Data Register (Lower) (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashrddatalower::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHRDDATALOWER_SPEC;
impl crate::RegisterSpec for FLASHRDDATALOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrddatalower::R`](R) reader structure"]
impl crate::Readable for FLASHRDDATALOWER_SPEC {}
#[doc = "`reset()` method sets FLASHRDDATALOWER to value 0"]
impl crate::Resettable for FLASHRDDATALOWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
