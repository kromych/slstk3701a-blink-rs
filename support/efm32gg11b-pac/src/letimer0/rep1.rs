#[doc = "Register `REP1` reader"]
pub type R = crate::R<REP1_SPEC>;
#[doc = "Register `REP1` writer"]
pub type W = crate::W<REP1_SPEC>;
#[doc = "Field `REP1` reader - Repeat Counter 1"]
pub type REP1_R = crate::FieldReader;
#[doc = "Field `REP1` writer - Repeat Counter 1"]
pub type REP1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 1"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 1"]
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> REP1_W<REP1_SPEC> {
        REP1_W::new(self, 0)
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
#[doc = "Repeat Counter Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rep1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rep1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REP1_SPEC;
impl crate::RegisterSpec for REP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rep1::R`](R) reader structure"]
impl crate::Readable for REP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rep1::W`](W) writer structure"]
impl crate::Writable for REP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REP1 to value 0"]
impl crate::Resettable for REP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
