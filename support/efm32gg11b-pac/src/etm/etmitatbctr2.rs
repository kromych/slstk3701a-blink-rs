#[doc = "Register `ETMITATBCTR2` reader"]
pub type R = crate::R<ETMITATBCTR2_SPEC>;
#[doc = "Field `ATREADY` reader - ATREADY Input Value"]
pub type ATREADY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ATREADY Input Value"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ETM Integration Test ATB Control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmitatbctr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMITATBCTR2_SPEC;
impl crate::RegisterSpec for ETMITATBCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmitatbctr2::R`](R) reader structure"]
impl crate::Readable for ETMITATBCTR2_SPEC {}
#[doc = "`reset()` method sets ETMITATBCTR2 to value 0x01"]
impl crate::Resettable for ETMITATBCTR2_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
