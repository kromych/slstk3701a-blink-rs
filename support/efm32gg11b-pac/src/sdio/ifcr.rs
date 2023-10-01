#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IFCR_SPEC>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCR_SPEC>;
#[doc = "Field `CMDCOM` reader - Command Complete"]
pub type CMDCOM_R = crate::BitReader;
#[doc = "Field `CMDCOM` writer - Command Complete"]
pub type CMDCOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANCOM` reader - Transfer Complete"]
pub type TRANCOM_R = crate::BitReader;
#[doc = "Field `TRANCOM` writer - Transfer Complete"]
pub type TRANCOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKGAPEVT` reader - Block Gap Event"]
pub type BLKGAPEVT_R = crate::BitReader;
#[doc = "Field `BLKGAPEVT` writer - Block Gap Event"]
pub type BLKGAPEVT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAINT` reader - DMA Interrupt"]
pub type DMAINT_R = crate::BitReader;
#[doc = "Field `DMAINT` writer - DMA Interrupt"]
pub type DMAINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BFRWRRDY` reader - Buffer Write Ready"]
pub type BFRWRRDY_R = crate::BitReader;
#[doc = "Field `BFRWRRDY` writer - Buffer Write Ready"]
pub type BFRWRRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BFRRDRDY` reader - Buffer Read Ready"]
pub type BFRRDRDY_R = crate::BitReader;
#[doc = "Field `BFRRDRDY` writer - Buffer Read Ready"]
pub type BFRRDRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDINS` reader - Card Insertion"]
pub type CARDINS_R = crate::BitReader;
#[doc = "Field `CARDINS` writer - Card Insertion"]
pub type CARDINS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDRM` reader - Card Removal"]
pub type CARDRM_R = crate::BitReader;
#[doc = "Field `CARDRM` writer - Card Removal"]
pub type CARDRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDINT` reader - Card Interrupt"]
pub type CARDINT_R = crate::BitReader;
#[doc = "Field `RETUNINGEVT` reader - Re-Tunning Event"]
pub type RETUNINGEVT_R = crate::BitReader;
#[doc = "Field `BOOTACKRCV` reader - Boot Ack Received"]
pub type BOOTACKRCV_R = crate::BitReader;
#[doc = "Field `BOOTACKRCV` writer - Boot Ack Received"]
pub type BOOTACKRCV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOTTERMINATE` reader - Boot Terminate Interrupt"]
pub type BOOTTERMINATE_R = crate::BitReader;
#[doc = "Field `BOOTTERMINATE` writer - Boot Terminate Interrupt"]
pub type BOOTTERMINATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub type ERRINT_R = crate::BitReader;
#[doc = "Field `CMDTOUTERR` reader - Command Timeout Error"]
pub type CMDTOUTERR_R = crate::BitReader;
#[doc = "Field `CMDTOUTERR` writer - Command Timeout Error"]
pub type CMDTOUTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDCRCERR` reader - CMD CRC Error"]
pub type CMDCRCERR_R = crate::BitReader;
#[doc = "Field `CMDCRCERR` writer - CMD CRC Error"]
pub type CMDCRCERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDENDBITERR` reader - Command End Bit Error"]
pub type CMDENDBITERR_R = crate::BitReader;
#[doc = "Field `CMDENDBITERR` writer - Command End Bit Error"]
pub type CMDENDBITERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDINDEXERR` reader - Command Index Error"]
pub type CMDINDEXERR_R = crate::BitReader;
#[doc = "Field `CMDINDEXERR` writer - Command Index Error"]
pub type CMDINDEXERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATTOUTERR` reader - Data Time-out Error"]
pub type DATTOUTERR_R = crate::BitReader;
#[doc = "Field `DATTOUTERR` writer - Data Time-out Error"]
pub type DATTOUTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATCRCERR` reader - Data CRC Error"]
pub type DATCRCERR_R = crate::BitReader;
#[doc = "Field `DATCRCERR` writer - Data CRC Error"]
pub type DATCRCERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATENDBITERR` reader - Data End Bit Error"]
pub type DATENDBITERR_R = crate::BitReader;
#[doc = "Field `DATENDBITERR` writer - Data End Bit Error"]
pub type DATENDBITERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CURRENTLIMITERR` reader - Current Limit Error"]
pub type CURRENTLIMITERR_R = crate::BitReader;
#[doc = "Field `CURRENTLIMITERR` writer - Current Limit Error"]
pub type CURRENTLIMITERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOCMDERR` reader - Auto CMD Error"]
pub type AUTOCMDERR_R = crate::BitReader;
#[doc = "Field `AUTOCMDERR` writer - Auto CMD Error"]
pub type AUTOCMDERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADMAERR` reader - ADMA Error"]
pub type ADMAERR_R = crate::BitReader;
#[doc = "Field `ADMAERR` writer - ADMA Error"]
pub type ADMAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGETRESP` reader - Specific Error STAT"]
pub type TARGETRESP_R = crate::BitReader;
#[doc = "Field `TARGETRESP` writer - Specific Error STAT"]
pub type TARGETRESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdcom(&self) -> CMDCOM_R {
        CMDCOM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trancom(&self) -> TRANCOM_R {
        TRANCOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkgapevt(&self) -> BLKGAPEVT_R {
        BLKGAPEVT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bfrwrrdy(&self) -> BFRWRRDY_R {
        BFRWRRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn bfrrdrdy(&self) -> BFRRDRDY_R {
        BFRRDRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cardins(&self) -> CARDINS_R {
        CARDINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn cardrm(&self) -> CARDRM_R {
        CARDRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cardint(&self) -> CARDINT_R {
        CARDINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event"]
    #[inline(always)]
    pub fn retuningevt(&self) -> RETUNINGEVT_R {
        RETUNINGEVT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received"]
    #[inline(always)]
    pub fn bootackrcv(&self) -> BOOTACKRCV_R {
        BOOTACKRCV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt"]
    #[inline(always)]
    pub fn bootterminate(&self) -> BOOTTERMINATE_R {
        BOOTTERMINATE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtouterr(&self) -> CMDTOUTERR_R {
        CMDTOUTERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmdcrcerr(&self) -> CMDCRCERR_R {
        CMDCRCERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmdendbiterr(&self) -> CMDENDBITERR_R {
        CMDENDBITERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cmdindexerr(&self) -> CMDINDEXERR_R {
        CMDINDEXERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Time-out Error"]
    #[inline(always)]
    pub fn dattouterr(&self) -> DATTOUTERR_R {
        DATTOUTERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn datcrcerr(&self) -> DATCRCERR_R {
        DATCRCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn datendbiterr(&self) -> DATENDBITERR_R {
        DATENDBITERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error"]
    #[inline(always)]
    pub fn currentlimiterr(&self) -> CURRENTLIMITERR_R {
        CURRENTLIMITERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD Error"]
    #[inline(always)]
    pub fn autocmderr(&self) -> AUTOCMDERR_R {
        AUTOCMDERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error"]
    #[inline(always)]
    pub fn admaerr(&self) -> ADMAERR_R {
        ADMAERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Specific Error STAT"]
    #[inline(always)]
    pub fn targetresp(&self) -> TARGETRESP_R {
        TARGETRESP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcom(&mut self) -> CMDCOM_W<IFCR_SPEC, 0> {
        CMDCOM_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn trancom(&mut self) -> TRANCOM_W<IFCR_SPEC, 1> {
        TRANCOM_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    #[must_use]
    pub fn blkgapevt(&mut self) -> BLKGAPEVT_W<IFCR_SPEC, 2> {
        BLKGAPEVT_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DMAINT_W<IFCR_SPEC, 3> {
        DMAINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bfrwrrdy(&mut self) -> BFRWRRDY_W<IFCR_SPEC, 4> {
        BFRWRRDY_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bfrrdrdy(&mut self) -> BFRRDRDY_W<IFCR_SPEC, 5> {
        BFRRDRDY_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn cardins(&mut self) -> CARDINS_W<IFCR_SPEC, 6> {
        CARDINS_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn cardrm(&mut self) -> CARDRM_W<IFCR_SPEC, 7> {
        CARDRM_W::new(self)
    }
    #[doc = "Bit 13 - Boot Ack Received"]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcv(&mut self) -> BOOTACKRCV_W<IFCR_SPEC, 13> {
        BOOTACKRCV_W::new(self)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminate(&mut self) -> BOOTTERMINATE_W<IFCR_SPEC, 14> {
        BOOTTERMINATE_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtouterr(&mut self) -> CMDTOUTERR_W<IFCR_SPEC, 16> {
        CMDTOUTERR_W::new(self)
    }
    #[doc = "Bit 17 - CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerr(&mut self) -> CMDCRCERR_W<IFCR_SPEC, 17> {
        CMDCRCERR_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterr(&mut self) -> CMDENDBITERR_W<IFCR_SPEC, 18> {
        CMDENDBITERR_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerr(&mut self) -> CMDINDEXERR_W<IFCR_SPEC, 19> {
        CMDINDEXERR_W::new(self)
    }
    #[doc = "Bit 20 - Data Time-out Error"]
    #[inline(always)]
    #[must_use]
    pub fn dattouterr(&mut self) -> DATTOUTERR_W<IFCR_SPEC, 20> {
        DATTOUTERR_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn datcrcerr(&mut self) -> DATCRCERR_W<IFCR_SPEC, 21> {
        DATCRCERR_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn datendbiterr(&mut self) -> DATENDBITERR_W<IFCR_SPEC, 22> {
        DATENDBITERR_W::new(self)
    }
    #[doc = "Bit 23 - Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterr(&mut self) -> CURRENTLIMITERR_W<IFCR_SPEC, 23> {
        CURRENTLIMITERR_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn autocmderr(&mut self) -> AUTOCMDERR_W<IFCR_SPEC, 24> {
        AUTOCMDERR_W::new(self)
    }
    #[doc = "Bit 25 - ADMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn admaerr(&mut self) -> ADMAERR_W<IFCR_SPEC, 25> {
        ADMAERR_W::new(self)
    }
    #[doc = "Bit 28 - Specific Error STAT"]
    #[inline(always)]
    #[must_use]
    pub fn targetresp(&mut self) -> TARGETRESP_W<IFCR_SPEC, 28> {
        TARGETRESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Normal and Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
