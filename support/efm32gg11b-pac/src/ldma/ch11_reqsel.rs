#[doc = "Register `CH11_REQSEL` reader"]
pub type R = crate::R<Ch11ReqselSpec>;
#[doc = "Register `CH11_REQSEL` writer"]
pub type W = crate::W<Ch11ReqselSpec>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sourcesel {
    #[doc = "0: No source selected"]
    None = 0,
    #[doc = "1: Peripheral Reflex System"]
    Prs = 1,
    #[doc = "8: Analog to Digital Converter 0"]
    Adc0 = 8,
    #[doc = "9: Analog to Digital Converter 0"]
    Adc1 = 9,
    #[doc = "10: Digital to Analog Converter 0"]
    Vdac0 = 10,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 13,
    #[doc = "14: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    Usart2 = 14,
    #[doc = "15: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    Usart3 = 15,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    Usart4 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    Usart5 = 17,
    #[doc = "18: Universal Asynchronous Receiver/Transmitter 0"]
    Uart0 = 18,
    #[doc = "19: Universal Asynchronous Receiver/Transmitter 1"]
    Uart1 = 19,
    #[doc = "20: Low Energy UART 0"]
    Leuart0 = 20,
    #[doc = "21: Low Energy UART 1"]
    Leuart1 = 21,
    #[doc = "22: I2C 0"]
    I2c0 = 22,
    #[doc = "23: I2C 1"]
    I2c1 = 23,
    #[doc = "24: I2C 2"]
    I2c2 = 24,
    #[doc = "25: Timer 0"]
    Timer0 = 25,
    #[doc = "26: Timer 1"]
    Timer1 = 26,
    #[doc = "27: Timer 2"]
    Timer2 = 27,
    #[doc = "28: Timer 3"]
    Timer3 = 28,
    #[doc = "29: Timer 4"]
    Timer4 = 29,
    #[doc = "30: Timer 5"]
    Timer5 = 30,
    #[doc = "31: Timer 6"]
    Timer6 = 31,
    #[doc = "32: Wide Timer 0"]
    Wtimer0 = 32,
    #[doc = "33: Wide Timer 0"]
    Wtimer1 = 33,
    #[doc = "34: Wide Timer 2"]
    Wtimer2 = 34,
    #[doc = "35: Wide Timer 3"]
    Wtimer3 = 35,
    #[doc = "48: Memory System Controller"]
    Msc = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    Crypto0 = 49,
    #[doc = "50: External Bus Interface"]
    Ebi = 50,
    #[doc = "61: Capacitive touch sense module"]
    Csen = 61,
    #[doc = "62: Low Energy Sensor Interface"]
    Lesense = 62,
}
impl From<Sourcesel> for u8 {
    #[inline(always)]
    fn from(variant: Sourcesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sourcesel {
    type Ux = u8;
}
impl crate::IsEnum for Sourcesel {}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader<Sourcesel>;
impl SourceselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sourcesel> {
        match self.bits {
            0 => Some(Sourcesel::None),
            1 => Some(Sourcesel::Prs),
            8 => Some(Sourcesel::Adc0),
            9 => Some(Sourcesel::Adc1),
            10 => Some(Sourcesel::Vdac0),
            12 => Some(Sourcesel::Usart0),
            13 => Some(Sourcesel::Usart1),
            14 => Some(Sourcesel::Usart2),
            15 => Some(Sourcesel::Usart3),
            16 => Some(Sourcesel::Usart4),
            17 => Some(Sourcesel::Usart5),
            18 => Some(Sourcesel::Uart0),
            19 => Some(Sourcesel::Uart1),
            20 => Some(Sourcesel::Leuart0),
            21 => Some(Sourcesel::Leuart1),
            22 => Some(Sourcesel::I2c0),
            23 => Some(Sourcesel::I2c1),
            24 => Some(Sourcesel::I2c2),
            25 => Some(Sourcesel::Timer0),
            26 => Some(Sourcesel::Timer1),
            27 => Some(Sourcesel::Timer2),
            28 => Some(Sourcesel::Timer3),
            29 => Some(Sourcesel::Timer4),
            30 => Some(Sourcesel::Timer5),
            31 => Some(Sourcesel::Timer6),
            32 => Some(Sourcesel::Wtimer0),
            33 => Some(Sourcesel::Wtimer1),
            34 => Some(Sourcesel::Wtimer2),
            35 => Some(Sourcesel::Wtimer3),
            48 => Some(Sourcesel::Msc),
            49 => Some(Sourcesel::Crypto0),
            50 => Some(Sourcesel::Ebi),
            61 => Some(Sourcesel::Csen),
            62 => Some(Sourcesel::Lesense),
            _ => None,
        }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sourcesel::None
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Sourcesel::Prs
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Sourcesel::Adc0
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == Sourcesel::Adc1
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == Sourcesel::Vdac0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == Sourcesel::Usart0
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == Sourcesel::Usart1
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == Sourcesel::Usart2
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == Sourcesel::Usart3
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == Sourcesel::Usart4
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn is_usart5(&self) -> bool {
        *self == Sourcesel::Usart5
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == Sourcesel::Uart0
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == Sourcesel::Uart1
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == Sourcesel::Leuart0
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == Sourcesel::Leuart1
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == Sourcesel::I2c0
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == Sourcesel::I2c1
    }
    #[doc = "I2C 2"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        *self == Sourcesel::I2c2
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == Sourcesel::Timer0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == Sourcesel::Timer1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == Sourcesel::Timer2
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == Sourcesel::Timer3
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn is_timer4(&self) -> bool {
        *self == Sourcesel::Timer4
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn is_timer5(&self) -> bool {
        *self == Sourcesel::Timer5
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn is_timer6(&self) -> bool {
        *self == Sourcesel::Timer6
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == Sourcesel::Wtimer0
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == Sourcesel::Wtimer1
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn is_wtimer2(&self) -> bool {
        *self == Sourcesel::Wtimer2
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn is_wtimer3(&self) -> bool {
        *self == Sourcesel::Wtimer3
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == Sourcesel::Msc
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == Sourcesel::Crypto0
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn is_ebi(&self) -> bool {
        *self == Sourcesel::Ebi
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == Sourcesel::Csen
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == Sourcesel::Lesense
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Sourcesel>;
impl<'a, REG> SourceselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::None)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Prs)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Adc0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Adc1)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Vdac0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart2)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn usart3(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart3)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn usart4(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart4)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn usart5(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usart5)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Uart0)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Uart1)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Leuart0)
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn leuart1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Leuart1)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::I2c0)
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::I2c1)
    }
    #[doc = "I2C 2"]
    #[inline(always)]
    pub fn i2c2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::I2c2)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer2)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer3)
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn timer4(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer4)
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn timer5(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer5)
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn timer6(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer6)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Wtimer0)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Wtimer1)
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn wtimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Wtimer2)
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn wtimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Wtimer3)
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Msc)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn crypto0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Crypto0)
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn ebi(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Ebi)
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn csen(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Csen)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Lesense)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<'_, Ch11ReqselSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, Ch11ReqselSpec> {
        SourceselW::new(self, 16)
    }
}
#[doc = "Channel Peripheral Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_reqsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_reqsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch11ReqselSpec;
impl crate::RegisterSpec for Ch11ReqselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch11_reqsel::R`](R) reader structure"]
impl crate::Readable for Ch11ReqselSpec {}
#[doc = "`write(|w| ..)` method takes [`ch11_reqsel::W`](W) writer structure"]
impl crate::Writable for Ch11ReqselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH11_REQSEL to value 0"]
impl crate::Resettable for Ch11ReqselSpec {}
