#[doc = "Register `PL_DIN` reader"]
pub type R = crate::R<PlDinSpec>;
#[doc = "Field `DIN` reader - Data in"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlDinSpec;
impl crate::RegisterSpec for PlDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pl_din::R`](R) reader structure"]
impl crate::Readable for PlDinSpec {}
#[doc = "`reset()` method sets PL_DIN to value 0"]
impl crate::Resettable for PlDinSpec {}
