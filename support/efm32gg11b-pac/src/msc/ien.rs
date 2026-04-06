#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `ERASE` reader - ERASE Interrupt Enable"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - ERASE Interrupt Enable"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - WRITE Interrupt Enable"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - WRITE Interrupt Enable"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOF` reader - CHOF Interrupt Enable"]
pub type ChofR = crate::BitReader;
#[doc = "Field `CHOF` writer - CHOF Interrupt Enable"]
pub type ChofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOF` reader - CMOF Interrupt Enable"]
pub type CmofR = crate::BitReader;
#[doc = "Field `CMOF` writer - CMOF Interrupt Enable"]
pub type CmofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` reader - PWRUPF Interrupt Enable"]
pub type PwrupfR = crate::BitReader;
#[doc = "Field `PWRUPF` writer - PWRUPF Interrupt Enable"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHERR` reader - ICACHERR Interrupt Enable"]
pub type IcacherrR = crate::BitReader;
#[doc = "Field `ICACHERR` writer - ICACHERR Interrupt Enable"]
pub type IcacherrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` reader - WDATAOV Interrupt Enable"]
pub type WdataovR = crate::BitReader;
#[doc = "Field `WDATAOV` writer - WDATAOV Interrupt Enable"]
pub type WdataovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVEWRITE` reader - LVEWRITE Interrupt Enable"]
pub type LvewriteR = crate::BitReader;
#[doc = "Field `LVEWRITE` writer - LVEWRITE Interrupt Enable"]
pub type LvewriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR1B` reader - RAMERR1B Interrupt Enable"]
pub type Ramerr1bR = crate::BitReader;
#[doc = "Field `RAMERR1B` writer - RAMERR1B Interrupt Enable"]
pub type Ramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR2B` reader - RAMERR2B Interrupt Enable"]
pub type Ramerr2bR = crate::BitReader;
#[doc = "Field `RAMERR2B` writer - RAMERR2B Interrupt Enable"]
pub type Ramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR1B` reader - RAM1ERR1B Interrupt Enable"]
pub type Ram1err1bR = crate::BitReader;
#[doc = "Field `RAM1ERR1B` writer - RAM1ERR1B Interrupt Enable"]
pub type Ram1err1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ERR2B` reader - RAM1ERR2B Interrupt Enable"]
pub type Ram1err2bR = crate::BitReader;
#[doc = "Field `RAM1ERR2B` writer - RAM1ERR2B Interrupt Enable"]
pub type Ram1err2bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> ChofR {
        ChofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CmofR {
        CmofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PwrupfR {
        PwrupfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&self) -> IcacherrR {
        IcacherrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDATAOV Interrupt Enable"]
    #[inline(always)]
    pub fn wdataov(&self) -> WdataovR {
        WdataovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LVEWRITE Interrupt Enable"]
    #[inline(always)]
    pub fn lvewrite(&self) -> LvewriteR {
        LvewriteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> Ramerr1bR {
        Ramerr1bR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> Ramerr2bR {
        Ramerr2bR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RAM1ERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err1b(&self) -> Ram1err1bR {
        Ram1err1bR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM1ERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err2b(&self) -> Ram1err2bR {
        Ram1err2bR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, IenSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<'_, IenSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&mut self) -> ChofW<'_, IenSpec> {
        ChofW::new(self, 2)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CmofW<'_, IenSpec> {
        CmofW::new(self, 3)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PwrupfW<'_, IenSpec> {
        PwrupfW::new(self, 4)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> IcacherrW<'_, IenSpec> {
        IcacherrW::new(self, 5)
    }
    #[doc = "Bit 6 - WDATAOV Interrupt Enable"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WdataovW<'_, IenSpec> {
        WdataovW::new(self, 6)
    }
    #[doc = "Bit 8 - LVEWRITE Interrupt Enable"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LvewriteW<'_, IenSpec> {
        LvewriteW::new(self, 8)
    }
    #[doc = "Bit 16 - RAMERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> Ramerr1bW<'_, IenSpec> {
        Ramerr1bW::new(self, 16)
    }
    #[doc = "Bit 17 - RAMERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> Ramerr2bW<'_, IenSpec> {
        Ramerr2bW::new(self, 17)
    }
    #[doc = "Bit 18 - RAM1ERR1B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err1b(&mut self) -> Ram1err1bW<'_, IenSpec> {
        Ram1err1bW::new(self, 18)
    }
    #[doc = "Bit 19 - RAM1ERR2B Interrupt Enable"]
    #[inline(always)]
    pub fn ram1err2b(&mut self) -> Ram1err2bW<'_, IenSpec> {
        Ram1err2bW::new(self, 19)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
