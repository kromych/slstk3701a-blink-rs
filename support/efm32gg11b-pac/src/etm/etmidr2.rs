#[doc = "Register `ETMIDR2` reader"]
pub type R = crate::R<ETMIDR2_SPEC>;
#[doc = "Field `RFE` reader - RFE Transfer Order"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `SWP` reader - SWP Transfer Order"]
pub type SWP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RFE Transfer Order"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWP Transfer Order"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ETM ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMIDR2_SPEC;
impl crate::RegisterSpec for ETMIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmidr2::R`](R) reader structure"]
impl crate::Readable for ETMIDR2_SPEC {}
#[doc = "`reset()` method sets ETMIDR2 to value 0"]
impl crate::Resettable for ETMIDR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
