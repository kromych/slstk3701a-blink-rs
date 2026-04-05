#[doc = "Register `ETMCCER` reader"]
pub type R = crate::R<EtmccerSpec>;
#[doc = "Field `EXTINPSEL` reader - Extended External Input Selectors"]
pub type ExtinpselR = crate::FieldReader;
#[doc = "Field `EXTINPBUS` reader - Extended External Input Bus"]
pub type ExtinpbusR = crate::FieldReader;
#[doc = "Field `READREGS` reader - Readable Registers"]
pub type ReadregsR = crate::BitReader;
#[doc = "Field `DADDRCMP` reader - Data Address comparisons"]
pub type DaddrcmpR = crate::BitReader;
#[doc = "Field `INSTRES` reader - Instrumentation Resources"]
pub type InstresR = crate::FieldReader;
#[doc = "Field `EICEWPNT` reader - EmbeddedICE watchpoint inputs"]
pub type EicewpntR = crate::FieldReader;
#[doc = "Field `TEICEWPNT` reader - Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs"]
pub type TeicewpntR = crate::BitReader;
#[doc = "Field `EICEIMP` reader - EmbeddedICE Behavior control Implemented"]
pub type EiceimpR = crate::BitReader;
#[doc = "Field `TIMP` reader - Timestamping Implemented"]
pub type TimpR = crate::BitReader;
#[doc = "Field `RFCNT` reader - Reduced Function Counter"]
pub type RfcntR = crate::BitReader;
#[doc = "Field `TENC` reader - Timestamp Encoding"]
pub type TencR = crate::BitReader;
#[doc = "Field `TSIZE` reader - Timestamp Size"]
pub type TsizeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Extended External Input Selectors"]
    #[inline(always)]
    pub fn extinpsel(&self) -> ExtinpselR {
        ExtinpselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:10 - Extended External Input Bus"]
    #[inline(always)]
    pub fn extinpbus(&self) -> ExtinpbusR {
        ExtinpbusR::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Readable Registers"]
    #[inline(always)]
    pub fn readregs(&self) -> ReadregsR {
        ReadregsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Address comparisons"]
    #[inline(always)]
    pub fn daddrcmp(&self) -> DaddrcmpR {
        DaddrcmpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Instrumentation Resources"]
    #[inline(always)]
    pub fn instres(&self) -> InstresR {
        InstresR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - EmbeddedICE watchpoint inputs"]
    #[inline(always)]
    pub fn eicewpnt(&self) -> EicewpntR {
        EicewpntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs"]
    #[inline(always)]
    pub fn teicewpnt(&self) -> TeicewpntR {
        TeicewpntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EmbeddedICE Behavior control Implemented"]
    #[inline(always)]
    pub fn eiceimp(&self) -> EiceimpR {
        EiceimpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timestamping Implemented"]
    #[inline(always)]
    pub fn timp(&self) -> TimpR {
        TimpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - Reduced Function Counter"]
    #[inline(always)]
    pub fn rfcnt(&self) -> RfcntR {
        RfcntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timestamp Encoding"]
    #[inline(always)]
    pub fn tenc(&self) -> TencR {
        TencR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Size"]
    #[inline(always)]
    pub fn tsize(&self) -> TsizeR {
        TsizeR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Configuration Code Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmccer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmccerSpec;
impl crate::RegisterSpec for EtmccerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmccer::R`](R) reader structure"]
impl crate::Readable for EtmccerSpec {}
#[doc = "`reset()` method sets ETMCCER to value 0x1854_1800"]
impl crate::Resettable for EtmccerSpec {
    const RESET_VALUE: u32 = 0x1854_1800;
}
