#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `COMP0` writer - Clear COMP0 Interrupt Flag"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` writer - Clear COMP1 Interrupt Flag"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Clear UF Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP0` writer - Clear REP0 Interrupt Flag"]
pub type Rep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP1` writer - Clear REP1 Interrupt Flag"]
pub type Rep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear COMP0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&mut self) -> Comp0W<'_, IfcSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear COMP1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&mut self) -> Comp1W<'_, IfcSpec> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfcSpec> {
        UfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear REP0 Interrupt Flag"]
    #[inline(always)]
    pub fn rep0(&mut self) -> Rep0W<'_, IfcSpec> {
        Rep0W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear REP1 Interrupt Flag"]
    #[inline(always)]
    pub fn rep1(&mut self) -> Rep1W<'_, IfcSpec> {
        Rep1W::new(self, 4)
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
