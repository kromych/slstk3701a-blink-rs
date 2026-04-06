#[doc = "Register `ETMLSR` reader"]
pub type R = crate::R<EtmlsrSpec>;
#[doc = "Field `LOCKIMP` reader - ETM Locking Implemented"]
pub type LockimpR = crate::BitReader;
#[doc = "Field `LOCKED` reader - ETM locked"]
pub type LockedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ETM Locking Implemented"]
    #[inline(always)]
    pub fn lockimp(&self) -> LockimpR {
        LockimpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM locked"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmlsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmlsrSpec;
impl crate::RegisterSpec for EtmlsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmlsr::R`](R) reader structure"]
impl crate::Readable for EtmlsrSpec {}
#[doc = "`reset()` method sets ETMLSR to value 0x03"]
impl crate::Resettable for EtmlsrSpec {
    const RESET_VALUE: u32 = 0x03;
}
