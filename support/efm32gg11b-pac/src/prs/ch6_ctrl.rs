#[doc = "Register `CH6_CTRL` reader"]
pub type R = crate::R<CH6_CTRL_SPEC>;
#[doc = "Register `CH6_CTRL` writer"]
pub type W = crate::W<CH6_CTRL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Peripheral Reflex System"]
    PRSL = 1,
    #[doc = "2: Peripheral Reflex System"]
    PRS = 2,
    #[doc = "3: Peripheral Reflex System"]
    PRSH = 3,
    #[doc = "4: Analog Comparator 0"]
    ACMP0 = 4,
    #[doc = "5: Analog Comparator 1"]
    ACMP1 = 5,
    #[doc = "6: Analog to Digital Converter 0"]
    ADC0 = 6,
    #[doc = "7: Real-Time Counter"]
    RTC = 7,
    #[doc = "8: Real-Time Counter and Calendar"]
    RTCC = 8,
    #[doc = "9: General purpose Input/Output"]
    GPIOL = 9,
    #[doc = "10: General purpose Input/Output"]
    GPIOH = 10,
    #[doc = "11: Low Energy Timer 0"]
    LETIMER0 = 11,
    #[doc = "12: Low Energy Timer 1"]
    LETIMER1 = 12,
    #[doc = "13: Pulse Counter 0"]
    PCNT0 = 13,
    #[doc = "14: Pulse Counter 1"]
    PCNT1 = 14,
    #[doc = "15: Pulse Counter 2"]
    PCNT2 = 15,
    #[doc = "16: CRYOTIMER"]
    CRYOTIMER = 16,
    #[doc = "17: Clock Management Unit"]
    CMU = 17,
    #[doc = "23: Digital to Analog Converter 0"]
    VDAC0 = 23,
    #[doc = "24: Low Energy Sensor Interface"]
    LESENSEL = 24,
    #[doc = "25: Low Energy Sensor Interface"]
    LESENSEH = 25,
    #[doc = "26: Low Energy Sensor Interface"]
    LESENSED = 26,
    #[doc = "27: Low Energy Sensor Interface"]
    LESENSE = 27,
    #[doc = "28: Analog Comparator 1"]
    ACMP2 = 28,
    #[doc = "29: Analog Comparator 3"]
    ACMP3 = 29,
    #[doc = "30: Analog to Digital Converter 0"]
    ADC1 = 30,
    #[doc = "48: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 48,
    #[doc = "49: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 49,
    #[doc = "50: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 51,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4 = 52,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5 = 53,
    #[doc = "54: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 54,
    #[doc = "55: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 55,
    #[doc = "60: Timer 0"]
    TIMER0 = 60,
    #[doc = "61: Timer 1"]
    TIMER1 = 61,
    #[doc = "62: Timer 2"]
    TIMER2 = 62,
    #[doc = "64: Universal Serial Bus Interface"]
    USB = 64,
    #[doc = "67: `1000011`"]
    CM4 = 67,
    #[doc = "80: Timer 3"]
    TIMER3 = 80,
    #[doc = "82: Wide Timer 0"]
    WTIMER0 = 82,
    #[doc = "83: Wide Timer 0"]
    WTIMER1 = 83,
    #[doc = "84: Wide Timer 2"]
    WTIMER2 = 84,
    #[doc = "85: Wide Timer 3"]
    WTIMER3 = 85,
    #[doc = "98: Timer 4"]
    TIMER4 = 98,
    #[doc = "99: Timer 5"]
    TIMER5 = 99,
    #[doc = "100: Timer 6"]
    TIMER6 = 100,
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
            1 => Some(SOURCESEL_A::PRSL),
            2 => Some(SOURCESEL_A::PRS),
            3 => Some(SOURCESEL_A::PRSH),
            4 => Some(SOURCESEL_A::ACMP0),
            5 => Some(SOURCESEL_A::ACMP1),
            6 => Some(SOURCESEL_A::ADC0),
            7 => Some(SOURCESEL_A::RTC),
            8 => Some(SOURCESEL_A::RTCC),
            9 => Some(SOURCESEL_A::GPIOL),
            10 => Some(SOURCESEL_A::GPIOH),
            11 => Some(SOURCESEL_A::LETIMER0),
            12 => Some(SOURCESEL_A::LETIMER1),
            13 => Some(SOURCESEL_A::PCNT0),
            14 => Some(SOURCESEL_A::PCNT1),
            15 => Some(SOURCESEL_A::PCNT2),
            16 => Some(SOURCESEL_A::CRYOTIMER),
            17 => Some(SOURCESEL_A::CMU),
            23 => Some(SOURCESEL_A::VDAC0),
            24 => Some(SOURCESEL_A::LESENSEL),
            25 => Some(SOURCESEL_A::LESENSEH),
            26 => Some(SOURCESEL_A::LESENSED),
            27 => Some(SOURCESEL_A::LESENSE),
            28 => Some(SOURCESEL_A::ACMP2),
            29 => Some(SOURCESEL_A::ACMP3),
            30 => Some(SOURCESEL_A::ADC1),
            48 => Some(SOURCESEL_A::USART0),
            49 => Some(SOURCESEL_A::USART1),
            50 => Some(SOURCESEL_A::USART2),
            51 => Some(SOURCESEL_A::USART3),
            52 => Some(SOURCESEL_A::USART4),
            53 => Some(SOURCESEL_A::USART5),
            54 => Some(SOURCESEL_A::UART0),
            55 => Some(SOURCESEL_A::UART1),
            60 => Some(SOURCESEL_A::TIMER0),
            61 => Some(SOURCESEL_A::TIMER1),
            62 => Some(SOURCESEL_A::TIMER2),
            64 => Some(SOURCESEL_A::USB),
            67 => Some(SOURCESEL_A::CM4),
            80 => Some(SOURCESEL_A::TIMER3),
            82 => Some(SOURCESEL_A::WTIMER0),
            83 => Some(SOURCESEL_A::WTIMER1),
            84 => Some(SOURCESEL_A::WTIMER2),
            85 => Some(SOURCESEL_A::WTIMER3),
            98 => Some(SOURCESEL_A::TIMER4),
            99 => Some(SOURCESEL_A::TIMER5),
            100 => Some(SOURCESEL_A::TIMER6),
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
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESEL_A::PRSL
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESEL_A::PRS
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESEL_A::PRSH
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL_A::ACMP1
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESEL_A::RTC
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESEL_A::RTCC
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL_A::LETIMER0
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == SOURCESEL_A::LETIMER1
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL_A::PCNT0
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == SOURCESEL_A::PCNT1
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == SOURCESEL_A::PCNT2
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESEL_A::CRYOTIMER
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESEL_A::CMU
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESEL_A::VDAC0
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesensel(&self) -> bool {
        *self == SOURCESEL_A::LESENSEL
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesenseh(&self) -> bool {
        *self == SOURCESEL_A::LESENSEH
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesensed(&self) -> bool {
        *self == SOURCESEL_A::LESENSED
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESEL_A::LESENSE
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == SOURCESEL_A::ACMP2
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == SOURCESEL_A::ACMP3
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == SOURCESEL_A::ADC1
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESEL_A::USB
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESEL_A::CM4
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESEL_A::TIMER3
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
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, SOURCESEL_A>;
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
    pub fn prsl(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRSL)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRS)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PRSH)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP1)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::RTC)
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::RTCC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LETIMER0)
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn letimer1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LETIMER1)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PCNT0)
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn pcnt1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PCNT1)
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn pcnt2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::PCNT2)
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CRYOTIMER)
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn cmu(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CMU)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::VDAC0)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensel(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LESENSEL)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesenseh(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LESENSEH)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensed(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LESENSED)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::LESENSE)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP2)
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ACMP3)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::ADC1)
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::USB)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn cm4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::CM4)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCESEL_A::TIMER3)
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
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EDSEL_R = crate::FieldReader<EDSEL_A>;
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF = 0,
    #[doc = "1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE = 1,
    #[doc = "2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE = 2,
    #[doc = "3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES = 3,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDSEL_A {
    type Ux = u8;
}
impl EDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EDSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EDSEL_A>;
impl<'a, REG, const O: u8> EDSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(EDSEL_A::BOTHEDGES)
    }
}
#[doc = "Field `STRETCH` reader - Stretch Channel Output"]
pub type STRETCH_R = crate::BitReader;
#[doc = "Field `STRETCH` writer - Stretch Channel Output"]
pub type STRETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INV` reader - Invert Channel"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Invert Channel"]
pub type INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ORPREV` reader - Or Previous"]
pub type ORPREV_R = crate::BitReader;
#[doc = "Field `ORPREV` writer - Or Previous"]
pub type ORPREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ANDNEXT` reader - And Next"]
pub type ANDNEXT_R = crate::BitReader;
#[doc = "Field `ANDNEXT` writer - And Next"]
pub type ANDNEXT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNC` reader - Asynchronous Reflex"]
pub type ASYNC_R = crate::BitReader;
#[doc = "Field `ASYNC` writer - Asynchronous Reflex"]
pub type ASYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&self) -> ORPREV_R {
        ORPREV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&self) -> ANDNEXT_R {
        ANDNEXT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC_R {
        ASYNC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<CH6_CTRL_SPEC, 0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<CH6_CTRL_SPEC, 8> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    #[must_use]
    pub fn edsel(&mut self) -> EDSEL_W<CH6_CTRL_SPEC, 20> {
        EDSEL_W::new(self)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<CH6_CTRL_SPEC, 25> {
        STRETCH_W::new(self)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<CH6_CTRL_SPEC, 26> {
        INV_W::new(self)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    #[must_use]
    pub fn orprev(&mut self) -> ORPREV_W<CH6_CTRL_SPEC, 27> {
        ORPREV_W::new(self)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    #[must_use]
    pub fn andnext(&mut self) -> ANDNEXT_W<CH6_CTRL_SPEC, 28> {
        ANDNEXT_W::new(self)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    #[must_use]
    pub fn async_(&mut self) -> ASYNC_W<CH6_CTRL_SPEC, 30> {
        ASYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6_CTRL_SPEC;
impl crate::RegisterSpec for CH6_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_ctrl::R`](R) reader structure"]
impl crate::Readable for CH6_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch6_ctrl::W`](W) writer structure"]
impl crate::Writable for CH6_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6_CTRL to value 0"]
impl crate::Resettable for CH6_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
