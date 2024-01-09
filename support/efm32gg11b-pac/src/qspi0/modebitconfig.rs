#[doc = "Register `MODEBITCONFIG` reader"]
pub type R = crate::R<MODEBITCONFIG_SPEC>;
#[doc = "Register `MODEBITCONFIG` writer"]
pub type W = crate::W<MODEBITCONFIG_SPEC>;
#[doc = "Field `MODE` reader - Mode Bits"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Mode Bits"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHUNKSIZE` reader - Chunk Size"]
pub type CHUNKSIZE_R = crate::FieldReader;
#[doc = "Field `CHUNKSIZE` writer - Chunk Size"]
pub type CHUNKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRCOUTENABLE` reader - CRC# Output Enable Bit"]
pub type CRCOUTENABLE_R = crate::BitReader;
#[doc = "Field `CRCOUTENABLE` writer - CRC# Output Enable Bit"]
pub type CRCOUTENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAUP` reader - RX CRC Data (upper)"]
pub type RXCRCDATAUP_R = crate::FieldReader;
#[doc = "Field `RXCRCDATALOW` reader - RX CRC Data (lower)"]
pub type RXCRCDATALOW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    pub fn chunksize(&self) -> CHUNKSIZE_R {
        CHUNKSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    pub fn crcoutenable(&self) -> CRCOUTENABLE_R {
        CRCOUTENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - RX CRC Data (upper)"]
    #[inline(always)]
    pub fn rxcrcdataup(&self) -> RXCRCDATAUP_R {
        RXCRCDATAUP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX CRC Data (lower)"]
    #[inline(always)]
    pub fn rxcrcdatalow(&self) -> RXCRCDATALOW_R {
        RXCRCDATALOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<MODEBITCONFIG_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Chunk Size"]
    #[inline(always)]
    #[must_use]
    pub fn chunksize(&mut self) -> CHUNKSIZE_W<MODEBITCONFIG_SPEC> {
        CHUNKSIZE_W::new(self, 8)
    }
    #[doc = "Bit 15 - CRC# Output Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcoutenable(&mut self) -> CRCOUTENABLE_W<MODEBITCONFIG_SPEC> {
        CRCOUTENABLE_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Bit Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modebitconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modebitconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEBITCONFIG_SPEC;
impl crate::RegisterSpec for MODEBITCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modebitconfig::R`](R) reader structure"]
impl crate::Readable for MODEBITCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modebitconfig::W`](W) writer structure"]
impl crate::Writable for MODEBITCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODEBITCONFIG to value 0x0200"]
impl crate::Resettable for MODEBITCONFIG_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
