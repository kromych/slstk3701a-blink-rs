#[doc = "Register `CH2_REQSEL` reader"]
pub type R = crate::R<CH2_REQSEL_SPEC>;
#[doc = "Register `CH2_REQSEL` writer"]
pub type W = crate::W<CH2_REQSEL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Peripheral Reflex System"]
    PRS = 1,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "9: Analog to Digital Converter 0"]
    ADC1 = 9,
    #[doc = "10: Digital to Analog Converter 0"]
    VDAC0 = 10,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 13,
    #[doc = "14: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 14,
    #[doc = "15: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 15,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5 = 17,
    #[doc = "18: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 18,
    #[doc = "19: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 19,
    #[doc = "20: Low Energy UART 0"]
    LEUART0 = 20,
    #[doc = "21: Low Energy UART 1"]
    LEUART1 = 21,
    #[doc = "22: I2C 0"]
    I2C0 = 22,
    #[doc = "23: I2C 1"]
    I2C1 = 23,
    #[doc = "24: I2C 2"]
    I2C2 = 24,
    #[doc = "25: Timer 0"]
    TIMER0 = 25,
    #[doc = "26: Timer 1"]
    TIMER1 = 26,
    #[doc = "27: Timer 2"]
    TIMER2 = 27,
    #[doc = "28: Timer 3"]
    TIMER3 = 28,
    #[doc = "29: Timer 4"]
    TIMER4 = 29,
    #[doc = "30: Timer 5"]
    TIMER5 = 30,
    #[doc = "31: Timer 6"]
    TIMER6 = 31,
    #[doc = "32: Wide Timer 0"]
    WTIMER0 = 32,
    #[doc = "33: Wide Timer 0"]
    WTIMER1 = 33,
    #[doc = "34: Wide Timer 2"]
    WTIMER2 = 34,
    #[doc = "35: Wide Timer 3"]
    WTIMER3 = 35,
    #[doc = "48: Memory System Controller"]
    MSC = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    CRYPTO0 = 49,
    #[doc = "50: External Bus Interface"]
    EBI = 50,
    #[doc = "61: Capacitive touch sense module"]
    CSEN = 61,
    #[doc = "62: Low Energy Sensor Interface"]
    LESENSE = 62,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOURCESEL_A {
    type Ux = u8;
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::PRS),
            8 => Some(SOURCESEL_A::ADC0),
            9 => Some(SOURCESEL_A::ADC1),
            10 => Some(SOURCESEL_A::VDAC0),
            12 => Some(SOURCESEL_A::USART0),
            13 => Some(SOURCESEL_A::USART1),
            14 => Some(SOURCESEL_A::USART2),
            15 => Some(SOURCESEL_A::USART3),
            16 => Some(SOURCESEL_A::USART4),
            17 => Some(SOURCESEL_A::USART5),
            18 => Some(SOURCESEL_A::UART0),
            19 => Some(SOURCESEL_A::UART1),
            20 => Some(SOURCESEL_A::LEUART0),
            21 => Some(SOURCESEL_A::LEUART1),
            22 => Some(SOURCESEL_A::I2C0),
            23 => Some(SOURCESEL_A::I2C1),
            24 => Some(SOURCESEL_A::I2C2),
            25 => Some(SOURCESEL_A::TIMER0),
            26 => Some(SOURCESEL_A::TIMER1),
            27 => Some(SOURCESEL_A::TIMER2),
            28 => Some(SOURCESEL_A::TIMER3),
            29 => Some(SOURCESEL_A::TIMER4),
            30 => Some(SOURCESEL_A::TIMER5),
            31 => Some(SOURCESEL_A::TIMER6),
            32 => Some(SOURCESEL_A::WTIMER0),
            33 => Some(SOURCESEL_A::WTIMER1),
            34 => Some(SOURCESEL_A::WTIMER2),
            35 => Some(SOURCESEL_A::WTIMER3),
            48 => Some(SOURCESEL_A::MSC),
            49 => Some(SOURCESEL_A::CRYPTO0),
            50 => Some(SOURCESEL_A::EBI),
            61 => Some(SOURCESEL_A::CSEN),
            62 => Some(SOURCESEL_A::LESENSE),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESEL_A::PRS
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == SOURCESEL_A::ADC1
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESEL_A::VDAC0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESEL_A::USART2
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == SOURCESEL_A::USART3
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == SOURCESEL_A::USART4
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn is_usart5(&self) -> bool {
        *self == SOURCESEL_A::USART5
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == SOURCESEL_A::UART0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == SOURCESEL_A::UART1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESEL_A::LEUART0
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == SOURCESEL_A::LEUART1
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESEL_A::I2C0
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == SOURCESEL_A::I2C1
    }
    #[doc = "I2C 2"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        *self == SOURCESEL_A::I2C2
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESEL_A::TIMER2
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESEL_A::TIMER3
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn is_timer4(&self) -> bool {
        *self == SOURCESEL_A::TIMER4
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn is_timer5(&self) -> bool {
        *self == SOURCESEL_A::TIMER5
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn is_timer6(&self) -> bool {
        *self == SOURCESEL_A::TIMER6
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == SOURCESEL_A::WTIMER0
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == SOURCESEL_A::WTIMER1
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn is_wtimer2(&self) -> bool {
        *self == SOURCESEL_A::WTIMER2
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn is_wtimer3(&self) -> bool {
        *self == SOURCESEL_A::WTIMER3
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESEL_A::MSC
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == SOURCESEL_A::CRYPTO0
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn is_ebi(&self) -> bool {
        *self == SOURCESEL_A::EBI
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == SOURCESEL_A::CSEN
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESEL_A::LESENSE
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, SOURCESEL_A>;
impl<'a, REG, const O: u8> SOURCESEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRS)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ADC1)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::VDAC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART2)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn usart3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART3)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn usart4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART4)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn usart5(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USART5)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::UART0)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::UART1)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LEUART0)
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn leuart1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LEUART1)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::I2C0)
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::I2C1)
    }
    #[doc = "I2C 2"]
    #[inline(always)]
    pub fn i2c2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::I2C2)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER3)
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn timer4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER4)
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn timer5(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER5)
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn timer6(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER6)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::WTIMER0)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::WTIMER1)
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn wtimer2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::WTIMER2)
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn wtimer3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::WTIMER3)
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::MSC)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn crypto0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CRYPTO0)
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn ebi(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::EBI)
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn csen(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CSEN)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LESENSE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<CH2_REQSEL_SPEC, 0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<CH2_REQSEL_SPEC, 16> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Peripheral Request Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_reqsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_reqsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_REQSEL_SPEC;
impl crate::RegisterSpec for CH2_REQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_reqsel::R`](R) reader structure"]
impl crate::Readable for CH2_REQSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_reqsel::W`](W) writer structure"]
impl crate::Writable for CH2_REQSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2_REQSEL to value 0"]
impl crate::Resettable for CH2_REQSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
