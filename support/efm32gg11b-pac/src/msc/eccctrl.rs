#[doc = "Register `ECCCTRL` reader"]
pub type R = crate::R<EccctrlSpec>;
#[doc = "Register `ECCCTRL` writer"]
pub type W = crate::W<EccctrlSpec>;
#[doc = "Field `RAMECCEWEN` reader - RAM ECC Write Enable"]
pub type RameccewenR = crate::BitReader;
#[doc = "Field `RAMECCEWEN` writer - RAM ECC Write Enable"]
pub type RameccewenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECCCHKEN` reader - RAM ECC Check Enable"]
pub type RameccchkenR = crate::BitReader;
#[doc = "Field `RAMECCCHKEN` writer - RAM ECC Check Enable"]
pub type RameccchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ECCEWEN` reader - RAM1 ECC Write Enable"]
pub type Ram1eccewenR = crate::BitReader;
#[doc = "Field `RAM1ECCEWEN` writer - RAM1 ECC Write Enable"]
pub type Ram1eccewenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1ECCCHKEN` reader - RAM1 ECC Check Enable"]
pub type Ram1eccchkenR = crate::BitReader;
#[doc = "Field `RAM1ECCCHKEN` writer - RAM1 ECC Check Enable"]
pub type Ram1eccchkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&self) -> RameccewenR {
        RameccewenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&self) -> RameccchkenR {
        RameccchkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&self) -> Ram1eccewenR {
        Ram1eccewenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&self) -> Ram1eccchkenR {
        Ram1eccchkenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&mut self) -> RameccewenW<'_, EccctrlSpec> {
        RameccewenW::new(self, 0)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&mut self) -> RameccchkenW<'_, EccctrlSpec> {
        RameccchkenW::new(self, 1)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&mut self) -> Ram1eccewenW<'_, EccctrlSpec> {
        Ram1eccewenW::new(self, 2)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&mut self) -> Ram1eccchkenW<'_, EccctrlSpec> {
        Ram1eccchkenW::new(self, 3)
    }
}
#[doc = "RAM ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccctrlSpec;
impl crate::RegisterSpec for EccctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccctrl::R`](R) reader structure"]
impl crate::Readable for EccctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`eccctrl::W`](W) writer structure"]
impl crate::Writable for EccctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCCTRL to value 0"]
impl crate::Resettable for EccctrlSpec {}
