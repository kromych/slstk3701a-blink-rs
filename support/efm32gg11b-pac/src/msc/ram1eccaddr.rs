#[doc = "Register `RAM1ECCADDR` reader"]
pub type R = crate::R<RAM1ECCADDR_SPEC>;
#[doc = "Field `RAM1ECCADDR` reader - RAM1 ECC Error Address"]
pub type RAM1ECCADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RAM1 ECC Error Address"]
    #[inline(always)]
    pub fn ram1eccaddr(&self) -> RAM1ECCADDR_R {
        RAM1ECCADDR_R::new(self.bits)
    }
}
#[doc = "RAM1 ECC Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1eccaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM1ECCADDR_SPEC;
impl crate::RegisterSpec for RAM1ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1eccaddr::R`](R) reader structure"]
impl crate::Readable for RAM1ECCADDR_SPEC {}
#[doc = "`reset()` method sets RAM1ECCADDR to value 0"]
impl crate::Resettable for RAM1ECCADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
