#[doc = "Register `ETMCIDR2` reader"]
pub type R = crate::R<ETMCIDR2_SPEC>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PREAMB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PREAMB_R {
        PREAMB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCIDR2_SPEC;
impl crate::RegisterSpec for ETMCIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr2::R`](R) reader structure"]
impl crate::Readable for ETMCIDR2_SPEC {}
#[doc = "`reset()` method sets ETMCIDR2 to value 0x05"]
impl crate::Resettable for ETMCIDR2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
