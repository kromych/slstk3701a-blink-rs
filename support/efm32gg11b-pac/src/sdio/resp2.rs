#[doc = "Register `RESP2` reader"]
pub type R = crate::R<Resp2Spec>;
#[doc = "Field `CMDRESP1` reader - Command Response 1"]
pub type Cmdresp1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 1"]
    #[inline(always)]
    pub fn cmdresp1(&self) -> Cmdresp1R {
        Cmdresp1R::new(self.bits)
    }
}
#[doc = "Response2 and Response3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp2Spec;
impl crate::RegisterSpec for Resp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2::R`](R) reader structure"]
impl crate::Readable for Resp2Spec {}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for Resp2Spec {}
