#[doc = "Register `LFACLKEN0` reader"]
pub type R = crate::R<LFACLKEN0_SPEC>;
#[doc = "Register `LFACLKEN0` writer"]
pub type W = crate::W<LFACLKEN0_SPEC>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 Clock Enable"]
pub type LETIMER0_R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 Clock Enable"]
pub type LETIMER0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1` reader - Low Energy Timer 1 Clock Enable"]
pub type LETIMER1_R = crate::BitReader;
#[doc = "Field `LETIMER1` writer - Low Energy Timer 1 Clock Enable"]
pub type LETIMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface Clock Enable"]
pub type LESENSE_R = crate::BitReader;
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface Clock Enable"]
pub type LESENSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller Clock Enable"]
pub type LCD_R = crate::BitReader;
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller Clock Enable"]
pub type LCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Real-Time Counter Clock Enable"]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - Real-Time Counter Clock Enable"]
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Energy Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn letimer1(&self) -> LETIMER1_R {
        LETIMER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Sensor Interface Clock Enable"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Liquid Crystal Display Controller Clock Enable"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> LETIMER0_W<LFACLKEN0_SPEC> {
        LETIMER0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Energy Timer 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn letimer1(&mut self) -> LETIMER1_W<LFACLKEN0_SPEC> {
        LETIMER1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Energy Sensor Interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lesense(&mut self) -> LESENSE_W<LFACLKEN0_SPEC> {
        LESENSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Liquid Crystal Display Controller Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcd(&mut self) -> LCD_W<LFACLKEN0_SPEC> {
        LCD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<LFACLKEN0_SPEC> {
        RTC_W::new(self, 4)
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
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfaclken0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfaclken0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFACLKEN0_SPEC;
impl crate::RegisterSpec for LFACLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfaclken0::R`](R) reader structure"]
impl crate::Readable for LFACLKEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfaclken0::W`](W) writer structure"]
impl crate::Writable for LFACLKEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFACLKEN0 to value 0"]
impl crate::Resettable for LFACLKEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
