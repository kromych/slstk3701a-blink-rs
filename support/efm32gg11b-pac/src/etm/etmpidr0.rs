#[doc = "Register `ETMPIDR0` reader"]
pub type R = crate::R<ETMPIDR0_SPEC>;
#[doc = "Field `PARTNUM` reader - Part Number"]
pub type PARTNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPIDR0_SPEC;
impl crate::RegisterSpec for ETMPIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpidr0::R`](R) reader structure"]
impl crate::Readable for ETMPIDR0_SPEC {}
#[doc = "`reset()` method sets ETMPIDR0 to value 0x25"]
impl crate::Resettable for ETMPIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x25;
}
