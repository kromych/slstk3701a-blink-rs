#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `OF` writer - Clear OF Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` writer - Clear COMP Interrupt Flag"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bit 0 - Clear OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfcSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bits 1:6 - Clear COMP Interrupt Flag"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, IfcSpec> {
        CompW::new(self, 1)
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
