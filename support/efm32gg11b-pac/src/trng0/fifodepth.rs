#[doc = "Register `FIFODEPTH` reader"]
pub type R = crate::R<FifodepthSpec>;
#[doc = "Field `VALUE` reader - FIFO Depth."]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Depth."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "FIFO Depth Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodepth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodepthSpec;
impl crate::RegisterSpec for FifodepthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodepth::R`](R) reader structure"]
impl crate::Readable for FifodepthSpec {}
#[doc = "`reset()` method sets FIFODEPTH to value 0x40"]
impl crate::Resettable for FifodepthSpec {
    const RESET_VALUE: u32 = 0x40;
}
