#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `VBUSDETH` writer - Clear VBUSDETH Interrupt Flag"]
pub type VbusdethW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETL` writer - Clear VBUSDETL Interrupt Flag"]
pub type VbusdetlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` writer - Clear ERR Interrupt Flag"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` writer - Clear DCD Interrupt Flag"]
pub type DcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` writer - Clear PD Interrupt Flag"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` writer - Clear SD Interrupt Flag"]
pub type SdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear VBUSDETH Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdeth(&mut self) -> VbusdethW<'_, IfcSpec> {
        VbusdethW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear VBUSDETL Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdetl(&mut self) -> VbusdetlW<'_, IfcSpec> {
        VbusdetlW::new(self, 1)
    }
    #[doc = "Bit 8 - Clear ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IfcSpec> {
        ErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear DCD Interrupt Flag"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DcdW<'_, IfcSpec> {
        DcdW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear PD Interrupt Flag"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, IfcSpec> {
        PdW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear SD Interrupt Flag"]
    #[inline(always)]
    pub fn sd(&mut self) -> SdW<'_, IfcSpec> {
        SdW::new(self, 11)
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
