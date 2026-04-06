#[doc = "Register `ETMPIDR1` reader"]
pub type R = crate::R<Etmpidr1Spec>;
#[doc = "Field `PARTNUM` reader - Part Number"]
pub type PartnumR = crate::FieldReader;
#[doc = "Field `IDCODE` reader - JEP106 Identity Code"]
pub type IdcodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Part Number"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IdcodeR {
        IdcodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr1Spec;
impl crate::RegisterSpec for Etmpidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr1::R`](R) reader structure"]
impl crate::Readable for Etmpidr1Spec {}
#[doc = "`reset()` method sets ETMPIDR1 to value 0xb9"]
impl crate::Resettable for Etmpidr1Spec {
    const RESET_VALUE: u32 = 0xb9;
}
