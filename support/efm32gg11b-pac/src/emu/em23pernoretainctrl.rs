#[doc = "Register `EM23PERNORETAINCTRL` reader"]
pub type R = crate::R<EM23PERNORETAINCTRL_SPEC>;
#[doc = "Register `EM23PERNORETAINCTRL` writer"]
pub type W = crate::W<EM23PERNORETAINCTRL_SPEC>;
#[doc = "Field `ACMP0DIS` reader - Allow Power Down of ACMP0 During EM23"]
pub type ACMP0DIS_R = crate::BitReader;
#[doc = "Field `ACMP0DIS` writer - Allow Power Down of ACMP0 During EM23"]
pub type ACMP0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1DIS` reader - Allow Power Down of ACMP1 During EM23"]
pub type ACMP1DIS_R = crate::BitReader;
#[doc = "Field `ACMP1DIS` writer - Allow Power Down of ACMP1 During EM23"]
pub type ACMP1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0DIS` reader - Allow Power Down of PCNT0 During EM23"]
pub type PCNT0DIS_R = crate::BitReader;
#[doc = "Field `PCNT0DIS` writer - Allow Power Down of PCNT0 During EM23"]
pub type PCNT0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1DIS` reader - Allow Power Down of PCNT1 During EM23"]
pub type PCNT1DIS_R = crate::BitReader;
#[doc = "Field `PCNT1DIS` writer - Allow Power Down of PCNT1 During EM23"]
pub type PCNT1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2DIS` reader - Allow Power Down of PCNT2 During EM23"]
pub type PCNT2DIS_R = crate::BitReader;
#[doc = "Field `PCNT2DIS` writer - Allow Power Down of PCNT2 During EM23"]
pub type PCNT2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0DIS` reader - Allow Power Down of I2C0 During EM23"]
pub type I2C0DIS_R = crate::BitReader;
#[doc = "Field `I2C0DIS` writer - Allow Power Down of I2C0 During EM23"]
pub type I2C0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1DIS` reader - Allow Power Down of I2C1 During EM23"]
pub type I2C1DIS_R = crate::BitReader;
#[doc = "Field `I2C1DIS` writer - Allow Power Down of I2C1 During EM23"]
pub type I2C1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0DIS` reader - Allow Power Down of DAC0 During EM23"]
pub type VDAC0DIS_R = crate::BitReader;
#[doc = "Field `VDAC0DIS` writer - Allow Power Down of DAC0 During EM23"]
pub type VDAC0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0DIS` reader - Allow Power Down of IDAC0 During EM23"]
pub type IDAC0DIS_R = crate::BitReader;
#[doc = "Field `IDAC0DIS` writer - Allow Power Down of IDAC0 During EM23"]
pub type IDAC0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0DIS` reader - Allow Power Down of ADC0 During EM23"]
pub type ADC0DIS_R = crate::BitReader;
#[doc = "Field `ADC0DIS` writer - Allow Power Down of ADC0 During EM23"]
pub type ADC0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0DIS` reader - Allow Power Down of LETIMER0 During EM23"]
pub type LETIMER0DIS_R = crate::BitReader;
#[doc = "Field `LETIMER0DIS` writer - Allow Power Down of LETIMER0 During EM23"]
pub type LETIMER0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0DIS` reader - Allow Power Down of WDOG0 During EM23"]
pub type WDOG0DIS_R = crate::BitReader;
#[doc = "Field `WDOG0DIS` writer - Allow Power Down of WDOG0 During EM23"]
pub type WDOG0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1DIS` reader - Allow Power Down of WDOG1 During EM23"]
pub type WDOG1DIS_R = crate::BitReader;
#[doc = "Field `WDOG1DIS` writer - Allow Power Down of WDOG1 During EM23"]
pub type WDOG1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE0DIS` reader - Allow Power Down of LESENSE0 During EM23"]
pub type LESENSE0DIS_R = crate::BitReader;
#[doc = "Field `LESENSE0DIS` writer - Allow Power Down of LESENSE0 During EM23"]
pub type LESENSE0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSENDIS` reader - Allow Power Down of CSEN During EM23"]
pub type CSENDIS_R = crate::BitReader;
#[doc = "Field `CSENDIS` writer - Allow Power Down of CSEN During EM23"]
pub type CSENDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART0DIS` reader - Allow Power Down of LEUART0 During EM23"]
pub type LEUART0DIS_R = crate::BitReader;
#[doc = "Field `LEUART0DIS` writer - Allow Power Down of LEUART0 During EM23"]
pub type LEUART0DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1DIS` reader - Allow Power Down of LEUART1 During EM23"]
pub type LEUART1DIS_R = crate::BitReader;
#[doc = "Field `LEUART1DIS` writer - Allow Power Down of LEUART1 During EM23"]
pub type LEUART1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDDIS` reader - Allow Power Down of LCD During EM23"]
pub type LCDDIS_R = crate::BitReader;
#[doc = "Field `LCDDIS` writer - Allow Power Down of LCD During EM23"]
pub type LCDDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1DIS` reader - Allow Power Down of LETIMER1 During EM23"]
pub type LETIMER1DIS_R = crate::BitReader;
#[doc = "Field `LETIMER1DIS` writer - Allow Power Down of LETIMER1 During EM23"]
pub type LETIMER1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2DIS` reader - Allow Power Down of I2C2 During EM23"]
pub type I2C2DIS_R = crate::BitReader;
#[doc = "Field `I2C2DIS` writer - Allow Power Down of I2C2 During EM23"]
pub type I2C2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1DIS` reader - Allow Power Down of ADC1 During EM23"]
pub type ADC1DIS_R = crate::BitReader;
#[doc = "Field `ADC1DIS` writer - Allow Power Down of ADC1 During EM23"]
pub type ADC1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2DIS` reader - Allow Power Down of ACMP2 During EM23"]
pub type ACMP2DIS_R = crate::BitReader;
#[doc = "Field `ACMP2DIS` writer - Allow Power Down of ACMP2 During EM23"]
pub type ACMP2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3DIS` reader - Allow Power Down of ACMP3 During EM23"]
pub type ACMP3DIS_R = crate::BitReader;
#[doc = "Field `ACMP3DIS` writer - Allow Power Down of ACMP3 During EM23"]
pub type ACMP3DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCDIS` reader - Allow Power Down of RTC During EM23"]
pub type RTCDIS_R = crate::BitReader;
#[doc = "Field `RTCDIS` writer - Allow Power Down of RTC During EM23"]
pub type RTCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBDIS` reader - Allow Power Down of USB During EM23"]
pub type USBDIS_R = crate::BitReader;
#[doc = "Field `USBDIS` writer - Allow Power Down of USB During EM23"]
pub type USBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&self) -> ACMP0DIS_R {
        ACMP0DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&self) -> ACMP1DIS_R {
        ACMP1DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&self) -> PCNT0DIS_R {
        PCNT0DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow Power Down of PCNT1 During EM23"]
    #[inline(always)]
    pub fn pcnt1dis(&self) -> PCNT1DIS_R {
        PCNT1DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Allow Power Down of PCNT2 During EM23"]
    #[inline(always)]
    pub fn pcnt2dis(&self) -> PCNT2DIS_R {
        PCNT2DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&self) -> I2C0DIS_R {
        I2C0DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&self) -> I2C1DIS_R {
        I2C1DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&self) -> VDAC0DIS_R {
        VDAC0DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Allow Power Down of IDAC0 During EM23"]
    #[inline(always)]
    pub fn idac0dis(&self) -> IDAC0DIS_R {
        IDAC0DIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&self) -> ADC0DIS_R {
        ADC0DIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&self) -> LETIMER0DIS_R {
        LETIMER0DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&self) -> WDOG0DIS_R {
        WDOG0DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Allow Power Down of WDOG1 During EM23"]
    #[inline(always)]
    pub fn wdog1dis(&self) -> WDOG1DIS_R {
        WDOG1DIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&self) -> LESENSE0DIS_R {
        LESENSE0DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&self) -> CSENDIS_R {
        CSENDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&self) -> LEUART0DIS_R {
        LEUART0DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Allow Power Down of LEUART1 During EM23"]
    #[inline(always)]
    pub fn leuart1dis(&self) -> LEUART1DIS_R {
        LEUART1DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Allow Power Down of LCD During EM23"]
    #[inline(always)]
    pub fn lcddis(&self) -> LCDDIS_R {
        LCDDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Allow Power Down of LETIMER1 During EM23"]
    #[inline(always)]
    pub fn letimer1dis(&self) -> LETIMER1DIS_R {
        LETIMER1DIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Allow Power Down of I2C2 During EM23"]
    #[inline(always)]
    pub fn i2c2dis(&self) -> I2C2DIS_R {
        I2C2DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Allow Power Down of ADC1 During EM23"]
    #[inline(always)]
    pub fn adc1dis(&self) -> ADC1DIS_R {
        ADC1DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Allow Power Down of ACMP2 During EM23"]
    #[inline(always)]
    pub fn acmp2dis(&self) -> ACMP2DIS_R {
        ACMP2DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Allow Power Down of ACMP3 During EM23"]
    #[inline(always)]
    pub fn acmp3dis(&self) -> ACMP3DIS_R {
        ACMP3DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Allow Power Down of RTC During EM23"]
    #[inline(always)]
    pub fn rtcdis(&self) -> RTCDIS_R {
        RTCDIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Allow Power Down of USB During EM23"]
    #[inline(always)]
    pub fn usbdis(&self) -> USBDIS_R {
        USBDIS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0dis(&mut self) -> ACMP0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        ACMP0DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1dis(&mut self) -> ACMP1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        ACMP1DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0dis(&mut self) -> PCNT0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        PCNT0DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Allow Power Down of PCNT1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1dis(&mut self) -> PCNT1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        PCNT1DIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Allow Power Down of PCNT2 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2dis(&mut self) -> PCNT2DIS_W<EM23PERNORETAINCTRL_SPEC> {
        PCNT2DIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0dis(&mut self) -> I2C0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        I2C0DIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1dis(&mut self) -> I2C1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        I2C1DIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn vdac0dis(&mut self) -> VDAC0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        VDAC0DIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Allow Power Down of IDAC0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn idac0dis(&mut self) -> IDAC0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        IDAC0DIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn adc0dis(&mut self) -> ADC0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        ADC0DIS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0dis(&mut self) -> LETIMER0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        LETIMER0DIS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0dis(&mut self) -> WDOG0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        WDOG0DIS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Allow Power Down of WDOG1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1dis(&mut self) -> WDOG1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        WDOG1DIS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn lesense0dis(&mut self) -> LESENSE0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        LESENSE0DIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn csendis(&mut self) -> CSENDIS_W<EM23PERNORETAINCTRL_SPEC> {
        CSENDIS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0dis(&mut self) -> LEUART0DIS_W<EM23PERNORETAINCTRL_SPEC> {
        LEUART0DIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Allow Power Down of LEUART1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn leuart1dis(&mut self) -> LEUART1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        LEUART1DIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Allow Power Down of LCD During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn lcddis(&mut self) -> LCDDIS_W<EM23PERNORETAINCTRL_SPEC> {
        LCDDIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Allow Power Down of LETIMER1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn letimer1dis(&mut self) -> LETIMER1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        LETIMER1DIS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Allow Power Down of I2C2 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2dis(&mut self) -> I2C2DIS_W<EM23PERNORETAINCTRL_SPEC> {
        I2C2DIS_W::new(self, 19)
    }
    #[doc = "Bit 20 - Allow Power Down of ADC1 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn adc1dis(&mut self) -> ADC1DIS_W<EM23PERNORETAINCTRL_SPEC> {
        ADC1DIS_W::new(self, 20)
    }
    #[doc = "Bit 21 - Allow Power Down of ACMP2 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2dis(&mut self) -> ACMP2DIS_W<EM23PERNORETAINCTRL_SPEC> {
        ACMP2DIS_W::new(self, 21)
    }
    #[doc = "Bit 22 - Allow Power Down of ACMP3 During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3dis(&mut self) -> ACMP3DIS_W<EM23PERNORETAINCTRL_SPEC> {
        ACMP3DIS_W::new(self, 22)
    }
    #[doc = "Bit 23 - Allow Power Down of RTC During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn rtcdis(&mut self) -> RTCDIS_W<EM23PERNORETAINCTRL_SPEC> {
        RTCDIS_W::new(self, 23)
    }
    #[doc = "Bit 24 - Allow Power Down of USB During EM23"]
    #[inline(always)]
    #[must_use]
    pub fn usbdis(&mut self) -> USBDIS_W<EM23PERNORETAINCTRL_SPEC> {
        USBDIS_W::new(self, 24)
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
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`em23pernoretainctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em23pernoretainctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM23PERNORETAINCTRL_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em23pernoretainctrl::R`](R) reader structure"]
impl crate::Readable for EM23PERNORETAINCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`em23pernoretainctrl::W`](W) writer structure"]
impl crate::Writable for EM23PERNORETAINCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EM23PERNORETAINCTRL to value 0"]
impl crate::Resettable for EM23PERNORETAINCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
