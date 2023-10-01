#[doc = "Register `ETMTEEVR` reader"]
pub type R = crate::R<ETMTEEVR_SPEC>;
#[doc = "Register `ETMTEEVR` writer"]
pub type W = crate::W<ETMTEEVR_SPEC>;
#[doc = "Field `RESA` reader - ETM Resource A Trace Enable"]
pub type RESA_R = crate::FieldReader;
#[doc = "Field `RESA` writer - ETM Resource A Trace Enable"]
pub type RESA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `RESB` reader - ETM Resource B Trace Enable"]
pub type RESB_R = crate::FieldReader;
#[doc = "Field `RESB` writer - ETM Resource B Trace Enable"]
pub type RESB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ETMFCNEN` reader - ETM Function Trace Enable"]
pub type ETMFCNEN_R = crate::FieldReader;
#[doc = "Field `ETMFCNEN` writer - ETM Function Trace Enable"]
pub type ETMFCNEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    pub fn resa(&self) -> RESA_R {
        RESA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    pub fn resb(&self) -> RESB_R {
        RESB_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    pub fn etmfcnen(&self) -> ETMFCNEN_R {
        ETMFCNEN_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resa(&mut self) -> RESA_W<ETMTEEVR_SPEC, 0> {
        RESA_W::new(self)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resb(&mut self) -> RESB_W<ETMTEEVR_SPEC, 7> {
        RESB_W::new(self)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmfcnen(&mut self) -> ETMFCNEN_W<ETMTEEVR_SPEC, 14> {
        ETMFCNEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ETM TraceEnable Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmteevr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmteevr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMTEEVR_SPEC;
impl crate::RegisterSpec for ETMTEEVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmteevr::R`](R) reader structure"]
impl crate::Readable for ETMTEEVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmteevr::W`](W) writer structure"]
impl crate::Writable for ETMTEEVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMTEEVR to value 0"]
impl crate::Resettable for ETMTEEVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
