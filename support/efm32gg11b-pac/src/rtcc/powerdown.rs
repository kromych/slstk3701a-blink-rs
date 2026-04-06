#[doc = "Register `POWERDOWN` reader"]
pub type R = crate::R<PowerdownSpec>;
#[doc = "Register `POWERDOWN` writer"]
pub type W = crate::W<PowerdownSpec>;
#[doc = "Field `RAM` reader - Retention RAM Power-down"]
pub type RamR = crate::BitReader;
#[doc = "Field `RAM` writer - Retention RAM Power-down"]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention RAM Power-down"]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<'_, PowerdownSpec> {
        RamW::new(self, 0)
    }
}
#[doc = "Retention RAM Power-down Register\n\nYou can [`read`](crate::Reg::read) this register and get [`powerdown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerdownSpec;
impl crate::RegisterSpec for PowerdownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerdown::R`](R) reader structure"]
impl crate::Readable for PowerdownSpec {}
#[doc = "`write(|w| ..)` method takes [`powerdown::W`](W) writer structure"]
impl crate::Writable for PowerdownSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for PowerdownSpec {}
