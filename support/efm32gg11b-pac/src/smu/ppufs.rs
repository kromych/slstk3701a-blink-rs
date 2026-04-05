#[doc = "Register `PPUFS` reader"]
pub type R = crate::R<PpufsSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Periphid {
    #[doc = "0: Analog Comparator 0"]
    Acmp0 = 0,
    #[doc = "1: Analog Comparator 1"]
    Acmp1 = 1,
    #[doc = "2: Analog Comparator 1"]
    Acmp2 = 2,
    #[doc = "3: Analog Comparator 3"]
    Acmp3 = 3,
    #[doc = "4: Analog to Digital Converter 0"]
    Adc0 = 4,
    #[doc = "5: Analog to Digital Converter 0"]
    Adc1 = 5,
    #[doc = "6: CAN 0"]
    Can0 = 6,
    #[doc = "7: CAN 1"]
    Can1 = 7,
    #[doc = "8: Clock Management Unit"]
    Cmu = 8,
    #[doc = "9: CRYOTIMER"]
    Cryotimer = 9,
    #[doc = "10: Advanced Encryption Standard Accelerator"]
    Crypto0 = 10,
    #[doc = "11: Capacitive touch sense module"]
    Csen = 11,
    #[doc = "12: Digital to Analog Converter 0"]
    Vdac0 = 12,
    #[doc = "13: Peripheral Reflex System"]
    Prs = 13,
    #[doc = "14: External Bus Interface"]
    Ebi = 14,
    #[doc = "15: Energy Management Unit"]
    Emu = 15,
    #[doc = "16: Ethernet Controller"]
    Eth = 16,
    #[doc = "17: FPU Exception Handler"]
    Fpueh = 17,
    #[doc = "18: General Purpose CRC"]
    Gpcrc = 18,
    #[doc = "19: General purpose Input/Output"]
    Gpio = 19,
    #[doc = "20: I2C 0"]
    I2c0 = 20,
    #[doc = "21: I2C 1"]
    I2c1 = 21,
    #[doc = "22: I2C 2"]
    I2c2 = 22,
    #[doc = "23: Current Digital to Analog Converter 0"]
    Idac0 = 23,
    #[doc = "24: Memory System Controller"]
    Msc = 24,
    #[doc = "25: Liquid Crystal Display Controller"]
    Lcd = 25,
    #[doc = "26: Linked Direct Memory Access Controller"]
    Ldma = 26,
    #[doc = "27: Low Energy Sensor Interface"]
    Lesense = 27,
    #[doc = "28: Low Energy Timer 0"]
    Letimer0 = 28,
    #[doc = "29: Low Energy Timer 1"]
    Letimer1 = 29,
    #[doc = "30: Low Energy UART 0"]
    Leuart0 = 30,
    #[doc = "31: Low Energy UART 1"]
    Leuart1 = 31,
    #[doc = "32: Pulse Counter 0"]
    Pcnt0 = 32,
    #[doc = "33: Pulse Counter 1"]
    Pcnt1 = 33,
    #[doc = "34: Pulse Counter 2"]
    Pcnt2 = 34,
    #[doc = "35: Quad-SPI"]
    Qspi0 = 35,
    #[doc = "36: Reset Management Unit"]
    Rmu = 36,
    #[doc = "37: Real-Time Counter"]
    Rtc = 37,
    #[doc = "38: Real-Time Counter and Calendar"]
    Rtcc = 38,
    #[doc = "39: SDIO Controller"]
    Sdio = 39,
    #[doc = "40: Security Management Unit"]
    Smu = 40,
    #[doc = "41: Timer 0"]
    Timer0 = 41,
    #[doc = "42: Timer 1"]
    Timer1 = 42,
    #[doc = "43: Timer 2"]
    Timer2 = 43,
    #[doc = "44: Timer 3"]
    Timer3 = 44,
    #[doc = "45: Timer 4"]
    Timer4 = 45,
    #[doc = "46: Timer 5"]
    Timer5 = 46,
    #[doc = "47: Timer 6"]
    Timer6 = 47,
    #[doc = "48: True Random Number Generator 0"]
    Trng0 = 48,
    #[doc = "49: Universal Asynchronous Receiver/Transmitter 0"]
    Uart0 = 49,
    #[doc = "50: Universal Asynchronous Receiver/Transmitter 1"]
    Uart1 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 51,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 52,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    Usart2 = 53,
    #[doc = "54: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    Usart3 = 54,
    #[doc = "55: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    Usart4 = 55,
    #[doc = "56: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    Usart5 = 56,
    #[doc = "57: Universal Serial Bus Interface"]
    Usb = 57,
    #[doc = "58: Watchdog"]
    Wdog0 = 58,
    #[doc = "59: Watchdog"]
    Wdog1 = 59,
    #[doc = "60: Wide Timer 0"]
    Wtimer0 = 60,
    #[doc = "61: Wide Timer 0"]
    Wtimer1 = 61,
    #[doc = "62: Wide Timer 2"]
    Wtimer2 = 62,
    #[doc = "63: Wide Timer 3"]
    Wtimer3 = 63,
}
impl From<Periphid> for u8 {
    #[inline(always)]
    fn from(variant: Periphid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Periphid {
    type Ux = u8;
}
impl crate::IsEnum for Periphid {}
#[doc = "Field `PERIPHID` reader - "]
pub type PeriphidR = crate::FieldReader<Periphid>;
impl PeriphidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Periphid> {
        match self.bits {
            0 => Some(Periphid::Acmp0),
            1 => Some(Periphid::Acmp1),
            2 => Some(Periphid::Acmp2),
            3 => Some(Periphid::Acmp3),
            4 => Some(Periphid::Adc0),
            5 => Some(Periphid::Adc1),
            6 => Some(Periphid::Can0),
            7 => Some(Periphid::Can1),
            8 => Some(Periphid::Cmu),
            9 => Some(Periphid::Cryotimer),
            10 => Some(Periphid::Crypto0),
            11 => Some(Periphid::Csen),
            12 => Some(Periphid::Vdac0),
            13 => Some(Periphid::Prs),
            14 => Some(Periphid::Ebi),
            15 => Some(Periphid::Emu),
            16 => Some(Periphid::Eth),
            17 => Some(Periphid::Fpueh),
            18 => Some(Periphid::Gpcrc),
            19 => Some(Periphid::Gpio),
            20 => Some(Periphid::I2c0),
            21 => Some(Periphid::I2c1),
            22 => Some(Periphid::I2c2),
            23 => Some(Periphid::Idac0),
            24 => Some(Periphid::Msc),
            25 => Some(Periphid::Lcd),
            26 => Some(Periphid::Ldma),
            27 => Some(Periphid::Lesense),
            28 => Some(Periphid::Letimer0),
            29 => Some(Periphid::Letimer1),
            30 => Some(Periphid::Leuart0),
            31 => Some(Periphid::Leuart1),
            32 => Some(Periphid::Pcnt0),
            33 => Some(Periphid::Pcnt1),
            34 => Some(Periphid::Pcnt2),
            35 => Some(Periphid::Qspi0),
            36 => Some(Periphid::Rmu),
            37 => Some(Periphid::Rtc),
            38 => Some(Periphid::Rtcc),
            39 => Some(Periphid::Sdio),
            40 => Some(Periphid::Smu),
            41 => Some(Periphid::Timer0),
            42 => Some(Periphid::Timer1),
            43 => Some(Periphid::Timer2),
            44 => Some(Periphid::Timer3),
            45 => Some(Periphid::Timer4),
            46 => Some(Periphid::Timer5),
            47 => Some(Periphid::Timer6),
            48 => Some(Periphid::Trng0),
            49 => Some(Periphid::Uart0),
            50 => Some(Periphid::Uart1),
            51 => Some(Periphid::Usart0),
            52 => Some(Periphid::Usart1),
            53 => Some(Periphid::Usart2),
            54 => Some(Periphid::Usart3),
            55 => Some(Periphid::Usart4),
            56 => Some(Periphid::Usart5),
            57 => Some(Periphid::Usb),
            58 => Some(Periphid::Wdog0),
            59 => Some(Periphid::Wdog1),
            60 => Some(Periphid::Wtimer0),
            61 => Some(Periphid::Wtimer1),
            62 => Some(Periphid::Wtimer2),
            63 => Some(Periphid::Wtimer3),
            _ => None,
        }
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Periphid::Acmp0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Periphid::Acmp1
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Periphid::Acmp2
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Periphid::Acmp3
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Periphid::Adc0
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == Periphid::Adc1
    }
    #[doc = "CAN 0"]
    #[inline(always)]
    pub fn is_can0(&self) -> bool {
        *self == Periphid::Can0
    }
    #[doc = "CAN 1"]
    #[inline(always)]
    pub fn is_can1(&self) -> bool {
        *self == Periphid::Can1
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == Periphid::Cmu
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == Periphid::Cryotimer
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == Periphid::Crypto0
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == Periphid::Csen
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == Periphid::Vdac0
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Periphid::Prs
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn is_ebi(&self) -> bool {
        *self == Periphid::Ebi
    }
    #[doc = "Energy Management Unit"]
    #[inline(always)]
    pub fn is_emu(&self) -> bool {
        *self == Periphid::Emu
    }
    #[doc = "Ethernet Controller"]
    #[inline(always)]
    pub fn is_eth(&self) -> bool {
        *self == Periphid::Eth
    }
    #[doc = "FPU Exception Handler"]
    #[inline(always)]
    pub fn is_fpueh(&self) -> bool {
        *self == Periphid::Fpueh
    }
    #[doc = "General Purpose CRC"]
    #[inline(always)]
    pub fn is_gpcrc(&self) -> bool {
        *self == Periphid::Gpcrc
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Periphid::Gpio
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == Periphid::I2c0
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == Periphid::I2c1
    }
    #[doc = "I2C 2"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        *self == Periphid::I2c2
    }
    #[doc = "Current Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_idac0(&self) -> bool {
        *self == Periphid::Idac0
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == Periphid::Msc
    }
    #[doc = "Liquid Crystal Display Controller"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        *self == Periphid::Lcd
    }
    #[doc = "Linked Direct Memory Access Controller"]
    #[inline(always)]
    pub fn is_ldma(&self) -> bool {
        *self == Periphid::Ldma
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == Periphid::Lesense
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == Periphid::Letimer0
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == Periphid::Letimer1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == Periphid::Leuart0
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == Periphid::Leuart1
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == Periphid::Pcnt0
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == Periphid::Pcnt1
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == Periphid::Pcnt2
    }
    #[doc = "Quad-SPI"]
    #[inline(always)]
    pub fn is_qspi0(&self) -> bool {
        *self == Periphid::Qspi0
    }
    #[doc = "Reset Management Unit"]
    #[inline(always)]
    pub fn is_rmu(&self) -> bool {
        *self == Periphid::Rmu
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Periphid::Rtc
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == Periphid::Rtcc
    }
    #[doc = "SDIO Controller"]
    #[inline(always)]
    pub fn is_sdio(&self) -> bool {
        *self == Periphid::Sdio
    }
    #[doc = "Security Management Unit"]
    #[inline(always)]
    pub fn is_smu(&self) -> bool {
        *self == Periphid::Smu
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == Periphid::Timer0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == Periphid::Timer1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == Periphid::Timer2
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == Periphid::Timer3
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn is_timer4(&self) -> bool {
        *self == Periphid::Timer4
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn is_timer5(&self) -> bool {
        *self == Periphid::Timer5
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn is_timer6(&self) -> bool {
        *self == Periphid::Timer6
    }
    #[doc = "True Random Number Generator 0"]
    #[inline(always)]
    pub fn is_trng0(&self) -> bool {
        *self == Periphid::Trng0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == Periphid::Uart0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == Periphid::Uart1
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == Periphid::Usart0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == Periphid::Usart1
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == Periphid::Usart2
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == Periphid::Usart3
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == Periphid::Usart4
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn is_usart5(&self) -> bool {
        *self == Periphid::Usart5
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == Periphid::Usb
    }
    #[doc = "Watchdog"]
    #[inline(always)]
    pub fn is_wdog0(&self) -> bool {
        *self == Periphid::Wdog0
    }
    #[doc = "Watchdog"]
    #[inline(always)]
    pub fn is_wdog1(&self) -> bool {
        *self == Periphid::Wdog1
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == Periphid::Wtimer0
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == Periphid::Wtimer1
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn is_wtimer2(&self) -> bool {
        *self == Periphid::Wtimer2
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn is_wtimer3(&self) -> bool {
        *self == Periphid::Wtimer3
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn periphid(&self) -> PeriphidR {
        PeriphidR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "PPU Fault Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ppufs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpufsSpec;
impl crate::RegisterSpec for PpufsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppufs::R`](R) reader structure"]
impl crate::Readable for PpufsSpec {}
#[doc = "`reset()` method sets PPUFS to value 0"]
impl crate::Resettable for PpufsSpec {}
