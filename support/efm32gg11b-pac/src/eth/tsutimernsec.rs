#[doc = "Register `TSUTIMERNSEC` reader"]
pub type R = crate::R<TsutimernsecSpec>;
#[doc = "Register `TSUTIMERNSEC` writer"]
pub type W = crate::W<TsutimernsecSpec>;
#[doc = "Field `TIMER` reader - Timer count in nanoseconds"]
pub type TimerR = crate::FieldReader<u32>;
#[doc = "Field `TIMER` writer - Timer count in nanoseconds"]
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, TsutimernsecSpec> {
        TimerW::new(self, 0)
    }
}
#[doc = "1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimernsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimernsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsutimernsecSpec;
impl crate::RegisterSpec for TsutimernsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimernsec::R`](R) reader structure"]
impl crate::Readable for TsutimernsecSpec {}
#[doc = "`write(|w| ..)` method takes [`tsutimernsec::W`](W) writer structure"]
impl crate::Writable for TsutimernsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUTIMERNSEC to value 0"]
impl crate::Resettable for TsutimernsecSpec {}
