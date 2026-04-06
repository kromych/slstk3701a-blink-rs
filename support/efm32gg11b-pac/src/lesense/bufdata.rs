#[doc = "Register `BUFDATA` reader"]
pub type R = crate::R<BufdataSpec>;
#[doc = "Field `BUFDATA` reader - Result Data"]
pub type BufdataR = crate::FieldReader<u16>;
#[doc = "Field `BUFDATASRC` reader - Result Data Source"]
pub type BufdatasrcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Result Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BufdataR {
        BufdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn bufdatasrc(&self) -> BufdatasrcR {
        BufdatasrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Result Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct BufdataSpec;
impl crate::RegisterSpec for BufdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufdata::R`](R) reader structure"]
impl crate::Readable for BufdataSpec {}
#[doc = "`reset()` method sets BUFDATA to value 0"]
impl crate::Resettable for BufdataSpec {}
