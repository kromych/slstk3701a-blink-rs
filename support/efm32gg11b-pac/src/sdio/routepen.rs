#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `CLKPEN` reader - CLK I/O Enable"]
pub type ClkpenR = crate::BitReader;
#[doc = "Field `CLKPEN` writer - CLK I/O Enable"]
pub type ClkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDPEN` reader - CMD I/O Enable"]
pub type CmdpenR = crate::BitReader;
#[doc = "Field `CMDPEN` writer - CMD I/O Enable"]
pub type CmdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D0PEN` reader - Dat0 I/O Enable"]
pub type D0penR = crate::BitReader;
#[doc = "Field `D0PEN` writer - Dat0 I/O Enable"]
pub type D0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1PEN` reader - Dat1 I/O Enable"]
pub type D1penR = crate::BitReader;
#[doc = "Field `D1PEN` writer - Dat1 I/O Enable"]
pub type D1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2PEN` reader - Dat2 I/O Enable"]
pub type D2penR = crate::BitReader;
#[doc = "Field `D2PEN` writer - Dat2 I/O Enable"]
pub type D2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D3PEN` reader - Dat3 I/O Enable"]
pub type D3penR = crate::BitReader;
#[doc = "Field `D3PEN` writer - Dat3 I/O Enable"]
pub type D3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D4PEN` reader - Dat4 I/O Enable"]
pub type D4penR = crate::BitReader;
#[doc = "Field `D4PEN` writer - Dat4 I/O Enable"]
pub type D4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D5PEN` reader - Dat5 Enable"]
pub type D5penR = crate::BitReader;
#[doc = "Field `D5PEN` writer - Dat5 Enable"]
pub type D5penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D6PEN` reader - Dat6 Enable"]
pub type D6penR = crate::BitReader;
#[doc = "Field `D6PEN` writer - Dat6 Enable"]
pub type D6penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D7PEN` reader - Data7 I/O Enable"]
pub type D7penR = crate::BitReader;
#[doc = "Field `D7PEN` writer - Data7 I/O Enable"]
pub type D7penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> ClkpenR {
        ClkpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&self) -> CmdpenR {
        CmdpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&self) -> D0penR {
        D0penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&self) -> D1penR {
        D1penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&self) -> D2penR {
        D2penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&self) -> D3penR {
        D3penR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&self) -> D4penR {
        D4penR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&self) -> D5penR {
        D5penR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&self) -> D6penR {
        D6penR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&self) -> D7penR {
        D7penR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> ClkpenW<'_, RoutepenSpec> {
        ClkpenW::new(self, 0)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&mut self) -> CmdpenW<'_, RoutepenSpec> {
        CmdpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&mut self) -> D0penW<'_, RoutepenSpec> {
        D0penW::new(self, 2)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&mut self) -> D1penW<'_, RoutepenSpec> {
        D1penW::new(self, 3)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&mut self) -> D2penW<'_, RoutepenSpec> {
        D2penW::new(self, 4)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&mut self) -> D3penW<'_, RoutepenSpec> {
        D3penW::new(self, 5)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&mut self) -> D4penW<'_, RoutepenSpec> {
        D4penW::new(self, 6)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&mut self) -> D5penW<'_, RoutepenSpec> {
        D5penW::new(self, 7)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&mut self) -> D6penW<'_, RoutepenSpec> {
        D6penW::new(self, 8)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&mut self) -> D7penW<'_, RoutepenSpec> {
        D7penW::new(self, 9)
    }
}
#[doc = "I/O LOCATION Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoutepenSpec;
impl crate::RegisterSpec for RoutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for RoutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for RoutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for RoutepenSpec {}
