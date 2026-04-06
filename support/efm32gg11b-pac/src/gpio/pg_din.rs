#[doc = "Register `PG_DIN` reader"]
pub type R = crate::R<PgDinSpec>;
#[doc = "Field `DIN` reader - Data in"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgDinSpec;
impl crate::RegisterSpec for PgDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg_din::R`](R) reader structure"]
impl crate::Readable for PgDinSpec {}
#[doc = "`reset()` method sets PG_DIN to value 0"]
impl crate::Resettable for PgDinSpec {}
