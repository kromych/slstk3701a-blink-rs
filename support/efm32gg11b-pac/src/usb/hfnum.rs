#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HFNUM_SPEC>;
#[doc = "Field `FRNUM` reader - Frame Number"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FRREM` reader - Frame Time Remaining"]
pub type FRREM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn frrem(&self) -> FRREM_R {
        FRREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HFNUM_SPEC {}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUM_SPEC {
    const RESET_VALUE: u32 = 0x3fff;
}
