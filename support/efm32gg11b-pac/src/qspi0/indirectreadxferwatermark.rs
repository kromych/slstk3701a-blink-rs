#[doc = "Register `INDIRECTREADXFERWATERMARK` reader"]
pub type R = crate::R<INDIRECTREADXFERWATERMARK_SPEC>;
#[doc = "Register `INDIRECTREADXFERWATERMARK` writer"]
pub type W = crate::W<INDIRECTREADXFERWATERMARK_SPEC>;
#[doc = "Field `LEVEL` reader - Watermark Value"]
pub type LEVEL_R = crate::FieldReader<u32>;
#[doc = "Field `LEVEL` writer - Watermark Value"]
pub type LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<INDIRECTREADXFERWATERMARK_SPEC> {
        LEVEL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indirect Read Transfer Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectreadxferwatermark::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectreadxferwatermark::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECTREADXFERWATERMARK_SPEC;
impl crate::RegisterSpec for INDIRECTREADXFERWATERMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectreadxferwatermark::R`](R) reader structure"]
impl crate::Readable for INDIRECTREADXFERWATERMARK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirectreadxferwatermark::W`](W) writer structure"]
impl crate::Writable for INDIRECTREADXFERWATERMARK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INDIRECTREADXFERWATERMARK to value 0"]
impl crate::Resettable for INDIRECTREADXFERWATERMARK_SPEC {
    const RESET_VALUE: u32 = 0;
}
