#[doc = "Register `TSUTIMERMSBSEC` reader"]
pub type R = crate::R<TsutimermsbsecSpec>;
#[doc = "Register `TSUTIMERMSBSEC` writer"]
pub type W = crate::W<TsutimermsbsecSpec>;
#[doc = "Field `TIMER` reader - MSB 16 bits of seconds timer count."]
pub type TimerR = crate::FieldReader<u16>;
#[doc = "Field `TIMER` writer - MSB 16 bits of seconds timer count."]
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, TsutimermsbsecSpec> {
        TimerW::new(self, 0)
    }
}
#[doc = "1588 Timer Seconds Register 47:32\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimermsbsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimermsbsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsutimermsbsecSpec;
impl crate::RegisterSpec for TsutimermsbsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimermsbsec::R`](R) reader structure"]
impl crate::Readable for TsutimermsbsecSpec {}
#[doc = "`write(|w| ..)` method takes [`tsutimermsbsec::W`](W) writer structure"]
impl crate::Writable for TsutimermsbsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUTIMERMSBSEC to value 0"]
impl crate::Resettable for TsutimermsbsecSpec {}
