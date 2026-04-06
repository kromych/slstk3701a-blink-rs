#[doc = "Register `INTID` reader"]
pub type R = crate::R<IntidSpec>;
#[doc = "Field `INTID` reader - Interrupt Identifier"]
pub type IntidR = crate::FieldReader;
#[doc = "Field `INTSTAT` reader - Status Interupt"]
pub type IntstatR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Interrupt Identifier"]
    #[inline(always)]
    pub fn intid(&self) -> IntidR {
        IntidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Status Interupt"]
    #[inline(always)]
    pub fn intstat(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntidSpec;
impl crate::RegisterSpec for IntidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intid::R`](R) reader structure"]
impl crate::Readable for IntidSpec {}
#[doc = "`reset()` method sets INTID to value 0"]
impl crate::Resettable for IntidSpec {}
