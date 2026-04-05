#[doc = "Register `LFACLKEN0` reader"]
pub type R = crate::R<Lfaclken0Spec>;
#[doc = "Register `LFACLKEN0` writer"]
pub type W = crate::W<Lfaclken0Spec>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 Clock Enable"]
pub type Letimer0R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 Clock Enable"]
pub type Letimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1` reader - Low Energy Timer 1 Clock Enable"]
pub type Letimer1R = crate::BitReader;
#[doc = "Field `LETIMER1` writer - Low Energy Timer 1 Clock Enable"]
pub type Letimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface Clock Enable"]
pub type LesenseR = crate::BitReader;
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface Clock Enable"]
pub type LesenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller Clock Enable"]
pub type LcdR = crate::BitReader;
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller Clock Enable"]
pub type LcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Real-Time Counter Clock Enable"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - Real-Time Counter Clock Enable"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&self) -> Letimer0R {
        Letimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Energy Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn letimer1(&self) -> Letimer1R {
        Letimer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Sensor Interface Clock Enable"]
    #[inline(always)]
    pub fn lesense(&self) -> LesenseR {
        LesenseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Liquid Crystal Display Controller Clock Enable"]
    #[inline(always)]
    pub fn lcd(&self) -> LcdR {
        LcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> Letimer0W<'_, Lfaclken0Spec> {
        Letimer0W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Energy Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> Letimer1W<'_, Lfaclken0Spec> {
        Letimer1W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Energy Sensor Interface Clock Enable"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LesenseW<'_, Lfaclken0Spec> {
        LesenseW::new(self, 2)
    }
    #[doc = "Bit 3 - Liquid Crystal Display Controller Clock Enable"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LcdW<'_, Lfaclken0Spec> {
        LcdW::new(self, 3)
    }
    #[doc = "Bit 4 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Lfaclken0Spec> {
        RtcW::new(self, 4)
    }
}
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfaclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfaclken0Spec;
impl crate::RegisterSpec for Lfaclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclken0::R`](R) reader structure"]
impl crate::Readable for Lfaclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfaclken0::W`](W) writer structure"]
impl crate::Writable for Lfaclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFACLKEN0 to value 0"]
impl crate::Resettable for Lfaclken0Spec {}
