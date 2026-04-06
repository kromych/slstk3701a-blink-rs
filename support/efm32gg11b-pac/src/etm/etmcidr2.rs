#[doc = "Register `ETMCIDR2` reader"]
pub type R = crate::R<Etmcidr2Spec>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PreambR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PreambR {
        PreambR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmcidr2Spec;
impl crate::RegisterSpec for Etmcidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr2::R`](R) reader structure"]
impl crate::Readable for Etmcidr2Spec {}
#[doc = "`reset()` method sets ETMCIDR2 to value 0x05"]
impl crate::Resettable for Etmcidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}
