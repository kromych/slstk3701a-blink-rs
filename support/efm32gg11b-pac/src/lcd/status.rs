#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `ASTATE` reader - Current Animation State"]
pub type ASTATE_R = crate::FieldReader;
#[doc = "Field `BLINK` reader - Blink State"]
pub type BLINK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Current Animation State"]
    #[inline(always)]
    pub fn astate(&self) -> ASTATE_R {
        ASTATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Blink State"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
