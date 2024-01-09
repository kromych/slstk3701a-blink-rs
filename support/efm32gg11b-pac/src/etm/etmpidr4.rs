#[doc = "Register `ETMPIDR4` reader"]
pub type R = crate::R<ETMPIDR4_SPEC>;
#[doc = "Field `CONTCODE` reader - JEP106 Continuation Code"]
pub type CONTCODE_R = crate::FieldReader;
#[doc = "Field `COUNT` reader - 4KB Count"]
pub type COUNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106 Continuation Code"]
    #[inline(always)]
    pub fn contcode(&self) -> CONTCODE_R {
        CONTCODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPIDR4_SPEC;
impl crate::RegisterSpec for ETMPIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr4::R`](R) reader structure"]
impl crate::Readable for ETMPIDR4_SPEC {}
#[doc = "`reset()` method sets ETMPIDR4 to value 0x04"]
impl crate::Resettable for ETMPIDR4_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
