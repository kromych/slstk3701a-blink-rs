#[doc = "Register `TFTDD` reader"]
pub type R = crate::R<TftddSpec>;
#[doc = "Register `TFTDD` writer"]
pub type W = crate::W<TftddSpec>;
#[doc = "Field `DATA` reader - TFT Direct Drive Data From Internal Memory"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - TFT Direct Drive Data From Internal Memory"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - TFT Direct Drive Data From Internal Memory"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - TFT Direct Drive Data From Internal Memory"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, TftddSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "TFT Direct Drive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftdd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftdd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftddSpec;
impl crate::RegisterSpec for TftddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftdd::R`](R) reader structure"]
impl crate::Readable for TftddSpec {}
#[doc = "`write(|w| ..)` method takes [`tftdd::W`](W) writer structure"]
impl crate::Writable for TftddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTDD to value 0"]
impl crate::Resettable for TftddSpec {}
