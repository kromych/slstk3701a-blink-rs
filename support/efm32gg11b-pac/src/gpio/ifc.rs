#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `EXT` writer - Clear EXT Interrupt Flag"]
pub type ExtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM4WU` writer - Clear EM4WU Interrupt Flag"]
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Clear EXT Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&mut self) -> ExtW<'_, IfcSpec> {
        ExtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Clear EM4WU Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> Em4wuW<'_, IfcSpec> {
        Em4wuW::new(self, 16)
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
