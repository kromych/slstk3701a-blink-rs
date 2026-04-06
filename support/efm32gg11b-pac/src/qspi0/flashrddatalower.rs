#[doc = "Register `FLASHRDDATALOWER` reader"]
pub type R = crate::R<FlashrddatalowerSpec>;
#[doc = "Field `DATA` reader - Read Data Lower"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data Lower"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Flash Command Read Data Register (Lower) (STIG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrddatalower::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashrddatalowerSpec;
impl crate::RegisterSpec for FlashrddatalowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashrddatalower::R`](R) reader structure"]
impl crate::Readable for FlashrddatalowerSpec {}
#[doc = "`reset()` method sets FLASHRDDATALOWER to value 0"]
impl crate::Resettable for FlashrddatalowerSpec {}
