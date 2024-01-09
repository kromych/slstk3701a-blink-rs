#[doc = "Register `ETMLSR` reader"]
pub type R = crate::R<ETMLSR_SPEC>;
#[doc = "Field `LOCKIMP` reader - ETM Locking Implemented"]
pub type LOCKIMP_R = crate::BitReader;
#[doc = "Field `LOCKED` reader - ETM locked"]
pub type LOCKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ETM Locking Implemented"]
    #[inline(always)]
    pub fn lockimp(&self) -> LOCKIMP_R {
        LOCKIMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmlsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMLSR_SPEC;
impl crate::RegisterSpec for ETMLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmlsr::R`](R) reader structure"]
impl crate::Readable for ETMLSR_SPEC {}
#[doc = "`reset()` method sets ETMLSR to value 0x03"]
impl crate::Resettable for ETMLSR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
