#[doc = "Register `TSUTIMERINCR` reader"]
pub type R = crate::R<TSUTIMERINCR_SPEC>;
#[doc = "Register `TSUTIMERINCR` writer"]
pub type W = crate::W<TSUTIMERINCR_SPEC>;
#[doc = "Field `NSINCREMENT` reader - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
pub type NSINCREMENT_R = crate::FieldReader;
#[doc = "Field `NSINCREMENT` writer - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
pub type NSINCREMENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ALTNSINCR` reader - Alternative nanoseconds count"]
pub type ALTNSINCR_R = crate::FieldReader;
#[doc = "Field `ALTNSINCR` writer - Alternative nanoseconds count"]
pub type ALTNSINCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `NUMINCS` reader - Number of incs before alt inc"]
pub type NUMINCS_R = crate::FieldReader;
#[doc = "Field `NUMINCS` writer - Number of incs before alt inc"]
pub type NUMINCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&self) -> NSINCREMENT_R {
        NSINCREMENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&self) -> ALTNSINCR_R {
        ALTNSINCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&self) -> NUMINCS_R {
        NUMINCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn nsincrement(&mut self) -> NSINCREMENT_W<TSUTIMERINCR_SPEC, 0> {
        NSINCREMENT_W::new(self)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    #[must_use]
    pub fn altnsincr(&mut self) -> ALTNSINCR_W<TSUTIMERINCR_SPEC, 8> {
        ALTNSINCR_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    #[must_use]
    pub fn numincs(&mut self) -> NUMINCS_W<TSUTIMERINCR_SPEC, 16> {
        NUMINCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "1588 Timer Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimerincr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimerincr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUTIMERINCR_SPEC;
impl crate::RegisterSpec for TSUTIMERINCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimerincr::R`](R) reader structure"]
impl crate::Readable for TSUTIMERINCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsutimerincr::W`](W) writer structure"]
impl crate::Writable for TSUTIMERINCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSUTIMERINCR to value 0"]
impl crate::Resettable for TSUTIMERINCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
