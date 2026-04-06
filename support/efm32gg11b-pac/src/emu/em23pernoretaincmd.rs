#[doc = "Register `EM23PERNORETAINCMD` writer"]
pub type W = crate::W<Em23pernoretaincmdSpec>;
#[doc = "Field `ACMP0UNLOCK` writer - Clears Status Bit of ACMP0 and Unlocks Access to It"]
pub type Acmp0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1UNLOCK` writer - Clears Status Bit of ACMP1 and Unlocks Access to It"]
pub type Acmp1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0UNLOCK` writer - Clears Status Bit of PCNT0 and Unlocks Access to It"]
pub type Pcnt0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1UNLOCK` writer - Clears Status Bit of PCNT1 and Unlocks Access to It"]
pub type Pcnt1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2UNLOCK` writer - Clears Status Bit of PCNT2 and Unlocks Access to It"]
pub type Pcnt2unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0UNLOCK` writer - Clears Status Bit of I2C0 and Unlocks Access to It"]
pub type I2c0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1UNLOCK` writer - Clears Status Bit of I2C1 and Unlocks Access to It"]
pub type I2c1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC0UNLOCK` writer - Clears Status Bit of DAC0 and Unlocks Access to It"]
pub type Dac0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0UNLOCK` writer - Clears Status Bit of IDAC0 and Unlocks Access to It"]
pub type Idac0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0UNLOCK` writer - Clears Status Bit of ADC0 and Unlocks Access to It"]
pub type Adc0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0UNLOCK` writer - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
pub type Letimer0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0UNLOCK` writer - Clears Status Bit of WDOG0 and Unlocks Access to It"]
pub type Wdog0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1UNLOCK` writer - Clears Status Bit of WDOG1 and Unlocks Access to It"]
pub type Wdog1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE0UNLOCK` writer - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
pub type Lesense0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSENUNLOCK` writer - Clears Status Bit of CSEN and Unlocks Access to It"]
pub type CsenunlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART0UNLOCK` writer - Clears Status Bit of LEUART0 and Unlocks Access to It"]
pub type Leuart0unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1UNLOCK` writer - Clears Status Bit of LEUART1 and Unlocks Access to It"]
pub type Leuart1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDUNLOCK` writer - Clears Status Bit of LCD and Unlocks Access to It"]
pub type LcdunlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1UNLOCK` writer - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
pub type Letimer1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2UNLOCK` writer - Clears Status Bit of I2C2 and Unlocks Access to It"]
pub type I2c2unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1UNLOCK` writer - Clears Status Bit of ADC1 and Unlocks Access to It"]
pub type Adc1unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2UNLOCK` writer - Clears Status Bit of ACMP2 and Unlocks Access to It"]
pub type Acmp2unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3UNLOCK` writer - Clears Status Bit of ACMP3 and Unlocks Access to It"]
pub type Acmp3unlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCUNLOCK` writer - Clears Status Bit of RTC and Unlocks Access to It"]
pub type RtcunlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBUNLOCK` writer - Clears Status Bit of USB and Unlocks Access to It"]
pub type UsbunlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp0unlock(&mut self) -> Acmp0unlockW<'_, Em23pernoretaincmdSpec> {
        Acmp0unlockW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp1unlock(&mut self) -> Acmp1unlockW<'_, Em23pernoretaincmdSpec> {
        Acmp1unlockW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt0unlock(&mut self) -> Pcnt0unlockW<'_, Em23pernoretaincmdSpec> {
        Pcnt0unlockW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears Status Bit of PCNT1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt1unlock(&mut self) -> Pcnt1unlockW<'_, Em23pernoretaincmdSpec> {
        Pcnt1unlockW::new(self, 3)
    }
    #[doc = "Bit 4 - Clears Status Bit of PCNT2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt2unlock(&mut self) -> Pcnt2unlockW<'_, Em23pernoretaincmdSpec> {
        Pcnt2unlockW::new(self, 4)
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c0unlock(&mut self) -> I2c0unlockW<'_, Em23pernoretaincmdSpec> {
        I2c0unlockW::new(self, 5)
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c1unlock(&mut self) -> I2c1unlockW<'_, Em23pernoretaincmdSpec> {
        I2c1unlockW::new(self, 6)
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn dac0unlock(&mut self) -> Dac0unlockW<'_, Em23pernoretaincmdSpec> {
        Dac0unlockW::new(self, 7)
    }
    #[doc = "Bit 8 - Clears Status Bit of IDAC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn idac0unlock(&mut self) -> Idac0unlockW<'_, Em23pernoretaincmdSpec> {
        Idac0unlockW::new(self, 8)
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn adc0unlock(&mut self) -> Adc0unlockW<'_, Em23pernoretaincmdSpec> {
        Adc0unlockW::new(self, 9)
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn letimer0unlock(&mut self) -> Letimer0unlockW<'_, Em23pernoretaincmdSpec> {
        Letimer0unlockW::new(self, 10)
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn wdog0unlock(&mut self) -> Wdog0unlockW<'_, Em23pernoretaincmdSpec> {
        Wdog0unlockW::new(self, 11)
    }
    #[doc = "Bit 12 - Clears Status Bit of WDOG1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn wdog1unlock(&mut self) -> Wdog1unlockW<'_, Em23pernoretaincmdSpec> {
        Wdog1unlockW::new(self, 12)
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn lesense0unlock(&mut self) -> Lesense0unlockW<'_, Em23pernoretaincmdSpec> {
        Lesense0unlockW::new(self, 13)
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline(always)]
    pub fn csenunlock(&mut self) -> CsenunlockW<'_, Em23pernoretaincmdSpec> {
        CsenunlockW::new(self, 14)
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn leuart0unlock(&mut self) -> Leuart0unlockW<'_, Em23pernoretaincmdSpec> {
        Leuart0unlockW::new(self, 15)
    }
    #[doc = "Bit 16 - Clears Status Bit of LEUART1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn leuart1unlock(&mut self) -> Leuart1unlockW<'_, Em23pernoretaincmdSpec> {
        Leuart1unlockW::new(self, 16)
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline(always)]
    pub fn lcdunlock(&mut self) -> LcdunlockW<'_, Em23pernoretaincmdSpec> {
        LcdunlockW::new(self, 17)
    }
    #[doc = "Bit 18 - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn letimer1unlock(&mut self) -> Letimer1unlockW<'_, Em23pernoretaincmdSpec> {
        Letimer1unlockW::new(self, 18)
    }
    #[doc = "Bit 19 - Clears Status Bit of I2C2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c2unlock(&mut self) -> I2c2unlockW<'_, Em23pernoretaincmdSpec> {
        I2c2unlockW::new(self, 19)
    }
    #[doc = "Bit 20 - Clears Status Bit of ADC1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn adc1unlock(&mut self) -> Adc1unlockW<'_, Em23pernoretaincmdSpec> {
        Adc1unlockW::new(self, 20)
    }
    #[doc = "Bit 21 - Clears Status Bit of ACMP2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp2unlock(&mut self) -> Acmp2unlockW<'_, Em23pernoretaincmdSpec> {
        Acmp2unlockW::new(self, 21)
    }
    #[doc = "Bit 22 - Clears Status Bit of ACMP3 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp3unlock(&mut self) -> Acmp3unlockW<'_, Em23pernoretaincmdSpec> {
        Acmp3unlockW::new(self, 22)
    }
    #[doc = "Bit 23 - Clears Status Bit of RTC and Unlocks Access to It"]
    #[inline(always)]
    pub fn rtcunlock(&mut self) -> RtcunlockW<'_, Em23pernoretaincmdSpec> {
        RtcunlockW::new(self, 23)
    }
    #[doc = "Bit 24 - Clears Status Bit of USB and Unlocks Access to It"]
    #[inline(always)]
    pub fn usbunlock(&mut self) -> UsbunlockW<'_, Em23pernoretaincmdSpec> {
        UsbunlockW::new(self, 24)
    }
}
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23pernoretaincmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em23pernoretaincmdSpec;
impl crate::RegisterSpec for Em23pernoretaincmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`em23pernoretaincmd::W`](W) writer structure"]
impl crate::Writable for Em23pernoretaincmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM23PERNORETAINCMD to value 0"]
impl crate::Resettable for Em23pernoretaincmdSpec {}
