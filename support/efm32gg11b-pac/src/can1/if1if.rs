#[doc = "Register `IF1IF` reader"]
pub type R = crate::R<IF1IF_SPEC>;
#[doc = "Field `STATUS` reader - Status Interrupt Flag"]
pub type STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status Interrupt Flag"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if1if::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF1IF_SPEC;
impl crate::RegisterSpec for IF1IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if1if::R`](R) reader structure"]
impl crate::Readable for IF1IF_SPEC {}
#[doc = "`reset()` method sets IF1IF to value 0"]
impl crate::Resettable for IF1IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
