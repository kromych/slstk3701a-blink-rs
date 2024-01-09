#[doc = "Register `PPUPATD0` reader"]
pub type R = crate::R<PPUPATD0_SPEC>;
#[doc = "Register `PPUPATD0` writer"]
pub type W = crate::W<PPUPATD0_SPEC>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 access control bit"]
pub type ACMP0_R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 access control bit"]
pub type ACMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 access control bit"]
pub type ACMP1_R = crate::BitReader;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 access control bit"]
pub type ACMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2` reader - Analog Comparator 1 access control bit"]
pub type ACMP2_R = crate::BitReader;
#[doc = "Field `ACMP2` writer - Analog Comparator 1 access control bit"]
pub type ACMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3` reader - Analog Comparator 3 access control bit"]
pub type ACMP3_R = crate::BitReader;
#[doc = "Field `ACMP3` writer - Analog Comparator 3 access control bit"]
pub type ACMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 access control bit"]
pub type ADC0_R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 access control bit"]
pub type ADC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - Analog to Digital Converter 0 access control bit"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - Analog to Digital Converter 0 access control bit"]
pub type ADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0` reader - CAN 0 access control bit"]
pub type CAN0_R = crate::BitReader;
#[doc = "Field `CAN0` writer - CAN 0 access control bit"]
pub type CAN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN 1 access control bit"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN 1 access control bit"]
pub type CAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMU` reader - Clock Management Unit access control bit"]
pub type CMU_R = crate::BitReader;
#[doc = "Field `CMU` writer - Clock Management Unit access control bit"]
pub type CMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER access control bit"]
pub type CRYOTIMER_R = crate::BitReader;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER access control bit"]
pub type CRYOTIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO0` reader - Advanced Encryption Standard Accelerator access control bit"]
pub type CRYPTO0_R = crate::BitReader;
#[doc = "Field `CRYPTO0` writer - Advanced Encryption Standard Accelerator access control bit"]
pub type CRYPTO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module access control bit"]
pub type CSEN_R = crate::BitReader;
#[doc = "Field `CSEN` writer - Capacitive touch sense module access control bit"]
pub type CSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 access control bit"]
pub type VDAC0_R = crate::BitReader;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 access control bit"]
pub type VDAC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - Peripheral Reflex System access control bit"]
pub type PRS_R = crate::BitReader;
#[doc = "Field `PRS` writer - Peripheral Reflex System access control bit"]
pub type PRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBI` reader - External Bus Interface access control bit"]
pub type EBI_R = crate::BitReader;
#[doc = "Field `EBI` writer - External Bus Interface access control bit"]
pub type EBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMU` reader - Energy Management Unit access control bit"]
pub type EMU_R = crate::BitReader;
#[doc = "Field `EMU` writer - Energy Management Unit access control bit"]
pub type EMU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH` reader - Ethernet Controller access control bit"]
pub type ETH_R = crate::BitReader;
#[doc = "Field `ETH` writer - Ethernet Controller access control bit"]
pub type ETH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUEH` reader - FPU Exception Handler access control bit"]
pub type FPUEH_R = crate::BitReader;
#[doc = "Field `FPUEH` writer - FPU Exception Handler access control bit"]
pub type FPUEH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPCRC` reader - General Purpose CRC access control bit"]
pub type GPCRC_R = crate::BitReader;
#[doc = "Field `GPCRC` writer - General Purpose CRC access control bit"]
pub type GPCRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - General purpose Input/Output access control bit"]
pub type GPIO_R = crate::BitReader;
#[doc = "Field `GPIO` writer - General purpose Input/Output access control bit"]
pub type GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C 0 access control bit"]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 access control bit"]
pub type I2C0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C 1 access control bit"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C 1 access control bit"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C 2 access control bit"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C 2 access control bit"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 access control bit"]
pub type IDAC0_R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 access control bit"]
pub type IDAC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - Memory System Controller access control bit"]
pub type MSC_R = crate::BitReader;
#[doc = "Field `MSC` writer - Memory System Controller access control bit"]
pub type MSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller access control bit"]
pub type LCD_R = crate::BitReader;
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller access control bit"]
pub type LCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller access control bit"]
pub type LDMA_R = crate::BitReader;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller access control bit"]
pub type LDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface access control bit"]
pub type LESENSE_R = crate::BitReader;
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface access control bit"]
pub type LESENSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 access control bit"]
pub type LETIMER0_R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 access control bit"]
pub type LETIMER0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER1` reader - Low Energy Timer 1 access control bit"]
pub type LETIMER1_R = crate::BitReader;
#[doc = "Field `LETIMER1` writer - Low Energy Timer 1 access control bit"]
pub type LETIMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 access control bit"]
pub type LEUART0_R = crate::BitReader;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 access control bit"]
pub type LEUART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEUART1` reader - Low Energy UART 1 access control bit"]
pub type LEUART1_R = crate::BitReader;
#[doc = "Field `LEUART1` writer - Low Energy UART 1 access control bit"]
pub type LEUART1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp2(&self) -> ACMP2_R {
        ACMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 3 access control bit"]
    #[inline(always)]
    pub fn acmp3(&self) -> ACMP3_R {
        ACMP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&self) -> CMU_R {
        CMU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRYOTIMER access control bit"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CRYOTIMER_R {
        CRYOTIMER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&self) -> CRYPTO0_R {
        CRYPTO0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&self) -> EBI_R {
        EBI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&self) -> EMU_R {
        EMU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Controller access control bit"]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&self) -> FPUEH_R {
        FPUEH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 access control bit"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&self) -> LETIMER1_R {
        LETIMER1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<PPUPATD0_SPEC> {
        ACMP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> ACMP1_W<PPUPATD0_SPEC> {
        ACMP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2(&mut self) -> ACMP2_W<PPUPATD0_SPEC> {
        ACMP2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog Comparator 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3(&mut self) -> ACMP3_W<PPUPATD0_SPEC> {
        ACMP3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<PPUPATD0_SPEC> {
        ADC0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<PPUPATD0_SPEC> {
        ADC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - CAN 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn can0(&mut self) -> CAN0_W<PPUPATD0_SPEC> {
        CAN0_W::new(self, 6)
    }
    #[doc = "Bit 7 - CAN 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<PPUPATD0_SPEC> {
        CAN1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clock Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmu(&mut self) -> CMU_W<PPUPATD0_SPEC> {
        CMU_W::new(self, 8)
    }
    #[doc = "Bit 9 - CRYOTIMER access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cryotimer(&mut self) -> CRYOTIMER_W<PPUPATD0_SPEC> {
        CRYOTIMER_W::new(self, 9)
    }
    #[doc = "Bit 10 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crypto0(&mut self) -> CRYPTO0_W<PPUPATD0_SPEC> {
        CRYPTO0_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<PPUPATD0_SPEC> {
        CSEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn vdac0(&mut self) -> VDAC0_W<PPUPATD0_SPEC> {
        VDAC0_W::new(self, 12)
    }
    #[doc = "Bit 13 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<PPUPATD0_SPEC> {
        PRS_W::new(self, 13)
    }
    #[doc = "Bit 14 - External Bus Interface access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ebi(&mut self) -> EBI_W<PPUPATD0_SPEC> {
        EBI_W::new(self, 14)
    }
    #[doc = "Bit 15 - Energy Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn emu(&mut self) -> EMU_W<PPUPATD0_SPEC> {
        EMU_W::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn eth(&mut self) -> ETH_W<PPUPATD0_SPEC> {
        ETH_W::new(self, 16)
    }
    #[doc = "Bit 17 - FPU Exception Handler access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn fpueh(&mut self) -> FPUEH_W<PPUPATD0_SPEC> {
        FPUEH_W::new(self, 17)
    }
    #[doc = "Bit 18 - General Purpose CRC access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GPCRC_W<PPUPATD0_SPEC> {
        GPCRC_W::new(self, 18)
    }
    #[doc = "Bit 19 - General purpose Input/Output access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<PPUPATD0_SPEC> {
        GPIO_W::new(self, 19)
    }
    #[doc = "Bit 20 - I2C 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<PPUPATD0_SPEC> {
        I2C0_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<PPUPATD0_SPEC> {
        I2C1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<PPUPATD0_SPEC> {
        I2C2_W::new(self, 22)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn idac0(&mut self) -> IDAC0_W<PPUPATD0_SPEC> {
        IDAC0_W::new(self, 23)
    }
    #[doc = "Bit 24 - Memory System Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn msc(&mut self) -> MSC_W<PPUPATD0_SPEC> {
        MSC_W::new(self, 24)
    }
    #[doc = "Bit 25 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn lcd(&mut self) -> LCD_W<PPUPATD0_SPEC> {
        LCD_W::new(self, 25)
    }
    #[doc = "Bit 26 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LDMA_W<PPUPATD0_SPEC> {
        LDMA_W::new(self, 26)
    }
    #[doc = "Bit 27 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn lesense(&mut self) -> LESENSE_W<PPUPATD0_SPEC> {
        LESENSE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0(&mut self) -> LETIMER0_W<PPUPATD0_SPEC> {
        LETIMER0_W::new(self, 28)
    }
    #[doc = "Bit 29 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn letimer1(&mut self) -> LETIMER1_W<PPUPATD0_SPEC> {
        LETIMER1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<PPUPATD0_SPEC> {
        LEUART0_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn leuart1(&mut self) -> LEUART1_W<PPUPATD0_SPEC> {
        LEUART1_W::new(self, 31)
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
#[doc = "PPU Privilege Access Type Descriptor 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppupatd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppupatd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPUPATD0_SPEC;
impl crate::RegisterSpec for PPUPATD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppupatd0::R`](R) reader structure"]
impl crate::Readable for PPUPATD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppupatd0::W`](W) writer structure"]
impl crate::Writable for PPUPATD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PPUPATD0 to value 0"]
impl crate::Resettable for PPUPATD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
