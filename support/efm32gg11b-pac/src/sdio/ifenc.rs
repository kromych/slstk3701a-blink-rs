#[doc = "Register `IFENC` reader"]
pub type R = crate::R<IfencSpec>;
#[doc = "Register `IFENC` writer"]
pub type W = crate::W<IfencSpec>;
#[doc = "Field `CMDCOMEN` reader - Command Complete Signal Enable"]
pub type CmdcomenR = crate::BitReader;
#[doc = "Field `CMDCOMEN` writer - Command Complete Signal Enable"]
pub type CmdcomenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANCOMEN` reader - Transfer Complete Signal Enable"]
pub type TrancomenR = crate::BitReader;
#[doc = "Field `TRANCOMEN` writer - Transfer Complete Signal Enable"]
pub type TrancomenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKGAPEVTEN` reader - Block Gap Event Signal Enable"]
pub type BlkgapevtenR = crate::BitReader;
#[doc = "Field `BLKGAPEVTEN` writer - Block Gap Event Signal Enable"]
pub type BlkgapevtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINTEN` reader - DMA Interrupt Signal Enable"]
pub type DmaintenR = crate::BitReader;
#[doc = "Field `DMAINTEN` writer - DMA Interrupt Signal Enable"]
pub type DmaintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFWRRDYEN` reader - Buffer Write Ready Signal Enable"]
pub type BufwrrdyenR = crate::BitReader;
#[doc = "Field `BUFWRRDYEN` writer - Buffer Write Ready Signal Enable"]
pub type BufwrrdyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFRDRDYEN` reader - Buffer Read Ready Signal Enable"]
pub type BufrdrdyenR = crate::BitReader;
#[doc = "Field `BUFRDRDYEN` writer - Buffer Read Ready Signal Enable"]
pub type BufrdrdyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINSEN` reader - Card Insertion Signal Enable"]
pub type CardinsenR = crate::BitReader;
#[doc = "Field `CARDINSEN` writer - Card Insertion Signal Enable"]
pub type CardinsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDRMEN` reader - Card Removal Signal Enable"]
pub type CardrmenR = crate::BitReader;
#[doc = "Field `CARDRMEN` writer - Card Removal Signal Enable"]
pub type CardrmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINTEN` reader - Card Interrupt Signal Enable"]
pub type CardintenR = crate::BitReader;
#[doc = "Field `CARDINTEN` writer - Card Interrupt Signal Enable"]
pub type CardintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNINGEVTEN` reader - Re-Tunning Event Signal Enable"]
pub type RetuningevtenR = crate::BitReader;
#[doc = "Field `RETUNINGEVTEN` writer - Re-Tunning Event Signal Enable"]
pub type RetuningevtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKRCVEN` reader - Boot Ack Received Signal Enable"]
pub type BootackrcvenR = crate::BitReader;
#[doc = "Field `BOOTACKRCVEN` writer - Boot Ack Received Signal Enable"]
pub type BootackrcvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTTERMINATEEN` reader - Boot Terminate Interrupt Signal Enable"]
pub type BootterminateenR = crate::BitReader;
#[doc = "Field `BOOTTERMINATEEN` writer - Boot Terminate Interrupt Signal Enable"]
pub type BootterminateenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTOUTERREN` reader - Command Time-out Error Status Enable"]
pub type CmdtouterrenR = crate::BitReader;
#[doc = "Field `CMDTOUTERREN` writer - Command Time-out Error Status Enable"]
pub type CmdtouterrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCRCERREN` reader - Command CRC Error Status Enable"]
pub type CmdcrcerrenR = crate::BitReader;
#[doc = "Field `CMDCRCERREN` writer - Command CRC Error Status Enable"]
pub type CmdcrcerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDENDBITERREN` reader - Command End Bit Error Status Enable"]
pub type CmdendbiterrenR = crate::BitReader;
#[doc = "Field `CMDENDBITERREN` writer - Command End Bit Error Status Enable"]
pub type CmdendbiterrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDEXERREN` reader - Command Index Error Status Enable"]
pub type CmdindexerrenR = crate::BitReader;
#[doc = "Field `CMDINDEXERREN` writer - Command Index Error Status Enable"]
pub type CmdindexerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATTOUTERREN` reader - Data Timeout Error Status Enable"]
pub type DattouterrenR = crate::BitReader;
#[doc = "Field `DATTOUTERREN` writer - Data Timeout Error Status Enable"]
pub type DattouterrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCRCERREN` reader - Data CRC Error Status Enable"]
pub type DatcrcerrenR = crate::BitReader;
#[doc = "Field `DATCRCERREN` writer - Data CRC Error Status Enable"]
pub type DatcrcerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATENDBITERREN` reader - Data End Bit Error Status Enable"]
pub type DatendbiterrenR = crate::BitReader;
#[doc = "Field `DATENDBITERREN` writer - Data End Bit Error Status Enable"]
pub type DatendbiterrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURRENTLIMITERREN` reader - Current Limit Error Status Enable"]
pub type CurrentlimiterrenR = crate::BitReader;
#[doc = "Field `CURRENTLIMITERREN` writer - Current Limit Error Status Enable"]
pub type CurrentlimiterrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCMDERREN` reader - Auto CMD12 Error Status Enable"]
pub type AutocmderrenR = crate::BitReader;
#[doc = "Field `AUTOCMDERREN` writer - Auto CMD12 Error Status Enable"]
pub type AutocmderrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMAERREN` reader - ADMA Error Status Enable"]
pub type AdmaerrenR = crate::BitReader;
#[doc = "Field `ADMAERREN` writer - ADMA Error Status Enable"]
pub type AdmaerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNINGERREN` reader - Tuning Error Status Enable"]
pub type TuningerrenR = crate::BitReader;
#[doc = "Field `TUNINGERREN` writer - Tuning Error Status Enable"]
pub type TuningerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGETRESPEN` reader - Target Response/Host Error Status Enable"]
pub type TargetrespenR = crate::BitReader;
#[doc = "Field `TARGETRESPEN` writer - Target Response/Host Error Status Enable"]
pub type TargetrespenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomen(&self) -> CmdcomenR {
        CmdcomenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomen(&self) -> TrancomenR {
        TrancomenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevten(&self) -> BlkgapevtenR {
        BlkgapevtenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmainten(&self) -> DmaintenR {
        DmaintenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdyen(&self) -> BufwrrdyenR {
        BufwrrdyenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdyen(&self) -> BufrdrdyenR {
        BufrdrdyenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinsen(&self) -> CardinsenR {
        CardinsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardrmen(&self) -> CardrmenR {
        CardrmenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardinten(&self) -> CardintenR {
        CardintenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevten(&self) -> RetuningevtenR {
        RetuningevtenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcven(&self) -> BootackrcvenR {
        BootackrcvenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminateen(&self) -> BootterminateenR {
        BootterminateenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    pub fn cmdtouterren(&self) -> CmdtouterrenR {
        CmdtouterrenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmdcrcerren(&self) -> CmdcrcerrenR {
        CmdcrcerrenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmdendbiterren(&self) -> CmdendbiterrenR {
        CmdendbiterrenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmdindexerren(&self) -> CmdindexerrenR {
        CmdindexerrenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dattouterren(&self) -> DattouterrenR {
        DattouterrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn datcrcerren(&self) -> DatcrcerrenR {
        DatcrcerrenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn datendbiterren(&self) -> DatendbiterrenR {
        DatendbiterrenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn currentlimiterren(&self) -> CurrentlimiterrenR {
        CurrentlimiterrenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn autocmderren(&self) -> AutocmderrenR {
        AutocmderrenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    pub fn admaerren(&self) -> AdmaerrenR {
        AdmaerrenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tuningerren(&self) -> TuningerrenR {
        TuningerrenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    pub fn targetrespen(&self) -> TargetrespenR {
        TargetrespenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomen(&mut self) -> CmdcomenW<'_, IfencSpec> {
        CmdcomenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomen(&mut self) -> TrancomenW<'_, IfencSpec> {
        TrancomenW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevten(&mut self) -> BlkgapevtenW<'_, IfencSpec> {
        BlkgapevtenW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmainten(&mut self) -> DmaintenW<'_, IfencSpec> {
        DmaintenW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdyen(&mut self) -> BufwrrdyenW<'_, IfencSpec> {
        BufwrrdyenW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdyen(&mut self) -> BufrdrdyenW<'_, IfencSpec> {
        BufrdrdyenW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinsen(&mut self) -> CardinsenW<'_, IfencSpec> {
        CardinsenW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardrmen(&mut self) -> CardrmenW<'_, IfencSpec> {
        CardrmenW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardinten(&mut self) -> CardintenW<'_, IfencSpec> {
        CardintenW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevten(&mut self) -> RetuningevtenW<'_, IfencSpec> {
        RetuningevtenW::new(self, 12)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcven(&mut self) -> BootackrcvenW<'_, IfencSpec> {
        BootackrcvenW::new(self, 13)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminateen(&mut self) -> BootterminateenW<'_, IfencSpec> {
        BootterminateenW::new(self, 14)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    pub fn cmdtouterren(&mut self) -> CmdtouterrenW<'_, IfencSpec> {
        CmdtouterrenW::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmdcrcerren(&mut self) -> CmdcrcerrenW<'_, IfencSpec> {
        CmdcrcerrenW::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmdendbiterren(&mut self) -> CmdendbiterrenW<'_, IfencSpec> {
        CmdendbiterrenW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmdindexerren(&mut self) -> CmdindexerrenW<'_, IfencSpec> {
        CmdindexerrenW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dattouterren(&mut self) -> DattouterrenW<'_, IfencSpec> {
        DattouterrenW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn datcrcerren(&mut self) -> DatcrcerrenW<'_, IfencSpec> {
        DatcrcerrenW::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn datendbiterren(&mut self) -> DatendbiterrenW<'_, IfencSpec> {
        DatendbiterrenW::new(self, 22)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn currentlimiterren(&mut self) -> CurrentlimiterrenW<'_, IfencSpec> {
        CurrentlimiterrenW::new(self, 23)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn autocmderren(&mut self) -> AutocmderrenW<'_, IfencSpec> {
        AutocmderrenW::new(self, 24)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    pub fn admaerren(&mut self) -> AdmaerrenW<'_, IfencSpec> {
        AdmaerrenW::new(self, 25)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tuningerren(&mut self) -> TuningerrenW<'_, IfencSpec> {
        TuningerrenW::new(self, 26)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    pub fn targetrespen(&mut self) -> TargetrespenW<'_, IfencSpec> {
        TargetrespenW::new(self, 28)
    }
}
#[doc = "Normal and Error Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifenc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifenc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfencSpec;
impl crate::RegisterSpec for IfencSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifenc::R`](R) reader structure"]
impl crate::Readable for IfencSpec {}
#[doc = "`write(|w| ..)` method takes [`ifenc::W`](W) writer structure"]
impl crate::Writable for IfencSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFENC to value 0"]
impl crate::Resettable for IfencSpec {}
