#[doc = "Register `RXTHRESH` reader"]
pub type R = crate::R<RxthreshSpec>;
#[doc = "Register `RXTHRESH` writer"]
pub type W = crate::W<RxthreshSpec>;
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, RxthreshSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "RX Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxthresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxthresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxthreshSpec;
impl crate::RegisterSpec for RxthreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxthresh::R`](R) reader structure"]
impl crate::Readable for RxthreshSpec {}
#[doc = "`write(|w| ..)` method takes [`rxthresh::W`](W) writer structure"]
impl crate::Writable for RxthreshSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTHRESH to value 0x01"]
impl crate::Resettable for RxthreshSpec {
    const RESET_VALUE: u32 = 0x01;
}
