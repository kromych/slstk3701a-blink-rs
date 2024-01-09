#[doc = "Register `TSUTIMERNSEC` reader"]
pub type R = crate::R<TSUTIMERNSEC_SPEC>;
#[doc = "Register `TSUTIMERNSEC` writer"]
pub type W = crate::W<TSUTIMERNSEC_SPEC>;
#[doc = "Field `TIMER` reader - Timer count in nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER` writer - Timer count in nanoseconds"]
pub type TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<TSUTIMERNSEC_SPEC> {
        TIMER_W::new(self, 0)
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
#[doc = "1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimernsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimernsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUTIMERNSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimernsec::R`](R) reader structure"]
impl crate::Readable for TSUTIMERNSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsutimernsec::W`](W) writer structure"]
impl crate::Writable for TSUTIMERNSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSUTIMERNSEC to value 0"]
impl crate::Resettable for TSUTIMERNSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
