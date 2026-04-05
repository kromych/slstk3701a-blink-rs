#[doc = "Register `PERIODSEL` reader"]
pub type R = crate::R<PeriodselSpec>;
#[doc = "Register `PERIODSEL` writer"]
pub type W = crate::W<PeriodselSpec>;
#[doc = "Field `PERIODSEL` reader - Interrupts/Wakeup Events Period Setting"]
pub type PeriodselR = crate::FieldReader;
#[doc = "Field `PERIODSEL` writer - Interrupts/Wakeup Events Period Setting"]
pub type PeriodselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    pub fn periodsel(&self) -> PeriodselR {
        PeriodselR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    pub fn periodsel(&mut self) -> PeriodselW<'_, PeriodselSpec> {
        PeriodselW::new(self, 0)
    }
}
#[doc = "Interrupt Duration\n\nYou can [`read`](crate::Reg::read) this register and get [`periodsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periodsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodselSpec;
impl crate::RegisterSpec for PeriodselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periodsel::R`](R) reader structure"]
impl crate::Readable for PeriodselSpec {}
#[doc = "`write(|w| ..)` method takes [`periodsel::W`](W) writer structure"]
impl crate::Writable for PeriodselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIODSEL to value 0x20"]
impl crate::Resettable for PeriodselSpec {
    const RESET_VALUE: u32 = 0x20;
}
