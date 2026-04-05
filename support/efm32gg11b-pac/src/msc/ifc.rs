#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `ERASE` writer - Clear ERASE Interrupt Flag"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Clear WRITE Interrupt Flag"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Clear CHOF Interrupt Flag"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Clear CMOF Interrupt Flag"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Clear PWRUPF Interrupt Flag"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Clear ICACHERR Interrupt Flag"]
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` writer - Clear WDATAOV Interrupt Flag"]
pub type WdataovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVEWRITE` writer - Clear LVEWRITE Interrupt Flag"]
pub type LvewriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR1B` writer - Clear RAMERR1B Interrupt Flag"]
pub type Ramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR2B` writer - Clear RAMERR2B Interrupt Flag"]
pub type Ramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR1B` writer - Clear RAM1ERR1B Interrupt Flag"]
pub type Ram1err1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR2B` writer - Clear RAM1ERR2B Interrupt Flag"]
pub type Ram1err2bW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear ERASE Interrupt Flag"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, IfcSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear WRITE Interrupt Flag"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IfcSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear CHOF Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&mut self) -> ChofW<'_, IfcSpec> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CMOF Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CmofW<'_, IfcSpec> {
        CmofW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear PWRUPF Interrupt Flag"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PwrupfW<'_, IfcSpec> {
        PwrupfW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear ICACHERR Interrupt Flag"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> IcacherrW<'_, IfcSpec> {
        IcacherrW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear WDATAOV Interrupt Flag"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WdataovW<'_, IfcSpec> {
        WdataovW::new(self, 6)
    }
    #[doc = "Bit 8 - Clear LVEWRITE Interrupt Flag"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LvewriteW<'_, IfcSpec> {
        LvewriteW::new(self, 8)
    }
    #[doc = "Bit 16 - Clear RAMERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> Ramerr1bW<'_, IfcSpec> {
        Ramerr1bW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear RAMERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> Ramerr2bW<'_, IfcSpec> {
        Ramerr2bW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&mut self) -> Ram1err1bW<'_, IfcSpec> {
        Ram1err1bW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&mut self) -> Ram1err2bW<'_, IfcSpec> {
        Ram1err2bW::new(self, 19)
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
