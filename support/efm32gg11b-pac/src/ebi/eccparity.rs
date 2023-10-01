#[doc = "Register `ECCPARITY` reader"]
pub type R = crate::R<ECCPARITY_SPEC>;
#[doc = "Field `ECCPARITY` reader - ECC Parity Data"]
pub type ECCPARITY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Parity Data"]
    #[inline(always)]
    pub fn eccparity(&self) -> ECCPARITY_R {
        ECCPARITY_R::new(self.bits)
    }
}
#[doc = "ECC Parity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccparity::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCPARITY_SPEC;
impl crate::RegisterSpec for ECCPARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccparity::R`](R) reader structure"]
impl crate::Readable for ECCPARITY_SPEC {}
#[doc = "`reset()` method sets ECCPARITY to value 0"]
impl crate::Resettable for ECCPARITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
