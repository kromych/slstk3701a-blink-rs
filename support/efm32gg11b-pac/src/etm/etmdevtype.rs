#[doc = "Register `ETMDEVTYPE` reader"]
pub type R = crate::R<EtmdevtypeSpec>;
#[doc = "Field `TRACESRC` reader - Trace Source"]
pub type TracesrcR = crate::FieldReader;
#[doc = "Field `PROCTRACE` reader - Processor Trace"]
pub type ProctraceR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Trace Source"]
    #[inline(always)]
    pub fn tracesrc(&self) -> TracesrcR {
        TracesrcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Processor Trace"]
    #[inline(always)]
    pub fn proctrace(&self) -> ProctraceR {
        ProctraceR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Device Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmdevtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmdevtypeSpec;
impl crate::RegisterSpec for EtmdevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmdevtype::R`](R) reader structure"]
impl crate::Readable for EtmdevtypeSpec {}
#[doc = "`reset()` method sets ETMDEVTYPE to value 0x13"]
impl crate::Resettable for EtmdevtypeSpec {
    const RESET_VALUE: u32 = 0x13;
}
