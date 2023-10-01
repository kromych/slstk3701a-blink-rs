#[doc = "Register `EM23PERNORETAINCMD` writer"]
pub type W = crate::W<EM23PERNORETAINCMD_SPEC>;
#[doc = "Field `ACMP0UNLOCK` writer - Clears Status Bit of ACMP0 and Unlocks Access to It"]
pub type ACMP0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMP1UNLOCK` writer - Clears Status Bit of ACMP1 and Unlocks Access to It"]
pub type ACMP1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCNT0UNLOCK` writer - Clears Status Bit of PCNT0 and Unlocks Access to It"]
pub type PCNT0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCNT1UNLOCK` writer - Clears Status Bit of PCNT1 and Unlocks Access to It"]
pub type PCNT1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCNT2UNLOCK` writer - Clears Status Bit of PCNT2 and Unlocks Access to It"]
pub type PCNT2UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C0UNLOCK` writer - Clears Status Bit of I2C0 and Unlocks Access to It"]
pub type I2C0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1UNLOCK` writer - Clears Status Bit of I2C1 and Unlocks Access to It"]
pub type I2C1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC0UNLOCK` writer - Clears Status Bit of DAC0 and Unlocks Access to It"]
pub type DAC0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDAC0UNLOCK` writer - Clears Status Bit of IDAC0 and Unlocks Access to It"]
pub type IDAC0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC0UNLOCK` writer - Clears Status Bit of ADC0 and Unlocks Access to It"]
pub type ADC0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LETIMER0UNLOCK` writer - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
pub type LETIMER0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDOG0UNLOCK` writer - Clears Status Bit of WDOG0 and Unlocks Access to It"]
pub type WDOG0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDOG1UNLOCK` writer - Clears Status Bit of WDOG1 and Unlocks Access to It"]
pub type WDOG1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LESENSE0UNLOCK` writer - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
pub type LESENSE0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSENUNLOCK` writer - Clears Status Bit of CSEN and Unlocks Access to It"]
pub type CSENUNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEUART0UNLOCK` writer - Clears Status Bit of LEUART0 and Unlocks Access to It"]
pub type LEUART0UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEUART1UNLOCK` writer - Clears Status Bit of LEUART1 and Unlocks Access to It"]
pub type LEUART1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCDUNLOCK` writer - Clears Status Bit of LCD and Unlocks Access to It"]
pub type LCDUNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LETIMER1UNLOCK` writer - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
pub type LETIMER1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2UNLOCK` writer - Clears Status Bit of I2C2 and Unlocks Access to It"]
pub type I2C2UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1UNLOCK` writer - Clears Status Bit of ADC1 and Unlocks Access to It"]
pub type ADC1UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMP2UNLOCK` writer - Clears Status Bit of ACMP2 and Unlocks Access to It"]
pub type ACMP2UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMP3UNLOCK` writer - Clears Status Bit of ACMP3 and Unlocks Access to It"]
pub type ACMP3UNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCUNLOCK` writer - Clears Status Bit of RTC and Unlocks Access to It"]
pub type RTCUNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBUNLOCK` writer - Clears Status Bit of USB and Unlocks Access to It"]
pub type USBUNLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0unlock(&mut self) -> ACMP0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 0> {
        ACMP0UNLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1unlock(&mut self) -> ACMP1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 1> {
        ACMP1UNLOCK_W::new(self)
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0unlock(&mut self) -> PCNT0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 2> {
        PCNT0UNLOCK_W::new(self)
    }
    #[doc = "Bit 3 - Clears Status Bit of PCNT1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1unlock(&mut self) -> PCNT1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 3> {
        PCNT1UNLOCK_W::new(self)
    }
    #[doc = "Bit 4 - Clears Status Bit of PCNT2 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2unlock(&mut self) -> PCNT2UNLOCK_W<EM23PERNORETAINCMD_SPEC, 4> {
        PCNT2UNLOCK_W::new(self)
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0unlock(&mut self) -> I2C0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 5> {
        I2C0UNLOCK_W::new(self)
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1unlock(&mut self) -> I2C1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 6> {
        I2C1UNLOCK_W::new(self)
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn dac0unlock(&mut self) -> DAC0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 7> {
        DAC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 8 - Clears Status Bit of IDAC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn idac0unlock(&mut self) -> IDAC0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 8> {
        IDAC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn adc0unlock(&mut self) -> ADC0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 9> {
        ADC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0unlock(&mut self) -> LETIMER0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 10> {
        LETIMER0UNLOCK_W::new(self)
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0unlock(&mut self) -> WDOG0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 11> {
        WDOG0UNLOCK_W::new(self)
    }
    #[doc = "Bit 12 - Clears Status Bit of WDOG1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1unlock(&mut self) -> WDOG1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 12> {
        WDOG1UNLOCK_W::new(self)
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn lesense0unlock(&mut self) -> LESENSE0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 13> {
        LESENSE0UNLOCK_W::new(self)
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn csenunlock(&mut self) -> CSENUNLOCK_W<EM23PERNORETAINCMD_SPEC, 14> {
        CSENUNLOCK_W::new(self)
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0unlock(&mut self) -> LEUART0UNLOCK_W<EM23PERNORETAINCMD_SPEC, 15> {
        LEUART0UNLOCK_W::new(self)
    }
    #[doc = "Bit 16 - Clears Status Bit of LEUART1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn leuart1unlock(&mut self) -> LEUART1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 16> {
        LEUART1UNLOCK_W::new(self)
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn lcdunlock(&mut self) -> LCDUNLOCK_W<EM23PERNORETAINCMD_SPEC, 17> {
        LCDUNLOCK_W::new(self)
    }
    #[doc = "Bit 18 - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn letimer1unlock(&mut self) -> LETIMER1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 18> {
        LETIMER1UNLOCK_W::new(self)
    }
    #[doc = "Bit 19 - Clears Status Bit of I2C2 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2unlock(&mut self) -> I2C2UNLOCK_W<EM23PERNORETAINCMD_SPEC, 19> {
        I2C2UNLOCK_W::new(self)
    }
    #[doc = "Bit 20 - Clears Status Bit of ADC1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn adc1unlock(&mut self) -> ADC1UNLOCK_W<EM23PERNORETAINCMD_SPEC, 20> {
        ADC1UNLOCK_W::new(self)
    }
    #[doc = "Bit 21 - Clears Status Bit of ACMP2 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2unlock(&mut self) -> ACMP2UNLOCK_W<EM23PERNORETAINCMD_SPEC, 21> {
        ACMP2UNLOCK_W::new(self)
    }
    #[doc = "Bit 22 - Clears Status Bit of ACMP3 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3unlock(&mut self) -> ACMP3UNLOCK_W<EM23PERNORETAINCMD_SPEC, 22> {
        ACMP3UNLOCK_W::new(self)
    }
    #[doc = "Bit 23 - Clears Status Bit of RTC and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn rtcunlock(&mut self) -> RTCUNLOCK_W<EM23PERNORETAINCMD_SPEC, 23> {
        RTCUNLOCK_W::new(self)
    }
    #[doc = "Bit 24 - Clears Status Bit of USB and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn usbunlock(&mut self) -> USBUNLOCK_W<EM23PERNORETAINCMD_SPEC, 24> {
        USBUNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM23PERNORETAINCMD to value 0"]
impl crate::Resettable for EM23PERNORETAINCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
