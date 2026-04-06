#[doc = "Register `RESP0` reader"]
pub type R = crate::R<Resp0Spec>;
#[doc = "Field `CMDRESP0` reader - Command Response 0"]
pub type Cmdresp0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 0"]
    #[inline(always)]
    pub fn cmdresp0(&self) -> Cmdresp0R {
        Cmdresp0R::new(self.bits)
    }
}
#[doc = "Response0 and Response1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp0Spec;
impl crate::RegisterSpec for Resp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp0::R`](R) reader structure"]
impl crate::Readable for Resp0Spec {}
#[doc = "`reset()` method sets RESP0 to value 0"]
impl crate::Resettable for Resp0Spec {}
