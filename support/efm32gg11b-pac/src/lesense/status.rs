#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `BUFDATAV` reader - Result Data Valid"]
pub type BUFDATAV_R = crate::BitReader;
#[doc = "Field `BUFHALFFULL` reader - Result Buffer Half Full"]
pub type BUFHALFFULL_R = crate::BitReader;
#[doc = "Field `BUFFULL` reader - Result Buffer Full"]
pub type BUFFULL_R = crate::BitReader;
#[doc = "Field `RUNNING` reader - LESENSE Periodic Counter Running"]
pub type RUNNING_R = crate::BitReader;
#[doc = "Field `SCANACTIVE` reader - LESENSE Scan Active"]
pub type SCANACTIVE_R = crate::BitReader;
#[doc = "Field `DACACTIVE` reader - LESENSE VDAC Interface is Active"]
pub type DACACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Result Data Valid"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Buffer Half Full"]
    #[inline(always)]
    pub fn bufhalffull(&self) -> BUFHALFFULL_R {
        BUFHALFFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Buffer Full"]
    #[inline(always)]
    pub fn buffull(&self) -> BUFFULL_R {
        BUFFULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LESENSE Periodic Counter Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LESENSE Scan Active"]
    #[inline(always)]
    pub fn scanactive(&self) -> SCANACTIVE_R {
        SCANACTIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LESENSE VDAC Interface is Active"]
    #[inline(always)]
    pub fn dacactive(&self) -> DACACTIVE_R {
        DACACTIVE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
