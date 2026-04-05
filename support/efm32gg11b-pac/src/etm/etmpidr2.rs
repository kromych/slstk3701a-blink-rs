#[doc = "Register `ETMPIDR2` reader"]
pub type R = crate::R<Etmpidr2Spec>;
#[doc = "Field `IDCODE` reader - JEP106 Identity Code"]
pub type IdcodeR = crate::FieldReader;
#[doc = "Field `ALWAYS1` reader - Always 1"]
pub type Always1R = crate::BitReader;
#[doc = "Field `REV` reader - Revision"]
pub type RevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IdcodeR {
        IdcodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Always 1"]
    #[inline(always)]
    pub fn always1(&self) -> Always1R {
        Always1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr2Spec;
impl crate::RegisterSpec for Etmpidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr2::R`](R) reader structure"]
impl crate::Readable for Etmpidr2Spec {}
#[doc = "`reset()` method sets ETMPIDR2 to value 0x0b"]
impl crate::Resettable for Etmpidr2Spec {
    const RESET_VALUE: u32 = 0x0b;
}
