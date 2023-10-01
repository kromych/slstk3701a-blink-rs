#[doc = "Register `PH_DIN` reader"]
pub type R = crate::R<PH_DIN_SPEC>;
#[doc = "Field `DIN` reader - Data in"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PH_DIN_SPEC;
impl crate::RegisterSpec for PH_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ph_din::R`](R) reader structure"]
impl crate::Readable for PH_DIN_SPEC {}
#[doc = "`reset()` method sets PH_DIN to value 0"]
impl crate::Resettable for PH_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
