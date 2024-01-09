#[doc = "Register `ETMPIDR1` reader"]
pub type R = crate::R<ETMPIDR1_SPEC>;
#[doc = "Field `PARTNUM` reader - Part Number"]
pub type PARTNUM_R = crate::FieldReader;
#[doc = "Field `IDCODE` reader - JEP106 Identity Code"]
pub type IDCODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Part Number"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPIDR1_SPEC;
impl crate::RegisterSpec for ETMPIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr1::R`](R) reader structure"]
impl crate::Readable for ETMPIDR1_SPEC {}
#[doc = "`reset()` method sets ETMPIDR1 to value 0xb9"]
impl crate::Resettable for ETMPIDR1_SPEC {
    const RESET_VALUE: u32 = 0xb9;
}
