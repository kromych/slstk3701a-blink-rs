#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `SINGLEOF` writer - Clear SINGLEOF Interrupt Flag"]
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOF` writer - Clear SCANOF Interrupt Flag"]
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEUF` writer - Clear SINGLEUF Interrupt Flag"]
pub type SingleufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANUF` writer - Clear SCANUF Interrupt Flag"]
pub type ScanufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` writer - Clear SINGLECMP Interrupt Flag"]
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` writer - Clear SCANCMP Interrupt Flag"]
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOV` writer - Clear VREFOV Interrupt Flag"]
pub type VrefovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` writer - Clear PROGERR Interrupt Flag"]
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANEXTPEND` writer - Clear SCANEXTPEND Interrupt Flag"]
pub type ScanextpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANPEND` writer - Clear SCANPEND Interrupt Flag"]
pub type ScanpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTIMEDERR` writer - Clear PRSTIMEDERR Interrupt Flag"]
pub type PrstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` writer - Clear EM23ERR Interrupt Flag"]
pub type Em23errW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Clear SINGLEOF Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SingleofW<'_, IfcSpec> {
        SingleofW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear SCANOF Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&mut self) -> ScanofW<'_, IfcSpec> {
        ScanofW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear SINGLEUF Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SingleufW<'_, IfcSpec> {
        SingleufW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear SCANUF Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> ScanufW<'_, IfcSpec> {
        ScanufW::new(self, 11)
    }
    #[doc = "Bit 16 - Clear SINGLECMP Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SinglecmpW<'_, IfcSpec> {
        SinglecmpW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear SCANCMP Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> ScancmpW<'_, IfcSpec> {
        ScancmpW::new(self, 17)
    }
    #[doc = "Bit 24 - Clear VREFOV Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VrefovW<'_, IfcSpec> {
        VrefovW::new(self, 24)
    }
    #[doc = "Bit 25 - Clear PROGERR Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&mut self) -> ProgerrW<'_, IfcSpec> {
        ProgerrW::new(self, 25)
    }
    #[doc = "Bit 26 - Clear SCANEXTPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> ScanextpendW<'_, IfcSpec> {
        ScanextpendW::new(self, 26)
    }
    #[doc = "Bit 27 - Clear SCANPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> ScanpendW<'_, IfcSpec> {
        ScanpendW::new(self, 27)
    }
    #[doc = "Bit 28 - Clear PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PrstimederrW<'_, IfcSpec> {
        PrstimederrW::new(self, 28)
    }
    #[doc = "Bit 29 - Clear EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> Em23errW<'_, IfcSpec> {
        Em23errW::new(self, 29)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
