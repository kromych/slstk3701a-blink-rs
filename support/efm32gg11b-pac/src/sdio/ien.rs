#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CMDCOMSEN` reader - Command Complete Signal Enable"]
pub type CmdcomsenR = crate::BitReader;
#[doc = "Field `CMDCOMSEN` writer - Command Complete Signal Enable"]
pub type CmdcomsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANCOMSEN` reader - Transfer Complete Signal Enable"]
pub type TrancomsenR = crate::BitReader;
#[doc = "Field `TRANCOMSEN` writer - Transfer Complete Signal Enable"]
pub type TrancomsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKGAPEVTSEN` reader - Block Gap Event Signal Enable"]
pub type BlkgapevtsenR = crate::BitReader;
#[doc = "Field `BLKGAPEVTSEN` writer - Block Gap Event Signal Enable"]
pub type BlkgapevtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINTSEN` reader - DMA Interrupt Signal Enable"]
pub type DmaintsenR = crate::BitReader;
#[doc = "Field `DMAINTSEN` writer - DMA Interrupt Signal Enable"]
pub type DmaintsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFWRRDYSEN` reader - Buffer Write Ready Signal Enable"]
pub type BufwrrdysenR = crate::BitReader;
#[doc = "Field `BUFWRRDYSEN` writer - Buffer Write Ready Signal Enable"]
pub type BufwrrdysenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFRDRDYSEN` reader - Buffer Read Ready Signal Enable"]
pub type BufrdrdysenR = crate::BitReader;
#[doc = "Field `BUFRDRDYSEN` writer - Buffer Read Ready Signal Enable"]
pub type BufrdrdysenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINSSEN` reader - Card Insertion Signal Enable"]
pub type CardinssenR = crate::BitReader;
#[doc = "Field `CARDINSSEN` writer - Card Insertion Signal Enable"]
pub type CardinssenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDREMSEN` reader - Card Removal Signal Enable"]
pub type CardremsenR = crate::BitReader;
#[doc = "Field `CARDREMSEN` writer - Card Removal Signal Enable"]
pub type CardremsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINTSEN` reader - Card Interrupt Signal Enable"]
pub type CardintsenR = crate::BitReader;
#[doc = "Field `CARDINTSEN` writer - Card Interrupt Signal Enable"]
pub type CardintsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNINGEVTSEN` reader - Re-Tuning Event Signal Enable"]
pub type RetuningevtsenR = crate::BitReader;
#[doc = "Field `RETUNINGEVTSEN` writer - Re-Tuning Event Signal Enable"]
pub type RetuningevtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKRCVSEN` reader - Boot Ack Received Signal Enable"]
pub type BootackrcvsenR = crate::BitReader;
#[doc = "Field `BOOTACKRCVSEN` writer - Boot Ack Received Signal Enable"]
pub type BootackrcvsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTTERMINATESEN` reader - Boot Terminate Interrupt Signal Enable"]
pub type BootterminatesenR = crate::BitReader;
#[doc = "Field `BOOTTERMINATESEN` writer - Boot Terminate Interrupt Signal Enable"]
pub type BootterminatesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTOUTERRSEN` reader - Command Timeout Error Signal Enable"]
pub type CmdtouterrsenR = crate::BitReader;
#[doc = "Field `CMDTOUTERRSEN` writer - Command Timeout Error Signal Enable"]
pub type CmdtouterrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCRCERRSEN` reader - Command CRC Error Signal Enable"]
pub type CmdcrcerrsenR = crate::BitReader;
#[doc = "Field `CMDCRCERRSEN` writer - Command CRC Error Signal Enable"]
pub type CmdcrcerrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDENDBITERRSEN` reader - Command End Bit Error Signal Enable"]
pub type CmdendbiterrsenR = crate::BitReader;
#[doc = "Field `CMDENDBITERRSEN` writer - Command End Bit Error Signal Enable"]
pub type CmdendbiterrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDEXERRSEN` reader - Command Index Error Signal Enable"]
pub type CmdindexerrsenR = crate::BitReader;
#[doc = "Field `CMDINDEXERRSEN` writer - Command Index Error Signal Enable"]
pub type CmdindexerrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATTOUTERRSEN` reader - Data Timeout Error Signal Enable"]
pub type DattouterrsenR = crate::BitReader;
#[doc = "Field `DATTOUTERRSEN` writer - Data Timeout Error Signal Enable"]
pub type DattouterrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCRCERRSEN` reader - Data CRC Error Signal Enable"]
pub type DatcrcerrsenR = crate::BitReader;
#[doc = "Field `DATCRCERRSEN` writer - Data CRC Error Signal Enable"]
pub type DatcrcerrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATENDBITERRSEN` reader - Data End Bit Error Signal Enable"]
pub type DatendbiterrsenR = crate::BitReader;
#[doc = "Field `DATENDBITERRSEN` writer - Data End Bit Error Signal Enable"]
pub type DatendbiterrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURRENTLIMITERRSEN` reader - Current Limit Error Signal Enable"]
pub type CurrentlimiterrsenR = crate::BitReader;
#[doc = "Field `CURRENTLIMITERRSEN` writer - Current Limit Error Signal Enable"]
pub type CurrentlimiterrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCMDERRSEN` reader - Auto CMD12 Error Signal Enable"]
pub type AutocmderrsenR = crate::BitReader;
#[doc = "Field `AUTOCMDERRSEN` writer - Auto CMD12 Error Signal Enable"]
pub type AutocmderrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMAERRSEN` reader - ADMA Error Signal Enable"]
pub type AdmaerrsenR = crate::BitReader;
#[doc = "Field `ADMAERRSEN` writer - ADMA Error Signal Enable"]
pub type AdmaerrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNINGERRSIGNALENABLE` reader - Tuning Error Signal Enable"]
pub type TuningerrsignalenableR = crate::BitReader;
#[doc = "Field `TUNINGERRSIGNALENABLE` writer - Tuning Error Signal Enable"]
pub type TuningerrsignalenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGETRESPERRSEN` reader - Target Response Error Signal Enable"]
pub type TargetresperrsenR = crate::BitReader;
#[doc = "Field `TARGETRESPERRSEN` writer - Target Response Error Signal Enable"]
pub type TargetresperrsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomsen(&self) -> CmdcomsenR {
        CmdcomsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomsen(&self) -> TrancomsenR {
        TrancomsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevtsen(&self) -> BlkgapevtsenR {
        BlkgapevtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaintsen(&self) -> DmaintsenR {
        DmaintsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdysen(&self) -> BufwrrdysenR {
        BufwrrdysenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdysen(&self) -> BufrdrdysenR {
        BufrdrdysenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinssen(&self) -> CardinssenR {
        CardinssenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardremsen(&self) -> CardremsenR {
        CardremsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardintsen(&self) -> CardintsenR {
        CardintsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevtsen(&self) -> RetuningevtsenR {
        RetuningevtsenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcvsen(&self) -> BootackrcvsenR {
        BootackrcvsenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminatesen(&self) -> BootterminatesenR {
        BootterminatesenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdtouterrsen(&self) -> CmdtouterrsenR {
        CmdtouterrsenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrcerrsen(&self) -> CmdcrcerrsenR {
        CmdcrcerrsenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdendbiterrsen(&self) -> CmdendbiterrsenR {
        CmdendbiterrsenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdindexerrsen(&self) -> CmdindexerrsenR {
        CmdindexerrsenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn dattouterrsen(&self) -> DattouterrsenR {
        DattouterrsenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrcerrsen(&self) -> DatcrcerrsenR {
        DatcrcerrsenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datendbiterrsen(&self) -> DatendbiterrsenR {
        DatendbiterrsenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn currentlimiterrsen(&self) -> CurrentlimiterrsenR {
        CurrentlimiterrsenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn autocmderrsen(&self) -> AutocmderrsenR {
        AutocmderrsenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn admaerrsen(&self) -> AdmaerrsenR {
        AdmaerrsenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Signal Enable"]
    #[inline(always)]
    pub fn tuningerrsignalenable(&self) -> TuningerrsignalenableR {
        TuningerrsignalenableR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn targetresperrsen(&self) -> TargetresperrsenR {
        TargetresperrsenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomsen(&mut self) -> CmdcomsenW<'_, IenSpec> {
        CmdcomsenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomsen(&mut self) -> TrancomsenW<'_, IenSpec> {
        TrancomsenW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevtsen(&mut self) -> BlkgapevtsenW<'_, IenSpec> {
        BlkgapevtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaintsen(&mut self) -> DmaintsenW<'_, IenSpec> {
        DmaintsenW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdysen(&mut self) -> BufwrrdysenW<'_, IenSpec> {
        BufwrrdysenW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdysen(&mut self) -> BufrdrdysenW<'_, IenSpec> {
        BufrdrdysenW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinssen(&mut self) -> CardinssenW<'_, IenSpec> {
        CardinssenW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardremsen(&mut self) -> CardremsenW<'_, IenSpec> {
        CardremsenW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardintsen(&mut self) -> CardintsenW<'_, IenSpec> {
        CardintsenW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevtsen(&mut self) -> RetuningevtsenW<'_, IenSpec> {
        RetuningevtsenW::new(self, 12)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcvsen(&mut self) -> BootackrcvsenW<'_, IenSpec> {
        BootackrcvsenW::new(self, 13)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminatesen(&mut self) -> BootterminatesenW<'_, IenSpec> {
        BootterminatesenW::new(self, 14)
    }
    #[doc = "Bit 16 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdtouterrsen(&mut self) -> CmdtouterrsenW<'_, IenSpec> {
        CmdtouterrsenW::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrcerrsen(&mut self) -> CmdcrcerrsenW<'_, IenSpec> {
        CmdcrcerrsenW::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdendbiterrsen(&mut self) -> CmdendbiterrsenW<'_, IenSpec> {
        CmdendbiterrsenW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdindexerrsen(&mut self) -> CmdindexerrsenW<'_, IenSpec> {
        CmdindexerrsenW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn dattouterrsen(&mut self) -> DattouterrsenW<'_, IenSpec> {
        DattouterrsenW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrcerrsen(&mut self) -> DatcrcerrsenW<'_, IenSpec> {
        DatcrcerrsenW::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datendbiterrsen(&mut self) -> DatendbiterrsenW<'_, IenSpec> {
        DatendbiterrsenW::new(self, 22)
    }
    #[doc = "Bit 23 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn currentlimiterrsen(&mut self) -> CurrentlimiterrsenW<'_, IenSpec> {
        CurrentlimiterrsenW::new(self, 23)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn autocmderrsen(&mut self) -> AutocmderrsenW<'_, IenSpec> {
        AutocmderrsenW::new(self, 24)
    }
    #[doc = "Bit 25 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn admaerrsen(&mut self) -> AdmaerrsenW<'_, IenSpec> {
        AdmaerrsenW::new(self, 25)
    }
    #[doc = "Bit 26 - Tuning Error Signal Enable"]
    #[inline(always)]
    pub fn tuningerrsignalenable(&mut self) -> TuningerrsignalenableW<'_, IenSpec> {
        TuningerrsignalenableW::new(self, 26)
    }
    #[doc = "Bit 28 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn targetresperrsen(&mut self) -> TargetresperrsenW<'_, IenSpec> {
        TargetresperrsenW::new(self, 28)
    }
}
#[doc = "Normal and Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
