#[doc = "Register `PTR` reader"]
pub type R = crate::R<PtrSpec>;
#[doc = "Field `RD` reader - Result Buffer Read Pointer"]
pub type RdR = crate::FieldReader;
#[doc = "Field `WR` reader - Result Buffer Write Pointer"]
pub type WrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Result Buffer Read Pointer"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Result Buffer Write Pointer"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Result Buffer Pointers\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtrSpec;
impl crate::RegisterSpec for PtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr::R`](R) reader structure"]
impl crate::Readable for PtrSpec {}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PtrSpec {}
