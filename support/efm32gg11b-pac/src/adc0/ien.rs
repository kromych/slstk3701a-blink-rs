#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `SINGLE` reader - SINGLE Interrupt Enable"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SINGLE` writer - SINGLE Interrupt Enable"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` reader - SCAN Interrupt Enable"]
pub type ScanR = crate::BitReader;
#[doc = "Field `SCAN` writer - SCAN Interrupt Enable"]
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEOF` reader - SINGLEOF Interrupt Enable"]
pub type SingleofR = crate::BitReader;
#[doc = "Field `SINGLEOF` writer - SINGLEOF Interrupt Enable"]
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` reader - SCANOF Interrupt Enable"]
pub type ScanofR = crate::BitReader;
#[doc = "Field `SCANOF` writer - SCANOF Interrupt Enable"]
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` reader - SINGLEUF Interrupt Enable"]
pub type SingleufR = crate::BitReader;
#[doc = "Field `SINGLEUF` writer - SINGLEUF Interrupt Enable"]
pub type SingleufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` reader - SCANUF Interrupt Enable"]
pub type ScanufR = crate::BitReader;
#[doc = "Field `SCANUF` writer - SCANUF Interrupt Enable"]
pub type ScanufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` reader - SINGLECMP Interrupt Enable"]
pub type SinglecmpR = crate::BitReader;
#[doc = "Field `SINGLECMP` writer - SINGLECMP Interrupt Enable"]
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` reader - SCANCMP Interrupt Enable"]
pub type ScancmpR = crate::BitReader;
#[doc = "Field `SCANCMP` writer - SCANCMP Interrupt Enable"]
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` reader - VREFOV Interrupt Enable"]
pub type VrefovR = crate::BitReader;
#[doc = "Field `VREFOV` writer - VREFOV Interrupt Enable"]
pub type VrefovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` reader - PROGERR Interrupt Enable"]
pub type ProgerrR = crate::BitReader;
#[doc = "Field `PROGERR` writer - PROGERR Interrupt Enable"]
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANEXTPEND` reader - SCANEXTPEND Interrupt Enable"]
pub type ScanextpendR = crate::BitReader;
#[doc = "Field `SCANEXTPEND` writer - SCANEXTPEND Interrupt Enable"]
pub type ScanextpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANPEND` reader - SCANPEND Interrupt Enable"]
pub type ScanpendR = crate::BitReader;
#[doc = "Field `SCANPEND` writer - SCANPEND Interrupt Enable"]
pub type ScanpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTIMEDERR` reader - PRSTIMEDERR Interrupt Enable"]
pub type PrstimederrR = crate::BitReader;
#[doc = "Field `PRSTIMEDERR` writer - PRSTIMEDERR Interrupt Enable"]
pub type PrstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` reader - EM23ERR Interrupt Enable"]
pub type Em23errR = crate::BitReader;
#[doc = "Field `EM23ERR` writer - EM23ERR Interrupt Enable"]
pub type Em23errW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SingleofR {
        SingleofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> ScanofR {
        ScanofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&self) -> SingleufR {
        SingleufR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&self) -> ScanufR {
        ScanufR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SinglecmpR {
        SinglecmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&self) -> ScancmpR {
        ScancmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&self) -> VrefovR {
        VrefovR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&self) -> ScanextpendR {
        ScanextpendR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&self) -> ScanpendR {
        ScanpendR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PrstimederrR {
        PrstimederrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> Em23errR {
        Em23errR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<'_, IenSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> ScanW<'_, IenSpec> {
        ScanW::new(self, 1)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IenSpec> {
        SingleofW::new(self, 8)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IenSpec> {
        ScanofW::new(self, 9)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SingleufW<'_, IenSpec> {
        SingleufW::new(self, 10)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> ScanufW<'_, IenSpec> {
        ScanufW::new(self, 11)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SinglecmpW<'_, IenSpec> {
        SinglecmpW::new(self, 16)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> ScancmpW<'_, IenSpec> {
        ScancmpW::new(self, 17)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VrefovW<'_, IenSpec> {
        VrefovW::new(self, 24)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&mut self) -> ProgerrW<'_, IenSpec> {
        ProgerrW::new(self, 25)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> ScanextpendW<'_, IenSpec> {
        ScanextpendW::new(self, 26)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> ScanpendW<'_, IenSpec> {
        ScanpendW::new(self, 27)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PrstimederrW<'_, IenSpec> {
        PrstimederrW::new(self, 28)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&mut self) -> Em23errW<'_, IenSpec> {
        Em23errW::new(self, 29)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
