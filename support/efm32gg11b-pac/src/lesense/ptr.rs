#[doc = "Register `PTR` reader"]
pub type R = crate::R<PTR_SPEC>;
#[doc = "Field `RD` reader - Result Buffer Read Pointer"]
pub type RD_R = crate::FieldReader;
#[doc = "Field `WR` reader - Result Buffer Write Pointer"]
pub type WR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Result Buffer Read Pointer"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Result Buffer Write Pointer"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Result Buffer Pointers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTR_SPEC;
impl crate::RegisterSpec for PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr::R`](R) reader structure"]
impl crate::Readable for PTR_SPEC {}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
