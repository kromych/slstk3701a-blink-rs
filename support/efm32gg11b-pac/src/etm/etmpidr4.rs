#[doc = "Register `ETMPIDR4` reader"]
pub type R = crate::R<Etmpidr4Spec>;
#[doc = "Field `CONTCODE` reader - JEP106 Continuation Code"]
pub type ContcodeR = crate::FieldReader;
#[doc = "Field `COUNT` reader - 4KB Count"]
pub type CountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106 Continuation Code"]
    #[inline(always)]
    pub fn contcode(&self) -> ContcodeR {
        ContcodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr4Spec;
impl crate::RegisterSpec for Etmpidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr4::R`](R) reader structure"]
impl crate::Readable for Etmpidr4Spec {}
#[doc = "`reset()` method sets ETMPIDR4 to value 0x04"]
impl crate::Resettable for Etmpidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
