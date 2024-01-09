#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `ACMPACT` reader - Analog Comparator Active"]
pub type ACMPACT_R = crate::BitReader;
#[doc = "Field `ACMPOUT` reader - Analog Comparator Output"]
pub type ACMPOUT_R = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Output"]
pub type APORTCONFLICT_R = crate::BitReader;
#[doc = "Field `EXTIFACT` reader - External Override Interface Active"]
pub type EXTIFACT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog Comparator Active"]
    #[inline(always)]
    pub fn acmpact(&self) -> ACMPACT_R {
        ACMPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> ACMPOUT_R {
        ACMPOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APORT Conflict Output"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Override Interface Active"]
    #[inline(always)]
    pub fn extifact(&self) -> EXTIFACT_R {
        EXTIFACT_R::new(((self.bits >> 3) & 1) != 0)
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
