#[doc = "Register `RAMCTRL` reader"]
pub type R = crate::R<RamctrlSpec>;
#[doc = "Register `RAMCTRL` writer"]
pub type W = crate::W<RamctrlSpec>;
#[doc = "Field `RAMWSEN` reader - RAM WAIT STATE Enable"]
pub type RamwsenR = crate::BitReader;
#[doc = "Field `RAMWSEN` writer - RAM WAIT STATE Enable"]
pub type RamwsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPREFETCHEN` reader - RAM Prefetch Enable"]
pub type RamprefetchenR = crate::BitReader;
#[doc = "Field `RAMPREFETCHEN` writer - RAM Prefetch Enable"]
pub type RamprefetchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1WSEN` reader - RAM1 WAIT STATE Enable"]
pub type Ram1wsenR = crate::BitReader;
#[doc = "Field `RAM1WSEN` writer - RAM1 WAIT STATE Enable"]
pub type Ram1wsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1PREFETCHEN` reader - RAM1 Prefetch Enable"]
pub type Ram1prefetchenR = crate::BitReader;
#[doc = "Field `RAM1PREFETCHEN` writer - RAM1 Prefetch Enable"]
pub type Ram1prefetchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM2CACHEEN` reader - RAM2 CACHE Enable"]
pub type Ram2cacheenR = crate::BitReader;
#[doc = "Field `RAM2CACHEEN` writer - RAM2 CACHE Enable"]
pub type Ram2cacheenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM2WSEN` reader - RAM2 WAIT STATE Enable"]
pub type Ram2wsenR = crate::BitReader;
#[doc = "Field `RAM2WSEN` writer - RAM2 WAIT STATE Enable"]
pub type Ram2wsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM2PREFETCHEN` reader - RAM2 Prefetch Enable"]
pub type Ram2prefetchenR = crate::BitReader;
#[doc = "Field `RAM2PREFETCHEN` writer - RAM2 Prefetch Enable"]
pub type Ram2prefetchenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&self) -> RamwsenR {
        RamwsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&self) -> RamprefetchenR {
        RamprefetchenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&self) -> Ram1wsenR {
        Ram1wsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&self) -> Ram1prefetchenR {
        Ram1prefetchenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&self) -> Ram2cacheenR {
        Ram2cacheenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&self) -> Ram2wsenR {
        Ram2wsenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&self) -> Ram2prefetchenR {
        Ram2prefetchenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&mut self) -> RamwsenW<'_, RamctrlSpec> {
        RamwsenW::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&mut self) -> RamprefetchenW<'_, RamctrlSpec> {
        RamprefetchenW::new(self, 2)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&mut self) -> Ram1wsenW<'_, RamctrlSpec> {
        Ram1wsenW::new(self, 9)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&mut self) -> Ram1prefetchenW<'_, RamctrlSpec> {
        Ram1prefetchenW::new(self, 10)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&mut self) -> Ram2cacheenW<'_, RamctrlSpec> {
        Ram2cacheenW::new(self, 16)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&mut self) -> Ram2wsenW<'_, RamctrlSpec> {
        Ram2wsenW::new(self, 17)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&mut self) -> Ram2prefetchenW<'_, RamctrlSpec> {
        Ram2prefetchenW::new(self, 18)
    }
}
#[doc = "RAM Control Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ramctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamctrlSpec;
impl crate::RegisterSpec for RamctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramctrl::R`](R) reader structure"]
impl crate::Readable for RamctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ramctrl::W`](W) writer structure"]
impl crate::Writable for RamctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMCTRL to value 0"]
impl crate::Resettable for RamctrlSpec {}
