#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `OF` writer - Clear OF Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Clear UF Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - Clear DIRCHG Interrupt Flag"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` writer - Clear CC0 Interrupt Flag"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` writer - Clear CC1 Interrupt Flag"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` writer - Clear CC2 Interrupt Flag"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3` writer - Clear CC3 Interrupt Flag"]
pub type Cc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF0` writer - Clear ICBOF0 Interrupt Flag"]
pub type Icbof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF1` writer - Clear ICBOF1 Interrupt Flag"]
pub type Icbof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF2` writer - Clear ICBOF2 Interrupt Flag"]
pub type Icbof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF3` writer - Clear ICBOF3 Interrupt Flag"]
pub type Icbof3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfcSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfcSpec> {
        UfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear DIRCHG Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DirchgW<'_, IfcSpec> {
        DirchgW::new(self, 2)
    }
    #[doc = "Bit 4 - Clear CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<'_, IfcSpec> {
        Cc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<'_, IfcSpec> {
        Cc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, IfcSpec> {
        Cc2W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear CC3 Interrupt Flag"]
    #[inline(always)]
    pub fn cc3(&mut self) -> Cc3W<'_, IfcSpec> {
        Cc3W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear ICBOF0 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> Icbof0W<'_, IfcSpec> {
        Icbof0W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear ICBOF1 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> Icbof1W<'_, IfcSpec> {
        Icbof1W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear ICBOF2 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> Icbof2W<'_, IfcSpec> {
        Icbof2W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear ICBOF3 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof3(&mut self) -> Icbof3W<'_, IfcSpec> {
        Icbof3W::new(self, 11)
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
