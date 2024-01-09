#[doc = "Register `TSUTIMERMSBSEC` reader"]
pub type R = crate::R<TSUTIMERMSBSEC_SPEC>;
#[doc = "Register `TSUTIMERMSBSEC` writer"]
pub type W = crate::W<TSUTIMERMSBSEC_SPEC>;
#[doc = "Field `TIMER` reader - MSB 16 bits of seconds timer count."]
pub type TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER` writer - MSB 16 bits of seconds timer count."]
pub type TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<TSUTIMERMSBSEC_SPEC> {
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
#[doc = "1588 Timer Seconds Register 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimermsbsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimermsbsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUTIMERMSBSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimermsbsec::R`](R) reader structure"]
impl crate::Readable for TSUTIMERMSBSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsutimermsbsec::W`](W) writer structure"]
impl crate::Writable for TSUTIMERMSBSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSUTIMERMSBSEC to value 0"]
impl crate::Resettable for TSUTIMERMSBSEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
