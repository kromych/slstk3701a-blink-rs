#[doc = "Register `FIFOLEVEL` reader"]
pub type R = crate::R<FIFOLEVEL_SPEC>;
#[doc = "Field `VALUE` reader - FIFO Level"]
pub type VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Level"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "FIFO Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifolevel::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOLEVEL_SPEC;
impl crate::RegisterSpec for FIFOLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolevel::R`](R) reader structure"]
impl crate::Readable for FIFOLEVEL_SPEC {}
#[doc = "`reset()` method sets FIFOLEVEL to value 0"]
impl crate::Resettable for FIFOLEVEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
