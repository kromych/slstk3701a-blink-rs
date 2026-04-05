#[doc = "Register `SCANDATAX` reader"]
pub type R = crate::R<ScandataxSpec>;
#[doc = "Field `DATA` reader - Scan Conversion Result Data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `SCANINPUTID` reader - Scan Conversion Input ID"]
pub type ScaninputidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Input ID"]
    #[inline(always)]
    pub fn scaninputid(&self) -> ScaninputidR {
        ScaninputidR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Scan Sequence Result Data + Data Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scandatax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct ScandataxSpec;
impl crate::RegisterSpec for ScandataxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandatax::R`](R) reader structure"]
impl crate::Readable for ScandataxSpec {}
#[doc = "`reset()` method sets SCANDATAX to value 0"]
impl crate::Resettable for ScandataxSpec {}
