#[doc = "Register `TSUTIMERSEC` reader"]
pub type R = crate::R<TsutimersecSpec>;
#[doc = "Register `TSUTIMERSEC` writer"]
pub type W = crate::W<TsutimersecSpec>;
#[doc = "Field `TIMER` reader - 1588 Timer Seconds Register"]
pub type TimerR = crate::FieldReader<u32>;
#[doc = "Field `TIMER` writer - 1588 Timer Seconds Register"]
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register"]
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, TsutimersecSpec> {
        TimerW::new(self, 0)
    }
}
#[doc = "1588 Timer Seconds Register 31:0\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimersec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimersec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsutimersecSpec;
impl crate::RegisterSpec for TsutimersecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimersec::R`](R) reader structure"]
impl crate::Readable for TsutimersecSpec {}
#[doc = "`write(|w| ..)` method takes [`tsutimersec::W`](W) writer structure"]
impl crate::Writable for TsutimersecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUTIMERSEC to value 0"]
impl crate::Resettable for TsutimersecSpec {}
