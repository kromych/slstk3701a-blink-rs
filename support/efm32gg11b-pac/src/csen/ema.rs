#[doc = "Register `EMA` reader"]
pub type R = crate::R<EmaSpec>;
#[doc = "Register `EMA` writer"]
pub type W = crate::W<EmaSpec>;
#[doc = "Field `EMA` reader - Calculated Exponential Moving Average"]
pub type EmaR = crate::FieldReader<u32>;
#[doc = "Field `EMA` writer - Calculated Exponential Moving Average"]
pub type EmaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&self) -> EmaR {
        EmaR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&mut self) -> EmaW<'_, EmaSpec> {
        EmaW::new(self, 0)
    }
}
#[doc = "Exponential Moving Average\n\nYou can [`read`](crate::Reg::read) this register and get [`ema::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ema::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmaSpec;
impl crate::RegisterSpec for EmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ema::R`](R) reader structure"]
impl crate::Readable for EmaSpec {}
#[doc = "`write(|w| ..)` method takes [`ema::W`](W) writer structure"]
impl crate::Writable for EmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMA to value 0"]
impl crate::Resettable for EmaSpec {}
