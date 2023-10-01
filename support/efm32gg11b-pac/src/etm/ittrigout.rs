#[doc = "Register `ITTRIGOUT` reader"]
pub type R = crate::R<ITTRIGOUT_SPEC>;
#[doc = "Register `ITTRIGOUT` writer"]
pub type W = crate::W<ITTRIGOUT_SPEC>;
#[doc = "Field `TRIGGEROUT` reader - Trigger output value"]
pub type TRIGGEROUT_R = crate::BitReader;
#[doc = "Field `TRIGGEROUT` writer - Trigger output value"]
pub type TRIGGEROUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    pub fn triggerout(&self) -> TRIGGEROUT_R {
        TRIGGEROUT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output value"]
    #[inline(always)]
    #[must_use]
    pub fn triggerout(&mut self) -> TRIGGEROUT_W<ITTRIGOUT_SPEC, 0> {
        TRIGGEROUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Integration Test Trigger Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ittrigout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ittrigout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITTRIGOUT_SPEC;
impl crate::RegisterSpec for ITTRIGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ittrigout::R`](R) reader structure"]
impl crate::Readable for ITTRIGOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ittrigout::W`](W) writer structure"]
impl crate::Writable for ITTRIGOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITTRIGOUT to value 0"]
impl crate::Resettable for ITTRIGOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
