#[doc = "Register `EM23PERNORETAINCMD` writer"]
pub type W = crate::W<EM23PERNORETAINCMD_SPEC>;
#[doc = "Field `ACMP0UNLOCK` writer - Clears Status Bit of ACMP0 and Unlocks Access to It"]
pub type ACMP0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1UNLOCK` writer - Clears Status Bit of ACMP1 and Unlocks Access to It"]
pub type ACMP1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0UNLOCK` writer - Clears Status Bit of PCNT0 and Unlocks Access to It"]
pub type PCNT0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1UNLOCK` writer - Clears Status Bit of PCNT1 and Unlocks Access to It"]
pub type PCNT1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2UNLOCK` writer - Clears Status Bit of PCNT2 and Unlocks Access to It"]
pub type PCNT2UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0UNLOCK` writer - Clears Status Bit of I2C0 and Unlocks Access to It"]
pub type I2C0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1UNLOCK` writer - Clears Status Bit of I2C1 and Unlocks Access to It"]
pub type I2C1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC0UNLOCK` writer - Clears Status Bit of DAC0 and Unlocks Access to It"]
pub type DAC0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0UNLOCK` writer - Clears Status Bit of IDAC0 and Unlocks Access to It"]
pub type IDAC0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0UNLOCK` writer - Clears Status Bit of ADC0 and Unlocks Access to It"]
pub type ADC0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0UNLOCK` writer - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
pub type LETIMER0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0UNLOCK` writer - Clears Status Bit of WDOG0 and Unlocks Access to It"]
pub type WDOG0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1UNLOCK` writer - Clears Status Bit of WDOG1 and Unlocks Access to It"]
pub type WDOG1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE0UNLOCK` writer - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
pub type LESENSE0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSENUNLOCK` writer - Clears Status Bit of CSEN and Unlocks Access to It"]
pub type CSENUNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART0UNLOCK` writer - Clears Status Bit of LEUART0 and Unlocks Access to It"]
pub type LEUART0UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1UNLOCK` writer - Clears Status Bit of LEUART1 and Unlocks Access to It"]
pub type LEUART1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDUNLOCK` writer - Clears Status Bit of LCD and Unlocks Access to It"]
pub type LCDUNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1UNLOCK` writer - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
pub type LETIMER1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2UNLOCK` writer - Clears Status Bit of I2C2 and Unlocks Access to It"]
pub type I2C2UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1UNLOCK` writer - Clears Status Bit of ADC1 and Unlocks Access to It"]
pub type ADC1UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2UNLOCK` writer - Clears Status Bit of ACMP2 and Unlocks Access to It"]
pub type ACMP2UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3UNLOCK` writer - Clears Status Bit of ACMP3 and Unlocks Access to It"]
pub type ACMP3UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCUNLOCK` writer - Clears Status Bit of RTC and Unlocks Access to It"]
pub type RTCUNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBUNLOCK` writer - Clears Status Bit of USB and Unlocks Access to It"]
pub type USBUNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0unlock(&mut self) -> ACMP0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        ACMP0UNLOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1unlock(&mut self) -> ACMP1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        ACMP1UNLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0unlock(&mut self) -> PCNT0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        PCNT0UNLOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clears Status Bit of PCNT1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1unlock(&mut self) -> PCNT1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        PCNT1UNLOCK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clears Status Bit of PCNT2 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2unlock(&mut self) -> PCNT2UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        PCNT2UNLOCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0unlock(&mut self) -> I2C0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        I2C0UNLOCK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1unlock(&mut self) -> I2C1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        I2C1UNLOCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn dac0unlock(&mut self) -> DAC0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        DAC0UNLOCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clears Status Bit of IDAC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn idac0unlock(&mut self) -> IDAC0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        IDAC0UNLOCK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn adc0unlock(&mut self) -> ADC0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        ADC0UNLOCK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0unlock(&mut self) -> LETIMER0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        LETIMER0UNLOCK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0unlock(&mut self) -> WDOG0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        WDOG0UNLOCK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clears Status Bit of WDOG1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1unlock(&mut self) -> WDOG1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        WDOG1UNLOCK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn lesense0unlock(&mut self) -> LESENSE0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        LESENSE0UNLOCK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn csenunlock(&mut self) -> CSENUNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        CSENUNLOCK_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0unlock(&mut self) -> LEUART0UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        LEUART0UNLOCK_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clears Status Bit of LEUART1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn leuart1unlock(&mut self) -> LEUART1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        LEUART1UNLOCK_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn lcdunlock(&mut self) -> LCDUNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        LCDUNLOCK_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn letimer1unlock(&mut self) -> LETIMER1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        LETIMER1UNLOCK_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clears Status Bit of I2C2 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2unlock(&mut self) -> I2C2UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        I2C2UNLOCK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clears Status Bit of ADC1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn adc1unlock(&mut self) -> ADC1UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        ADC1UNLOCK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clears Status Bit of ACMP2 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2unlock(&mut self) -> ACMP2UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        ACMP2UNLOCK_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clears Status Bit of ACMP3 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3unlock(&mut self) -> ACMP3UNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        ACMP3UNLOCK_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clears Status Bit of RTC and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn rtcunlock(&mut self) -> RTCUNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        RTCUNLOCK_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clears Status Bit of USB and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn usbunlock(&mut self) -> USBUNLOCK_W<EM23PERNORETAINCMD_SPEC> {
        USBUNLOCK_W::new(self, 24)
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
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em23pernoretaincmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM23PERNORETAINCMD_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`em23pernoretaincmd::W`](W) writer structure"]
impl crate::Writable for EM23PERNORETAINCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EM23PERNORETAINCMD to value 0"]
impl crate::Resettable for EM23PERNORETAINCMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
