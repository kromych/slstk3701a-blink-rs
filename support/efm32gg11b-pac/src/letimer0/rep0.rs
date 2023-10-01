#[doc = "Register `REP0` reader"]
pub type R = crate::R<REP0_SPEC>;
#[doc = "Register `REP0` writer"]
pub type W = crate::W<REP0_SPEC>;
#[doc = "Field `REP0` reader - Repeat Counter 0"]
pub type REP0_R = crate::FieldReader;
#[doc = "Field `REP0` writer - Repeat Counter 0"]
pub type REP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> REP0_W<REP0_SPEC, 0> {
        REP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Repeat Counter Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rep0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rep0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REP0_SPEC;
impl crate::RegisterSpec for REP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rep0::R`](R) reader structure"]
impl crate::Readable for REP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rep0::W`](W) writer structure"]
impl crate::Writable for REP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REP0 to value 0"]
impl crate::Resettable for REP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
