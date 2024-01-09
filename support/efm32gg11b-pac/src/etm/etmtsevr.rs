#[doc = "Register `ETMTSEVR` reader"]
pub type R = crate::R<ETMTSEVR_SPEC>;
#[doc = "Register `ETMTSEVR` writer"]
pub type W = crate::W<ETMTSEVR_SPEC>;
#[doc = "Field `RESAEVT` reader - ETM Resource A Event"]
pub type RESAEVT_R = crate::FieldReader;
#[doc = "Field `RESAEVT` writer - ETM Resource A Event"]
pub type RESAEVT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESBEVT` reader - ETM Resource B Event"]
pub type RESBEVT_R = crate::FieldReader;
#[doc = "Field `RESBEVT` writer - ETM Resource B Event"]
pub type RESBEVT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ETMFCNEVT` reader - ETM Function Event"]
pub type ETMFCNEVT_R = crate::FieldReader;
#[doc = "Field `ETMFCNEVT` writer - ETM Function Event"]
pub type ETMFCNEVT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    pub fn resaevt(&self) -> RESAEVT_R {
        RESAEVT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    pub fn resbevt(&self) -> RESBEVT_R {
        RESBEVT_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    pub fn etmfcnevt(&self) -> ETMFCNEVT_R {
        ETMFCNEVT_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    #[must_use]
    pub fn resaevt(&mut self) -> RESAEVT_W<ETMTSEVR_SPEC> {
        RESAEVT_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    #[must_use]
    pub fn resbevt(&mut self) -> RESBEVT_W<ETMTSEVR_SPEC> {
        RESBEVT_W::new(self, 7)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    #[must_use]
    pub fn etmfcnevt(&mut self) -> ETMFCNEVT_W<ETMTSEVR_SPEC> {
        ETMFCNEVT_W::new(self, 14)
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
#[doc = "Timestamp Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtsevr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtsevr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMTSEVR_SPEC;
impl crate::RegisterSpec for ETMTSEVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtsevr::R`](R) reader structure"]
impl crate::Readable for ETMTSEVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmtsevr::W`](W) writer structure"]
impl crate::Writable for ETMTSEVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMTSEVR to value 0"]
impl crate::Resettable for ETMTSEVR_SPEC {
    const RESET_VALUE: u32 = 0;
}
