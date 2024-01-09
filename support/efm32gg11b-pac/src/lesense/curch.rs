#[doc = "Register `CURCH` reader"]
pub type R = crate::R<CURCH_SPEC>;
#[doc = "Field `CURCH` reader - Current Channel Index"]
pub type CURCH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Current Channel Index"]
    #[inline(always)]
    pub fn curch(&self) -> CURCH_R {
        CURCH_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Current Channel Index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURCH_SPEC;
impl crate::RegisterSpec for CURCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curch::R`](R) reader structure"]
impl crate::Readable for CURCH_SPEC {}
#[doc = "`reset()` method sets CURCH to value 0"]
impl crate::Resettable for CURCH_SPEC {
    const RESET_VALUE: u32 = 0;
}
