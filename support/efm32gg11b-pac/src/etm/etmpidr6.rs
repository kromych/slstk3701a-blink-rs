#[doc = "Register `ETMPIDR6` writer"]
pub type W = crate::W<Etmpidr6Spec>;
impl core::fmt::Debug for crate::generic::Reg<Etmpidr6Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Peripheral ID6 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmpidr6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr6Spec;
impl crate::RegisterSpec for Etmpidr6Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etmpidr6::W`](W) writer structure"]
impl crate::Writable for Etmpidr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMPIDR6 to value 0"]
impl crate::Resettable for Etmpidr6Spec {}
