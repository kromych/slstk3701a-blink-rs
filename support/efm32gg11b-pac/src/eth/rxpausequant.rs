#[doc = "Register `RXPAUSEQUANT` reader"]
pub type R = crate::R<RxpausequantSpec>;
#[doc = "Field `QUANT` reader - Received pause quantum"]
pub type QuantR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QuantR {
        QuantR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxpausequant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxpausequantSpec;
impl crate::RegisterSpec for RxpausequantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxpausequant::R`](R) reader structure"]
impl crate::Readable for RxpausequantSpec {}
#[doc = "`reset()` method sets RXPAUSEQUANT to value 0"]
impl crate::Resettable for RxpausequantSpec {}
