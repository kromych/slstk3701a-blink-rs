#[doc = "Register `RAMECCADDR` reader"]
pub type R = crate::R<RAMECCADDR_SPEC>;
#[doc = "Field `RAMECCADDR` reader - RAM ECC Error Address"]
pub type RAMECCADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RAM ECC Error Address"]
    #[inline(always)]
    pub fn rameccaddr(&self) -> RAMECCADDR_R {
        RAMECCADDR_R::new(self.bits)
    }
}
#[doc = "RAM ECC Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rameccaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMECCADDR_SPEC;
impl crate::RegisterSpec for RAMECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rameccaddr::R`](R) reader structure"]
impl crate::Readable for RAMECCADDR_SPEC {}
#[doc = "`reset()` method sets RAMECCADDR to value 0x2000_0000"]
impl crate::Resettable for RAMECCADDR_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
