#[doc = "Register `ETMCIDR1` reader"]
pub type R = crate::R<Etmcidr1Spec>;
#[doc = "Field `PREAMB` reader - CoreSight Preamble"]
pub type PreambR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PreambR {
        PreambR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmcidr1Spec;
impl crate::RegisterSpec for Etmcidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcidr1::R`](R) reader structure"]
impl crate::Readable for Etmcidr1Spec {}
#[doc = "`reset()` method sets ETMCIDR1 to value 0x90"]
impl crate::Resettable for Etmcidr1Spec {
    const RESET_VALUE: u32 = 0x90;
}
