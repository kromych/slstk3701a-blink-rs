#[doc = "Register `RESP0` reader"]
pub type R = crate::R<RESP0_SPEC>;
#[doc = "Field `CMDRESP0` reader - Command Response 0"]
pub type CMDRESP0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 0"]
    #[inline(always)]
    pub fn cmdresp0(&self) -> CMDRESP0_R {
        CMDRESP0_R::new(self.bits)
    }
}
#[doc = "Response0 and Response1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP0_SPEC;
impl crate::RegisterSpec for RESP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp0::R`](R) reader structure"]
impl crate::Readable for RESP0_SPEC {}
#[doc = "`reset()` method sets RESP0 to value 0"]
impl crate::Resettable for RESP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
