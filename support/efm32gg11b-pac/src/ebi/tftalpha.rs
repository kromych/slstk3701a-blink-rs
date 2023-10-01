#[doc = "Register `TFTALPHA` reader"]
pub type R = crate::R<TFTALPHA_SPEC>;
#[doc = "Register `TFTALPHA` writer"]
pub type W = crate::W<TFTALPHA_SPEC>;
#[doc = "Field `ALPHA` reader - TFT Alpha Blending Factor"]
pub type ALPHA_R = crate::FieldReader<u16>;
#[doc = "Field `ALPHA` writer - TFT Alpha Blending Factor"]
pub type ALPHA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<TFTALPHA_SPEC, 0> {
        ALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Alpha Blending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftalpha::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftalpha::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTALPHA_SPEC;
impl crate::RegisterSpec for TFTALPHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftalpha::R`](R) reader structure"]
impl crate::Readable for TFTALPHA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftalpha::W`](W) writer structure"]
impl crate::Writable for TFTALPHA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTALPHA to value 0"]
impl crate::Resettable for TFTALPHA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
