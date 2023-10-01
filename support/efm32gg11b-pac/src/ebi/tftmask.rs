#[doc = "Register `TFTMASK` reader"]
pub type R = crate::R<TFTMASK_SPEC>;
#[doc = "Register `TFTMASK` writer"]
pub type W = crate::W<TFTMASK_SPEC>;
#[doc = "Field `TFTMASK` reader - TFT Mask Value"]
pub type TFTMASK_R = crate::FieldReader<u32>;
#[doc = "Field `TFTMASK` writer - TFT Mask Value"]
pub type TFTMASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&self) -> TFTMASK_R {
        TFTMASK_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - TFT Mask Value"]
    #[inline(always)]
    #[must_use]
    pub fn tftmask(&mut self) -> TFTMASK_W<TFTMASK_SPEC, 0> {
        TFTMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Masking Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTMASK_SPEC;
impl crate::RegisterSpec for TFTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftmask::R`](R) reader structure"]
impl crate::Readable for TFTMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftmask::W`](W) writer structure"]
impl crate::Writable for TFTMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTMASK to value 0"]
impl crate::Resettable for TFTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
