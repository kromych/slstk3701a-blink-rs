#[doc = "Register `SINGLEFIFOCLEAR` writer"]
pub type W = crate::W<SINGLEFIFOCLEAR_SPEC>;
#[doc = "Field `SINGLEFIFOCLEAR` writer - Clear Single FIFO Content"]
pub type SINGLEFIFOCLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear Single FIFO Content"]
    #[inline(always)]
    #[must_use]
    pub fn singlefifoclear(&mut self) -> SINGLEFIFOCLEAR_W<SINGLEFIFOCLEAR_SPEC, 0> {
        SINGLEFIFOCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Single FIFO Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlefifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLEFIFOCLEAR_SPEC;
impl crate::RegisterSpec for SINGLEFIFOCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`singlefifoclear::W`](W) writer structure"]
impl crate::Writable for SINGLEFIFOCLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINGLEFIFOCLEAR to value 0"]
impl crate::Resettable for SINGLEFIFOCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
