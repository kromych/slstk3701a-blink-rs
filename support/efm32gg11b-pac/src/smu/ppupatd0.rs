#[doc = "Register `PPUPATD0` reader"]
pub type R = crate::R<Ppupatd0Spec>;
#[doc = "Register `PPUPATD0` writer"]
pub type W = crate::W<Ppupatd0Spec>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 access control bit"]
pub type Acmp0R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 access control bit"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 access control bit"]
pub type Acmp1R = crate::BitReader;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 access control bit"]
pub type Acmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2` reader - Analog Comparator 1 access control bit"]
pub type Acmp2R = crate::BitReader;
#[doc = "Field `ACMP2` writer - Analog Comparator 1 access control bit"]
pub type Acmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3` reader - Analog Comparator 3 access control bit"]
pub type Acmp3R = crate::BitReader;
#[doc = "Field `ACMP3` writer - Analog Comparator 3 access control bit"]
pub type Acmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 access control bit"]
pub type Adc0R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 access control bit"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - Analog to Digital Converter 0 access control bit"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - Analog to Digital Converter 0 access control bit"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0` reader - CAN 0 access control bit"]
pub type Can0R = crate::BitReader;
#[doc = "Field `CAN0` writer - CAN 0 access control bit"]
pub type Can0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN 1 access control bit"]
pub type Can1R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN 1 access control bit"]
pub type Can1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMU` reader - Clock Management Unit access control bit"]
pub type CmuR = crate::BitReader;
#[doc = "Field `CMU` writer - Clock Management Unit access control bit"]
pub type CmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER access control bit"]
pub type CryotimerR = crate::BitReader;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER access control bit"]
pub type CryotimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO0` reader - Advanced Encryption Standard Accelerator access control bit"]
pub type Crypto0R = crate::BitReader;
#[doc = "Field `CRYPTO0` writer - Advanced Encryption Standard Accelerator access control bit"]
pub type Crypto0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module access control bit"]
pub type CsenR = crate::BitReader;
#[doc = "Field `CSEN` writer - Capacitive touch sense module access control bit"]
pub type CsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 access control bit"]
pub type Vdac0R = crate::BitReader;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 access control bit"]
pub type Vdac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - Peripheral Reflex System access control bit"]
pub type PrsR = crate::BitReader;
#[doc = "Field `PRS` writer - Peripheral Reflex System access control bit"]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBI` reader - External Bus Interface access control bit"]
pub type EbiR = crate::BitReader;
#[doc = "Field `EBI` writer - External Bus Interface access control bit"]
pub type EbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMU` reader - Energy Management Unit access control bit"]
pub type EmuR = crate::BitReader;
#[doc = "Field `EMU` writer - Energy Management Unit access control bit"]
pub type EmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH` reader - Ethernet Controller access control bit"]
pub type EthR = crate::BitReader;
#[doc = "Field `ETH` writer - Ethernet Controller access control bit"]
pub type EthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUEH` reader - FPU Exception Handler access control bit"]
pub type FpuehR = crate::BitReader;
#[doc = "Field `FPUEH` writer - FPU Exception Handler access control bit"]
pub type FpuehW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPCRC` reader - General Purpose CRC access control bit"]
pub type GpcrcR = crate::BitReader;
#[doc = "Field `GPCRC` writer - General Purpose CRC access control bit"]
pub type GpcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - General purpose Input/Output access control bit"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - General purpose Input/Output access control bit"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C 0 access control bit"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 access control bit"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C 1 access control bit"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C 1 access control bit"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C 2 access control bit"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C 2 access control bit"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 access control bit"]
pub type Idac0R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 access control bit"]
pub type Idac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - Memory System Controller access control bit"]
pub type MscR = crate::BitReader;
#[doc = "Field `MSC` writer - Memory System Controller access control bit"]
pub type MscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller access control bit"]
pub type LcdR = crate::BitReader;
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller access control bit"]
pub type LcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller access control bit"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller access control bit"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface access control bit"]
pub type LesenseR = crate::BitReader;
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface access control bit"]
pub type LesenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 access control bit"]
pub type Letimer0R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 access control bit"]
pub type Letimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1` reader - Low Energy Timer 1 access control bit"]
pub type Letimer1R = crate::BitReader;
#[doc = "Field `LETIMER1` writer - Low Energy Timer 1 access control bit"]
pub type Letimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 access control bit"]
pub type Leuart0R = crate::BitReader;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 access control bit"]
pub type Leuart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1` reader - Low Energy UART 1 access control bit"]
pub type Leuart1R = crate::BitReader;
#[doc = "Field `LEUART1` writer - Low Energy UART 1 access control bit"]
pub type Leuart1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&self) -> Acmp1R {
        Acmp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp2(&self) -> Acmp2R {
        Acmp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 3 access control bit"]
    #[inline(always)]
    pub fn acmp3(&self) -> Acmp3R {
        Acmp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&self) -> Can0R {
        Can0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&self) -> Can1R {
        Can1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&self) -> CmuR {
        CmuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRYOTIMER access control bit"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CryotimerR {
        CryotimerR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&self) -> Crypto0R {
        Crypto0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&self) -> CsenR {
        CsenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&self) -> Vdac0R {
        Vdac0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&self) -> EbiR {
        EbiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&self) -> EmuR {
        EmuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Controller access control bit"]
    #[inline(always)]
    pub fn eth(&self) -> EthR {
        EthR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&self) -> FpuehR {
        FpuehR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GpcrcR {
        GpcrcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 access control bit"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&self) -> Idac0R {
        Idac0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&self) -> MscR {
        MscR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&self) -> LcdR {
        LcdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&self) -> LesenseR {
        LesenseR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&self) -> Letimer0R {
        Letimer0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&self) -> Letimer1R {
        Letimer1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&self) -> Leuart0R {
        Leuart0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&self) -> Leuart1R {
        Leuart1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<'_, Ppupatd0Spec> {
        Acmp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> Acmp1W<'_, Ppupatd0Spec> {
        Acmp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp2(&mut self) -> Acmp2W<'_, Ppupatd0Spec> {
        Acmp2W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog Comparator 3 access control bit"]
    #[inline(always)]
    pub fn acmp3(&mut self) -> Acmp3W<'_, Ppupatd0Spec> {
        Acmp3W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<'_, Ppupatd0Spec> {
        Adc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Ppupatd0Spec> {
        Adc1W::new(self, 5)
    }
    #[doc = "Bit 6 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&mut self) -> Can0W<'_, Ppupatd0Spec> {
        Can0W::new(self, 6)
    }
    #[doc = "Bit 7 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&mut self) -> Can1W<'_, Ppupatd0Spec> {
        Can1W::new(self, 7)
    }
    #[doc = "Bit 8 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&mut self) -> CmuW<'_, Ppupatd0Spec> {
        CmuW::new(self, 8)
    }
    #[doc = "Bit 9 - CRYOTIMER access control bit"]
    #[inline(always)]
    pub fn cryotimer(&mut self) -> CryotimerW<'_, Ppupatd0Spec> {
        CryotimerW::new(self, 9)
    }
    #[doc = "Bit 10 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&mut self) -> Crypto0W<'_, Ppupatd0Spec> {
        Crypto0W::new(self, 10)
    }
    #[doc = "Bit 11 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&mut self) -> CsenW<'_, Ppupatd0Spec> {
        CsenW::new(self, 11)
    }
    #[doc = "Bit 12 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> Vdac0W<'_, Ppupatd0Spec> {
        Vdac0W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, Ppupatd0Spec> {
        PrsW::new(self, 13)
    }
    #[doc = "Bit 14 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EbiW<'_, Ppupatd0Spec> {
        EbiW::new(self, 14)
    }
    #[doc = "Bit 15 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&mut self) -> EmuW<'_, Ppupatd0Spec> {
        EmuW::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet Controller access control bit"]
    #[inline(always)]
    pub fn eth(&mut self) -> EthW<'_, Ppupatd0Spec> {
        EthW::new(self, 16)
    }
    #[doc = "Bit 17 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&mut self) -> FpuehW<'_, Ppupatd0Spec> {
        FpuehW::new(self, 17)
    }
    #[doc = "Bit 18 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&mut self) -> GpcrcW<'_, Ppupatd0Spec> {
        GpcrcW::new(self, 18)
    }
    #[doc = "Bit 19 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<'_, Ppupatd0Spec> {
        GpioW::new(self, 19)
    }
    #[doc = "Bit 20 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, Ppupatd0Spec> {
        I2c0W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Ppupatd0Spec> {
        I2c1W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 access control bit"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, Ppupatd0Spec> {
        I2c2W::new(self, 22)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&mut self) -> Idac0W<'_, Ppupatd0Spec> {
        Idac0W::new(self, 23)
    }
    #[doc = "Bit 24 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&mut self) -> MscW<'_, Ppupatd0Spec> {
        MscW::new(self, 24)
    }
    #[doc = "Bit 25 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LcdW<'_, Ppupatd0Spec> {
        LcdW::new(self, 25)
    }
    #[doc = "Bit 26 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<'_, Ppupatd0Spec> {
        LdmaW::new(self, 26)
    }
    #[doc = "Bit 27 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LesenseW<'_, Ppupatd0Spec> {
        LesenseW::new(self, 27)
    }
    #[doc = "Bit 28 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> Letimer0W<'_, Ppupatd0Spec> {
        Letimer0W::new(self, 28)
    }
    #[doc = "Bit 29 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> Letimer1W<'_, Ppupatd0Spec> {
        Letimer1W::new(self, 29)
    }
    #[doc = "Bit 30 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> Leuart0W<'_, Ppupatd0Spec> {
        Leuart0W::new(self, 30)
    }
    #[doc = "Bit 31 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&mut self) -> Leuart1W<'_, Ppupatd0Spec> {
        Leuart1W::new(self, 31)
    }
}
#[doc = "PPU Privilege Access Type Descriptor 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppupatd0Spec;
impl crate::RegisterSpec for Ppupatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppupatd0::R`](R) reader structure"]
impl crate::Readable for Ppupatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`ppupatd0::W`](W) writer structure"]
impl crate::Writable for Ppupatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUPATD0 to value 0"]
impl crate::Resettable for Ppupatd0Spec {}
