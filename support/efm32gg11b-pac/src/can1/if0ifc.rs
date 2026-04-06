#[doc = "Register `IF0IFC` writer"]
pub type W = crate::W<If0ifcSpec>;
#[doc = "Field `MESSAGE` writer - Clear MESSAGE Interrupt Flag"]
pub type MessageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear MESSAGE Interrupt Flag"]
    #[inline(always)]
    pub fn message(&mut self) -> MessageW<'_, If0ifcSpec> {
        MessageW::new(self, 0)
    }
}
#[doc = "Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if0ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct If0ifcSpec;
impl crate::RegisterSpec for If0ifcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if0ifc::W`](W) writer structure"]
impl crate::Writable for If0ifcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF0IFC to value 0"]
impl crate::Resettable for If0ifcSpec {}
