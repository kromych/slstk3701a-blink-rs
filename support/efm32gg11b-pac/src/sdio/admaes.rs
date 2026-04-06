#[doc = "Register `ADMAES` reader"]
pub type R = crate::R<AdmaesSpec>;
#[doc = "Field `ADMAES` reader - ADMA Error State"]
pub type AdmaesR = crate::FieldReader;
#[doc = "Field `ADMALME` reader - ADMA Length Mismatch Error"]
pub type AdmalmeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn admaes(&self) -> AdmaesR {
        AdmaesR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> AdmalmeR {
        AdmalmeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`admaes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaesSpec;
impl crate::RegisterSpec for AdmaesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`admaes::R`](R) reader structure"]
impl crate::Readable for AdmaesSpec {}
#[doc = "`reset()` method sets ADMAES to value 0"]
impl crate::Resettable for AdmaesSpec {}
