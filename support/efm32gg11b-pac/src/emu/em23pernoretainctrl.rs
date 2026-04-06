#[doc = "Register `EM23PERNORETAINCTRL` reader"]
pub type R = crate::R<Em23pernoretainctrlSpec>;
#[doc = "Register `EM23PERNORETAINCTRL` writer"]
pub type W = crate::W<Em23pernoretainctrlSpec>;
#[doc = "Field `ACMP0DIS` reader - Allow Power Down of ACMP0 During EM23"]
pub type Acmp0disR = crate::BitReader;
#[doc = "Field `ACMP0DIS` writer - Allow Power Down of ACMP0 During EM23"]
pub type Acmp0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1DIS` reader - Allow Power Down of ACMP1 During EM23"]
pub type Acmp1disR = crate::BitReader;
#[doc = "Field `ACMP1DIS` writer - Allow Power Down of ACMP1 During EM23"]
pub type Acmp1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0DIS` reader - Allow Power Down of PCNT0 During EM23"]
pub type Pcnt0disR = crate::BitReader;
#[doc = "Field `PCNT0DIS` writer - Allow Power Down of PCNT0 During EM23"]
pub type Pcnt0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1DIS` reader - Allow Power Down of PCNT1 During EM23"]
pub type Pcnt1disR = crate::BitReader;
#[doc = "Field `PCNT1DIS` writer - Allow Power Down of PCNT1 During EM23"]
pub type Pcnt1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2DIS` reader - Allow Power Down of PCNT2 During EM23"]
pub type Pcnt2disR = crate::BitReader;
#[doc = "Field `PCNT2DIS` writer - Allow Power Down of PCNT2 During EM23"]
pub type Pcnt2disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0DIS` reader - Allow Power Down of I2C0 During EM23"]
pub type I2c0disR = crate::BitReader;
#[doc = "Field `I2C0DIS` writer - Allow Power Down of I2C0 During EM23"]
pub type I2c0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1DIS` reader - Allow Power Down of I2C1 During EM23"]
pub type I2c1disR = crate::BitReader;
#[doc = "Field `I2C1DIS` writer - Allow Power Down of I2C1 During EM23"]
pub type I2c1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0DIS` reader - Allow Power Down of DAC0 During EM23"]
pub type Vdac0disR = crate::BitReader;
#[doc = "Field `VDAC0DIS` writer - Allow Power Down of DAC0 During EM23"]
pub type Vdac0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0DIS` reader - Allow Power Down of IDAC0 During EM23"]
pub type Idac0disR = crate::BitReader;
#[doc = "Field `IDAC0DIS` writer - Allow Power Down of IDAC0 During EM23"]
pub type Idac0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0DIS` reader - Allow Power Down of ADC0 During EM23"]
pub type Adc0disR = crate::BitReader;
#[doc = "Field `ADC0DIS` writer - Allow Power Down of ADC0 During EM23"]
pub type Adc0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0DIS` reader - Allow Power Down of LETIMER0 During EM23"]
pub type Letimer0disR = crate::BitReader;
#[doc = "Field `LETIMER0DIS` writer - Allow Power Down of LETIMER0 During EM23"]
pub type Letimer0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0DIS` reader - Allow Power Down of WDOG0 During EM23"]
pub type Wdog0disR = crate::BitReader;
#[doc = "Field `WDOG0DIS` writer - Allow Power Down of WDOG0 During EM23"]
pub type Wdog0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1DIS` reader - Allow Power Down of WDOG1 During EM23"]
pub type Wdog1disR = crate::BitReader;
#[doc = "Field `WDOG1DIS` writer - Allow Power Down of WDOG1 During EM23"]
pub type Wdog1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE0DIS` reader - Allow Power Down of LESENSE0 During EM23"]
pub type Lesense0disR = crate::BitReader;
#[doc = "Field `LESENSE0DIS` writer - Allow Power Down of LESENSE0 During EM23"]
pub type Lesense0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSENDIS` reader - Allow Power Down of CSEN During EM23"]
pub type CsendisR = crate::BitReader;
#[doc = "Field `CSENDIS` writer - Allow Power Down of CSEN During EM23"]
pub type CsendisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART0DIS` reader - Allow Power Down of LEUART0 During EM23"]
pub type Leuart0disR = crate::BitReader;
#[doc = "Field `LEUART0DIS` writer - Allow Power Down of LEUART0 During EM23"]
pub type Leuart0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1DIS` reader - Allow Power Down of LEUART1 During EM23"]
pub type Leuart1disR = crate::BitReader;
#[doc = "Field `LEUART1DIS` writer - Allow Power Down of LEUART1 During EM23"]
pub type Leuart1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDDIS` reader - Allow Power Down of LCD During EM23"]
pub type LcddisR = crate::BitReader;
#[doc = "Field `LCDDIS` writer - Allow Power Down of LCD During EM23"]
pub type LcddisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1DIS` reader - Allow Power Down of LETIMER1 During EM23"]
pub type Letimer1disR = crate::BitReader;
#[doc = "Field `LETIMER1DIS` writer - Allow Power Down of LETIMER1 During EM23"]
pub type Letimer1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2DIS` reader - Allow Power Down of I2C2 During EM23"]
pub type I2c2disR = crate::BitReader;
#[doc = "Field `I2C2DIS` writer - Allow Power Down of I2C2 During EM23"]
pub type I2c2disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1DIS` reader - Allow Power Down of ADC1 During EM23"]
pub type Adc1disR = crate::BitReader;
#[doc = "Field `ADC1DIS` writer - Allow Power Down of ADC1 During EM23"]
pub type Adc1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2DIS` reader - Allow Power Down of ACMP2 During EM23"]
pub type Acmp2disR = crate::BitReader;
#[doc = "Field `ACMP2DIS` writer - Allow Power Down of ACMP2 During EM23"]
pub type Acmp2disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3DIS` reader - Allow Power Down of ACMP3 During EM23"]
pub type Acmp3disR = crate::BitReader;
#[doc = "Field `ACMP3DIS` writer - Allow Power Down of ACMP3 During EM23"]
pub type Acmp3disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCDIS` reader - Allow Power Down of RTC During EM23"]
pub type RtcdisR = crate::BitReader;
#[doc = "Field `RTCDIS` writer - Allow Power Down of RTC During EM23"]
pub type RtcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBDIS` reader - Allow Power Down of USB During EM23"]
pub type UsbdisR = crate::BitReader;
#[doc = "Field `USBDIS` writer - Allow Power Down of USB During EM23"]
pub type UsbdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&self) -> Acmp0disR {
        Acmp0disR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&self) -> Acmp1disR {
        Acmp1disR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&self) -> Pcnt0disR {
        Pcnt0disR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow Power Down of PCNT1 During EM23"]
    #[inline(always)]
    pub fn pcnt1dis(&self) -> Pcnt1disR {
        Pcnt1disR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Allow Power Down of PCNT2 During EM23"]
    #[inline(always)]
    pub fn pcnt2dis(&self) -> Pcnt2disR {
        Pcnt2disR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&self) -> I2c0disR {
        I2c0disR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&self) -> I2c1disR {
        I2c1disR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&self) -> Vdac0disR {
        Vdac0disR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Allow Power Down of IDAC0 During EM23"]
    #[inline(always)]
    pub fn idac0dis(&self) -> Idac0disR {
        Idac0disR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&self) -> Adc0disR {
        Adc0disR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&self) -> Letimer0disR {
        Letimer0disR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&self) -> Wdog0disR {
        Wdog0disR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Allow Power Down of WDOG1 During EM23"]
    #[inline(always)]
    pub fn wdog1dis(&self) -> Wdog1disR {
        Wdog1disR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&self) -> Lesense0disR {
        Lesense0disR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&self) -> CsendisR {
        CsendisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&self) -> Leuart0disR {
        Leuart0disR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Allow Power Down of LEUART1 During EM23"]
    #[inline(always)]
    pub fn leuart1dis(&self) -> Leuart1disR {
        Leuart1disR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Allow Power Down of LCD During EM23"]
    #[inline(always)]
    pub fn lcddis(&self) -> LcddisR {
        LcddisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Allow Power Down of LETIMER1 During EM23"]
    #[inline(always)]
    pub fn letimer1dis(&self) -> Letimer1disR {
        Letimer1disR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Allow Power Down of I2C2 During EM23"]
    #[inline(always)]
    pub fn i2c2dis(&self) -> I2c2disR {
        I2c2disR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Allow Power Down of ADC1 During EM23"]
    #[inline(always)]
    pub fn adc1dis(&self) -> Adc1disR {
        Adc1disR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Allow Power Down of ACMP2 During EM23"]
    #[inline(always)]
    pub fn acmp2dis(&self) -> Acmp2disR {
        Acmp2disR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Allow Power Down of ACMP3 During EM23"]
    #[inline(always)]
    pub fn acmp3dis(&self) -> Acmp3disR {
        Acmp3disR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Allow Power Down of RTC During EM23"]
    #[inline(always)]
    pub fn rtcdis(&self) -> RtcdisR {
        RtcdisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Allow Power Down of USB During EM23"]
    #[inline(always)]
    pub fn usbdis(&self) -> UsbdisR {
        UsbdisR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&mut self) -> Acmp0disW<'_, Em23pernoretainctrlSpec> {
        Acmp0disW::new(self, 0)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&mut self) -> Acmp1disW<'_, Em23pernoretainctrlSpec> {
        Acmp1disW::new(self, 1)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&mut self) -> Pcnt0disW<'_, Em23pernoretainctrlSpec> {
        Pcnt0disW::new(self, 2)
    }
    #[doc = "Bit 3 - Allow Power Down of PCNT1 During EM23"]
    #[inline(always)]
    pub fn pcnt1dis(&mut self) -> Pcnt1disW<'_, Em23pernoretainctrlSpec> {
        Pcnt1disW::new(self, 3)
    }
    #[doc = "Bit 4 - Allow Power Down of PCNT2 During EM23"]
    #[inline(always)]
    pub fn pcnt2dis(&mut self) -> Pcnt2disW<'_, Em23pernoretainctrlSpec> {
        Pcnt2disW::new(self, 4)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&mut self) -> I2c0disW<'_, Em23pernoretainctrlSpec> {
        I2c0disW::new(self, 5)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&mut self) -> I2c1disW<'_, Em23pernoretainctrlSpec> {
        I2c1disW::new(self, 6)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&mut self) -> Vdac0disW<'_, Em23pernoretainctrlSpec> {
        Vdac0disW::new(self, 7)
    }
    #[doc = "Bit 8 - Allow Power Down of IDAC0 During EM23"]
    #[inline(always)]
    pub fn idac0dis(&mut self) -> Idac0disW<'_, Em23pernoretainctrlSpec> {
        Idac0disW::new(self, 8)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&mut self) -> Adc0disW<'_, Em23pernoretainctrlSpec> {
        Adc0disW::new(self, 9)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&mut self) -> Letimer0disW<'_, Em23pernoretainctrlSpec> {
        Letimer0disW::new(self, 10)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&mut self) -> Wdog0disW<'_, Em23pernoretainctrlSpec> {
        Wdog0disW::new(self, 11)
    }
    #[doc = "Bit 12 - Allow Power Down of WDOG1 During EM23"]
    #[inline(always)]
    pub fn wdog1dis(&mut self) -> Wdog1disW<'_, Em23pernoretainctrlSpec> {
        Wdog1disW::new(self, 12)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&mut self) -> Lesense0disW<'_, Em23pernoretainctrlSpec> {
        Lesense0disW::new(self, 13)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&mut self) -> CsendisW<'_, Em23pernoretainctrlSpec> {
        CsendisW::new(self, 14)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&mut self) -> Leuart0disW<'_, Em23pernoretainctrlSpec> {
        Leuart0disW::new(self, 15)
    }
    #[doc = "Bit 16 - Allow Power Down of LEUART1 During EM23"]
    #[inline(always)]
    pub fn leuart1dis(&mut self) -> Leuart1disW<'_, Em23pernoretainctrlSpec> {
        Leuart1disW::new(self, 16)
    }
    #[doc = "Bit 17 - Allow Power Down of LCD During EM23"]
    #[inline(always)]
    pub fn lcddis(&mut self) -> LcddisW<'_, Em23pernoretainctrlSpec> {
        LcddisW::new(self, 17)
    }
    #[doc = "Bit 18 - Allow Power Down of LETIMER1 During EM23"]
    #[inline(always)]
    pub fn letimer1dis(&mut self) -> Letimer1disW<'_, Em23pernoretainctrlSpec> {
        Letimer1disW::new(self, 18)
    }
    #[doc = "Bit 19 - Allow Power Down of I2C2 During EM23"]
    #[inline(always)]
    pub fn i2c2dis(&mut self) -> I2c2disW<'_, Em23pernoretainctrlSpec> {
        I2c2disW::new(self, 19)
    }
    #[doc = "Bit 20 - Allow Power Down of ADC1 During EM23"]
    #[inline(always)]
    pub fn adc1dis(&mut self) -> Adc1disW<'_, Em23pernoretainctrlSpec> {
        Adc1disW::new(self, 20)
    }
    #[doc = "Bit 21 - Allow Power Down of ACMP2 During EM23"]
    #[inline(always)]
    pub fn acmp2dis(&mut self) -> Acmp2disW<'_, Em23pernoretainctrlSpec> {
        Acmp2disW::new(self, 21)
    }
    #[doc = "Bit 22 - Allow Power Down of ACMP3 During EM23"]
    #[inline(always)]
    pub fn acmp3dis(&mut self) -> Acmp3disW<'_, Em23pernoretainctrlSpec> {
        Acmp3disW::new(self, 22)
    }
    #[doc = "Bit 23 - Allow Power Down of RTC During EM23"]
    #[inline(always)]
    pub fn rtcdis(&mut self) -> RtcdisW<'_, Em23pernoretainctrlSpec> {
        RtcdisW::new(self, 23)
    }
    #[doc = "Bit 24 - Allow Power Down of USB During EM23"]
    #[inline(always)]
    pub fn usbdis(&mut self) -> UsbdisW<'_, Em23pernoretainctrlSpec> {
        UsbdisW::new(self, 24)
    }
}
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23\n\nYou can [`read`](crate::Reg::read) this register and get [`em23pernoretainctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23pernoretainctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em23pernoretainctrlSpec;
impl crate::RegisterSpec for Em23pernoretainctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em23pernoretainctrl::R`](R) reader structure"]
impl crate::Readable for Em23pernoretainctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`em23pernoretainctrl::W`](W) writer structure"]
impl crate::Writable for Em23pernoretainctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM23PERNORETAINCTRL to value 0"]
impl crate::Resettable for Em23pernoretainctrlSpec {}
