#[doc = "Register `SCANDATAXP` reader"]
pub type R = crate::R<SCANDATAXP_SPEC>;
#[doc = "Field `DATAP` reader - Scan Conversion Result Data Peek"]
pub type DATAP_R = crate::FieldReader<u16>;
#[doc = "Field `SCANINPUTIDPEEK` reader - Scan Conversion Data Source Peek"]
pub type SCANINPUTIDPEEK_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Data Source Peek"]
    #[inline(always)]
    pub fn scaninputidpeek(&self) -> SCANINPUTIDPEEK_R {
        SCANINPUTIDPEEK_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Scan Sequence Result Data + Data Source Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scandataxp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANDATAXP_SPEC;
impl crate::RegisterSpec for SCANDATAXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandataxp::R`](R) reader structure"]
impl crate::Readable for SCANDATAXP_SPEC {}
#[doc = "`reset()` method sets SCANDATAXP to value 0"]
impl crate::Resettable for SCANDATAXP_SPEC {
    const RESET_VALUE: u32 = 0;
}
