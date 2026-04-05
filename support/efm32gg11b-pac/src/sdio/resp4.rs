#[doc = "Register `RESP4` reader"]
pub type R = crate::R<Resp4Spec>;
#[doc = "Field `CMDRESP2` reader - Command Response 2"]
pub type Cmdresp2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdresp2(&self) -> Cmdresp2R {
        Cmdresp2R::new(self.bits)
    }
}
#[doc = "Response4 and Response5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp4Spec;
impl crate::RegisterSpec for Resp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4::R`](R) reader structure"]
impl crate::Readable for Resp4Spec {}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for Resp4Spec {}
