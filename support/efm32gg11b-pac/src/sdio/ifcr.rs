#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IfcrSpec>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `CMDCOM` reader - Command Complete"]
pub type CmdcomR = crate::BitReader;
#[doc = "Field `CMDCOM` writer - Command Complete"]
pub type CmdcomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANCOM` reader - Transfer Complete"]
pub type TrancomR = crate::BitReader;
#[doc = "Field `TRANCOM` writer - Transfer Complete"]
pub type TrancomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKGAPEVT` reader - Block Gap Event"]
pub type BlkgapevtR = crate::BitReader;
#[doc = "Field `BLKGAPEVT` writer - Block Gap Event"]
pub type BlkgapevtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINT` reader - DMA Interrupt"]
pub type DmaintR = crate::BitReader;
#[doc = "Field `DMAINT` writer - DMA Interrupt"]
pub type DmaintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRWRRDY` reader - Buffer Write Ready"]
pub type BfrwrrdyR = crate::BitReader;
#[doc = "Field `BFRWRRDY` writer - Buffer Write Ready"]
pub type BfrwrrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRRDRDY` reader - Buffer Read Ready"]
pub type BfrrdrdyR = crate::BitReader;
#[doc = "Field `BFRRDRDY` writer - Buffer Read Ready"]
pub type BfrrdrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINS` reader - Card Insertion"]
pub type CardinsR = crate::BitReader;
#[doc = "Field `CARDINS` writer - Card Insertion"]
pub type CardinsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDRM` reader - Card Removal"]
pub type CardrmR = crate::BitReader;
#[doc = "Field `CARDRM` writer - Card Removal"]
pub type CardrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINT` reader - Card Interrupt"]
pub type CardintR = crate::BitReader;
#[doc = "Field `RETUNINGEVT` reader - Re-Tunning Event"]
pub type RetuningevtR = crate::BitReader;
#[doc = "Field `BOOTACKRCV` reader - Boot Ack Received"]
pub type BootackrcvR = crate::BitReader;
#[doc = "Field `BOOTACKRCV` writer - Boot Ack Received"]
pub type BootackrcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTTERMINATE` reader - Boot Terminate Interrupt"]
pub type BootterminateR = crate::BitReader;
#[doc = "Field `BOOTTERMINATE` writer - Boot Terminate Interrupt"]
pub type BootterminateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub type ErrintR = crate::BitReader;
#[doc = "Field `CMDTOUTERR` reader - Command Timeout Error"]
pub type CmdtouterrR = crate::BitReader;
#[doc = "Field `CMDTOUTERR` writer - Command Timeout Error"]
pub type CmdtouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCRCERR` reader - CMD CRC Error"]
pub type CmdcrcerrR = crate::BitReader;
#[doc = "Field `CMDCRCERR` writer - CMD CRC Error"]
pub type CmdcrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDENDBITERR` reader - Command End Bit Error"]
pub type CmdendbiterrR = crate::BitReader;
#[doc = "Field `CMDENDBITERR` writer - Command End Bit Error"]
pub type CmdendbiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDEXERR` reader - Command Index Error"]
pub type CmdindexerrR = crate::BitReader;
#[doc = "Field `CMDINDEXERR` writer - Command Index Error"]
pub type CmdindexerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATTOUTERR` reader - Data Time-out Error"]
pub type DattouterrR = crate::BitReader;
#[doc = "Field `DATTOUTERR` writer - Data Time-out Error"]
pub type DattouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCRCERR` reader - Data CRC Error"]
pub type DatcrcerrR = crate::BitReader;
#[doc = "Field `DATCRCERR` writer - Data CRC Error"]
pub type DatcrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATENDBITERR` reader - Data End Bit Error"]
pub type DatendbiterrR = crate::BitReader;
#[doc = "Field `DATENDBITERR` writer - Data End Bit Error"]
pub type DatendbiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURRENTLIMITERR` reader - Current Limit Error"]
pub type CurrentlimiterrR = crate::BitReader;
#[doc = "Field `CURRENTLIMITERR` writer - Current Limit Error"]
pub type CurrentlimiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCMDERR` reader - Auto CMD Error"]
pub type AutocmderrR = crate::BitReader;
#[doc = "Field `AUTOCMDERR` writer - Auto CMD Error"]
pub type AutocmderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMAERR` reader - ADMA Error"]
pub type AdmaerrR = crate::BitReader;
#[doc = "Field `ADMAERR` writer - ADMA Error"]
pub type AdmaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGETRESP` reader - Specific Error STAT"]
pub type TargetrespR = crate::BitReader;
#[doc = "Field `TARGETRESP` writer - Specific Error STAT"]
pub type TargetrespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdcom(&self) -> CmdcomR {
        CmdcomR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trancom(&self) -> TrancomR {
        TrancomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkgapevt(&self) -> BlkgapevtR {
        BlkgapevtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&self) -> DmaintR {
        DmaintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bfrwrrdy(&self) -> BfrwrrdyR {
        BfrwrrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn bfrrdrdy(&self) -> BfrrdrdyR {
        BfrrdrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cardins(&self) -> CardinsR {
        CardinsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn cardrm(&self) -> CardrmR {
        CardrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cardint(&self) -> CardintR {
        CardintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event"]
    #[inline(always)]
    pub fn retuningevt(&self) -> RetuningevtR {
        RetuningevtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received"]
    #[inline(always)]
    pub fn bootackrcv(&self) -> BootackrcvR {
        BootackrcvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt"]
    #[inline(always)]
    pub fn bootterminate(&self) -> BootterminateR {
        BootterminateR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ErrintR {
        ErrintR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtouterr(&self) -> CmdtouterrR {
        CmdtouterrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmdcrcerr(&self) -> CmdcrcerrR {
        CmdcrcerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmdendbiterr(&self) -> CmdendbiterrR {
        CmdendbiterrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cmdindexerr(&self) -> CmdindexerrR {
        CmdindexerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Time-out Error"]
    #[inline(always)]
    pub fn dattouterr(&self) -> DattouterrR {
        DattouterrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn datcrcerr(&self) -> DatcrcerrR {
        DatcrcerrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn datendbiterr(&self) -> DatendbiterrR {
        DatendbiterrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error"]
    #[inline(always)]
    pub fn currentlimiterr(&self) -> CurrentlimiterrR {
        CurrentlimiterrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD Error"]
    #[inline(always)]
    pub fn autocmderr(&self) -> AutocmderrR {
        AutocmderrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error"]
    #[inline(always)]
    pub fn admaerr(&self) -> AdmaerrR {
        AdmaerrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Specific Error STAT"]
    #[inline(always)]
    pub fn targetresp(&self) -> TargetrespR {
        TargetrespR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdcom(&mut self) -> CmdcomW<'_, IfcrSpec> {
        CmdcomW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trancom(&mut self) -> TrancomW<'_, IfcrSpec> {
        TrancomW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkgapevt(&mut self) -> BlkgapevtW<'_, IfcrSpec> {
        BlkgapevtW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&mut self) -> DmaintW<'_, IfcrSpec> {
        DmaintW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bfrwrrdy(&mut self) -> BfrwrrdyW<'_, IfcrSpec> {
        BfrwrrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn bfrrdrdy(&mut self) -> BfrrdrdyW<'_, IfcrSpec> {
        BfrrdrdyW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cardins(&mut self) -> CardinsW<'_, IfcrSpec> {
        CardinsW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn cardrm(&mut self) -> CardrmW<'_, IfcrSpec> {
        CardrmW::new(self, 7)
    }
    #[doc = "Bit 13 - Boot Ack Received"]
    #[inline(always)]
    pub fn bootackrcv(&mut self) -> BootackrcvW<'_, IfcrSpec> {
        BootackrcvW::new(self, 13)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt"]
    #[inline(always)]
    pub fn bootterminate(&mut self) -> BootterminateW<'_, IfcrSpec> {
        BootterminateW::new(self, 14)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtouterr(&mut self) -> CmdtouterrW<'_, IfcrSpec> {
        CmdtouterrW::new(self, 16)
    }
    #[doc = "Bit 17 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmdcrcerr(&mut self) -> CmdcrcerrW<'_, IfcrSpec> {
        CmdcrcerrW::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmdendbiterr(&mut self) -> CmdendbiterrW<'_, IfcrSpec> {
        CmdendbiterrW::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cmdindexerr(&mut self) -> CmdindexerrW<'_, IfcrSpec> {
        CmdindexerrW::new(self, 19)
    }
    #[doc = "Bit 20 - Data Time-out Error"]
    #[inline(always)]
    pub fn dattouterr(&mut self) -> DattouterrW<'_, IfcrSpec> {
        DattouterrW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn datcrcerr(&mut self) -> DatcrcerrW<'_, IfcrSpec> {
        DatcrcerrW::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn datendbiterr(&mut self) -> DatendbiterrW<'_, IfcrSpec> {
        DatendbiterrW::new(self, 22)
    }
    #[doc = "Bit 23 - Current Limit Error"]
    #[inline(always)]
    pub fn currentlimiterr(&mut self) -> CurrentlimiterrW<'_, IfcrSpec> {
        CurrentlimiterrW::new(self, 23)
    }
    #[doc = "Bit 24 - Auto CMD Error"]
    #[inline(always)]
    pub fn autocmderr(&mut self) -> AutocmderrW<'_, IfcrSpec> {
        AutocmderrW::new(self, 24)
    }
    #[doc = "Bit 25 - ADMA Error"]
    #[inline(always)]
    pub fn admaerr(&mut self) -> AdmaerrW<'_, IfcrSpec> {
        AdmaerrW::new(self, 25)
    }
    #[doc = "Bit 28 - Specific Error STAT"]
    #[inline(always)]
    pub fn targetresp(&mut self) -> TargetrespW<'_, IfcrSpec> {
        TargetrespW::new(self, 28)
    }
}
#[doc = "Normal and Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}
