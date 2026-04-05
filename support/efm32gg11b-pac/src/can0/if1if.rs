#[doc = "Register `IF1IF` reader"]
pub type R = crate::R<If1ifSpec>;
#[doc = "Field `STATUS` reader - Status Interrupt Flag"]
pub type StatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status Interrupt Flag"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if1if::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If1ifSpec;
impl crate::RegisterSpec for If1ifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if1if::R`](R) reader structure"]
impl crate::Readable for If1ifSpec {}
#[doc = "`reset()` method sets IF1IF to value 0"]
impl crate::Resettable for If1ifSpec {}
