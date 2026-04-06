#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `ERASE` writer - Set ERASE Interrupt Flag"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` writer - Set WRITE Interrupt Flag"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` writer - Set CHOF Interrupt Flag"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` writer - Set CMOF Interrupt Flag"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` writer - Set PWRUPF Interrupt Flag"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` writer - Set ICACHERR Interrupt Flag"]
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` writer - Set WDATAOV Interrupt Flag"]
pub type WdataovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVEWRITE` writer - Set LVEWRITE Interrupt Flag"]
pub type LvewriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR1B` writer - Set RAMERR1B Interrupt Flag"]
pub type Ramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR2B` writer - Set RAMERR2B Interrupt Flag"]
pub type Ramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR1B` writer - Set RAM1ERR1B Interrupt Flag"]
pub type Ram1err1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR2B` writer - Set RAM1ERR2B Interrupt Flag"]
pub type Ram1err2bW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set ERASE Interrupt Flag"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, IfsSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Set WRITE Interrupt Flag"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IfsSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Set CHOF Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&mut self) -> ChofW<'_, IfsSpec> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - Set CMOF Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CmofW<'_, IfsSpec> {
        CmofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set PWRUPF Interrupt Flag"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PwrupfW<'_, IfsSpec> {
        PwrupfW::new(self, 4)
    }
    #[doc = "Bit 5 - Set ICACHERR Interrupt Flag"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> IcacherrW<'_, IfsSpec> {
        IcacherrW::new(self, 5)
    }
    #[doc = "Bit 6 - Set WDATAOV Interrupt Flag"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WdataovW<'_, IfsSpec> {
        WdataovW::new(self, 6)
    }
    #[doc = "Bit 8 - Set LVEWRITE Interrupt Flag"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LvewriteW<'_, IfsSpec> {
        LvewriteW::new(self, 8)
    }
    #[doc = "Bit 16 - Set RAMERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> Ramerr1bW<'_, IfsSpec> {
        Ramerr1bW::new(self, 16)
    }
    #[doc = "Bit 17 - Set RAMERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> Ramerr2bW<'_, IfsSpec> {
        Ramerr2bW::new(self, 17)
    }
    #[doc = "Bit 18 - Set RAM1ERR1B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&mut self) -> Ram1err1bW<'_, IfsSpec> {
        Ram1err1bW::new(self, 18)
    }
    #[doc = "Bit 19 - Set RAM1ERR2B Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&mut self) -> Ram1err2bW<'_, IfsSpec> {
        Ram1err2bW::new(self, 19)
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
