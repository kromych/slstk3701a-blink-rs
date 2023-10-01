#[doc = "Register `ADSADDR` reader"]
pub type R = crate::R<ADSADDR_SPEC>;
#[doc = "Register `ADSADDR` writer"]
pub type W = crate::W<ADSADDR_SPEC>;
#[doc = "Field `ADSADDR` reader - ADMA System Address"]
pub type ADSADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADSADDR` writer - ADMA System Address"]
pub type ADSADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn adsaddr(&self) -> ADSADDR_R {
        ADSADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    #[must_use]
    pub fn adsaddr(&mut self) -> ADSADDR_W<ADSADDR_SPEC, 0> {
        ADSADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADMA System Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSADDR_SPEC;
impl crate::RegisterSpec for ADSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adsaddr::R`](R) reader structure"]
impl crate::Readable for ADSADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adsaddr::W`](W) writer structure"]
impl crate::Writable for ADSADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSADDR to value 0"]
impl crate::Resettable for ADSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
