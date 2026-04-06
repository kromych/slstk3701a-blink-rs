#[doc = "Register `IF1IFC` writer"]
pub type W = crate::W<If1ifcSpec>;
#[doc = "Field `STATUS` writer - Clear STATUS Interrupt Flag"]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear STATUS Interrupt Flag"]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<'_, If1ifcSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if1ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If1ifcSpec;
impl crate::RegisterSpec for If1ifcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if1ifc::W`](W) writer structure"]
impl crate::Writable for If1ifcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF1IFC to value 0"]
impl crate::Resettable for If1ifcSpec {}
