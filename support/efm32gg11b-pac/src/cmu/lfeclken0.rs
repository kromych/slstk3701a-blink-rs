#[doc = "Register `LFECLKEN0` reader"]
pub type R = crate::R<LFECLKEN0_SPEC>;
#[doc = "Register `LFECLKEN0` writer"]
pub type W = crate::W<LFECLKEN0_SPEC>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Clock Enable"]
pub type RTCC_R = crate::BitReader;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar Clock Enable"]
pub type RTCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcc(&mut self) -> RTCC_W<LFECLKEN0_SPEC, 0> {
        RTCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfeclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfeclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFECLKEN0_SPEC;
impl crate::RegisterSpec for LFECLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfeclken0::R`](R) reader structure"]
impl crate::Readable for LFECLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfeclken0::W`](W) writer structure"]
impl crate::Writable for LFECLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFECLKEN0 to value 0"]
impl crate::Resettable for LFECLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
