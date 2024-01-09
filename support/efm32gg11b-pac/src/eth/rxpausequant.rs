#[doc = "Register `RXPAUSEQUANT` reader"]
pub type R = crate::R<RXPAUSEQUANT_SPEC>;
#[doc = "Field `QUANT` reader - Received pause quantum"]
pub type QUANT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QUANT_R {
        QUANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpausequant::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPAUSEQUANT_SPEC;
impl crate::RegisterSpec for RXPAUSEQUANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxpausequant::R`](R) reader structure"]
impl crate::Readable for RXPAUSEQUANT_SPEC {}
#[doc = "`reset()` method sets RXPAUSEQUANT to value 0"]
impl crate::Resettable for RXPAUSEQUANT_SPEC {
    const RESET_VALUE: u32 = 0;
}
