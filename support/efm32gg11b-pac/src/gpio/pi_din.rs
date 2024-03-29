#[doc = "Register `PI_DIN` reader"]
pub type R = crate::R<PI_DIN_SPEC>;
#[doc = "Field `DIN` reader - Data in"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PI_DIN_SPEC;
impl crate::RegisterSpec for PI_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_din::R`](R) reader structure"]
impl crate::Readable for PI_DIN_SPEC {}
#[doc = "`reset()` method sets PI_DIN to value 0"]
impl crate::Resettable for PI_DIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
