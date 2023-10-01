#[doc = "Register `ADMAES` reader"]
pub type R = crate::R<ADMAES_SPEC>;
#[doc = "Field `ADMAES` reader - ADMA Error State"]
pub type ADMAES_R = crate::FieldReader;
#[doc = "Field `ADMALME` reader - ADMA Length Mismatch Error"]
pub type ADMALME_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn admaes(&self) -> ADMAES_R {
        ADMAES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> ADMALME_R {
        ADMALME_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admaes::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMAES_SPEC;
impl crate::RegisterSpec for ADMAES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`admaes::R`](R) reader structure"]
impl crate::Readable for ADMAES_SPEC {}
#[doc = "`reset()` method sets ADMAES to value 0"]
impl crate::Resettable for ADMAES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
