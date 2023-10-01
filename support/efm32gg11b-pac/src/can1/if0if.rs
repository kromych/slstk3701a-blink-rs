#[doc = "Register `IF0IF` reader"]
pub type R = crate::R<IF0IF_SPEC>;
#[doc = "Field `MESSAGE` reader - Message Object Interrupt Flag"]
pub type MESSAGE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Message Object Interrupt Flag"]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
#[doc = "Message Object Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if0if::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF0IF_SPEC;
impl crate::RegisterSpec for IF0IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if0if::R`](R) reader structure"]
impl crate::Readable for IF0IF_SPEC {}
#[doc = "`reset()` method sets IF0IF to value 0"]
impl crate::Resettable for IF0IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}