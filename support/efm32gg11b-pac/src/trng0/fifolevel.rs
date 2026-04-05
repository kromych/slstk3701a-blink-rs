#[doc = "Register `FIFOLEVEL` reader"]
pub type R = crate::R<FifolevelSpec>;
#[doc = "Field `VALUE` reader - FIFO Level"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Level"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolevel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct FifolevelSpec;
impl crate::RegisterSpec for FifolevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolevel::R`](R) reader structure"]
impl crate::Readable for FifolevelSpec {}
#[doc = "`reset()` method sets FIFOLEVEL to value 0"]
impl crate::Resettable for FifolevelSpec {}
