#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `CMP` writer - Clear CMP Interrupt Flag"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV` writer - Clear CONV Interrupt Flag"]
pub type ConvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` writer - Clear EOS Interrupt Flag"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOF` writer - Clear DMAOF Interrupt Flag"]
pub type DmaofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Clear APORTCONFLICT Interrupt Flag"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear CMP Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, IfcSpec> {
        CmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CONV Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&mut self) -> ConvW<'_, IfcSpec> {
        ConvW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear EOS Interrupt Flag"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IfcSpec> {
        EosW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear DMAOF Interrupt Flag"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DmaofW<'_, IfcSpec> {
        DmaofW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> AportconflictW<'_, IfcSpec> {
        AportconflictW::new(self, 4)
    }
}
#[doc = "Interrupt Flag Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
