#[doc = "Register `PK_DIN` reader"]
pub type R = crate::R<PkDinSpec>;
#[doc = "Field `DIN` reader - Data in"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pk_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkDinSpec;
impl crate::RegisterSpec for PkDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_din::R`](R) reader structure"]
impl crate::Readable for PkDinSpec {}
#[doc = "`reset()` method sets PK_DIN to value 0"]
impl crate::Resettable for PkDinSpec {}
