#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `SCLKPEN` reader - SCLK Pin Enable"]
pub type SclkpenR = crate::BitReader;
#[doc = "Field `SCLKPEN` writer - SCLK Pin Enable"]
pub type SclkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS0PEN` reader - CS0 Pin Enable"]
pub type Cs0penR = crate::BitReader;
#[doc = "Field `CS0PEN` writer - CS0 Pin Enable"]
pub type Cs0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1PEN` reader - CS1 Pin Enable"]
pub type Cs1penR = crate::BitReader;
#[doc = "Field `CS1PEN` writer - CS1 Pin Enable"]
pub type Cs1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ0PEN` reader - DQ0 Pin Enable"]
pub type Dq0penR = crate::BitReader;
#[doc = "Field `DQ0PEN` writer - DQ0 Pin Enable"]
pub type Dq0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ1PEN` reader - DQ1 Pin Enable"]
pub type Dq1penR = crate::BitReader;
#[doc = "Field `DQ1PEN` writer - DQ1 Pin Enable"]
pub type Dq1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ2PEN` reader - DQ2 Pin Enable"]
pub type Dq2penR = crate::BitReader;
#[doc = "Field `DQ2PEN` writer - DQ2 Pin Enable"]
pub type Dq2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ3PEN` reader - DQ3 Pin Enable"]
pub type Dq3penR = crate::BitReader;
#[doc = "Field `DQ3PEN` writer - DQ3 Pin Enable"]
pub type Dq3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ4PEN` reader - DQ4 Pin Enable"]
pub type Dq4penR = crate::BitReader;
#[doc = "Field `DQ4PEN` writer - DQ4 Pin Enable"]
pub type Dq4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ5PEN` reader - DQ5 Pin Enable"]
pub type Dq5penR = crate::BitReader;
#[doc = "Field `DQ5PEN` writer - DQ5 Pin Enable"]
pub type Dq5penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ6PEN` reader - DQ6 Pin Enable"]
pub type Dq6penR = crate::BitReader;
#[doc = "Field `DQ6PEN` writer - DQ6 Pin Enable"]
pub type Dq6penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ7PEN` reader - DQ7 Pin Enable"]
pub type Dq7penR = crate::BitReader;
#[doc = "Field `DQ7PEN` writer - DQ7 Pin Enable"]
pub type Dq7penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSPEN` reader - DQS Pin Enable"]
pub type DqspenR = crate::BitReader;
#[doc = "Field `DQSPEN` writer - DQS Pin Enable"]
pub type DqspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLKINPEN` reader - SCLKIN Pin Enable"]
pub type SclkinpenR = crate::BitReader;
#[doc = "Field `SCLKINPEN` writer - SCLKIN Pin Enable"]
pub type SclkinpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCLK Pin Enable"]
    #[inline(always)]
    pub fn sclkpen(&self) -> SclkpenR {
        SclkpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> Cs0penR {
        Cs0penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> Cs1penR {
        Cs1penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - DQ0 Pin Enable"]
    #[inline(always)]
    pub fn dq0pen(&self) -> Dq0penR {
        Dq0penR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DQ1 Pin Enable"]
    #[inline(always)]
    pub fn dq1pen(&self) -> Dq1penR {
        Dq1penR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DQ2 Pin Enable"]
    #[inline(always)]
    pub fn dq2pen(&self) -> Dq2penR {
        Dq2penR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DQ3 Pin Enable"]
    #[inline(always)]
    pub fn dq3pen(&self) -> Dq3penR {
        Dq3penR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DQ4 Pin Enable"]
    #[inline(always)]
    pub fn dq4pen(&self) -> Dq4penR {
        Dq4penR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DQ5 Pin Enable"]
    #[inline(always)]
    pub fn dq5pen(&self) -> Dq5penR {
        Dq5penR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DQ6 Pin Enable"]
    #[inline(always)]
    pub fn dq6pen(&self) -> Dq6penR {
        Dq6penR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DQ7 Pin Enable"]
    #[inline(always)]
    pub fn dq7pen(&self) -> Dq7penR {
        Dq7penR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DQS Pin Enable"]
    #[inline(always)]
    pub fn dqspen(&self) -> DqspenR {
        DqspenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCLKIN Pin Enable"]
    #[inline(always)]
    pub fn sclkinpen(&self) -> SclkinpenR {
        SclkinpenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCLK Pin Enable"]
    #[inline(always)]
    pub fn sclkpen(&mut self) -> SclkpenW<'_, RoutepenSpec> {
        SclkpenW::new(self, 0)
    }
    #[doc = "Bit 1 - CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&mut self) -> Cs0penW<'_, RoutepenSpec> {
        Cs0penW::new(self, 1)
    }
    #[doc = "Bit 2 - CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&mut self) -> Cs1penW<'_, RoutepenSpec> {
        Cs1penW::new(self, 2)
    }
    #[doc = "Bit 5 - DQ0 Pin Enable"]
    #[inline(always)]
    pub fn dq0pen(&mut self) -> Dq0penW<'_, RoutepenSpec> {
        Dq0penW::new(self, 5)
    }
    #[doc = "Bit 6 - DQ1 Pin Enable"]
    #[inline(always)]
    pub fn dq1pen(&mut self) -> Dq1penW<'_, RoutepenSpec> {
        Dq1penW::new(self, 6)
    }
    #[doc = "Bit 7 - DQ2 Pin Enable"]
    #[inline(always)]
    pub fn dq2pen(&mut self) -> Dq2penW<'_, RoutepenSpec> {
        Dq2penW::new(self, 7)
    }
    #[doc = "Bit 8 - DQ3 Pin Enable"]
    #[inline(always)]
    pub fn dq3pen(&mut self) -> Dq3penW<'_, RoutepenSpec> {
        Dq3penW::new(self, 8)
    }
    #[doc = "Bit 9 - DQ4 Pin Enable"]
    #[inline(always)]
    pub fn dq4pen(&mut self) -> Dq4penW<'_, RoutepenSpec> {
        Dq4penW::new(self, 9)
    }
    #[doc = "Bit 10 - DQ5 Pin Enable"]
    #[inline(always)]
    pub fn dq5pen(&mut self) -> Dq5penW<'_, RoutepenSpec> {
        Dq5penW::new(self, 10)
    }
    #[doc = "Bit 11 - DQ6 Pin Enable"]
    #[inline(always)]
    pub fn dq6pen(&mut self) -> Dq6penW<'_, RoutepenSpec> {
        Dq6penW::new(self, 11)
    }
    #[doc = "Bit 12 - DQ7 Pin Enable"]
    #[inline(always)]
    pub fn dq7pen(&mut self) -> Dq7penW<'_, RoutepenSpec> {
        Dq7penW::new(self, 12)
    }
    #[doc = "Bit 13 - DQS Pin Enable"]
    #[inline(always)]
    pub fn dqspen(&mut self) -> DqspenW<'_, RoutepenSpec> {
        DqspenW::new(self, 13)
    }
    #[doc = "Bit 14 - SCLKIN Pin Enable"]
    #[inline(always)]
    pub fn sclkinpen(&mut self) -> SclkinpenW<'_, RoutepenSpec> {
        SclkinpenW::new(self, 14)
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
