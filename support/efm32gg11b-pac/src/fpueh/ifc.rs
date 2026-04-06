#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `FPIOC` writer - Clear FPIOC Interrupt Flag"]
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` writer - Clear FPDZC Interrupt Flag"]
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` writer - Clear FPUFC Interrupt Flag"]
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` writer - Clear FPOFC Interrupt Flag"]
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` writer - Clear FPIDC Interrupt Flag"]
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` writer - Clear FPIXC Interrupt Flag"]
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear FPIOC Interrupt Flag"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FpiocW<'_, IfcSpec> {
        FpiocW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear FPDZC Interrupt Flag"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FpdzcW<'_, IfcSpec> {
        FpdzcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear FPUFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FpufcW<'_, IfcSpec> {
        FpufcW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear FPOFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FpofcW<'_, IfcSpec> {
        FpofcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear FPIDC Interrupt Flag"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FpidcW<'_, IfcSpec> {
        FpidcW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear FPIXC Interrupt Flag"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FpixcW<'_, IfcSpec> {
        FpixcW::new(self, 5)
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
