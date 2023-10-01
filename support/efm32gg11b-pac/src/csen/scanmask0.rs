#[doc = "Register `SCANMASK0` reader"]
pub type R = crate::R<SCANMASK0_SPEC>;
#[doc = "Register `SCANMASK0` writer"]
pub type W = crate::W<SCANMASK0_SPEC>;
#[doc = "Field `SCANINPUTEN` reader - Scan Channel Mask"]
pub type SCANINPUTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Channel Mask"]
pub type SCANINPUTEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    #[must_use]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W<SCANMASK0_SPEC, 0> {
        SCANINPUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scan Channel Mask 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanmask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanmask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANMASK0_SPEC;
impl crate::RegisterSpec for SCANMASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanmask0::R`](R) reader structure"]
impl crate::Readable for SCANMASK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scanmask0::W`](W) writer structure"]
impl crate::Writable for SCANMASK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANMASK0 to value 0"]
impl crate::Resettable for SCANMASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
