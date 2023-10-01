#[doc = "Register `ETMCIDR0` reader"]
pub type R = crate::R<ETMCIDR0_SPEC>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PREAMB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PREAMB_R {
        PREAMB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCIDR0_SPEC;
impl crate::RegisterSpec for ETMCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr0::R`](R) reader structure"]
impl crate::Readable for ETMCIDR0_SPEC {}
#[doc = "`reset()` method sets ETMCIDR0 to value 0x0d"]
impl crate::Resettable for ETMCIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
