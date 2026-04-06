#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HptxstsSpec>;
#[doc = "Field `PTXFSPCAVAIL` reader - Periodic Transmit Data FIFO Space Available"]
pub type PtxfspcavailR = crate::FieldReader<u16>;
#[doc = "Field `PTXQSPCAVAIL` reader - Periodic Transmit Request Queue Space Available"]
pub type PtxqspcavailR = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - Top of the Periodic Transmit Request Queue"]
pub type PtxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PtxfspcavailR {
        PtxfspcavailR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PtxqspcavailR {
        PtxqspcavailR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PtxqtopR {
        PtxqtopR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Host Periodic Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxstsSpec;
impl crate::RegisterSpec for HptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HptxstsSpec {}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0200"]
impl crate::Resettable for HptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0200;
}
