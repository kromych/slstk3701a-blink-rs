#[doc = "Register `CH6_LOOP` reader"]
pub type R = crate::R<CH6_LOOP_SPEC>;
#[doc = "Register `CH6_LOOP` writer"]
pub type W = crate::W<CH6_LOOP_SPEC>;
#[doc = "Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_R = crate::FieldReader;
#[doc = "Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LOOPCNT_W<CH6_LOOP_SPEC, 0> {
        LOOPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_loop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_loop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6_LOOP_SPEC;
impl crate::RegisterSpec for CH6_LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_loop::R`](R) reader structure"]
impl crate::Readable for CH6_LOOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch6_loop::W`](W) writer structure"]
impl crate::Writable for CH6_LOOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6_LOOP to value 0"]
impl crate::Resettable for CH6_LOOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
