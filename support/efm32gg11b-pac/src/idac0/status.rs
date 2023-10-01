#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `CURSTABLE` reader - IDAC Output Current Stable"]
pub type CURSTABLE_R = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Output"]
pub type APORTCONFLICT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IDAC Output Current Stable"]
    #[inline(always)]
    pub fn curstable(&self) -> CURSTABLE_R {
        CURSTABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APORT Conflict Output"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 1) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
