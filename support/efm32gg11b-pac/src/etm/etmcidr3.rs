#[doc = "Register `ETMCIDR3` reader"]
pub type R = crate::R<Etmcidr3Spec>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PreambR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PreambR {
        PreambR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmcidr3Spec;
impl crate::RegisterSpec for Etmcidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr3::R`](R) reader structure"]
impl crate::Readable for Etmcidr3Spec {}
#[doc = "`reset()` method sets ETMCIDR3 to value 0xb1"]
impl crate::Resettable for Etmcidr3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
