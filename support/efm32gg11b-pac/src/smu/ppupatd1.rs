#[doc = "Register `PPUPATD1` reader"]
pub type R = crate::R<Ppupatd1Spec>;
#[doc = "Register `PPUPATD1` writer"]
pub type W = crate::W<Ppupatd1Spec>;
#[doc = "Field `PCNT0` reader - Pulse Counter 0 access control bit"]
pub type Pcnt0R = crate::BitReader;
#[doc = "Field `PCNT0` writer - Pulse Counter 0 access control bit"]
pub type Pcnt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT1` reader - Pulse Counter 1 access control bit"]
pub type Pcnt1R = crate::BitReader;
#[doc = "Field `PCNT1` writer - Pulse Counter 1 access control bit"]
pub type Pcnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT2` reader - Pulse Counter 2 access control bit"]
pub type Pcnt2R = crate::BitReader;
#[doc = "Field `PCNT2` writer - Pulse Counter 2 access control bit"]
pub type Pcnt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI0` reader - Quad-SPI access control bit"]
pub type Qspi0R = crate::BitReader;
#[doc = "Field `QSPI0` writer - Quad-SPI access control bit"]
pub type Qspi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMU` reader - Reset Management Unit access control bit"]
pub type RmuR = crate::BitReader;
#[doc = "Field `RMU` writer - Reset Management Unit access control bit"]
pub type RmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Real-Time Counter access control bit"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - Real-Time Counter access control bit"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar access control bit"]
pub type RtccR = crate::BitReader;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar access control bit"]
pub type RtccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO` reader - SDIO Controller access control bit"]
pub type SdioR = crate::BitReader;
#[doc = "Field `SDIO` writer - SDIO Controller access control bit"]
pub type SdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMU` reader - Security Management Unit access control bit"]
pub type SmuR = crate::BitReader;
#[doc = "Field `SMU` writer - Security Management Unit access control bit"]
pub type SmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0` reader - Timer 0 access control bit"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 access control bit"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1` reader - Timer 1 access control bit"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 access control bit"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2` reader - Timer 2 access control bit"]
pub type Timer2R = crate::BitReader;
#[doc = "Field `TIMER2` writer - Timer 2 access control bit"]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3` reader - Timer 3 access control bit"]
pub type Timer3R = crate::BitReader;
#[doc = "Field `TIMER3` writer - Timer 3 access control bit"]
pub type Timer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4` reader - Timer 4 access control bit"]
pub type Timer4R = crate::BitReader;
#[doc = "Field `TIMER4` writer - Timer 4 access control bit"]
pub type Timer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER5` reader - Timer 5 access control bit"]
pub type Timer5R = crate::BitReader;
#[doc = "Field `TIMER5` writer - Timer 5 access control bit"]
pub type Timer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6` reader - Timer 6 access control bit"]
pub type Timer6R = crate::BitReader;
#[doc = "Field `TIMER6` writer - Timer 6 access control bit"]
pub type Timer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG0` reader - True Random Number Generator 0 access control bit"]
pub type Trng0R = crate::BitReader;
#[doc = "Field `TRNG0` writer - True Random Number Generator 0 access control bit"]
pub type Trng0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type Usart0R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
pub type Usart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `USART2` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
pub type Usart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type Usart3R = crate::BitReader;
#[doc = "Field `USART3` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
pub type Usart4R = crate::BitReader;
#[doc = "Field `USART4` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
pub type Usart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
pub type Usart5R = crate::BitReader;
#[doc = "Field `USART5` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
pub type Usart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface access control bit"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - Universal Serial Bus Interface access control bit"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0` reader - Watchdog access control bit"]
pub type Wdog0R = crate::BitReader;
#[doc = "Field `WDOG0` writer - Watchdog access control bit"]
pub type Wdog0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1` reader - Watchdog access control bit"]
pub type Wdog1R = crate::BitReader;
#[doc = "Field `WDOG1` writer - Watchdog access control bit"]
pub type Wdog1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 access control bit"]
pub type Wtimer0R = crate::BitReader;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 access control bit"]
pub type Wtimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER1` reader - Wide Timer 0 access control bit"]
pub type Wtimer1R = crate::BitReader;
#[doc = "Field `WTIMER1` writer - Wide Timer 0 access control bit"]
pub type Wtimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER2` reader - Wide Timer 2 access control bit"]
pub type Wtimer2R = crate::BitReader;
#[doc = "Field `WTIMER2` writer - Wide Timer 2 access control bit"]
pub type Wtimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER3` reader - Wide Timer 3 access control bit"]
pub type Wtimer3R = crate::BitReader;
#[doc = "Field `WTIMER3` writer - Wide Timer 3 access control bit"]
pub type Wtimer3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&self) -> Pcnt0R {
        Pcnt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&self) -> Pcnt1R {
        Pcnt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&self) -> Pcnt2R {
        Pcnt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quad-SPI access control bit"]
    #[inline(always)]
    pub fn qspi0(&self) -> Qspi0R {
        Qspi0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&self) -> RmuR {
        RmuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real-Time Counter access control bit"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&self) -> RtccR {
        RtccR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDIO Controller access control bit"]
    #[inline(always)]
    pub fn sdio(&self) -> SdioR {
        SdioR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&self) -> SmuR {
        SmuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 2 access control bit"]
    #[inline(always)]
    pub fn timer2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer 3 access control bit"]
    #[inline(always)]
    pub fn timer3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer 4 access control bit"]
    #[inline(always)]
    pub fn timer4(&self) -> Timer4R {
        Timer4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer 5 access control bit"]
    #[inline(always)]
    pub fn timer5(&self) -> Timer5R {
        Timer5R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer 6 access control bit"]
    #[inline(always)]
    pub fn timer6(&self) -> Timer6R {
        Timer6R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&self) -> Trng0R {
        Trng0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
    #[inline(always)]
    pub fn usart5(&self) -> Usart5R {
        Usart5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> Wdog0R {
        Wdog0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog1(&self) -> Wdog1R {
        Wdog1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> Wtimer0R {
        Wtimer0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> Wtimer1R {
        Wtimer1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wide Timer 2 access control bit"]
    #[inline(always)]
    pub fn wtimer2(&self) -> Wtimer2R {
        Wtimer2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wide Timer 3 access control bit"]
    #[inline(always)]
    pub fn wtimer3(&self) -> Wtimer3R {
        Wtimer3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&mut self) -> Pcnt0W<'_, Ppupatd1Spec> {
        Pcnt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&mut self) -> Pcnt1W<'_, Ppupatd1Spec> {
        Pcnt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&mut self) -> Pcnt2W<'_, Ppupatd1Spec> {
        Pcnt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Quad-SPI access control bit"]
    #[inline(always)]
    pub fn qspi0(&mut self) -> Qspi0W<'_, Ppupatd1Spec> {
        Qspi0W::new(self, 3)
    }
    #[doc = "Bit 4 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&mut self) -> RmuW<'_, Ppupatd1Spec> {
        RmuW::new(self, 4)
    }
    #[doc = "Bit 5 - Real-Time Counter access control bit"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Ppupatd1Spec> {
        RtcW::new(self, 5)
    }
    #[doc = "Bit 6 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RtccW<'_, Ppupatd1Spec> {
        RtccW::new(self, 6)
    }
    #[doc = "Bit 7 - SDIO Controller access control bit"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SdioW<'_, Ppupatd1Spec> {
        SdioW::new(self, 7)
    }
    #[doc = "Bit 8 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&mut self) -> SmuW<'_, Ppupatd1Spec> {
        SmuW::new(self, 8)
    }
    #[doc = "Bit 9 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&mut self) -> Timer0W<'_, Ppupatd1Spec> {
        Timer0W::new(self, 9)
    }
    #[doc = "Bit 10 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&mut self) -> Timer1W<'_, Ppupatd1Spec> {
        Timer1W::new(self, 10)
    }
    #[doc = "Bit 11 - Timer 2 access control bit"]
    #[inline(always)]
    pub fn timer2(&mut self) -> Timer2W<'_, Ppupatd1Spec> {
        Timer2W::new(self, 11)
    }
    #[doc = "Bit 12 - Timer 3 access control bit"]
    #[inline(always)]
    pub fn timer3(&mut self) -> Timer3W<'_, Ppupatd1Spec> {
        Timer3W::new(self, 12)
    }
    #[doc = "Bit 13 - Timer 4 access control bit"]
    #[inline(always)]
    pub fn timer4(&mut self) -> Timer4W<'_, Ppupatd1Spec> {
        Timer4W::new(self, 13)
    }
    #[doc = "Bit 14 - Timer 5 access control bit"]
    #[inline(always)]
    pub fn timer5(&mut self) -> Timer5W<'_, Ppupatd1Spec> {
        Timer5W::new(self, 14)
    }
    #[doc = "Bit 15 - Timer 6 access control bit"]
    #[inline(always)]
    pub fn timer6(&mut self) -> Timer6W<'_, Ppupatd1Spec> {
        Timer6W::new(self, 15)
    }
    #[doc = "Bit 16 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&mut self) -> Trng0W<'_, Ppupatd1Spec> {
        Trng0W::new(self, 16)
    }
    #[doc = "Bit 17 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<'_, Ppupatd1Spec> {
        Uart0W::new(self, 17)
    }
    #[doc = "Bit 18 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Ppupatd1Spec> {
        Uart1W::new(self, 18)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&mut self) -> Usart0W<'_, Ppupatd1Spec> {
        Usart0W::new(self, 19)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<'_, Ppupatd1Spec> {
        Usart1W::new(self, 20)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&mut self) -> Usart2W<'_, Ppupatd1Spec> {
        Usart2W::new(self, 21)
    }
    #[doc = "Bit 22 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<'_, Ppupatd1Spec> {
        Usart3W::new(self, 22)
    }
    #[doc = "Bit 23 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    pub fn usart4(&mut self) -> Usart4W<'_, Ppupatd1Spec> {
        Usart4W::new(self, 23)
    }
    #[doc = "Bit 24 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
    #[inline(always)]
    pub fn usart5(&mut self) -> Usart5W<'_, Ppupatd1Spec> {
        Usart5W::new(self, 24)
    }
    #[doc = "Bit 25 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Ppupatd1Spec> {
        UsbW::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> Wdog0W<'_, Ppupatd1Spec> {
        Wdog0W::new(self, 26)
    }
    #[doc = "Bit 27 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> Wdog1W<'_, Ppupatd1Spec> {
        Wdog1W::new(self, 27)
    }
    #[doc = "Bit 28 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> Wtimer0W<'_, Ppupatd1Spec> {
        Wtimer0W::new(self, 28)
    }
    #[doc = "Bit 29 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> Wtimer1W<'_, Ppupatd1Spec> {
        Wtimer1W::new(self, 29)
    }
    #[doc = "Bit 30 - Wide Timer 2 access control bit"]
    #[inline(always)]
    pub fn wtimer2(&mut self) -> Wtimer2W<'_, Ppupatd1Spec> {
        Wtimer2W::new(self, 30)
    }
    #[doc = "Bit 31 - Wide Timer 3 access control bit"]
    #[inline(always)]
    pub fn wtimer3(&mut self) -> Wtimer3W<'_, Ppupatd1Spec> {
        Wtimer3W::new(self, 31)
    }
}
#[doc = "PPU Privilege Access Type Descriptor 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppupatd1Spec;
impl crate::RegisterSpec for Ppupatd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppupatd1::R`](R) reader structure"]
impl crate::Readable for Ppupatd1Spec {}
#[doc = "`write(|w| ..)` method takes [`ppupatd1::W`](W) writer structure"]
impl crate::Writable for Ppupatd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUPATD1 to value 0"]
impl crate::Resettable for Ppupatd1Spec {}
