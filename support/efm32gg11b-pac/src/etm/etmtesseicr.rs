#[doc = "Register `ETMTESSEICR` reader"]
pub type R = crate::R<ETMTESSEICR_SPEC>;
#[doc = "Register `ETMTESSEICR` writer"]
pub type W = crate::W<ETMTESSEICR_SPEC>;
#[doc = "Field `STARTRSEL` reader - Stop Resource Selection"]
pub type STARTRSEL_R = crate::FieldReader;
#[doc = "Field `STARTRSEL` writer - Stop Resource Selection"]
pub type STARTRSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `STOPRSEL` reader - Stop Resource Selection"]
pub type STOPRSEL_R = crate::FieldReader;
#[doc = "Field `STOPRSEL` writer - Stop Resource Selection"]
pub type STOPRSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&self) -> STARTRSEL_R {
        STARTRSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&self) -> STOPRSEL_R {
        STOPRSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startrsel(&mut self) -> STARTRSEL_W<ETMTESSEICR_SPEC, 0> {
        STARTRSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stoprsel(&mut self) -> STOPRSEL_W<ETMTESSEICR_SPEC, 16> {
        STOPRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtesseicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtesseicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMTESSEICR_SPEC;
impl crate::RegisterSpec for ETMTESSEICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtesseicr::R`](R) reader structure"]
impl crate::Readable for ETMTESSEICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmtesseicr::W`](W) writer structure"]
impl crate::Writable for ETMTESSEICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMTESSEICR to value 0"]
impl crate::Resettable for ETMTESSEICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
