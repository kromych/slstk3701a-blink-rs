#[doc = "Register `INTID` reader"]
pub type R = crate::R<INTID_SPEC>;
#[doc = "Field `INTID` reader - Interrupt Identifier"]
pub type INTID_R = crate::FieldReader;
#[doc = "Field `INTSTAT` reader - Status Interupt"]
pub type INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Interrupt Identifier"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Status Interupt"]
    #[inline(always)]
    pub fn intstat(&self) -> INTSTAT_R {
        INTSTAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTID_SPEC;
impl crate::RegisterSpec for INTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intid::R`](R) reader structure"]
impl crate::Readable for INTID_SPEC {}
#[doc = "`reset()` method sets INTID to value 0"]
impl crate::Resettable for INTID_SPEC {
    const RESET_VALUE: u32 = 0;
}
