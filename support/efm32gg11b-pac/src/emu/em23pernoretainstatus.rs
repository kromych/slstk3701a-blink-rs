#[doc = "Register `EM23PERNORETAINSTATUS` reader"]
pub type R = crate::R<Em23pernoretainstatusSpec>;
#[doc = "Field `ACMP0LOCKED` reader - Indicates If ACMP0 Powered Down During EM23"]
pub type Acmp0lockedR = crate::BitReader;
#[doc = "Field `ACMP1LOCKED` reader - Indicates If ACMP1 Powered Down During EM23"]
pub type Acmp1lockedR = crate::BitReader;
#[doc = "Field `PCNT0LOCKED` reader - Indicates If PCNT0 Powered Down During EM23"]
pub type Pcnt0lockedR = crate::BitReader;
#[doc = "Field `PCNT1LOCKED` reader - Indicates If PCNT1 Powered Down During EM23"]
pub type Pcnt1lockedR = crate::BitReader;
#[doc = "Field `PCNT2LOCKED` reader - Indicates If PCNT2 Powered Down During EM23"]
pub type Pcnt2lockedR = crate::BitReader;
#[doc = "Field `I2C0LOCKED` reader - Indicates If I2C0 Powered Down During EM23"]
pub type I2c0lockedR = crate::BitReader;
#[doc = "Field `I2C1LOCKED` reader - Indicates If I2C1 Powered Down During EM23"]
pub type I2c1lockedR = crate::BitReader;
#[doc = "Field `DAC0LOCKED` reader - Indicates If DAC0 Powered Down During EM23"]
pub type Dac0lockedR = crate::BitReader;
#[doc = "Field `IDAC0LOCKED` reader - Indicates If IDAC0 Powered Down During EM23"]
pub type Idac0lockedR = crate::BitReader;
#[doc = "Field `ADC0LOCKED` reader - Indicates If ADC0 Powered Down During EM23"]
pub type Adc0lockedR = crate::BitReader;
#[doc = "Field `LETIMER0LOCKED` reader - Indicates If LETIMER0 Powered Down During EM23"]
pub type Letimer0lockedR = crate::BitReader;
#[doc = "Field `WDOG0LOCKED` reader - Indicates If WDOG0 Powered Down During EM23"]
pub type Wdog0lockedR = crate::BitReader;
#[doc = "Field `WDOG1LOCKED` reader - Indicates If WDOG1 Powered Down During EM23"]
pub type Wdog1lockedR = crate::BitReader;
#[doc = "Field `LESENSE0LOCKED` reader - Indicates If LESENSE0 Powered Down During EM23"]
pub type Lesense0lockedR = crate::BitReader;
#[doc = "Field `CSENLOCKED` reader - Indicates If CSEN Powered Down During EM23"]
pub type CsenlockedR = crate::BitReader;
#[doc = "Field `LEUART0LOCKED` reader - Indicates If LEUART0 Powered Down During EM23"]
pub type Leuart0lockedR = crate::BitReader;
#[doc = "Field `LEUART1LOCKED` reader - Indicates If LEUART1 Powered Down During EM23"]
pub type Leuart1lockedR = crate::BitReader;
#[doc = "Field `LCDLOCKED` reader - Indicates If LCD Powered Down During EM23"]
pub type LcdlockedR = crate::BitReader;
#[doc = "Field `LETIMER1LOCKED` reader - Indicates If LETIMER1 Powered Down During EM23"]
pub type Letimer1lockedR = crate::BitReader;
#[doc = "Field `I2C2LOCKED` reader - Indicates If I2C2 Powered Down During EM23"]
pub type I2c2lockedR = crate::BitReader;
#[doc = "Field `ADC1LOCKED` reader - Indicates If ADC1 Powered Down During EM23"]
pub type Adc1lockedR = crate::BitReader;
#[doc = "Field `ACMP2LOCKED` reader - Indicates If ACMP2 Powered Down During EM23"]
pub type Acmp2lockedR = crate::BitReader;
#[doc = "Field `ACMP3LOCKED` reader - Indicates If ACMP3 Powered Down During EM23"]
pub type Acmp3lockedR = crate::BitReader;
#[doc = "Field `RTCLOCKED` reader - Indicates If RTC Powered Down During EM23"]
pub type RtclockedR = crate::BitReader;
#[doc = "Field `USBLOCKED` reader - Indicates If USB Powered Down During EM23"]
pub type UsblockedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates If ACMP0 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp0locked(&self) -> Acmp0lockedR {
        Acmp0lockedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates If ACMP1 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp1locked(&self) -> Acmp1lockedR {
        Acmp1lockedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates If PCNT0 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt0locked(&self) -> Pcnt0lockedR {
        Pcnt0lockedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates If PCNT1 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt1locked(&self) -> Pcnt1lockedR {
        Pcnt1lockedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates If PCNT2 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt2locked(&self) -> Pcnt2lockedR {
        Pcnt2lockedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates If I2C0 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c0locked(&self) -> I2c0lockedR {
        I2c0lockedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates If I2C1 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c1locked(&self) -> I2c1lockedR {
        I2c1lockedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates If DAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn dac0locked(&self) -> Dac0lockedR {
        Dac0lockedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates If IDAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn idac0locked(&self) -> Idac0lockedR {
        Idac0lockedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates If ADC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc0locked(&self) -> Adc0lockedR {
        Adc0lockedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates If LETIMER0 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer0locked(&self) -> Letimer0lockedR {
        Letimer0lockedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates If WDOG0 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog0locked(&self) -> Wdog0lockedR {
        Wdog0lockedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates If WDOG1 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog1locked(&self) -> Wdog1lockedR {
        Wdog1lockedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates If LESENSE0 Powered Down During EM23"]
    #[inline(always)]
    pub fn lesense0locked(&self) -> Lesense0lockedR {
        Lesense0lockedR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates If CSEN Powered Down During EM23"]
    #[inline(always)]
    pub fn csenlocked(&self) -> CsenlockedR {
        CsenlockedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates If LEUART0 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart0locked(&self) -> Leuart0lockedR {
        Leuart0lockedR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates If LEUART1 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart1locked(&self) -> Leuart1lockedR {
        Leuart1lockedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates If LCD Powered Down During EM23"]
    #[inline(always)]
    pub fn lcdlocked(&self) -> LcdlockedR {
        LcdlockedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates If LETIMER1 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer1locked(&self) -> Letimer1lockedR {
        Letimer1lockedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates If I2C2 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c2locked(&self) -> I2c2lockedR {
        I2c2lockedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates If ADC1 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc1locked(&self) -> Adc1lockedR {
        Adc1lockedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates If ACMP2 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp2locked(&self) -> Acmp2lockedR {
        Acmp2lockedR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates If ACMP3 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp3locked(&self) -> Acmp3lockedR {
        Acmp3lockedR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicates If RTC Powered Down During EM23"]
    #[inline(always)]
    pub fn rtclocked(&self) -> RtclockedR {
        RtclockedR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates If USB Powered Down During EM23"]
    #[inline(always)]
    pub fn usblocked(&self) -> UsblockedR {
        UsblockedR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It\n\nYou can [`read`](crate::Reg::read) this register and get [`em23pernoretainstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em23pernoretainstatusSpec;
impl crate::RegisterSpec for Em23pernoretainstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em23pernoretainstatus::R`](R) reader structure"]
impl crate::Readable for Em23pernoretainstatusSpec {}
#[doc = "`reset()` method sets EM23PERNORETAINSTATUS to value 0"]
impl crate::Resettable for Em23pernoretainstatusSpec {}
