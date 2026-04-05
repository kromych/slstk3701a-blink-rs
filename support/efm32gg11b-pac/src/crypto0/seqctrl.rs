#[doc = "Register `SEQCTRL` reader"]
pub type R = crate::R<SeqctrlSpec>;
#[doc = "Register `SEQCTRL` writer"]
pub type W = crate::W<SeqctrlSpec>;
#[doc = "Field `LENGTHA` reader - Buffer Length a in Bytes"]
pub type LengthaR = crate::FieldReader<u16>;
#[doc = "Field `LENGTHA` writer - Buffer Length a in Bytes"]
pub type LengthaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Size of Data Blocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blocksize {
    #[doc = "0: A block is 16 bytes long"]
    _16bytes = 0,
    #[doc = "1: A block is 32 bytes long"]
    _32bytes = 1,
    #[doc = "2: A block is 64 bytes long"]
    _64bytes = 2,
}
impl From<Blocksize> for u8 {
    #[inline(always)]
    fn from(variant: Blocksize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blocksize {
    type Ux = u8;
}
impl crate::IsEnum for Blocksize {}
#[doc = "Field `BLOCKSIZE` reader - Size of Data Blocks"]
pub type BlocksizeR = crate::FieldReader<Blocksize>;
impl BlocksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blocksize> {
        match self.bits {
            0 => Some(Blocksize::_16bytes),
            1 => Some(Blocksize::_32bytes),
            2 => Some(Blocksize::_64bytes),
            _ => None,
        }
    }
    #[doc = "A block is 16 bytes long"]
    #[inline(always)]
    pub fn is_16bytes(&self) -> bool {
        *self == Blocksize::_16bytes
    }
    #[doc = "A block is 32 bytes long"]
    #[inline(always)]
    pub fn is_32bytes(&self) -> bool {
        *self == Blocksize::_32bytes
    }
    #[doc = "A block is 64 bytes long"]
    #[inline(always)]
    pub fn is_64bytes(&self) -> bool {
        *self == Blocksize::_64bytes
    }
}
#[doc = "Field `BLOCKSIZE` writer - Size of Data Blocks"]
pub type BlocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Blocksize>;
impl<'a, REG> BlocksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A block is 16 bytes long"]
    #[inline(always)]
    pub fn _16bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::_16bytes)
    }
    #[doc = "A block is 32 bytes long"]
    #[inline(always)]
    pub fn _32bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::_32bytes)
    }
    #[doc = "A block is 64 bytes long"]
    #[inline(always)]
    pub fn _64bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::_64bytes)
    }
}
#[doc = "Field `DMA0SKIP` reader - DMA0 Skip"]
pub type Dma0skipR = crate::FieldReader;
#[doc = "Field `DMA0SKIP` writer - DMA0 Skip"]
pub type Dma0skipW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA1SKIP` reader - DMA1 Skip"]
pub type Dma1skipR = crate::FieldReader;
#[doc = "Field `DMA1SKIP` writer - DMA1 Skip"]
pub type Dma1skipW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA0PRESA` reader - DMA0 Preserve a"]
pub type Dma0presaR = crate::BitReader;
#[doc = "Field `DMA0PRESA` writer - DMA0 Preserve a"]
pub type Dma0presaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1PRESA` reader - DMA1 Preserve a"]
pub type Dma1presaR = crate::BitReader;
#[doc = "Field `DMA1PRESA` writer - DMA1 Preserve a"]
pub type Dma1presaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT` reader - Halt Sequence"]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Halt Sequence"]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline(always)]
    pub fn lengtha(&self) -> LengthaR {
        LengthaR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline(always)]
    pub fn blocksize(&self) -> BlocksizeR {
        BlocksizeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline(always)]
    pub fn dma0skip(&self) -> Dma0skipR {
        Dma0skipR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline(always)]
    pub fn dma1skip(&self) -> Dma1skipR {
        Dma1skipR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline(always)]
    pub fn dma0presa(&self) -> Dma0presaR {
        Dma0presaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline(always)]
    pub fn dma1presa(&self) -> Dma1presaR {
        Dma1presaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline(always)]
    pub fn lengtha(&mut self) -> LengthaW<'_, SeqctrlSpec> {
        LengthaW::new(self, 0)
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BlocksizeW<'_, SeqctrlSpec> {
        BlocksizeW::new(self, 20)
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline(always)]
    pub fn dma0skip(&mut self) -> Dma0skipW<'_, SeqctrlSpec> {
        Dma0skipW::new(self, 24)
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline(always)]
    pub fn dma1skip(&mut self) -> Dma1skipW<'_, SeqctrlSpec> {
        Dma1skipW::new(self, 26)
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline(always)]
    pub fn dma0presa(&mut self) -> Dma0presaW<'_, SeqctrlSpec> {
        Dma0presaW::new(self, 28)
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline(always)]
    pub fn dma1presa(&mut self) -> Dma1presaW<'_, SeqctrlSpec> {
        Dma1presaW::new(self, 29)
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, SeqctrlSpec> {
        HaltW::new(self, 31)
    }
}
#[doc = "Sequence Control\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqctrlSpec;
impl crate::RegisterSpec for SeqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SeqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SeqctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQCTRL to value 0"]
impl crate::Resettable for SeqctrlSpec {}
