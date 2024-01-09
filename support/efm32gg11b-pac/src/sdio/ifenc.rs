#[doc = "Register `IFENC` reader"]
pub type R = crate::R<IFENC_SPEC>;
#[doc = "Register `IFENC` writer"]
pub type W = crate::W<IFENC_SPEC>;
#[doc = "Field `CMDCOMEN` reader - Command Complete Signal Enable"]
pub type CMDCOMEN_R = crate::BitReader;
#[doc = "Field `CMDCOMEN` writer - Command Complete Signal Enable"]
pub type CMDCOMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANCOMEN` reader - Transfer Complete Signal Enable"]
pub type TRANCOMEN_R = crate::BitReader;
#[doc = "Field `TRANCOMEN` writer - Transfer Complete Signal Enable"]
pub type TRANCOMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKGAPEVTEN` reader - Block Gap Event Signal Enable"]
pub type BLKGAPEVTEN_R = crate::BitReader;
#[doc = "Field `BLKGAPEVTEN` writer - Block Gap Event Signal Enable"]
pub type BLKGAPEVTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINTEN` reader - DMA Interrupt Signal Enable"]
pub type DMAINTEN_R = crate::BitReader;
#[doc = "Field `DMAINTEN` writer - DMA Interrupt Signal Enable"]
pub type DMAINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFWRRDYEN` reader - Buffer Write Ready Signal Enable"]
pub type BUFWRRDYEN_R = crate::BitReader;
#[doc = "Field `BUFWRRDYEN` writer - Buffer Write Ready Signal Enable"]
pub type BUFWRRDYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFRDRDYEN` reader - Buffer Read Ready Signal Enable"]
pub type BUFRDRDYEN_R = crate::BitReader;
#[doc = "Field `BUFRDRDYEN` writer - Buffer Read Ready Signal Enable"]
pub type BUFRDRDYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINSEN` reader - Card Insertion Signal Enable"]
pub type CARDINSEN_R = crate::BitReader;
#[doc = "Field `CARDINSEN` writer - Card Insertion Signal Enable"]
pub type CARDINSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDRMEN` reader - Card Removal Signal Enable"]
pub type CARDRMEN_R = crate::BitReader;
#[doc = "Field `CARDRMEN` writer - Card Removal Signal Enable"]
pub type CARDRMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDINTEN` reader - Card Interrupt Signal Enable"]
pub type CARDINTEN_R = crate::BitReader;
#[doc = "Field `CARDINTEN` writer - Card Interrupt Signal Enable"]
pub type CARDINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNINGEVTEN` reader - Re-Tunning Event Signal Enable"]
pub type RETUNINGEVTEN_R = crate::BitReader;
#[doc = "Field `RETUNINGEVTEN` writer - Re-Tunning Event Signal Enable"]
pub type RETUNINGEVTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKRCVEN` reader - Boot Ack Received Signal Enable"]
pub type BOOTACKRCVEN_R = crate::BitReader;
#[doc = "Field `BOOTACKRCVEN` writer - Boot Ack Received Signal Enable"]
pub type BOOTACKRCVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTTERMINATEEN` reader - Boot Terminate Interrupt Signal Enable"]
pub type BOOTTERMINATEEN_R = crate::BitReader;
#[doc = "Field `BOOTTERMINATEEN` writer - Boot Terminate Interrupt Signal Enable"]
pub type BOOTTERMINATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTOUTERREN` reader - Command Time-out Error Status Enable"]
pub type CMDTOUTERREN_R = crate::BitReader;
#[doc = "Field `CMDTOUTERREN` writer - Command Time-out Error Status Enable"]
pub type CMDTOUTERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCRCERREN` reader - Command CRC Error Status Enable"]
pub type CMDCRCERREN_R = crate::BitReader;
#[doc = "Field `CMDCRCERREN` writer - Command CRC Error Status Enable"]
pub type CMDCRCERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDENDBITERREN` reader - Command End Bit Error Status Enable"]
pub type CMDENDBITERREN_R = crate::BitReader;
#[doc = "Field `CMDENDBITERREN` writer - Command End Bit Error Status Enable"]
pub type CMDENDBITERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDEXERREN` reader - Command Index Error Status Enable"]
pub type CMDINDEXERREN_R = crate::BitReader;
#[doc = "Field `CMDINDEXERREN` writer - Command Index Error Status Enable"]
pub type CMDINDEXERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATTOUTERREN` reader - Data Timeout Error Status Enable"]
pub type DATTOUTERREN_R = crate::BitReader;
#[doc = "Field `DATTOUTERREN` writer - Data Timeout Error Status Enable"]
pub type DATTOUTERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCRCERREN` reader - Data CRC Error Status Enable"]
pub type DATCRCERREN_R = crate::BitReader;
#[doc = "Field `DATCRCERREN` writer - Data CRC Error Status Enable"]
pub type DATCRCERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATENDBITERREN` reader - Data End Bit Error Status Enable"]
pub type DATENDBITERREN_R = crate::BitReader;
#[doc = "Field `DATENDBITERREN` writer - Data End Bit Error Status Enable"]
pub type DATENDBITERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURRENTLIMITERREN` reader - Current Limit Error Status Enable"]
pub type CURRENTLIMITERREN_R = crate::BitReader;
#[doc = "Field `CURRENTLIMITERREN` writer - Current Limit Error Status Enable"]
pub type CURRENTLIMITERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCMDERREN` reader - Auto CMD12 Error Status Enable"]
pub type AUTOCMDERREN_R = crate::BitReader;
#[doc = "Field `AUTOCMDERREN` writer - Auto CMD12 Error Status Enable"]
pub type AUTOCMDERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMAERREN` reader - ADMA Error Status Enable"]
pub type ADMAERREN_R = crate::BitReader;
#[doc = "Field `ADMAERREN` writer - ADMA Error Status Enable"]
pub type ADMAERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNINGERREN` reader - Tuning Error Status Enable"]
pub type TUNINGERREN_R = crate::BitReader;
#[doc = "Field `TUNINGERREN` writer - Tuning Error Status Enable"]
pub type TUNINGERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGETRESPEN` reader - Target Response/Host Error Status Enable"]
pub type TARGETRESPEN_R = crate::BitReader;
#[doc = "Field `TARGETRESPEN` writer - Target Response/Host Error Status Enable"]
pub type TARGETRESPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomen(&self) -> CMDCOMEN_R {
        CMDCOMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomen(&self) -> TRANCOMEN_R {
        TRANCOMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevten(&self) -> BLKGAPEVTEN_R {
        BLKGAPEVTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmainten(&self) -> DMAINTEN_R {
        DMAINTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdyen(&self) -> BUFWRRDYEN_R {
        BUFWRRDYEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdyen(&self) -> BUFRDRDYEN_R {
        BUFRDRDYEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinsen(&self) -> CARDINSEN_R {
        CARDINSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardrmen(&self) -> CARDRMEN_R {
        CARDRMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardinten(&self) -> CARDINTEN_R {
        CARDINTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevten(&self) -> RETUNINGEVTEN_R {
        RETUNINGEVTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcven(&self) -> BOOTACKRCVEN_R {
        BOOTACKRCVEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminateen(&self) -> BOOTTERMINATEEN_R {
        BOOTTERMINATEEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    pub fn cmdtouterren(&self) -> CMDTOUTERREN_R {
        CMDTOUTERREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmdcrcerren(&self) -> CMDCRCERREN_R {
        CMDCRCERREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmdendbiterren(&self) -> CMDENDBITERREN_R {
        CMDENDBITERREN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmdindexerren(&self) -> CMDINDEXERREN_R {
        CMDINDEXERREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dattouterren(&self) -> DATTOUTERREN_R {
        DATTOUTERREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn datcrcerren(&self) -> DATCRCERREN_R {
        DATCRCERREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn datendbiterren(&self) -> DATENDBITERREN_R {
        DATENDBITERREN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn currentlimiterren(&self) -> CURRENTLIMITERREN_R {
        CURRENTLIMITERREN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn autocmderren(&self) -> AUTOCMDERREN_R {
        AUTOCMDERREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    pub fn admaerren(&self) -> ADMAERREN_R {
        ADMAERREN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tuningerren(&self) -> TUNINGERREN_R {
        TUNINGERREN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    pub fn targetrespen(&self) -> TARGETRESPEN_R {
        TARGETRESPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcomen(&mut self) -> CMDCOMEN_W<IFENC_SPEC> {
        CMDCOMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trancomen(&mut self) -> TRANCOMEN_W<IFENC_SPEC> {
        TRANCOMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkgapevten(&mut self) -> BLKGAPEVTEN_W<IFENC_SPEC> {
        BLKGAPEVTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmainten(&mut self) -> DMAINTEN_W<IFENC_SPEC> {
        DMAINTEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufwrrdyen(&mut self) -> BUFWRRDYEN_W<IFENC_SPEC> {
        BUFWRRDYEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufrdrdyen(&mut self) -> BUFRDRDYEN_W<IFENC_SPEC> {
        BUFRDRDYEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardinsen(&mut self) -> CARDINSEN_W<IFENC_SPEC> {
        CARDINSEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardrmen(&mut self) -> CARDRMEN_W<IFENC_SPEC> {
        CARDRMEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardinten(&mut self) -> CARDINTEN_W<IFENC_SPEC> {
        CARDINTEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn retuningevten(&mut self) -> RETUNINGEVTEN_W<IFENC_SPEC> {
        RETUNINGEVTEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcven(&mut self) -> BOOTACKRCVEN_W<IFENC_SPEC> {
        BOOTACKRCVEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminateen(&mut self) -> BOOTTERMINATEEN_W<IFENC_SPEC> {
        BOOTTERMINATEEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtouterren(&mut self) -> CMDTOUTERREN_W<IFENC_SPEC> {
        CMDTOUTERREN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerren(&mut self) -> CMDCRCERREN_W<IFENC_SPEC> {
        CMDCRCERREN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterren(&mut self) -> CMDENDBITERREN_W<IFENC_SPEC> {
        CMDENDBITERREN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerren(&mut self) -> CMDINDEXERREN_W<IFENC_SPEC> {
        CMDINDEXERREN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dattouterren(&mut self) -> DATTOUTERREN_W<IFENC_SPEC> {
        DATTOUTERREN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datcrcerren(&mut self) -> DATCRCERREN_W<IFENC_SPEC> {
        DATCRCERREN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datendbiterren(&mut self) -> DATENDBITERREN_W<IFENC_SPEC> {
        DATENDBITERREN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterren(&mut self) -> CURRENTLIMITERREN_W<IFENC_SPEC> {
        CURRENTLIMITERREN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autocmderren(&mut self) -> AUTOCMDERREN_W<IFENC_SPEC> {
        AUTOCMDERREN_W::new(self, 24)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn admaerren(&mut self) -> ADMAERREN_W<IFENC_SPEC> {
        ADMAERREN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuningerren(&mut self) -> TUNINGERREN_W<IFENC_SPEC> {
        TUNINGERREN_W::new(self, 26)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn targetrespen(&mut self) -> TARGETRESPEN_W<IFENC_SPEC> {
        TARGETRESPEN_W::new(self, 28)
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
#[doc = "Normal and Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifenc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifenc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFENC_SPEC;
impl crate::RegisterSpec for IFENC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifenc::R`](R) reader structure"]
impl crate::Readable for IFENC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifenc::W`](W) writer structure"]
impl crate::Writable for IFENC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFENC to value 0"]
impl crate::Resettable for IFENC_SPEC {
    const RESET_VALUE: u32 = 0;
}
