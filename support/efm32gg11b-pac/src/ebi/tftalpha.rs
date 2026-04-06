#[doc = "Register `TFTALPHA` reader"]
pub type R = crate::R<TftalphaSpec>;
#[doc = "Register `TFTALPHA` writer"]
pub type W = crate::W<TftalphaSpec>;
#[doc = "Field `ALPHA` reader - TFT Alpha Blending Factor"]
pub type AlphaR = crate::FieldReader<u16>;
#[doc = "Field `ALPHA` writer - TFT Alpha Blending Factor"]
pub type AlphaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    pub fn alpha(&self) -> AlphaR {
        AlphaR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    pub fn alpha(&mut self) -> AlphaW<'_, TftalphaSpec> {
        AlphaW::new(self, 0)
    }
}
#[doc = "TFT Alpha Blending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftalpha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftalpha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftalphaSpec;
impl crate::RegisterSpec for TftalphaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftalpha::R`](R) reader structure"]
impl crate::Readable for TftalphaSpec {}
#[doc = "`write(|w| ..)` method takes [`tftalpha::W`](W) writer structure"]
impl crate::Writable for TftalphaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTALPHA to value 0"]
impl crate::Resettable for TftalphaSpec {}
