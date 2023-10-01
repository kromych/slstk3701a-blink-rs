#[doc = "Register `DIEP2_DTXFSTS` reader"]
pub type R = crate::R<DIEP2_DTXFSTS_SPEC>;
#[doc = "Field `SPCAVAIL` reader - IN Endpoint TxFIFO Space Avail"]
pub type SPCAVAIL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn spcavail(&self) -> SPCAVAIL_R {
        SPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2_dtxfsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP2_DTXFSTS_SPEC;
impl crate::RegisterSpec for DIEP2_DTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep2_dtxfsts::R`](R) reader structure"]
impl crate::Readable for DIEP2_DTXFSTS_SPEC {}
#[doc = "`reset()` method sets DIEP2_DTXFSTS to value 0x0200"]
impl crate::Resettable for DIEP2_DTXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
