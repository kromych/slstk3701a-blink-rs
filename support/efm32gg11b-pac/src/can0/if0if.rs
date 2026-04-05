#[doc = "Register `IF0IF` reader"]
pub type R = crate::R<If0ifSpec>;
#[doc = "Field `MESSAGE` reader - Message Object Interrupt Flag"]
pub type MessageR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Message Object Interrupt Flag"]
    #[inline(always)]
    pub fn message(&self) -> MessageR {
        MessageR::new(self.bits)
    }
}
#[doc = "Message Object Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if0if::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If0ifSpec;
impl crate::RegisterSpec for If0ifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if0if::R`](R) reader structure"]
impl crate::Readable for If0ifSpec {}
#[doc = "`reset()` method sets IF0IF to value 0"]
impl crate::Resettable for If0ifSpec {}
