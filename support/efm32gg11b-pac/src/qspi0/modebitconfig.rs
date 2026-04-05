#[doc = "Register `MODEBITCONFIG` reader"]
pub type R = crate::R<ModebitconfigSpec>;
#[doc = "Register `MODEBITCONFIG` writer"]
pub type W = crate::W<ModebitconfigSpec>;
#[doc = "Field `MODE` reader - Mode Bits"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Mode Bits"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHUNKSIZE` reader - Chunk Size"]
pub type ChunksizeR = crate::FieldReader;
#[doc = "Field `CHUNKSIZE` writer - Chunk Size"]
pub type ChunksizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRCOUTENABLE` reader - CRC# Output Enable Bit"]
pub type CrcoutenableR = crate::BitReader;
#[doc = "Field `CRCOUTENABLE` writer - CRC# Output Enable Bit"]
pub type CrcoutenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAUP` reader - RX CRC Data (upper)"]
pub type RxcrcdataupR = crate::FieldReader;
#[doc = "Field `RXCRCDATALOW` reader - RX CRC Data (lower)"]
pub type RxcrcdatalowR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&self) -> ChunksizeR {
        ChunksizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&self) -> CrcoutenableR {
        CrcoutenableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX CRC Data (upper)"]
    #[inline(always)]
    pub fn rxcrcdataup(&self) -> RxcrcdataupR {
        RxcrcdataupR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX CRC Data (lower)"]
    #[inline(always)]
    pub fn rxcrcdatalow(&self) -> RxcrcdatalowR {
        RxcrcdatalowR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModebitconfigSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&mut self) -> ChunksizeW<'_, ModebitconfigSpec> {
        ChunksizeW::new(self, 8)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&mut self) -> CrcoutenableW<'_, ModebitconfigSpec> {
        CrcoutenableW::new(self, 15)
    }
}
#[doc = "Mode Bit Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modebitconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modebitconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModebitconfigSpec;
impl crate::RegisterSpec for ModebitconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modebitconfig::R`](R) reader structure"]
impl crate::Readable for ModebitconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`modebitconfig::W`](W) writer structure"]
impl crate::Writable for ModebitconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEBITCONFIG to value 0x0200"]
impl crate::Resettable for ModebitconfigSpec {
    const RESET_VALUE: u32 = 0x0200;
}
