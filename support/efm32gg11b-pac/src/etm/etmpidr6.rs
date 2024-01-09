#[doc = "Register `ETMPIDR6` writer"]
pub type W = crate::W<ETMPIDR6_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<ETMPIDR6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral ID6 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmpidr6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMPIDR6_SPEC;
impl crate::RegisterSpec for ETMPIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etmpidr6::W`](W) writer structure"]
impl crate::Writable for ETMPIDR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMPIDR6 to value 0"]
impl crate::Resettable for ETMPIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
