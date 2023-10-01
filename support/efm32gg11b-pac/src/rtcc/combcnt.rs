#[doc = "Register `COMBCNT` reader"]
pub type R = crate::R<COMBCNT_SPEC>;
#[doc = "Field `PRECNT` reader - Pre-Counter Value"]
pub type PRECNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNTLSB` reader - Counter Value"]
pub type CNTLSB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:31 - Counter Value"]
    #[inline(always)]
    pub fn cntlsb(&self) -> CNTLSB_R {
        CNTLSB_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
#[doc = "Combined Pre-Counter and Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMBCNT_SPEC;
impl crate::RegisterSpec for COMBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`combcnt::R`](R) reader structure"]
impl crate::Readable for COMBCNT_SPEC {}
#[doc = "`reset()` method sets COMBCNT to value 0"]
impl crate::Resettable for COMBCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
