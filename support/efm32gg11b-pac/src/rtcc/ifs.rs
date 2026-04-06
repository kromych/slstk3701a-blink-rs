#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` writer - Set CC0 Interrupt Flag"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` writer - Set CC1 Interrupt Flag"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` writer - Set CC2 Interrupt Flag"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCFAIL` writer - Set OSCFAIL Interrupt Flag"]
pub type OscfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTTICK` writer - Set CNTTICK Interrupt Flag"]
pub type CnttickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINTICK` writer - Set MINTICK Interrupt Flag"]
pub type MintickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOURTICK` writer - Set HOURTICK Interrupt Flag"]
pub type HourtickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYTICK` writer - Set DAYTICK Interrupt Flag"]
pub type DaytickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAYOWOF` writer - Set DAYOWOF Interrupt Flag"]
pub type DayowofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTHTICK` writer - Set MONTHTICK Interrupt Flag"]
pub type MonthtickW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfsSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Set CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<'_, IfsSpec> {
        Cc0W::new(self, 1)
    }
    #[doc = "Bit 2 - Set CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<'_, IfsSpec> {
        Cc1W::new(self, 2)
    }
    #[doc = "Bit 3 - Set CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, IfsSpec> {
        Cc2W::new(self, 3)
    }
    #[doc = "Bit 4 - Set OSCFAIL Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&mut self) -> OscfailW<'_, IfsSpec> {
        OscfailW::new(self, 4)
    }
    #[doc = "Bit 5 - Set CNTTICK Interrupt Flag"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CnttickW<'_, IfsSpec> {
        CnttickW::new(self, 5)
    }
    #[doc = "Bit 6 - Set MINTICK Interrupt Flag"]
    #[inline(always)]
    pub fn mintick(&mut self) -> MintickW<'_, IfsSpec> {
        MintickW::new(self, 6)
    }
    #[doc = "Bit 7 - Set HOURTICK Interrupt Flag"]
    #[inline(always)]
    pub fn hourtick(&mut self) -> HourtickW<'_, IfsSpec> {
        HourtickW::new(self, 7)
    }
    #[doc = "Bit 8 - Set DAYTICK Interrupt Flag"]
    #[inline(always)]
    pub fn daytick(&mut self) -> DaytickW<'_, IfsSpec> {
        DaytickW::new(self, 8)
    }
    #[doc = "Bit 9 - Set DAYOWOF Interrupt Flag"]
    #[inline(always)]
    pub fn dayowof(&mut self) -> DayowofW<'_, IfsSpec> {
        DayowofW::new(self, 9)
    }
    #[doc = "Bit 10 - Set MONTHTICK Interrupt Flag"]
    #[inline(always)]
    pub fn monthtick(&mut self) -> MonthtickW<'_, IfsSpec> {
        MonthtickW::new(self, 10)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
