#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HfnumSpec>;
#[doc = "Field `FRNUM` reader - Frame Number"]
pub type FrnumR = crate::FieldReader<u16>;
#[doc = "Field `FRREM` reader - Frame Time Remaining"]
pub type FrremR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        FrnumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn frrem(&self) -> FrremR {
        FrremR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfnumSpec;
impl crate::RegisterSpec for HfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HfnumSpec {}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HfnumSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
