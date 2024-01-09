#[doc = "Register `ETMCIDR3` reader"]
pub type R = crate::R<ETMCIDR3_SPEC>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PREAMB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PREAMB_R {
        PREAMB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCIDR3_SPEC;
impl crate::RegisterSpec for ETMCIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr3::R`](R) reader structure"]
impl crate::Readable for ETMCIDR3_SPEC {}
#[doc = "`reset()` method sets ETMCIDR3 to value 0xb1"]
impl crate::Resettable for ETMCIDR3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
