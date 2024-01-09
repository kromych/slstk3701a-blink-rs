#[doc = "Register `PPUFS` reader"]
pub type R = crate::R<PPUFS_SPEC>;
#[doc = "Field `PERIPHID` reader - "]
pub type PERIPHID_R = crate::FieldReader<PERIPHID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERIPHID_A {
    #[doc = "0: Analog Comparator 0"]
    ACMP0 = 0,
    #[doc = "1: Analog Comparator 1"]
    ACMP1 = 1,
    #[doc = "2: Analog Comparator 1"]
    ACMP2 = 2,
    #[doc = "3: Analog Comparator 3"]
    ACMP3 = 3,
    #[doc = "4: Analog to Digital Converter 0"]
    ADC0 = 4,
    #[doc = "5: Analog to Digital Converter 0"]
    ADC1 = 5,
    #[doc = "6: CAN 0"]
    CAN0 = 6,
    #[doc = "7: CAN 1"]
    CAN1 = 7,
    #[doc = "8: Clock Management Unit"]
    CMU = 8,
    #[doc = "9: CRYOTIMER"]
    CRYOTIMER = 9,
    #[doc = "10: Advanced Encryption Standard Accelerator"]
    CRYPTO0 = 10,
    #[doc = "11: Capacitive touch sense module"]
    CSEN = 11,
    #[doc = "12: Digital to Analog Converter 0"]
    VDAC0 = 12,
    #[doc = "13: Peripheral Reflex System"]
    PRS = 13,
    #[doc = "14: External Bus Interface"]
    EBI = 14,
    #[doc = "15: Energy Management Unit"]
    EMU = 15,
    #[doc = "16: Ethernet Controller"]
    ETH = 16,
    #[doc = "17: FPU Exception Handler"]
    FPUEH = 17,
    #[doc = "18: General Purpose CRC"]
    GPCRC = 18,
    #[doc = "19: General purpose Input/Output"]
    GPIO = 19,
    #[doc = "20: I2C 0"]
    I2C0 = 20,
    #[doc = "21: I2C 1"]
    I2C1 = 21,
    #[doc = "22: I2C 2"]
    I2C2 = 22,
    #[doc = "23: Current Digital to Analog Converter 0"]
    IDAC0 = 23,
    #[doc = "24: Memory System Controller"]
    MSC = 24,
    #[doc = "25: Liquid Crystal Display Controller"]
    LCD = 25,
    #[doc = "26: Linked Direct Memory Access Controller"]
    LDMA = 26,
    #[doc = "27: Low Energy Sensor Interface"]
    LESENSE = 27,
    #[doc = "28: Low Energy Timer 0"]
    LETIMER0 = 28,
    #[doc = "29: Low Energy Timer 1"]
    LETIMER1 = 29,
    #[doc = "30: Low Energy UART 0"]
    LEUART0 = 30,
    #[doc = "31: Low Energy UART 1"]
    LEUART1 = 31,
    #[doc = "32: Pulse Counter 0"]
    PCNT0 = 32,
    #[doc = "33: Pulse Counter 1"]
    PCNT1 = 33,
    #[doc = "34: Pulse Counter 2"]
    PCNT2 = 34,
    #[doc = "35: Quad-SPI"]
    QSPI0 = 35,
    #[doc = "36: Reset Management Unit"]
    RMU = 36,
    #[doc = "37: Real-Time Counter"]
    RTC = 37,
    #[doc = "38: Real-Time Counter and Calendar"]
    RTCC = 38,
    #[doc = "39: SDIO Controller"]
    SDIO = 39,
    #[doc = "40: Security Management Unit"]
    SMU = 40,
    #[doc = "41: Timer 0"]
    TIMER0 = 41,
    #[doc = "42: Timer 1"]
    TIMER1 = 42,
    #[doc = "43: Timer 2"]
    TIMER2 = 43,
    #[doc = "44: Timer 3"]
    TIMER3 = 44,
    #[doc = "45: Timer 4"]
    TIMER4 = 45,
    #[doc = "46: Timer 5"]
    TIMER5 = 46,
    #[doc = "47: Timer 6"]
    TIMER6 = 47,
    #[doc = "48: True Random Number Generator 0"]
    TRNG0 = 48,
    #[doc = "49: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 49,
    #[doc = "50: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 51,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 52,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 53,
    #[doc = "54: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 54,
    #[doc = "55: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4 = 55,
    #[doc = "56: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5 = 56,
    #[doc = "57: Universal Serial Bus Interface"]
    USB = 57,
    #[doc = "58: Watchdog"]
    WDOG0 = 58,
    #[doc = "59: Watchdog"]
    WDOG1 = 59,
    #[doc = "60: Wide Timer 0"]
    WTIMER0 = 60,
    #[doc = "61: Wide Timer 0"]
    WTIMER1 = 61,
    #[doc = "62: Wide Timer 2"]
    WTIMER2 = 62,
    #[doc = "63: Wide Timer 3"]
    WTIMER3 = 63,
}
impl From<PERIPHID_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPHID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERIPHID_A {
    type Ux = u8;
}
impl PERIPHID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PERIPHID_A> {
        match self.bits {
            0 => Some(PERIPHID_A::ACMP0),
            1 => Some(PERIPHID_A::ACMP1),
            2 => Some(PERIPHID_A::ACMP2),
            3 => Some(PERIPHID_A::ACMP3),
            4 => Some(PERIPHID_A::ADC0),
            5 => Some(PERIPHID_A::ADC1),
            6 => Some(PERIPHID_A::CAN0),
            7 => Some(PERIPHID_A::CAN1),
            8 => Some(PERIPHID_A::CMU),
            9 => Some(PERIPHID_A::CRYOTIMER),
            10 => Some(PERIPHID_A::CRYPTO0),
            11 => Some(PERIPHID_A::CSEN),
            12 => Some(PERIPHID_A::VDAC0),
            13 => Some(PERIPHID_A::PRS),
            14 => Some(PERIPHID_A::EBI),
            15 => Some(PERIPHID_A::EMU),
            16 => Some(PERIPHID_A::ETH),
            17 => Some(PERIPHID_A::FPUEH),
            18 => Some(PERIPHID_A::GPCRC),
            19 => Some(PERIPHID_A::GPIO),
            20 => Some(PERIPHID_A::I2C0),
            21 => Some(PERIPHID_A::I2C1),
            22 => Some(PERIPHID_A::I2C2),
            23 => Some(PERIPHID_A::IDAC0),
            24 => Some(PERIPHID_A::MSC),
            25 => Some(PERIPHID_A::LCD),
            26 => Some(PERIPHID_A::LDMA),
            27 => Some(PERIPHID_A::LESENSE),
            28 => Some(PERIPHID_A::LETIMER0),
            29 => Some(PERIPHID_A::LETIMER1),
            30 => Some(PERIPHID_A::LEUART0),
            31 => Some(PERIPHID_A::LEUART1),
            32 => Some(PERIPHID_A::PCNT0),
            33 => Some(PERIPHID_A::PCNT1),
            34 => Some(PERIPHID_A::PCNT2),
            35 => Some(PERIPHID_A::QSPI0),
            36 => Some(PERIPHID_A::RMU),
            37 => Some(PERIPHID_A::RTC),
            38 => Some(PERIPHID_A::RTCC),
            39 => Some(PERIPHID_A::SDIO),
            40 => Some(PERIPHID_A::SMU),
            41 => Some(PERIPHID_A::TIMER0),
            42 => Some(PERIPHID_A::TIMER1),
            43 => Some(PERIPHID_A::TIMER2),
            44 => Some(PERIPHID_A::TIMER3),
            45 => Some(PERIPHID_A::TIMER4),
            46 => Some(PERIPHID_A::TIMER5),
            47 => Some(PERIPHID_A::TIMER6),
            48 => Some(PERIPHID_A::TRNG0),
            49 => Some(PERIPHID_A::UART0),
            50 => Some(PERIPHID_A::UART1),
            51 => Some(PERIPHID_A::USART0),
            52 => Some(PERIPHID_A::USART1),
            53 => Some(PERIPHID_A::USART2),
            54 => Some(PERIPHID_A::USART3),
            55 => Some(PERIPHID_A::USART4),
            56 => Some(PERIPHID_A::USART5),
            57 => Some(PERIPHID_A::USB),
            58 => Some(PERIPHID_A::WDOG0),
            59 => Some(PERIPHID_A::WDOG1),
            60 => Some(PERIPHID_A::WTIMER0),
            61 => Some(PERIPHID_A::WTIMER1),
            62 => Some(PERIPHID_A::WTIMER2),
            63 => Some(PERIPHID_A::WTIMER3),
            _ => None,
        }
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == PERIPHID_A::ACMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == PERIPHID_A::ACMP1
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == PERIPHID_A::ACMP2
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == PERIPHID_A::ACMP3
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == PERIPHID_A::ADC0
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == PERIPHID_A::ADC1
    }
    #[doc = "CAN 0"]
    #[inline(always)]
    pub fn is_can0(&self) -> bool {
        *self == PERIPHID_A::CAN0
    }
    #[doc = "CAN 1"]
    #[inline(always)]
    pub fn is_can1(&self) -> bool {
        *self == PERIPHID_A::CAN1
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == PERIPHID_A::CMU
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == PERIPHID_A::CRYOTIMER
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == PERIPHID_A::CRYPTO0
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == PERIPHID_A::CSEN
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == PERIPHID_A::VDAC0
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == PERIPHID_A::PRS
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn is_ebi(&self) -> bool {
        *self == PERIPHID_A::EBI
    }
    #[doc = "Energy Management Unit"]
    #[inline(always)]
    pub fn is_emu(&self) -> bool {
        *self == PERIPHID_A::EMU
    }
    #[doc = "Ethernet Controller"]
    #[inline(always)]
    pub fn is_eth(&self) -> bool {
        *self == PERIPHID_A::ETH
    }
    #[doc = "FPU Exception Handler"]
    #[inline(always)]
    pub fn is_fpueh(&self) -> bool {
        *self == PERIPHID_A::FPUEH
    }
    #[doc = "General Purpose CRC"]
    #[inline(always)]
    pub fn is_gpcrc(&self) -> bool {
        *self == PERIPHID_A::GPCRC
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PERIPHID_A::GPIO
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == PERIPHID_A::I2C0
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == PERIPHID_A::I2C1
    }
    #[doc = "I2C 2"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        *self == PERIPHID_A::I2C2
    }
    #[doc = "Current Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_idac0(&self) -> bool {
        *self == PERIPHID_A::IDAC0
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == PERIPHID_A::MSC
    }
    #[doc = "Liquid Crystal Display Controller"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        *self == PERIPHID_A::LCD
    }
    #[doc = "Linked Direct Memory Access Controller"]
    #[inline(always)]
    pub fn is_ldma(&self) -> bool {
        *self == PERIPHID_A::LDMA
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == PERIPHID_A::LESENSE
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == PERIPHID_A::LETIMER0
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == PERIPHID_A::LETIMER1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == PERIPHID_A::LEUART0
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == PERIPHID_A::LEUART1
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == PERIPHID_A::PCNT0
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == PERIPHID_A::PCNT1
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == PERIPHID_A::PCNT2
    }
    #[doc = "Quad-SPI"]
    #[inline(always)]
    pub fn is_qspi0(&self) -> bool {
        *self == PERIPHID_A::QSPI0
    }
    #[doc = "Reset Management Unit"]
    #[inline(always)]
    pub fn is_rmu(&self) -> bool {
        *self == PERIPHID_A::RMU
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == PERIPHID_A::RTC
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == PERIPHID_A::RTCC
    }
    #[doc = "SDIO Controller"]
    #[inline(always)]
    pub fn is_sdio(&self) -> bool {
        *self == PERIPHID_A::SDIO
    }
    #[doc = "Security Management Unit"]
    #[inline(always)]
    pub fn is_smu(&self) -> bool {
        *self == PERIPHID_A::SMU
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == PERIPHID_A::TIMER0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == PERIPHID_A::TIMER1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == PERIPHID_A::TIMER2
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == PERIPHID_A::TIMER3
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn is_timer4(&self) -> bool {
        *self == PERIPHID_A::TIMER4
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn is_timer5(&self) -> bool {
        *self == PERIPHID_A::TIMER5
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn is_timer6(&self) -> bool {
        *self == PERIPHID_A::TIMER6
    }
    #[doc = "True Random Number Generator 0"]
    #[inline(always)]
    pub fn is_trng0(&self) -> bool {
        *self == PERIPHID_A::TRNG0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == PERIPHID_A::UART0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == PERIPHID_A::UART1
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == PERIPHID_A::USART0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == PERIPHID_A::USART1
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == PERIPHID_A::USART2
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == PERIPHID_A::USART3
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == PERIPHID_A::USART4
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn is_usart5(&self) -> bool {
        *self == PERIPHID_A::USART5
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == PERIPHID_A::USB
    }
    #[doc = "Watchdog"]
    #[inline(always)]
    pub fn is_wdog0(&self) -> bool {
        *self == PERIPHID_A::WDOG0
    }
    #[doc = "Watchdog"]
    #[inline(always)]
    pub fn is_wdog1(&self) -> bool {
        *self == PERIPHID_A::WDOG1
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == PERIPHID_A::WTIMER0
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == PERIPHID_A::WTIMER1
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn is_wtimer2(&self) -> bool {
        *self == PERIPHID_A::WTIMER2
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn is_wtimer3(&self) -> bool {
        *self == PERIPHID_A::WTIMER3
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "PPU Fault Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppufs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPUFS_SPEC;
impl crate::RegisterSpec for PPUFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppufs::R`](R) reader structure"]
impl crate::Readable for PPUFS_SPEC {}
#[doc = "`reset()` method sets PPUFS to value 0"]
impl crate::Resettable for PPUFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
