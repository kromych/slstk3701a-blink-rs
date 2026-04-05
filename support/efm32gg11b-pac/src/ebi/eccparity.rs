#[doc = "Register `ECCPARITY` reader"]
pub type R = crate::R<EccparitySpec>;
#[doc = "Field `ECCPARITY` reader - ECC Parity Data"]
pub type EccparityR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Parity Data"]
    #[inline(always)]
    pub fn eccparity(&self) -> EccparityR {
        EccparityR::new(self.bits)
    }
}
#[doc = "ECC Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccparity::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccparitySpec;
impl crate::RegisterSpec for EccparitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccparity::R`](R) reader structure"]
impl crate::Readable for EccparitySpec {}
#[doc = "`reset()` method sets ECCPARITY to value 0"]
impl crate::Resettable for EccparitySpec {}
