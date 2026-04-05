#[doc = "Register `SEQCTRLB` reader"]
pub type R = crate::R<SeqctrlbSpec>;
#[doc = "Register `SEQCTRLB` writer"]
pub type W = crate::W<SeqctrlbSpec>;
#[doc = "Field `LENGTHB` reader - Buffer Length B in Bytes"]
pub type LengthbR = crate::FieldReader<u16>;
#[doc = "Field `LENGTHB` writer - Buffer Length B in Bytes"]
pub type LengthbW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DMA0PRESB` reader - DMA0 Preserve B"]
pub type Dma0presbR = crate::BitReader;
#[doc = "Field `DMA0PRESB` writer - DMA0 Preserve B"]
pub type Dma0presbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1PRESB` reader - DMA1 Preserve B"]
pub type Dma1presbR = crate::BitReader;
#[doc = "Field `DMA1PRESB` writer - DMA1 Preserve B"]
pub type Dma1presbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&self) -> LengthbR {
        LengthbR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&self) -> Dma0presbR {
        Dma0presbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&self) -> Dma1presbR {
        Dma1presbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&mut self) -> LengthbW<'_, SeqctrlbSpec> {
        LengthbW::new(self, 0)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&mut self) -> Dma0presbW<'_, SeqctrlbSpec> {
        Dma0presbW::new(self, 28)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&mut self) -> Dma1presbW<'_, SeqctrlbSpec> {
        Dma1presbW::new(self, 29)
    }
}
#[doc = "Sequence Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqctrlbSpec;
impl crate::RegisterSpec for SeqctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqctrlb::R`](R) reader structure"]
impl crate::Readable for SeqctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`seqctrlb::W`](W) writer structure"]
impl crate::Writable for SeqctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQCTRLB to value 0"]
impl crate::Resettable for SeqctrlbSpec {}
