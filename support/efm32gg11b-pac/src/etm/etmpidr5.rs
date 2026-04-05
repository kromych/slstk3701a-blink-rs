#[doc = "Register `ETMPIDR5` writer"]
pub type W = crate::W<Etmpidr5Spec>;
impl core::fmt::Debug for crate::generic::Reg<Etmpidr5Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Peripheral ID5 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmpidr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmpidr5Spec;
impl crate::RegisterSpec for Etmpidr5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etmpidr5::W`](W) writer structure"]
impl crate::Writable for Etmpidr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMPIDR5 to value 0"]
impl crate::Resettable for Etmpidr5Spec {}
