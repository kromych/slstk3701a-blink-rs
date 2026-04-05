#[doc = "Register `RESP6` reader"]
pub type R = crate::R<Resp6Spec>;
#[doc = "Field `CMDRESP3` reader - Command Response 3"]
pub type Cmdresp3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdresp3(&self) -> Cmdresp3R {
        Cmdresp3R::new(self.bits)
    }
}
#[doc = "Response6 and Response7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp6Spec;
impl crate::RegisterSpec for Resp6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp6::R`](R) reader structure"]
impl crate::Readable for Resp6Spec {}
#[doc = "`reset()` method sets RESP6 to value 0"]
impl crate::Resettable for Resp6Spec {}
