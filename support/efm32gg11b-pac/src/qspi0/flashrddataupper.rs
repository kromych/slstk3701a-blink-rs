#[doc = "Register `FLASHRDDATAUPPER` reader"]
pub type R = crate::R<FlashrddataupperSpec>;
#[doc = "Field `DATA` reader - Read Data Upper"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data Upper"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Flash Command Read Data Register (Upper) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrddataupper::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashrddataupperSpec;
impl crate::RegisterSpec for FlashrddataupperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrddataupper::R`](R) reader structure"]
impl crate::Readable for FlashrddataupperSpec {}
#[doc = "`reset()` method sets FLASHRDDATAUPPER to value 0"]
impl crate::Resettable for FlashrddataupperSpec {}
