#[doc = "Register `PPUCTRL` reader"]
pub type R = crate::R<PpuctrlSpec>;
#[doc = "Register `PPUCTRL` writer"]
pub type W = crate::W<PpuctrlSpec>;
#[doc = "Field `ENABLE` reader - "]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - "]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, PpuctrlSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "PPU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppuctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppuctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpuctrlSpec;
impl crate::RegisterSpec for PpuctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppuctrl::R`](R) reader structure"]
impl crate::Readable for PpuctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ppuctrl::W`](W) writer structure"]
impl crate::Writable for PpuctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUCTRL to value 0"]
impl crate::Resettable for PpuctrlSpec {}
