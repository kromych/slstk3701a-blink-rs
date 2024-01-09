#[doc = "Register `ETMPIDR3` reader"]
pub type R = crate::R<ETMPIDR3_SPEC>;
#[doc = "Field `CUSTMOD` reader - Customer Modified"]
pub type CUSTMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - RevAnd"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer Modified"]
    #[inline(always)]
    pub fn custmod(&self) -> CUSTMOD_R {
        CUSTMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPIDR3_SPEC;
impl crate::RegisterSpec for ETMPIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr3::R`](R) reader structure"]
impl crate::Readable for ETMPIDR3_SPEC {}
#[doc = "`reset()` method sets ETMPIDR3 to value 0"]
impl crate::Resettable for ETMPIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
