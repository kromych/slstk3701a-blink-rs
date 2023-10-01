#[doc = "Register `TFTSTRIDE` reader"]
pub type R = crate::R<TFTSTRIDE_SPEC>;
#[doc = "Register `TFTSTRIDE` writer"]
pub type W = crate::W<TFTSTRIDE_SPEC>;
#[doc = "Field `HSTRIDE` reader - Horizontal Stride"]
pub type HSTRIDE_R = crate::FieldReader<u16>;
#[doc = "Field `HSTRIDE` writer - Horizontal Stride"]
pub type HSTRIDE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&self) -> HSTRIDE_R {
        HSTRIDE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    #[must_use]
    pub fn hstride(&mut self) -> HSTRIDE_W<TFTSTRIDE_SPEC, 0> {
        HSTRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Stride Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftstride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftstride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTSTRIDE_SPEC;
impl crate::RegisterSpec for TFTSTRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftstride::R`](R) reader structure"]
impl crate::Readable for TFTSTRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftstride::W`](W) writer structure"]
impl crate::Writable for TFTSTRIDE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTSTRIDE to value 0"]
impl crate::Resettable for TFTSTRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
