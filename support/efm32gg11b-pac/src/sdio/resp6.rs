#[doc = "Register `RESP6` reader"]
pub type R = crate::R<RESP6_SPEC>;
#[doc = "Field `CMDRESP3` reader - Command Response 3"]
pub type CMDRESP3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdresp3(&self) -> CMDRESP3_R {
        CMDRESP3_R::new(self.bits)
    }
}
#[doc = "Response6 and Response7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP6_SPEC;
impl crate::RegisterSpec for RESP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp6::R`](R) reader structure"]
impl crate::Readable for RESP6_SPEC {}
#[doc = "`reset()` method sets RESP6 to value 0"]
impl crate::Resettable for RESP6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
