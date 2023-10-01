#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `CMDCOMSEN` reader - Command Complete Signal Enable"]
pub type CMDCOMSEN_R = crate::BitReader;
#[doc = "Field `CMDCOMSEN` writer - Command Complete Signal Enable"]
pub type CMDCOMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANCOMSEN` reader - Transfer Complete Signal Enable"]
pub type TRANCOMSEN_R = crate::BitReader;
#[doc = "Field `TRANCOMSEN` writer - Transfer Complete Signal Enable"]
pub type TRANCOMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKGAPEVTSEN` reader - Block Gap Event Signal Enable"]
pub type BLKGAPEVTSEN_R = crate::BitReader;
#[doc = "Field `BLKGAPEVTSEN` writer - Block Gap Event Signal Enable"]
pub type BLKGAPEVTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAINTSEN` reader - DMA Interrupt Signal Enable"]
pub type DMAINTSEN_R = crate::BitReader;
#[doc = "Field `DMAINTSEN` writer - DMA Interrupt Signal Enable"]
pub type DMAINTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFWRRDYSEN` reader - Buffer Write Ready Signal Enable"]
pub type BUFWRRDYSEN_R = crate::BitReader;
#[doc = "Field `BUFWRRDYSEN` writer - Buffer Write Ready Signal Enable"]
pub type BUFWRRDYSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFRDRDYSEN` reader - Buffer Read Ready Signal Enable"]
pub type BUFRDRDYSEN_R = crate::BitReader;
#[doc = "Field `BUFRDRDYSEN` writer - Buffer Read Ready Signal Enable"]
pub type BUFRDRDYSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDINSSEN` reader - Card Insertion Signal Enable"]
pub type CARDINSSEN_R = crate::BitReader;
#[doc = "Field `CARDINSSEN` writer - Card Insertion Signal Enable"]
pub type CARDINSSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDREMSEN` reader - Card Removal Signal Enable"]
pub type CARDREMSEN_R = crate::BitReader;
#[doc = "Field `CARDREMSEN` writer - Card Removal Signal Enable"]
pub type CARDREMSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDINTSEN` reader - Card Interrupt Signal Enable"]
pub type CARDINTSEN_R = crate::BitReader;
#[doc = "Field `CARDINTSEN` writer - Card Interrupt Signal Enable"]
pub type CARDINTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RETUNINGEVTSEN` reader - Re-Tuning Event Signal Enable"]
pub type RETUNINGEVTSEN_R = crate::BitReader;
#[doc = "Field `RETUNINGEVTSEN` writer - Re-Tuning Event Signal Enable"]
pub type RETUNINGEVTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOTACKRCVSEN` reader - Boot Ack Received Signal Enable"]
pub type BOOTACKRCVSEN_R = crate::BitReader;
#[doc = "Field `BOOTACKRCVSEN` writer - Boot Ack Received Signal Enable"]
pub type BOOTACKRCVSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOTTERMINATESEN` reader - Boot Terminate Interrupt Signal Enable"]
pub type BOOTTERMINATESEN_R = crate::BitReader;
#[doc = "Field `BOOTTERMINATESEN` writer - Boot Terminate Interrupt Signal Enable"]
pub type BOOTTERMINATESEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDTOUTERRSEN` reader - Command Timeout Error Signal Enable"]
pub type CMDTOUTERRSEN_R = crate::BitReader;
#[doc = "Field `CMDTOUTERRSEN` writer - Command Timeout Error Signal Enable"]
pub type CMDTOUTERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDCRCERRSEN` reader - Command CRC Error Signal Enable"]
pub type CMDCRCERRSEN_R = crate::BitReader;
#[doc = "Field `CMDCRCERRSEN` writer - Command CRC Error Signal Enable"]
pub type CMDCRCERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDENDBITERRSEN` reader - Command End Bit Error Signal Enable"]
pub type CMDENDBITERRSEN_R = crate::BitReader;
#[doc = "Field `CMDENDBITERRSEN` writer - Command End Bit Error Signal Enable"]
pub type CMDENDBITERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDINDEXERRSEN` reader - Command Index Error Signal Enable"]
pub type CMDINDEXERRSEN_R = crate::BitReader;
#[doc = "Field `CMDINDEXERRSEN` writer - Command Index Error Signal Enable"]
pub type CMDINDEXERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATTOUTERRSEN` reader - Data Timeout Error Signal Enable"]
pub type DATTOUTERRSEN_R = crate::BitReader;
#[doc = "Field `DATTOUTERRSEN` writer - Data Timeout Error Signal Enable"]
pub type DATTOUTERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATCRCERRSEN` reader - Data CRC Error Signal Enable"]
pub type DATCRCERRSEN_R = crate::BitReader;
#[doc = "Field `DATCRCERRSEN` writer - Data CRC Error Signal Enable"]
pub type DATCRCERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATENDBITERRSEN` reader - Data End Bit Error Signal Enable"]
pub type DATENDBITERRSEN_R = crate::BitReader;
#[doc = "Field `DATENDBITERRSEN` writer - Data End Bit Error Signal Enable"]
pub type DATENDBITERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CURRENTLIMITERRSEN` reader - Current Limit Error Signal Enable"]
pub type CURRENTLIMITERRSEN_R = crate::BitReader;
#[doc = "Field `CURRENTLIMITERRSEN` writer - Current Limit Error Signal Enable"]
pub type CURRENTLIMITERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOCMDERRSEN` reader - Auto CMD12 Error Signal Enable"]
pub type AUTOCMDERRSEN_R = crate::BitReader;
#[doc = "Field `AUTOCMDERRSEN` writer - Auto CMD12 Error Signal Enable"]
pub type AUTOCMDERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADMAERRSEN` reader - ADMA Error Signal Enable"]
pub type ADMAERRSEN_R = crate::BitReader;
#[doc = "Field `ADMAERRSEN` writer - ADMA Error Signal Enable"]
pub type ADMAERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TUNINGERRSIGNALENABLE` reader - Tuning Error Signal Enable"]
pub type TUNINGERRSIGNALENABLE_R = crate::BitReader;
#[doc = "Field `TUNINGERRSIGNALENABLE` writer - Tuning Error Signal Enable"]
pub type TUNINGERRSIGNALENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGETRESPERRSEN` reader - Target Response Error Signal Enable"]
pub type TARGETRESPERRSEN_R = crate::BitReader;
#[doc = "Field `TARGETRESPERRSEN` writer - Target Response Error Signal Enable"]
pub type TARGETRESPERRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomsen(&self) -> CMDCOMSEN_R {
        CMDCOMSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomsen(&self) -> TRANCOMSEN_R {
        TRANCOMSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevtsen(&self) -> BLKGAPEVTSEN_R {
        BLKGAPEVTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaintsen(&self) -> DMAINTSEN_R {
        DMAINTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdysen(&self) -> BUFWRRDYSEN_R {
        BUFWRRDYSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdysen(&self) -> BUFRDRDYSEN_R {
        BUFRDRDYSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinssen(&self) -> CARDINSSEN_R {
        CARDINSSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardremsen(&self) -> CARDREMSEN_R {
        CARDREMSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardintsen(&self) -> CARDINTSEN_R {
        CARDINTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevtsen(&self) -> RETUNINGEVTSEN_R {
        RETUNINGEVTSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcvsen(&self) -> BOOTACKRCVSEN_R {
        BOOTACKRCVSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminatesen(&self) -> BOOTTERMINATESEN_R {
        BOOTTERMINATESEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdtouterrsen(&self) -> CMDTOUTERRSEN_R {
        CMDTOUTERRSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrcerrsen(&self) -> CMDCRCERRSEN_R {
        CMDCRCERRSEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdendbiterrsen(&self) -> CMDENDBITERRSEN_R {
        CMDENDBITERRSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdindexerrsen(&self) -> CMDINDEXERRSEN_R {
        CMDINDEXERRSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn dattouterrsen(&self) -> DATTOUTERRSEN_R {
        DATTOUTERRSEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrcerrsen(&self) -> DATCRCERRSEN_R {
        DATCRCERRSEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datendbiterrsen(&self) -> DATENDBITERRSEN_R {
        DATENDBITERRSEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn currentlimiterrsen(&self) -> CURRENTLIMITERRSEN_R {
        CURRENTLIMITERRSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn autocmderrsen(&self) -> AUTOCMDERRSEN_R {
        AUTOCMDERRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn admaerrsen(&self) -> ADMAERRSEN_R {
        ADMAERRSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Signal Enable"]
    #[inline(always)]
    pub fn tuningerrsignalenable(&self) -> TUNINGERRSIGNALENABLE_R {
        TUNINGERRSIGNALENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn targetresperrsen(&self) -> TARGETRESPERRSEN_R {
        TARGETRESPERRSEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcomsen(&mut self) -> CMDCOMSEN_W<IEN_SPEC, 0> {
        CMDCOMSEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trancomsen(&mut self) -> TRANCOMSEN_W<IEN_SPEC, 1> {
        TRANCOMSEN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkgapevtsen(&mut self) -> BLKGAPEVTSEN_W<IEN_SPEC, 2> {
        BLKGAPEVTSEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaintsen(&mut self) -> DMAINTSEN_W<IEN_SPEC, 3> {
        DMAINTSEN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufwrrdysen(&mut self) -> BUFWRRDYSEN_W<IEN_SPEC, 4> {
        BUFWRRDYSEN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufrdrdysen(&mut self) -> BUFRDRDYSEN_W<IEN_SPEC, 5> {
        BUFRDRDYSEN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardinssen(&mut self) -> CARDINSSEN_W<IEN_SPEC, 6> {
        CARDINSSEN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardremsen(&mut self) -> CARDREMSEN_W<IEN_SPEC, 7> {
        CARDREMSEN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cardintsen(&mut self) -> CARDINTSEN_W<IEN_SPEC, 8> {
        CARDINTSEN_W::new(self)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn retuningevtsen(&mut self) -> RETUNINGEVTSEN_W<IEN_SPEC, 12> {
        RETUNINGEVTSEN_W::new(self)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcvsen(&mut self) -> BOOTACKRCVSEN_W<IEN_SPEC, 13> {
        BOOTACKRCVSEN_W::new(self)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminatesen(&mut self) -> BOOTTERMINATESEN_W<IEN_SPEC, 14> {
        BOOTTERMINATESEN_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtouterrsen(&mut self) -> CMDTOUTERRSEN_W<IEN_SPEC, 16> {
        CMDTOUTERRSEN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerrsen(&mut self) -> CMDCRCERRSEN_W<IEN_SPEC, 17> {
        CMDCRCERRSEN_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterrsen(&mut self) -> CMDENDBITERRSEN_W<IEN_SPEC, 18> {
        CMDENDBITERRSEN_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerrsen(&mut self) -> CMDINDEXERRSEN_W<IEN_SPEC, 19> {
        CMDINDEXERRSEN_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dattouterrsen(&mut self) -> DATTOUTERRSEN_W<IEN_SPEC, 20> {
        DATTOUTERRSEN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datcrcerrsen(&mut self) -> DATCRCERRSEN_W<IEN_SPEC, 21> {
        DATCRCERRSEN_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datendbiterrsen(&mut self) -> DATENDBITERRSEN_W<IEN_SPEC, 22> {
        DATENDBITERRSEN_W::new(self)
    }
    #[doc = "Bit 23 - Current Limit Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterrsen(&mut self) -> CURRENTLIMITERRSEN_W<IEN_SPEC, 23> {
        CURRENTLIMITERRSEN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autocmderrsen(&mut self) -> AUTOCMDERRSEN_W<IEN_SPEC, 24> {
        AUTOCMDERRSEN_W::new(self)
    }
    #[doc = "Bit 25 - ADMA Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn admaerrsen(&mut self) -> ADMAERRSEN_W<IEN_SPEC, 25> {
        ADMAERRSEN_W::new(self)
    }
    #[doc = "Bit 26 - Tuning Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuningerrsignalenable(&mut self) -> TUNINGERRSIGNALENABLE_W<IEN_SPEC, 26> {
        TUNINGERRSIGNALENABLE_W::new(self)
    }
    #[doc = "Bit 28 - Target Response Error Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn targetresperrsen(&mut self) -> TARGETRESPERRSEN_W<IEN_SPEC, 28> {
        TARGETRESPERRSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Normal and Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
