#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `SINGLEOF` writer - Set SINGLEOF Interrupt Flag"]
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Set SCANOF Interrupt Flag"]
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` writer - Set SINGLEUF Interrupt Flag"]
pub type SingleufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` writer - Set SCANUF Interrupt Flag"]
pub type ScanufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` writer - Set SINGLECMP Interrupt Flag"]
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` writer - Set SCANCMP Interrupt Flag"]
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` writer - Set VREFOV Interrupt Flag"]
pub type VrefovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` writer - Set PROGERR Interrupt Flag"]
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANEXTPEND` writer - Set SCANEXTPEND Interrupt Flag"]
pub type ScanextpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANPEND` writer - Set SCANPEND Interrupt Flag"]
pub type ScanpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTIMEDERR` writer - Set PRSTIMEDERR Interrupt Flag"]
pub type PrstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` writer - Set EM23ERR Interrupt Flag"]
pub type Em23errW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Set SINGLEOF Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IfsSpec> {
        SingleofW::new(self, 8)
    }
    #[doc = "Bit 9 - Set SCANOF Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IfsSpec> {
        ScanofW::new(self, 9)
    }
    #[doc = "Bit 10 - Set SINGLEUF Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SingleufW<'_, IfsSpec> {
        SingleufW::new(self, 10)
    }
    #[doc = "Bit 11 - Set SCANUF Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> ScanufW<'_, IfsSpec> {
        ScanufW::new(self, 11)
    }
    #[doc = "Bit 16 - Set SINGLECMP Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SinglecmpW<'_, IfsSpec> {
        SinglecmpW::new(self, 16)
    }
    #[doc = "Bit 17 - Set SCANCMP Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> ScancmpW<'_, IfsSpec> {
        ScancmpW::new(self, 17)
    }
    #[doc = "Bit 24 - Set VREFOV Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VrefovW<'_, IfsSpec> {
        VrefovW::new(self, 24)
    }
    #[doc = "Bit 25 - Set PROGERR Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&mut self) -> ProgerrW<'_, IfsSpec> {
        ProgerrW::new(self, 25)
    }
    #[doc = "Bit 26 - Set SCANEXTPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> ScanextpendW<'_, IfsSpec> {
        ScanextpendW::new(self, 26)
    }
    #[doc = "Bit 27 - Set SCANPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> ScanpendW<'_, IfsSpec> {
        ScanpendW::new(self, 27)
    }
    #[doc = "Bit 28 - Set PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PrstimederrW<'_, IfsSpec> {
        PrstimederrW::new(self, 28)
    }
    #[doc = "Bit 29 - Set EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> Em23errW<'_, IfsSpec> {
        Em23errW::new(self, 29)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
