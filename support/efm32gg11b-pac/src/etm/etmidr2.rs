#[doc = "Register `ETMIDR2` reader"]
pub type R = crate::R<Etmidr2Spec>;
#[doc = "Field `RFE` reader - RFE Transfer Order"]
pub type RfeR = crate::BitReader;
#[doc = "Field `SWP` reader - SWP Transfer Order"]
pub type SwpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RFE Transfer Order"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWP Transfer Order"]
    #[inline(always)]
    pub fn swp(&self) -> SwpR {
        SwpR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ETM ID Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`etmidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmidr2Spec;
impl crate::RegisterSpec for Etmidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmidr2::R`](R) reader structure"]
impl crate::Readable for Etmidr2Spec {}
#[doc = "`reset()` method sets ETMIDR2 to value 0"]
impl crate::Resettable for Etmidr2Spec {}
