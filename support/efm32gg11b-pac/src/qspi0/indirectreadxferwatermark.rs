#[doc = "Register `INDIRECTREADXFERWATERMARK` reader"]
pub type R = crate::R<IndirectreadxferwatermarkSpec>;
#[doc = "Register `INDIRECTREADXFERWATERMARK` writer"]
pub type W = crate::W<IndirectreadxferwatermarkSpec>;
#[doc = "Field `LEVEL` reader - Watermark Value"]
pub type LevelR = crate::FieldReader<u32>;
#[doc = "Field `LEVEL` writer - Watermark Value"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, IndirectreadxferwatermarkSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Indirect Read Transfer Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxferwatermark::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxferwatermark::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectreadxferwatermarkSpec;
impl crate::RegisterSpec for IndirectreadxferwatermarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectreadxferwatermark::R`](R) reader structure"]
impl crate::Readable for IndirectreadxferwatermarkSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectreadxferwatermark::W`](W) writer structure"]
impl crate::Writable for IndirectreadxferwatermarkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTREADXFERWATERMARK to value 0"]
impl crate::Resettable for IndirectreadxferwatermarkSpec {}
