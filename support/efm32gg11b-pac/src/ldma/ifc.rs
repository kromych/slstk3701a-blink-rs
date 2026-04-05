#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `DONE` writer - Clear DONE Interrupt Flag"]
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ERROR` writer - Clear ERROR Interrupt Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - Clear DONE Interrupt Flag"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IfcSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 31 - Clear ERROR Interrupt Flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IfcSpec> {
        ErrorW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
