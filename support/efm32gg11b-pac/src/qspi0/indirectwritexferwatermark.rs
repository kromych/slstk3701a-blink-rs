#[doc = "Register `INDIRECTWRITEXFERWATERMARK` reader"]
pub type R = crate::R<IndirectwritexferwatermarkSpec>;
#[doc = "Register `INDIRECTWRITEXFERWATERMARK` writer"]
pub type W = crate::W<IndirectwritexferwatermarkSpec>;
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
    pub fn level(&mut self) -> LevelW<'_, IndirectwritexferwatermarkSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Indirect Write Transfer Watermark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexferwatermark::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexferwatermark::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectwritexferwatermarkSpec;
impl crate::RegisterSpec for IndirectwritexferwatermarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexferwatermark::R`](R) reader structure"]
impl crate::Readable for IndirectwritexferwatermarkSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexferwatermark::W`](W) writer structure"]
impl crate::Writable for IndirectwritexferwatermarkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERWATERMARK to value 0xffff_ffff"]
impl crate::Resettable for IndirectwritexferwatermarkSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
