#[doc = "Register `ETMCIDR1` reader"]
pub type R = crate::R<ETMCIDR1_SPEC>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PREAMB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PREAMB_R {
        PREAMB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCIDR1_SPEC;
impl crate::RegisterSpec for ETMCIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr1::R`](R) reader structure"]
impl crate::Readable for ETMCIDR1_SPEC {}
#[doc = "`reset()` method sets ETMCIDR1 to value 0x90"]
impl crate::Resettable for ETMCIDR1_SPEC {
    const RESET_VALUE: u32 = 0x90;
}
