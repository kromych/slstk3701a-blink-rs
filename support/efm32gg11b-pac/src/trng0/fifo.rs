#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FIFO_SPEC>;
#[doc = "Field `VALUE` reader - FIFO Read Data"]
pub type VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Read Data"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FIFO_SPEC {}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
