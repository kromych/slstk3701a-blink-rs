#[doc = "Register `MESSAGESTATE` reader"]
pub type R = crate::R<MessagestateSpec>;
#[doc = "Field `VALID` reader - Message Valid Bits (of All Message Objects)"]
pub type ValidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Message Valid Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(self.bits)
    }
}
#[doc = "Message Valid Register\n\nYou can [`read`](crate::Reg::read) this register and get [`messagestate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MessagestateSpec;
impl crate::RegisterSpec for MessagestateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`messagestate::R`](R) reader structure"]
impl crate::Readable for MessagestateSpec {}
#[doc = "`reset()` method sets MESSAGESTATE to value 0"]
impl crate::Resettable for MessagestateSpec {}
