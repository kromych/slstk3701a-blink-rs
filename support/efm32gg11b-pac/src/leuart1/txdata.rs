#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TXDATA_SPEC>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TXDATA_SPEC>;
#[doc = "Field `TXDATA` reader - TX Data"]
pub type TXDATA_R = crate::FieldReader;
#[doc = "Field `TXDATA` writer - TX Data"]
pub type TXDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<TXDATA_SPEC, 0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TXDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
