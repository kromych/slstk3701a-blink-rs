#[doc = "Register `PH_DIN` reader"]
pub type R = crate::R<PhDinSpec>;
#[doc = "Field `DIN` reader - Data in"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ph_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhDinSpec;
impl crate::RegisterSpec for PhDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ph_din::R`](R) reader structure"]
impl crate::Readable for PhDinSpec {}
#[doc = "`reset()` method sets PH_DIN to value 0"]
impl crate::Resettable for PhDinSpec {}
