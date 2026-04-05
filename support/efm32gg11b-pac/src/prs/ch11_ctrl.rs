#[doc = "Register `CH11_CTRL` reader"]
pub type R = crate::R<Ch11CtrlSpec>;
#[doc = "Register `CH11_CTRL` writer"]
pub type W = crate::W<Ch11CtrlSpec>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sourcesel {
    #[doc = "0: No source selected"]
    None = 0,
    #[doc = "1: Peripheral Reflex System"]
    Prsl = 1,
    #[doc = "2: Peripheral Reflex System"]
    Prs = 2,
    #[doc = "3: Peripheral Reflex System"]
    Prsh = 3,
    #[doc = "4: Analog Comparator 0"]
    Acmp0 = 4,
    #[doc = "5: Analog Comparator 1"]
    Acmp1 = 5,
    #[doc = "6: Analog to Digital Converter 0"]
    Adc0 = 6,
    #[doc = "7: Real-Time Counter"]
    Rtc = 7,
    #[doc = "8: Real-Time Counter and Calendar"]
    Rtcc = 8,
    #[doc = "9: General purpose Input/Output"]
    Gpiol = 9,
    #[doc = "10: General purpose Input/Output"]
    Gpioh = 10,
    #[doc = "11: Low Energy Timer 0"]
    Letimer0 = 11,
    #[doc = "12: Low Energy Timer 1"]
    Letimer1 = 12,
    #[doc = "13: Pulse Counter 0"]
    Pcnt0 = 13,
    #[doc = "14: Pulse Counter 1"]
    Pcnt1 = 14,
    #[doc = "15: Pulse Counter 2"]
    Pcnt2 = 15,
    #[doc = "16: CRYOTIMER"]
    Cryotimer = 16,
    #[doc = "17: Clock Management Unit"]
    Cmu = 17,
    #[doc = "23: Digital to Analog Converter 0"]
    Vdac0 = 23,
    #[doc = "24: Low Energy Sensor Interface"]
    Lesensel = 24,
    #[doc = "25: Low Energy Sensor Interface"]
    Lesenseh = 25,
    #[doc = "26: Low Energy Sensor Interface"]
    Lesensed = 26,
    #[doc = "27: Low Energy Sensor Interface"]
    Lesense = 27,
    #[doc = "28: Analog Comparator 1"]
    Acmp2 = 28,
    #[doc = "29: Analog Comparator 3"]
    Acmp3 = 29,
    #[doc = "30: Analog to Digital Converter 0"]
    Adc1 = 30,
    #[doc = "48: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    Usart0 = 48,
    #[doc = "49: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    Usart1 = 49,
    #[doc = "50: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    Usart2 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    Usart3 = 51,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    Usart4 = 52,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    Usart5 = 53,
    #[doc = "54: Universal Asynchronous Receiver/Transmitter 0"]
    Uart0 = 54,
    #[doc = "55: Universal Asynchronous Receiver/Transmitter 1"]
    Uart1 = 55,
    #[doc = "60: Timer 0"]
    Timer0 = 60,
    #[doc = "61: Timer 1"]
    Timer1 = 61,
    #[doc = "62: Timer 2"]
    Timer2 = 62,
    #[doc = "64: Universal Serial Bus Interface"]
    Usb = 64,
    #[doc = "67: `1000011`"]
    Cm4 = 67,
    #[doc = "80: Timer 3"]
    Timer3 = 80,
    #[doc = "82: Wide Timer 0"]
    Wtimer0 = 82,
    #[doc = "83: Wide Timer 0"]
    Wtimer1 = 83,
    #[doc = "84: Wide Timer 2"]
    Wtimer2 = 84,
    #[doc = "85: Wide Timer 3"]
    Wtimer3 = 85,
    #[doc = "98: Timer 4"]
    Timer4 = 98,
    #[doc = "99: Timer 5"]
    Timer5 = 99,
    #[doc = "100: Timer 6"]
    Timer6 = 100,
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
            1 => Some(Sourcesel::Prsl),
            2 => Some(Sourcesel::Prs),
            3 => Some(Sourcesel::Prsh),
            4 => Some(Sourcesel::Acmp0),
            5 => Some(Sourcesel::Acmp1),
            6 => Some(Sourcesel::Adc0),
            7 => Some(Sourcesel::Rtc),
            8 => Some(Sourcesel::Rtcc),
            9 => Some(Sourcesel::Gpiol),
            10 => Some(Sourcesel::Gpioh),
            11 => Some(Sourcesel::Letimer0),
            12 => Some(Sourcesel::Letimer1),
            13 => Some(Sourcesel::Pcnt0),
            14 => Some(Sourcesel::Pcnt1),
            15 => Some(Sourcesel::Pcnt2),
            16 => Some(Sourcesel::Cryotimer),
            17 => Some(Sourcesel::Cmu),
            23 => Some(Sourcesel::Vdac0),
            24 => Some(Sourcesel::Lesensel),
            25 => Some(Sourcesel::Lesenseh),
            26 => Some(Sourcesel::Lesensed),
            27 => Some(Sourcesel::Lesense),
            28 => Some(Sourcesel::Acmp2),
            29 => Some(Sourcesel::Acmp3),
            30 => Some(Sourcesel::Adc1),
            48 => Some(Sourcesel::Usart0),
            49 => Some(Sourcesel::Usart1),
            50 => Some(Sourcesel::Usart2),
            51 => Some(Sourcesel::Usart3),
            52 => Some(Sourcesel::Usart4),
            53 => Some(Sourcesel::Usart5),
            54 => Some(Sourcesel::Uart0),
            55 => Some(Sourcesel::Uart1),
            60 => Some(Sourcesel::Timer0),
            61 => Some(Sourcesel::Timer1),
            62 => Some(Sourcesel::Timer2),
            64 => Some(Sourcesel::Usb),
            67 => Some(Sourcesel::Cm4),
            80 => Some(Sourcesel::Timer3),
            82 => Some(Sourcesel::Wtimer0),
            83 => Some(Sourcesel::Wtimer1),
            84 => Some(Sourcesel::Wtimer2),
            85 => Some(Sourcesel::Wtimer3),
            98 => Some(Sourcesel::Timer4),
            99 => Some(Sourcesel::Timer5),
            100 => Some(Sourcesel::Timer6),
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
    pub fn is_prsl(&self) -> bool {
        *self == Sourcesel::Prsl
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Sourcesel::Prs
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == Sourcesel::Prsh
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Sourcesel::Acmp0
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Sourcesel::Acmp1
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Sourcesel::Adc0
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Sourcesel::Rtc
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == Sourcesel::Rtcc
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == Sourcesel::Gpiol
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == Sourcesel::Gpioh
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == Sourcesel::Letimer0
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == Sourcesel::Letimer1
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == Sourcesel::Pcnt0
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == Sourcesel::Pcnt1
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == Sourcesel::Pcnt2
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == Sourcesel::Cryotimer
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == Sourcesel::Cmu
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == Sourcesel::Vdac0
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesensel(&self) -> bool {
        *self == Sourcesel::Lesensel
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesenseh(&self) -> bool {
        *self == Sourcesel::Lesenseh
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesensed(&self) -> bool {
        *self == Sourcesel::Lesensed
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == Sourcesel::Lesense
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Sourcesel::Acmp2
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Sourcesel::Acmp3
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == Sourcesel::Adc1
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == Sourcesel::Usb
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == Sourcesel::Cm4
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == Sourcesel::Timer3
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
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 7, Sourcesel>;
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
    pub fn prsl(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Prsl)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Prs)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsh(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Prsh)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Acmp0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Acmp1)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Adc0)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Rtc)
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Rtcc)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Gpiol)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Gpioh)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Letimer0)
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn letimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Letimer1)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Pcnt0)
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn pcnt1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Pcnt1)
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn pcnt2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Pcnt2)
    }
    #[doc = "CRYOTIMER"]
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Cryotimer)
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn cmu(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Cmu)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Vdac0)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensel(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Lesensel)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesenseh(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Lesenseh)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensed(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Lesensed)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Lesense)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Acmp2)
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Acmp3)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Adc1)
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
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Usb)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn cm4(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Cm4)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut crate::W<REG> {
        self.variant(Sourcesel::Timer3)
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
}
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edsel {
    #[doc = "0: Signal is left as it is"]
    Off = 0,
    #[doc = "1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    Posedge = 1,
    #[doc = "2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    Negedge = 2,
    #[doc = "3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    Bothedges = 3,
}
impl From<Edsel> for u8 {
    #[inline(always)]
    fn from(variant: Edsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edsel {
    type Ux = u8;
}
impl crate::IsEnum for Edsel {}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub type EdselR = crate::FieldReader<Edsel>;
impl EdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edsel {
        match self.bits {
            0 => Edsel::Off,
            1 => Edsel::Posedge,
            2 => Edsel::Negedge,
            3 => Edsel::Bothedges,
            _ => unreachable!(),
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Edsel::Off
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == Edsel::Posedge
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == Edsel::Negedge
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == Edsel::Bothedges
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub type EdselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edsel, crate::Safe>;
impl<'a, REG> EdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Off)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Posedge)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Negedge)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(Edsel::Bothedges)
    }
}
#[doc = "Field `STRETCH` reader - Stretch Channel Output"]
pub type StretchR = crate::BitReader;
#[doc = "Field `STRETCH` writer - Stretch Channel Output"]
pub type StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Invert Channel"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Invert Channel"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORPREV` reader - Or Previous"]
pub type OrprevR = crate::BitReader;
#[doc = "Field `ORPREV` writer - Or Previous"]
pub type OrprevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANDNEXT` reader - And Next"]
pub type AndnextR = crate::BitReader;
#[doc = "Field `ANDNEXT` writer - And Next"]
pub type AndnextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC` reader - Asynchronous Reflex"]
pub type AsyncR = crate::BitReader;
#[doc = "Field `ASYNC` writer - Asynchronous Reflex"]
pub type AsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EdselR {
        EdselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&self) -> StretchR {
        StretchR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&self) -> OrprevR {
        OrprevR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&self) -> AndnextR {
        AndnextR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn async_(&self) -> AsyncR {
        AsyncR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<'_, Ch11CtrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, Ch11CtrlSpec> {
        SourceselW::new(self, 8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&mut self) -> EdselW<'_, Ch11CtrlSpec> {
        EdselW::new(self, 20)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&mut self) -> StretchW<'_, Ch11CtrlSpec> {
        StretchW::new(self, 25)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, Ch11CtrlSpec> {
        InvW::new(self, 26)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&mut self) -> OrprevW<'_, Ch11CtrlSpec> {
        OrprevW::new(self, 27)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&mut self) -> AndnextW<'_, Ch11CtrlSpec> {
        AndnextW::new(self, 28)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn async_(&mut self) -> AsyncW<'_, Ch11CtrlSpec> {
        AsyncW::new(self, 30)
    }
}
#[doc = "Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch11CtrlSpec;
impl crate::RegisterSpec for Ch11CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch11_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch11CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch11_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch11CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH11_CTRL to value 0"]
impl crate::Resettable for Ch11CtrlSpec {}
