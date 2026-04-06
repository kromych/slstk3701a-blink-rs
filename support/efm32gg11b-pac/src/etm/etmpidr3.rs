#[doc = "Register `ETMPIDR3` reader"]
pub type R = crate::R<Etmpidr3Spec>;
#[doc = "Field `CUSTMOD` reader - Customer Modified"]
pub type CustmodR = crate::FieldReader;
#[doc = "Field `REVAND` reader - RevAnd"]
pub type RevandR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer Modified"]
    #[inline(always)]
    pub fn custmod(&self) -> CustmodR {
        CustmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmpidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr3Spec;
impl crate::RegisterSpec for Etmpidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr3::R`](R) reader structure"]
impl crate::Readable for Etmpidr3Spec {}
#[doc = "`reset()` method sets ETMPIDR3 to value 0"]
impl crate::Resettable for Etmpidr3Spec {}
