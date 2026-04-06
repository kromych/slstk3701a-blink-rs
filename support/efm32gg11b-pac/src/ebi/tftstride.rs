#[doc = "Register `TFTSTRIDE` reader"]
pub type R = crate::R<TftstrideSpec>;
#[doc = "Register `TFTSTRIDE` writer"]
pub type W = crate::W<TftstrideSpec>;
#[doc = "Field `HSTRIDE` reader - Horizontal Stride"]
pub type HstrideR = crate::FieldReader<u16>;
#[doc = "Field `HSTRIDE` writer - Horizontal Stride"]
pub type HstrideW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&self) -> HstrideR {
        HstrideR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&mut self) -> HstrideW<'_, TftstrideSpec> {
        HstrideW::new(self, 0)
    }
}
#[doc = "TFT Stride Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftstride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftstride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftstrideSpec;
impl crate::RegisterSpec for TftstrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftstride::R`](R) reader structure"]
impl crate::Readable for TftstrideSpec {}
#[doc = "`write(|w| ..)` method takes [`tftstride::W`](W) writer structure"]
impl crate::Writable for TftstrideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTSTRIDE to value 0"]
impl crate::Resettable for TftstrideSpec {}
