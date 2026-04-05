#[doc = "Register `PCNTCTRL` reader"]
pub type R = crate::R<PcntctrlSpec>;
#[doc = "Register `PCNTCTRL` writer"]
pub type W = crate::W<PcntctrlSpec>;
#[doc = "Field `PCNT0CLKEN` reader - PCNT0 Clock Enable"]
pub type Pcnt0clkenR = crate::BitReader;
#[doc = "Field `PCNT0CLKEN` writer - PCNT0 Clock Enable"]
pub type Pcnt0clkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0CLKSEL` reader - PCNT0 Clock Select"]
pub type Pcnt0clkselR = crate::BitReader;
#[doc = "Field `PCNT0CLKSEL` writer - PCNT0 Clock Select"]
pub type Pcnt0clkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1CLKEN` reader - PCNT1 Clock Enable"]
pub type Pcnt1clkenR = crate::BitReader;
#[doc = "Field `PCNT1CLKEN` writer - PCNT1 Clock Enable"]
pub type Pcnt1clkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1CLKSEL` reader - PCNT1 Clock Select"]
pub type Pcnt1clkselR = crate::BitReader;
#[doc = "Field `PCNT1CLKSEL` writer - PCNT1 Clock Select"]
pub type Pcnt1clkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2CLKEN` reader - PCNT2 Clock Enable"]
pub type Pcnt2clkenR = crate::BitReader;
#[doc = "Field `PCNT2CLKEN` writer - PCNT2 Clock Enable"]
pub type Pcnt2clkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2CLKSEL` reader - PCNT2 Clock Select"]
pub type Pcnt2clkselR = crate::BitReader;
#[doc = "Field `PCNT2CLKSEL` writer - PCNT2 Clock Select"]
pub type Pcnt2clkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> Pcnt0clkenR {
        Pcnt0clkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> Pcnt0clkselR {
        Pcnt0clkselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCNT1 Clock Enable"]
    #[inline(always)]
    pub fn pcnt1clken(&self) -> Pcnt1clkenR {
        Pcnt1clkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCNT1 Clock Select"]
    #[inline(always)]
    pub fn pcnt1clksel(&self) -> Pcnt1clkselR {
        Pcnt1clkselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCNT2 Clock Enable"]
    #[inline(always)]
    pub fn pcnt2clken(&self) -> Pcnt2clkenR {
        Pcnt2clkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCNT2 Clock Select"]
    #[inline(always)]
    pub fn pcnt2clksel(&self) -> Pcnt2clkselR {
        Pcnt2clkselR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&mut self) -> Pcnt0clkenW<'_, PcntctrlSpec> {
        Pcnt0clkenW::new(self, 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&mut self) -> Pcnt0clkselW<'_, PcntctrlSpec> {
        Pcnt0clkselW::new(self, 1)
    }
    #[doc = "Bit 2 - PCNT1 Clock Enable"]
    #[inline(always)]
    pub fn pcnt1clken(&mut self) -> Pcnt1clkenW<'_, PcntctrlSpec> {
        Pcnt1clkenW::new(self, 2)
    }
    #[doc = "Bit 3 - PCNT1 Clock Select"]
    #[inline(always)]
    pub fn pcnt1clksel(&mut self) -> Pcnt1clkselW<'_, PcntctrlSpec> {
        Pcnt1clkselW::new(self, 3)
    }
    #[doc = "Bit 4 - PCNT2 Clock Enable"]
    #[inline(always)]
    pub fn pcnt2clken(&mut self) -> Pcnt2clkenW<'_, PcntctrlSpec> {
        Pcnt2clkenW::new(self, 4)
    }
    #[doc = "Bit 5 - PCNT2 Clock Select"]
    #[inline(always)]
    pub fn pcnt2clksel(&mut self) -> Pcnt2clkselW<'_, PcntctrlSpec> {
        Pcnt2clkselW::new(self, 5)
    }
}
#[doc = "PCNT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcntctrlSpec;
impl crate::RegisterSpec for PcntctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntctrl::R`](R) reader structure"]
impl crate::Readable for PcntctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcntctrl::W`](W) writer structure"]
impl crate::Writable for PcntctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNTCTRL to value 0"]
impl crate::Resettable for PcntctrlSpec {}
