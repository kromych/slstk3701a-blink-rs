#[doc = "Register `CURCH` reader"]
pub type R = crate::R<CurchSpec>;
#[doc = "Field `CURCH` reader - Current Channel Index"]
pub type CurchR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Current Channel Index"]
    #[inline(always)]
    pub fn curch(&self) -> CurchR {
        CurchR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Current Channel Index\n\nYou can [`read`](crate::Reg::read) this register and get [`curch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurchSpec;
impl crate::RegisterSpec for CurchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curch::R`](R) reader structure"]
impl crate::Readable for CurchSpec {}
#[doc = "`reset()` method sets CURCH to value 0"]
impl crate::Resettable for CurchSpec {}
