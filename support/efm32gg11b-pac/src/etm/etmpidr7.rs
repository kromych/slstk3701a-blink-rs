#[doc = "Register `ETMPIDR7` writer"]
pub type W = crate::W<ETMPIDR7_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<ETMPIDR7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral ID7 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmpidr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPIDR7_SPEC;
impl crate::RegisterSpec for ETMPIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etmpidr7::W`](W) writer structure"]
impl crate::Writable for ETMPIDR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMPIDR7 to value 0"]
impl crate::Resettable for ETMPIDR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
