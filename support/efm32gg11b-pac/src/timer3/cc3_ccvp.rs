#[doc = "Register `CC3_CCVP` reader"]
pub type R = crate::R<CC3_CCVP_SPEC>;
#[doc = "Field `CCVP` reader - CC Channel Value Peek"]
pub type CCVP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new(self.bits)
    }
}
#[doc = "CC Channel Value Peek Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc3_ccvp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC3_CCVP_SPEC;
impl crate::RegisterSpec for CC3_CCVP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc3_ccvp::R`](R) reader structure"]
impl crate::Readable for CC3_CCVP_SPEC {}
#[doc = "`reset()` method sets CC3_CCVP to value 0"]
impl crate::Resettable for CC3_CCVP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
