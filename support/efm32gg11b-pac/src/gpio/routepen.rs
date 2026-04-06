#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `SWCLKTCKPEN` reader - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SwclktckpenR = crate::BitReader;
#[doc = "Field `SWCLKTCKPEN` writer - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SwclktckpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDIOTMSPEN` reader - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SwdiotmspenR = crate::BitReader;
#[doc = "Field `SWDIOTMSPEN` writer - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SwdiotmspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TdopenR = crate::BitReader;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TdopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TdipenR = crate::BitReader;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TdipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenR = crate::BitReader;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMTCLKPEN` reader - ETM Trace Clock Pin Enable"]
pub type EtmtclkpenR = crate::BitReader;
#[doc = "Field `ETMTCLKPEN` writer - ETM Trace Clock Pin Enable"]
pub type EtmtclkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMTD0PEN` reader - ETM Trace Data Pin Enable"]
pub type Etmtd0penR = crate::BitReader;
#[doc = "Field `ETMTD0PEN` writer - ETM Trace Data Pin Enable"]
pub type Etmtd0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMTD1PEN` reader - ETM Trace Data Pin Enable"]
pub type Etmtd1penR = crate::BitReader;
#[doc = "Field `ETMTD1PEN` writer - ETM Trace Data Pin Enable"]
pub type Etmtd1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMTD2PEN` reader - ETM Trace Data Pin Enable"]
pub type Etmtd2penR = crate::BitReader;
#[doc = "Field `ETMTD2PEN` writer - ETM Trace Data Pin Enable"]
pub type Etmtd2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETMTD3PEN` reader - ETM Trace Data Pin Enable"]
pub type Etmtd3penR = crate::BitReader;
#[doc = "Field `ETMTD3PEN` writer - ETM Trace Data Pin Enable"]
pub type Etmtd3penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SwclktckpenR {
        SwclktckpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SwdiotmspenR {
        SwdiotmspenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TdopenR {
        TdopenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TdipenR {
        TdipenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SwvpenR {
        SwvpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    pub fn etmtclkpen(&self) -> EtmtclkpenR {
        EtmtclkpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd0pen(&self) -> Etmtd0penR {
        Etmtd0penR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd1pen(&self) -> Etmtd1penR {
        Etmtd1penR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd2pen(&self) -> Etmtd2penR {
        Etmtd2penR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd3pen(&self) -> Etmtd3penR {
        Etmtd3penR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&mut self) -> SwclktckpenW<'_, RoutepenSpec> {
        SwclktckpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&mut self) -> SwdiotmspenW<'_, RoutepenSpec> {
        SwdiotmspenW::new(self, 1)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&mut self) -> TdopenW<'_, RoutepenSpec> {
        TdopenW::new(self, 2)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&mut self) -> TdipenW<'_, RoutepenSpec> {
        TdipenW::new(self, 3)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&mut self) -> SwvpenW<'_, RoutepenSpec> {
        SwvpenW::new(self, 4)
    }
    #[doc = "Bit 16 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    pub fn etmtclkpen(&mut self) -> EtmtclkpenW<'_, RoutepenSpec> {
        EtmtclkpenW::new(self, 16)
    }
    #[doc = "Bit 17 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd0pen(&mut self) -> Etmtd0penW<'_, RoutepenSpec> {
        Etmtd0penW::new(self, 17)
    }
    #[doc = "Bit 18 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd1pen(&mut self) -> Etmtd1penW<'_, RoutepenSpec> {
        Etmtd1penW::new(self, 18)
    }
    #[doc = "Bit 19 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd2pen(&mut self) -> Etmtd2penW<'_, RoutepenSpec> {
        Etmtd2penW::new(self, 19)
    }
    #[doc = "Bit 20 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd3pen(&mut self) -> Etmtd3penW<'_, RoutepenSpec> {
        Etmtd3penW::new(self, 20)
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
#[doc = "`reset()` method sets ROUTEPEN to value 0x0f"]
impl crate::Resettable for RoutepenSpec {
    const RESET_VALUE: u32 = 0x0f;
}
