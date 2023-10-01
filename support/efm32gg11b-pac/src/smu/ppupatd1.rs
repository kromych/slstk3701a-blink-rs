#[doc = "Register `PPUPATD1` reader"]
pub type R = crate::R<PPUPATD1_SPEC>;
#[doc = "Register `PPUPATD1` writer"]
pub type W = crate::W<PPUPATD1_SPEC>;
#[doc = "Field `PCNT0` reader - Pulse Counter 0 access control bit"]
pub type PCNT0_R = crate::BitReader;
#[doc = "Field `PCNT0` writer - Pulse Counter 0 access control bit"]
pub type PCNT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCNT1` reader - Pulse Counter 1 access control bit"]
pub type PCNT1_R = crate::BitReader;
#[doc = "Field `PCNT1` writer - Pulse Counter 1 access control bit"]
pub type PCNT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCNT2` reader - Pulse Counter 2 access control bit"]
pub type PCNT2_R = crate::BitReader;
#[doc = "Field `PCNT2` writer - Pulse Counter 2 access control bit"]
pub type PCNT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI0` reader - Quad-SPI access control bit"]
pub type QSPI0_R = crate::BitReader;
#[doc = "Field `QSPI0` writer - Quad-SPI access control bit"]
pub type QSPI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RMU` reader - Reset Management Unit access control bit"]
pub type RMU_R = crate::BitReader;
#[doc = "Field `RMU` writer - Reset Management Unit access control bit"]
pub type RMU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC` reader - Real-Time Counter access control bit"]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - Real-Time Counter access control bit"]
pub type RTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar access control bit"]
pub type RTCC_R = crate::BitReader;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar access control bit"]
pub type RTCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO` reader - SDIO Controller access control bit"]
pub type SDIO_R = crate::BitReader;
#[doc = "Field `SDIO` writer - SDIO Controller access control bit"]
pub type SDIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMU` reader - Security Management Unit access control bit"]
pub type SMU_R = crate::BitReader;
#[doc = "Field `SMU` writer - Security Management Unit access control bit"]
pub type SMU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER0` reader - Timer 0 access control bit"]
pub type TIMER0_R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 access control bit"]
pub type TIMER0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER1` reader - Timer 1 access control bit"]
pub type TIMER1_R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 access control bit"]
pub type TIMER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER2` reader - Timer 2 access control bit"]
pub type TIMER2_R = crate::BitReader;
#[doc = "Field `TIMER2` writer - Timer 2 access control bit"]
pub type TIMER2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER3` reader - Timer 3 access control bit"]
pub type TIMER3_R = crate::BitReader;
#[doc = "Field `TIMER3` writer - Timer 3 access control bit"]
pub type TIMER3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER4` reader - Timer 4 access control bit"]
pub type TIMER4_R = crate::BitReader;
#[doc = "Field `TIMER4` writer - Timer 4 access control bit"]
pub type TIMER4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER5` reader - Timer 5 access control bit"]
pub type TIMER5_R = crate::BitReader;
#[doc = "Field `TIMER5` writer - Timer 5 access control bit"]
pub type TIMER5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER6` reader - Timer 6 access control bit"]
pub type TIMER6_R = crate::BitReader;
#[doc = "Field `TIMER6` writer - Timer 6 access control bit"]
pub type TIMER6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNG0` reader - True Random Number Generator 0 access control bit"]
pub type TRNG0_R = crate::BitReader;
#[doc = "Field `TRNG0` writer - True Random Number Generator 0 access control bit"]
pub type TRNG0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART0` reader - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
pub type UART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART1` reader - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
pub type UART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type USART0_R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type USART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type USART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type USART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART4` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
pub type USART4_R = crate::BitReader;
#[doc = "Field `USART4` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
pub type USART4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
pub type USART5_R = crate::BitReader;
#[doc = "Field `USART5` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
pub type USART5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface access control bit"]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - Universal Serial Bus Interface access control bit"]
pub type USB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDOG0` reader - Watchdog access control bit"]
pub type WDOG0_R = crate::BitReader;
#[doc = "Field `WDOG0` writer - Watchdog access control bit"]
pub type WDOG0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDOG1` reader - Watchdog access control bit"]
pub type WDOG1_R = crate::BitReader;
#[doc = "Field `WDOG1` writer - Watchdog access control bit"]
pub type WDOG1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 access control bit"]
pub type WTIMER0_R = crate::BitReader;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 access control bit"]
pub type WTIMER0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WTIMER1` reader - Wide Timer 0 access control bit"]
pub type WTIMER1_R = crate::BitReader;
#[doc = "Field `WTIMER1` writer - Wide Timer 0 access control bit"]
pub type WTIMER1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WTIMER2` reader - Wide Timer 2 access control bit"]
pub type WTIMER2_R = crate::BitReader;
#[doc = "Field `WTIMER2` writer - Wide Timer 2 access control bit"]
pub type WTIMER2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WTIMER3` reader - Wide Timer 3 access control bit"]
pub type WTIMER3_R = crate::BitReader;
#[doc = "Field `WTIMER3` writer - Wide Timer 3 access control bit"]
pub type WTIMER3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&self) -> PCNT0_R {
        PCNT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&self) -> PCNT1_R {
        PCNT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&self) -> PCNT2_R {
        PCNT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quad-SPI access control bit"]
    #[inline(always)]
    pub fn qspi0(&self) -> QSPI0_R {
        QSPI0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&self) -> RMU_R {
        RMU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real-Time Counter access control bit"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDIO Controller access control bit"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 2 access control bit"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 3 access control bit"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer 4 access control bit"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer 5 access control bit"]
    #[inline(always)]
    pub fn timer5(&self) -> TIMER5_R {
        TIMER5_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer 6 access control bit"]
    #[inline(always)]
    pub fn timer6(&self) -> TIMER6_R {
        TIMER6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&self) -> TRNG0_R {
        TRNG0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wide Timer 2 access control bit"]
    #[inline(always)]
    pub fn wtimer2(&self) -> WTIMER2_R {
        WTIMER2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wide Timer 3 access control bit"]
    #[inline(always)]
    pub fn wtimer3(&self) -> WTIMER3_R {
        WTIMER3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0(&mut self) -> PCNT0_W<PPUPATD1_SPEC, 0> {
        PCNT0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt1(&mut self) -> PCNT1_W<PPUPATD1_SPEC, 1> {
        PCNT1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt2(&mut self) -> PCNT2_W<PPUPATD1_SPEC, 2> {
        PCNT2_W::new(self)
    }
    #[doc = "Bit 3 - Quad-SPI access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn qspi0(&mut self) -> QSPI0_W<PPUPATD1_SPEC, 3> {
        QSPI0_W::new(self)
    }
    #[doc = "Bit 4 - Reset Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rmu(&mut self) -> RMU_W<PPUPATD1_SPEC, 4> {
        RMU_W::new(self)
    }
    #[doc = "Bit 5 - Real-Time Counter access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<PPUPATD1_SPEC, 5> {
        RTC_W::new(self)
    }
    #[doc = "Bit 6 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcc(&mut self) -> RTCC_W<PPUPATD1_SPEC, 6> {
        RTCC_W::new(self)
    }
    #[doc = "Bit 7 - SDIO Controller access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<PPUPATD1_SPEC, 7> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 8 - Security Management Unit access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn smu(&mut self) -> SMU_W<PPUPATD1_SPEC, 8> {
        SMU_W::new(self)
    }
    #[doc = "Bit 9 - Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<PPUPATD1_SPEC, 9> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 10 - Timer 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<PPUPATD1_SPEC, 10> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 11 - Timer 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<PPUPATD1_SPEC, 11> {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 12 - Timer 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer3(&mut self) -> TIMER3_W<PPUPATD1_SPEC, 12> {
        TIMER3_W::new(self)
    }
    #[doc = "Bit 13 - Timer 4 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer4(&mut self) -> TIMER4_W<PPUPATD1_SPEC, 13> {
        TIMER4_W::new(self)
    }
    #[doc = "Bit 14 - Timer 5 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer5(&mut self) -> TIMER5_W<PPUPATD1_SPEC, 14> {
        TIMER5_W::new(self)
    }
    #[doc = "Bit 15 - Timer 6 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer6(&mut self) -> TIMER6_W<PPUPATD1_SPEC, 15> {
        TIMER6_W::new(self)
    }
    #[doc = "Bit 16 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn trng0(&mut self) -> TRNG0_W<PPUPATD1_SPEC, 16> {
        TRNG0_W::new(self)
    }
    #[doc = "Bit 17 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<PPUPATD1_SPEC, 17> {
        UART0_W::new(self)
    }
    #[doc = "Bit 18 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<PPUPATD1_SPEC, 18> {
        UART1_W::new(self)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<PPUPATD1_SPEC, 19> {
        USART0_W::new(self)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<PPUPATD1_SPEC, 20> {
        USART1_W::new(self)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<PPUPATD1_SPEC, 21> {
        USART2_W::new(self)
    }
    #[doc = "Bit 22 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<PPUPATD1_SPEC, 22> {
        USART3_W::new(self)
    }
    #[doc = "Bit 23 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart4(&mut self) -> USART4_W<PPUPATD1_SPEC, 23> {
        USART4_W::new(self)
    }
    #[doc = "Bit 24 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart5(&mut self) -> USART5_W<PPUPATD1_SPEC, 24> {
        USART5_W::new(self)
    }
    #[doc = "Bit 25 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<PPUPATD1_SPEC, 25> {
        USB_W::new(self)
    }
    #[doc = "Bit 26 - Watchdog access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0(&mut self) -> WDOG0_W<PPUPATD1_SPEC, 26> {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 27 - Watchdog access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1(&mut self) -> WDOG1_W<PPUPATD1_SPEC, 27> {
        WDOG1_W::new(self)
    }
    #[doc = "Bit 28 - Wide Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer0(&mut self) -> WTIMER0_W<PPUPATD1_SPEC, 28> {
        WTIMER0_W::new(self)
    }
    #[doc = "Bit 29 - Wide Timer 0 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer1(&mut self) -> WTIMER1_W<PPUPATD1_SPEC, 29> {
        WTIMER1_W::new(self)
    }
    #[doc = "Bit 30 - Wide Timer 2 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer2(&mut self) -> WTIMER2_W<PPUPATD1_SPEC, 30> {
        WTIMER2_W::new(self)
    }
    #[doc = "Bit 31 - Wide Timer 3 access control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer3(&mut self) -> WTIMER3_W<PPUPATD1_SPEC, 31> {
        WTIMER3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPU Privilege Access Type Descriptor 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppupatd1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppupatd1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPUPATD1_SPEC;
impl crate::RegisterSpec for PPUPATD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppupatd1::R`](R) reader structure"]
impl crate::Readable for PPUPATD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppupatd1::W`](W) writer structure"]
impl crate::Writable for PPUPATD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUPATD1 to value 0"]
impl crate::Resettable for PPUPATD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
