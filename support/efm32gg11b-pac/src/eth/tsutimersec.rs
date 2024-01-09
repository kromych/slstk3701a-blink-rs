#[doc = "Register `TSUTIMERSEC` reader"]
pub type R = crate::R<TSUTIMERSEC_SPEC>;
#[doc = "Register `TSUTIMERSEC` writer"]
pub type W = crate::W<TSUTIMERSEC_SPEC>;
#[doc = "Field `TIMER` reader - 1588 Timer Seconds Register"]
pub type TIMER_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER` writer - 1588 Timer Seconds Register"]
pub type TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<TSUTIMERSEC_SPEC> {
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
#[doc = "1588 Timer Seconds Register 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimersec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimersec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUTIMERSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimersec::R`](R) reader structure"]
impl crate::Readable for TSUTIMERSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsutimersec::W`](W) writer structure"]
impl crate::Writable for TSUTIMERSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSUTIMERSEC to value 0"]
impl crate::Resettable for TSUTIMERSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
