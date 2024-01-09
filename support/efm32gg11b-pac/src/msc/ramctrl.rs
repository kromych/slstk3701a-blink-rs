#[doc = "Register `RAMCTRL` reader"]
pub type R = crate::R<RAMCTRL_SPEC>;
#[doc = "Register `RAMCTRL` writer"]
pub type W = crate::W<RAMCTRL_SPEC>;
#[doc = "Field `RAMWSEN` reader - RAM WAIT STATE Enable"]
pub type RAMWSEN_R = crate::BitReader;
#[doc = "Field `RAMWSEN` writer - RAM WAIT STATE Enable"]
pub type RAMWSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPREFETCHEN` reader - RAM Prefetch Enable"]
pub type RAMPREFETCHEN_R = crate::BitReader;
#[doc = "Field `RAMPREFETCHEN` writer - RAM Prefetch Enable"]
pub type RAMPREFETCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1WSEN` reader - RAM1 WAIT STATE Enable"]
pub type RAM1WSEN_R = crate::BitReader;
#[doc = "Field `RAM1WSEN` writer - RAM1 WAIT STATE Enable"]
pub type RAM1WSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1PREFETCHEN` reader - RAM1 Prefetch Enable"]
pub type RAM1PREFETCHEN_R = crate::BitReader;
#[doc = "Field `RAM1PREFETCHEN` writer - RAM1 Prefetch Enable"]
pub type RAM1PREFETCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM2CACHEEN` reader - RAM2 CACHE Enable"]
pub type RAM2CACHEEN_R = crate::BitReader;
#[doc = "Field `RAM2CACHEEN` writer - RAM2 CACHE Enable"]
pub type RAM2CACHEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM2WSEN` reader - RAM2 WAIT STATE Enable"]
pub type RAM2WSEN_R = crate::BitReader;
#[doc = "Field `RAM2WSEN` writer - RAM2 WAIT STATE Enable"]
pub type RAM2WSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM2PREFETCHEN` reader - RAM2 Prefetch Enable"]
pub type RAM2PREFETCHEN_R = crate::BitReader;
#[doc = "Field `RAM2PREFETCHEN` writer - RAM2 Prefetch Enable"]
pub type RAM2PREFETCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&self) -> RAMWSEN_R {
        RAMWSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&self) -> RAMPREFETCHEN_R {
        RAMPREFETCHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&self) -> RAM1WSEN_R {
        RAM1WSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&self) -> RAM1PREFETCHEN_R {
        RAM1PREFETCHEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&self) -> RAM2CACHEEN_R {
        RAM2CACHEEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&self) -> RAM2WSEN_R {
        RAM2WSEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&self) -> RAM2PREFETCHEN_R {
        RAM2PREFETCHEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramwsen(&mut self) -> RAMWSEN_W<RAMCTRL_SPEC> {
        RAMWSEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramprefetchen(&mut self) -> RAMPREFETCHEN_W<RAMCTRL_SPEC> {
        RAMPREFETCHEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram1wsen(&mut self) -> RAM1WSEN_W<RAMCTRL_SPEC> {
        RAM1WSEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram1prefetchen(&mut self) -> RAM1PREFETCHEN_W<RAMCTRL_SPEC> {
        RAM1PREFETCHEN_W::new(self, 10)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram2cacheen(&mut self) -> RAM2CACHEEN_W<RAMCTRL_SPEC> {
        RAM2CACHEEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram2wsen(&mut self) -> RAM2WSEN_W<RAMCTRL_SPEC> {
        RAM2WSEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram2prefetchen(&mut self) -> RAM2PREFETCHEN_W<RAMCTRL_SPEC> {
        RAM2PREFETCHEN_W::new(self, 18)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAM Control Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ramctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ramctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMCTRL_SPEC;
impl crate::RegisterSpec for RAMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramctrl::R`](R) reader structure"]
impl crate::Readable for RAMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ramctrl::W`](W) writer structure"]
impl crate::Writable for RAMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMCTRL to value 0"]
impl crate::Resettable for RAMCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
