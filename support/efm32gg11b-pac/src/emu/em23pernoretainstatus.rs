#[doc = "Register `EM23PERNORETAINSTATUS` reader"]
pub type R = crate::R<EM23PERNORETAINSTATUS_SPEC>;
#[doc = "Field `ACMP0LOCKED` reader - Indicates If ACMP0 Powered Down During EM23"]
pub type ACMP0LOCKED_R = crate::BitReader;
#[doc = "Field `ACMP1LOCKED` reader - Indicates If ACMP1 Powered Down During EM23"]
pub type ACMP1LOCKED_R = crate::BitReader;
#[doc = "Field `PCNT0LOCKED` reader - Indicates If PCNT0 Powered Down During EM23"]
pub type PCNT0LOCKED_R = crate::BitReader;
#[doc = "Field `PCNT1LOCKED` reader - Indicates If PCNT1 Powered Down During EM23"]
pub type PCNT1LOCKED_R = crate::BitReader;
#[doc = "Field `PCNT2LOCKED` reader - Indicates If PCNT2 Powered Down During EM23"]
pub type PCNT2LOCKED_R = crate::BitReader;
#[doc = "Field `I2C0LOCKED` reader - Indicates If I2C0 Powered Down During EM23"]
pub type I2C0LOCKED_R = crate::BitReader;
#[doc = "Field `I2C1LOCKED` reader - Indicates If I2C1 Powered Down During EM23"]
pub type I2C1LOCKED_R = crate::BitReader;
#[doc = "Field `DAC0LOCKED` reader - Indicates If DAC0 Powered Down During EM23"]
pub type DAC0LOCKED_R = crate::BitReader;
#[doc = "Field `IDAC0LOCKED` reader - Indicates If IDAC0 Powered Down During EM23"]
pub type IDAC0LOCKED_R = crate::BitReader;
#[doc = "Field `ADC0LOCKED` reader - Indicates If ADC0 Powered Down During EM23"]
pub type ADC0LOCKED_R = crate::BitReader;
#[doc = "Field `LETIMER0LOCKED` reader - Indicates If LETIMER0 Powered Down During EM23"]
pub type LETIMER0LOCKED_R = crate::BitReader;
#[doc = "Field `WDOG0LOCKED` reader - Indicates If WDOG0 Powered Down During EM23"]
pub type WDOG0LOCKED_R = crate::BitReader;
#[doc = "Field `WDOG1LOCKED` reader - Indicates If WDOG1 Powered Down During EM23"]
pub type WDOG1LOCKED_R = crate::BitReader;
#[doc = "Field `LESENSE0LOCKED` reader - Indicates If LESENSE0 Powered Down During EM23"]
pub type LESENSE0LOCKED_R = crate::BitReader;
#[doc = "Field `CSENLOCKED` reader - Indicates If CSEN Powered Down During EM23"]
pub type CSENLOCKED_R = crate::BitReader;
#[doc = "Field `LEUART0LOCKED` reader - Indicates If LEUART0 Powered Down During EM23"]
pub type LEUART0LOCKED_R = crate::BitReader;
#[doc = "Field `LEUART1LOCKED` reader - Indicates If LEUART1 Powered Down During EM23"]
pub type LEUART1LOCKED_R = crate::BitReader;
#[doc = "Field `LCDLOCKED` reader - Indicates If LCD Powered Down During EM23"]
pub type LCDLOCKED_R = crate::BitReader;
#[doc = "Field `LETIMER1LOCKED` reader - Indicates If LETIMER1 Powered Down During EM23"]
pub type LETIMER1LOCKED_R = crate::BitReader;
#[doc = "Field `I2C2LOCKED` reader - Indicates If I2C2 Powered Down During EM23"]
pub type I2C2LOCKED_R = crate::BitReader;
#[doc = "Field `ADC1LOCKED` reader - Indicates If ADC1 Powered Down During EM23"]
pub type ADC1LOCKED_R = crate::BitReader;
#[doc = "Field `ACMP2LOCKED` reader - Indicates If ACMP2 Powered Down During EM23"]
pub type ACMP2LOCKED_R = crate::BitReader;
#[doc = "Field `ACMP3LOCKED` reader - Indicates If ACMP3 Powered Down During EM23"]
pub type ACMP3LOCKED_R = crate::BitReader;
#[doc = "Field `RTCLOCKED` reader - Indicates If RTC Powered Down During EM23"]
pub type RTCLOCKED_R = crate::BitReader;
#[doc = "Field `USBLOCKED` reader - Indicates If USB Powered Down During EM23"]
pub type USBLOCKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates If ACMP0 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp0locked(&self) -> ACMP0LOCKED_R {
        ACMP0LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates If ACMP1 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp1locked(&self) -> ACMP1LOCKED_R {
        ACMP1LOCKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates If PCNT0 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt0locked(&self) -> PCNT0LOCKED_R {
        PCNT0LOCKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates If PCNT1 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt1locked(&self) -> PCNT1LOCKED_R {
        PCNT1LOCKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates If PCNT2 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt2locked(&self) -> PCNT2LOCKED_R {
        PCNT2LOCKED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates If I2C0 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c0locked(&self) -> I2C0LOCKED_R {
        I2C0LOCKED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates If I2C1 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c1locked(&self) -> I2C1LOCKED_R {
        I2C1LOCKED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates If DAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn dac0locked(&self) -> DAC0LOCKED_R {
        DAC0LOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates If IDAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn idac0locked(&self) -> IDAC0LOCKED_R {
        IDAC0LOCKED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates If ADC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc0locked(&self) -> ADC0LOCKED_R {
        ADC0LOCKED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates If LETIMER0 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer0locked(&self) -> LETIMER0LOCKED_R {
        LETIMER0LOCKED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates If WDOG0 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog0locked(&self) -> WDOG0LOCKED_R {
        WDOG0LOCKED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates If WDOG1 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog1locked(&self) -> WDOG1LOCKED_R {
        WDOG1LOCKED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates If LESENSE0 Powered Down During EM23"]
    #[inline(always)]
    pub fn lesense0locked(&self) -> LESENSE0LOCKED_R {
        LESENSE0LOCKED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates If CSEN Powered Down During EM23"]
    #[inline(always)]
    pub fn csenlocked(&self) -> CSENLOCKED_R {
        CSENLOCKED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates If LEUART0 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart0locked(&self) -> LEUART0LOCKED_R {
        LEUART0LOCKED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates If LEUART1 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart1locked(&self) -> LEUART1LOCKED_R {
        LEUART1LOCKED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates If LCD Powered Down During EM23"]
    #[inline(always)]
    pub fn lcdlocked(&self) -> LCDLOCKED_R {
        LCDLOCKED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates If LETIMER1 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer1locked(&self) -> LETIMER1LOCKED_R {
        LETIMER1LOCKED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates If I2C2 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c2locked(&self) -> I2C2LOCKED_R {
        I2C2LOCKED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates If ADC1 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc1locked(&self) -> ADC1LOCKED_R {
        ADC1LOCKED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates If ACMP2 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp2locked(&self) -> ACMP2LOCKED_R {
        ACMP2LOCKED_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates If ACMP3 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp3locked(&self) -> ACMP3LOCKED_R {
        ACMP3LOCKED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicates If RTC Powered Down During EM23"]
    #[inline(always)]
    pub fn rtclocked(&self) -> RTCLOCKED_R {
        RTCLOCKED_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates If USB Powered Down During EM23"]
    #[inline(always)]
    pub fn usblocked(&self) -> USBLOCKED_R {
        USBLOCKED_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em23pernoretainstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM23PERNORETAINSTATUS_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em23pernoretainstatus::R`](R) reader structure"]
impl crate::Readable for EM23PERNORETAINSTATUS_SPEC {}
#[doc = "`reset()` method sets EM23PERNORETAINSTATUS to value 0"]
impl crate::Resettable for EM23PERNORETAINSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
