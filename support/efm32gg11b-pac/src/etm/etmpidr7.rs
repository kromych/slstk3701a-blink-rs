#[doc = "Register `ETMPIDR7` writer"]
pub type W = crate::W<Etmpidr7Spec>;
impl core::fmt::Debug for crate::generic::Reg<Etmpidr7Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Peripheral ID7 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmpidr7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr7Spec;
impl crate::RegisterSpec for Etmpidr7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etmpidr7::W`](W) writer structure"]
impl crate::Writable for Etmpidr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMPIDR7 to value 0"]
impl crate::Resettable for Etmpidr7Spec {}
