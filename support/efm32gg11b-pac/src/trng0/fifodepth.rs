#[doc = "Register `FIFODEPTH` reader"]
pub type R = crate::R<FIFODEPTH_SPEC>;
#[doc = "Field `VALUE` reader - FIFO Depth."]
pub type VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Depth."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "FIFO Depth Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifodepth::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFODEPTH_SPEC;
impl crate::RegisterSpec for FIFODEPTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodepth::R`](R) reader structure"]
impl crate::Readable for FIFODEPTH_SPEC {}
#[doc = "`reset()` method sets FIFODEPTH to value 0x40"]
impl crate::Resettable for FIFODEPTH_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
