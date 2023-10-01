#[doc = "Register `ETMPDSR` reader"]
pub type R = crate::R<ETMPDSR_SPEC>;
#[doc = "Field `ETMUP` reader - ETM Powered Up"]
pub type ETMUP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ETM Powered Up"]
    #[inline(always)]
    pub fn etmup(&self) -> ETMUP_R {
        ETMUP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Device Power-down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmpdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPDSR_SPEC;
impl crate::RegisterSpec for ETMPDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmpdsr::R`](R) reader structure"]
impl crate::Readable for ETMPDSR_SPEC {}
#[doc = "`reset()` method sets ETMPDSR to value 0x01"]
impl crate::Resettable for ETMPDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
