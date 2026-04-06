#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HptxfsizSpec>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HptxfsizSpec>;
#[doc = "Field `PTXFSTADDR` reader - Host Periodic TxFIFO Start Address"]
pub type PtxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `PTXFSTADDR` writer - Host Periodic TxFIFO Start Address"]
pub type PtxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PTXFSIZE` reader - Host Periodic TxFIFO Depth"]
pub type PtxfsizeR = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZE` writer - Host Periodic TxFIFO Depth"]
pub type PtxfsizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PtxfstaddrR {
        PtxfstaddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PtxfsizeR {
        PtxfsizeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptxfstaddr(&mut self) -> PtxfstaddrW<'_, HptxfsizSpec> {
        PtxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptxfsize(&mut self) -> PtxfsizeW<'_, HptxfsizSpec> {
        PtxfsizeW::new(self, 16)
    }
}
#[doc = "Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxfsizSpec;
impl crate::RegisterSpec for HptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HptxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0200_0400"]
impl crate::Resettable for HptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0400;
}
