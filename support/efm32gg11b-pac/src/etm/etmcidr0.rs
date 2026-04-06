#[doc = "Register `ETMCIDR0` reader"]
pub type R = crate::R<Etmcidr0Spec>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PreambR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PreambR {
        PreambR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmcidr0Spec;
impl crate::RegisterSpec for Etmcidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr0::R`](R) reader structure"]
impl crate::Readable for Etmcidr0Spec {}
#[doc = "`reset()` method sets ETMCIDR0 to value 0x0d"]
impl crate::Resettable for Etmcidr0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
