#[doc = "Register `MESSAGEDATA` reader"]
pub type R = crate::R<MessagedataSpec>;
#[doc = "Field `VALID` reader - DATAVALID Bits (of All Message Objects)"]
pub type ValidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DATAVALID Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(self.bits)
    }
}
#[doc = "New Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`messagedata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MessagedataSpec;
impl crate::RegisterSpec for MessagedataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`messagedata::R`](R) reader structure"]
impl crate::Readable for MessagedataSpec {}
#[doc = "`reset()` method sets MESSAGEDATA to value 0"]
impl crate::Resettable for MessagedataSpec {}
