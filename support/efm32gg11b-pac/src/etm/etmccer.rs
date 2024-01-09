#[doc = "Register `ETMCCER` reader"]
pub type R = crate::R<ETMCCER_SPEC>;
#[doc = "Field `EXTINPSEL` reader - Extended External Input Selectors"]
pub type EXTINPSEL_R = crate::FieldReader;
#[doc = "Field `EXTINPBUS` reader - Extended External Input Bus"]
pub type EXTINPBUS_R = crate::FieldReader;
#[doc = "Field `READREGS` reader - Readable Registers"]
pub type READREGS_R = crate::BitReader;
#[doc = "Field `DADDRCMP` reader - Data Address comparisons"]
pub type DADDRCMP_R = crate::BitReader;
#[doc = "Field `INSTRES` reader - Instrumentation Resources"]
pub type INSTRES_R = crate::FieldReader;
#[doc = "Field `EICEWPNT` reader - EmbeddedICE watchpoint inputs"]
pub type EICEWPNT_R = crate::FieldReader;
#[doc = "Field `TEICEWPNT` reader - Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs"]
pub type TEICEWPNT_R = crate::BitReader;
#[doc = "Field `EICEIMP` reader - EmbeddedICE Behavior control Implemented"]
pub type EICEIMP_R = crate::BitReader;
#[doc = "Field `TIMP` reader - Timestamping Implemented"]
pub type TIMP_R = crate::BitReader;
#[doc = "Field `RFCNT` reader - Reduced Function Counter"]
pub type RFCNT_R = crate::BitReader;
#[doc = "Field `TENC` reader - Timestamp Encoding"]
pub type TENC_R = crate::BitReader;
#[doc = "Field `TSIZE` reader - Timestamp Size"]
pub type TSIZE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Extended External Input Selectors"]
    #[inline(always)]
    pub fn extinpsel(&self) -> EXTINPSEL_R {
        EXTINPSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:10 - Extended External Input Bus"]
    #[inline(always)]
    pub fn extinpbus(&self) -> EXTINPBUS_R {
        EXTINPBUS_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Readable Registers"]
    #[inline(always)]
    pub fn readregs(&self) -> READREGS_R {
        READREGS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Address comparisons"]
    #[inline(always)]
    pub fn daddrcmp(&self) -> DADDRCMP_R {
        DADDRCMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Instrumentation Resources"]
    #[inline(always)]
    pub fn instres(&self) -> INSTRES_R {
        INSTRES_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - EmbeddedICE watchpoint inputs"]
    #[inline(always)]
    pub fn eicewpnt(&self) -> EICEWPNT_R {
        EICEWPNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs"]
    #[inline(always)]
    pub fn teicewpnt(&self) -> TEICEWPNT_R {
        TEICEWPNT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EmbeddedICE Behavior control Implemented"]
    #[inline(always)]
    pub fn eiceimp(&self) -> EICEIMP_R {
        EICEIMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timestamping Implemented"]
    #[inline(always)]
    pub fn timp(&self) -> TIMP_R {
        TIMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - Reduced Function Counter"]
    #[inline(always)]
    pub fn rfcnt(&self) -> RFCNT_R {
        RFCNT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timestamp Encoding"]
    #[inline(always)]
    pub fn tenc(&self) -> TENC_R {
        TENC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Size"]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Configuration Code Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmccer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCCER_SPEC;
impl crate::RegisterSpec for ETMCCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmccer::R`](R) reader structure"]
impl crate::Readable for ETMCCER_SPEC {}
#[doc = "`reset()` method sets ETMCCER to value 0x1854_1800"]
impl crate::Resettable for ETMCCER_SPEC {
    const RESET_VALUE: u32 = 0x1854_1800;
}
