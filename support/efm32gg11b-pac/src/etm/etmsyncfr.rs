#[doc = "Register `ETMSYNCFR` reader"]
pub type R = crate::R<ETMSYNCFR_SPEC>;
#[doc = "Register `ETMSYNCFR` writer"]
pub type W = crate::W<ETMSYNCFR_SPEC>;
#[doc = "Field `FREQ` reader - Synchronisation Frequency Value"]
pub type FREQ_R = crate::FieldReader<u16>;
#[doc = "Field `FREQ` writer - Synchronisation Frequency Value"]
pub type FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<ETMSYNCFR_SPEC, 0> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Synchronisation Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmsyncfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmsyncfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMSYNCFR_SPEC;
impl crate::RegisterSpec for ETMSYNCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmsyncfr::R`](R) reader structure"]
impl crate::Readable for ETMSYNCFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmsyncfr::W`](W) writer structure"]
impl crate::Writable for ETMSYNCFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMSYNCFR to value 0x0400"]
impl crate::Resettable for ETMSYNCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
