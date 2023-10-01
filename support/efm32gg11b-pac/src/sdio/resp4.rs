#[doc = "Register `RESP4` reader"]
pub type R = crate::R<RESP4_SPEC>;
#[doc = "Field `CMDRESP2` reader - Command Response 2"]
pub type CMDRESP2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdresp2(&self) -> CMDRESP2_R {
        CMDRESP2_R::new(self.bits)
    }
}
#[doc = "Response4 and Response5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP4_SPEC;
impl crate::RegisterSpec for RESP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4::R`](R) reader structure"]
impl crate::Readable for RESP4_SPEC {}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for RESP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
