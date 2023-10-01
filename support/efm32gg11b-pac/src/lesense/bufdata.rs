#[doc = "Register `BUFDATA` reader"]
pub type R = crate::R<BUFDATA_SPEC>;
#[doc = "Field `BUFDATA` reader - Result Data"]
pub type BUFDATA_R = crate::FieldReader<u16>;
#[doc = "Field `BUFDATASRC` reader - Result Data Source"]
pub type BUFDATASRC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Result Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn bufdatasrc(&self) -> BUFDATASRC_R {
        BUFDATASRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Result Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufdata::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFDATA_SPEC;
impl crate::RegisterSpec for BUFDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufdata::R`](R) reader structure"]
impl crate::Readable for BUFDATA_SPEC {}
#[doc = "`reset()` method sets BUFDATA to value 0"]
impl crate::Resettable for BUFDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
