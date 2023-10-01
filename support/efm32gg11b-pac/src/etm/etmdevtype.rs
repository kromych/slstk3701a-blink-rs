#[doc = "Register `ETMDEVTYPE` reader"]
pub type R = crate::R<ETMDEVTYPE_SPEC>;
#[doc = "Field `TRACESRC` reader - Trace Source"]
pub type TRACESRC_R = crate::FieldReader;
#[doc = "Field `PROCTRACE` reader - Processor Trace"]
pub type PROCTRACE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Trace Source"]
    #[inline(always)]
    pub fn tracesrc(&self) -> TRACESRC_R {
        TRACESRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Processor Trace"]
    #[inline(always)]
    pub fn proctrace(&self) -> PROCTRACE_R {
        PROCTRACE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmdevtype::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMDEVTYPE_SPEC;
impl crate::RegisterSpec for ETMDEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmdevtype::R`](R) reader structure"]
impl crate::Readable for ETMDEVTYPE_SPEC {}
#[doc = "`reset()` method sets ETMDEVTYPE to value 0x13"]
impl crate::Resettable for ETMDEVTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x13;
}
