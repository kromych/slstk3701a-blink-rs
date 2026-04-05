#[doc = "Register `DIEP0_DTXFSTS` reader"]
pub type R = crate::R<Diep0DtxfstsSpec>;
#[doc = "Field `SPCAVAIL` reader - IN Endpoint TxFIFO Space Avail"]
pub type SpcavailR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn spcavail(&self) -> SpcavailR {
        SpcavailR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_dtxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0DtxfstsSpec;
impl crate::RegisterSpec for Diep0DtxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0_dtxfsts::R`](R) reader structure"]
impl crate::Readable for Diep0DtxfstsSpec {}
#[doc = "`reset()` method sets DIEP0_DTXFSTS to value 0x0200"]
impl crate::Resettable for Diep0DtxfstsSpec {
    const RESET_VALUE: u32 = 0x0200;
}
