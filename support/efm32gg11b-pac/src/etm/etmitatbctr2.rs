#[doc = "Register `ETMITATBCTR2` reader"]
pub type R = crate::R<Etmitatbctr2Spec>;
#[doc = "Field `ATREADY` reader - ATREADY Input Value"]
pub type AtreadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ATREADY Input Value"]
    #[inline(always)]
    pub fn atready(&self) -> AtreadyR {
        AtreadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "ETM Integration Test ATB Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmitatbctr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmitatbctr2Spec;
impl crate::RegisterSpec for Etmitatbctr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmitatbctr2::R`](R) reader structure"]
impl crate::Readable for Etmitatbctr2Spec {}
#[doc = "`reset()` method sets ETMITATBCTR2 to value 0x01"]
impl crate::Resettable for Etmitatbctr2Spec {
    const RESET_VALUE: u32 = 0x01;
}
