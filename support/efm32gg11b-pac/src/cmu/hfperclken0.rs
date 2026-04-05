#[doc = "Register `HFPERCLKEN0` reader"]
pub type R = crate::R<Hfperclken0Spec>;
#[doc = "Register `HFPERCLKEN0` writer"]
pub type W = crate::W<Hfperclken0Spec>;
#[doc = "Field `TIMER0` reader - Timer 0 Clock Enable"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER0` writer - Timer 0 Clock Enable"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1` reader - Timer 1 Clock Enable"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER1` writer - Timer 1 Clock Enable"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2` reader - Timer 2 Clock Enable"]
pub type Timer2R = crate::BitReader;
#[doc = "Field `TIMER2` writer - Timer 2 Clock Enable"]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3` reader - Timer 3 Clock Enable"]
pub type Timer3R = crate::BitReader;
#[doc = "Field `TIMER3` writer - Timer 3 Clock Enable"]
pub type Timer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4` reader - Timer 4 Clock Enable"]
pub type Timer4R = crate::BitReader;
#[doc = "Field `TIMER4` writer - Timer 4 Clock Enable"]
pub type Timer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER5` reader - Timer 5 Clock Enable"]
pub type Timer5R = crate::BitReader;
#[doc = "Field `TIMER5` writer - Timer 5 Clock Enable"]
pub type Timer5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6` reader - Timer 6 Clock Enable"]
pub type Timer6R = crate::BitReader;
#[doc = "Field `TIMER6` writer - Timer 6 Clock Enable"]
pub type Timer6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Usart0R = crate::BitReader;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Usart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `USART2` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
pub type Usart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
pub type Usart3R = crate::BitReader;
#[doc = "Field `USART3` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable"]
pub type Usart4R = crate::BitReader;
#[doc = "Field `USART4` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable"]
pub type Usart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable"]
pub type Usart5R = crate::BitReader;
#[doc = "Field `USART5` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable"]
pub type Usart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 Clock Enable"]
pub type Acmp0R = crate::BitReader;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 Clock Enable"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 Clock Enable"]
pub type Acmp1R = crate::BitReader;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 Clock Enable"]
pub type Acmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2` reader - Analog Comparator 1 Clock Enable"]
pub type Acmp2R = crate::BitReader;
#[doc = "Field `ACMP2` writer - Analog Comparator 1 Clock Enable"]
pub type Acmp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3` reader - Analog Comparator 3 Clock Enable"]
pub type Acmp3R = crate::BitReader;
#[doc = "Field `ACMP3` writer - Analog Comparator 3 Clock Enable"]
pub type Acmp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C 0 Clock Enable"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C 0 Clock Enable"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C 1 Clock Enable"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C 1 Clock Enable"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C 2 Clock Enable"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C 2 Clock Enable"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable"]
pub type Adc0R = crate::BitReader;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - Analog to Digital Converter 0 Clock Enable"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - Analog to Digital Converter 0 Clock Enable"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYOTIMER` reader - CRYOTIMER Clock Enable"]
pub type CryotimerR = crate::BitReader;
#[doc = "Field `CRYOTIMER` writer - CRYOTIMER Clock Enable"]
pub type CryotimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDAC0` reader - Current Digital to Analog Converter 0 Clock Enable"]
pub type Idac0R = crate::BitReader;
#[doc = "Field `IDAC0` writer - Current Digital to Analog Converter 0 Clock Enable"]
pub type Idac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG0` reader - True Random Number Generator 0 Clock Enable"]
pub type Trng0R = crate::BitReader;
#[doc = "Field `TRNG0` writer - True Random Number Generator 0 Clock Enable"]
pub type Trng0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn timer3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 4 Clock Enable"]
    #[inline(always)]
    pub fn timer4(&self) -> Timer4R {
        Timer4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 5 Clock Enable"]
    #[inline(always)]
    pub fn timer5(&self) -> Timer5R {
        Timer5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 6 Clock Enable"]
    #[inline(always)]
    pub fn timer6(&self) -> Timer6R {
        Timer6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable"]
    #[inline(always)]
    pub fn usart4(&self) -> Usart4R {
        Usart4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable"]
    #[inline(always)]
    pub fn usart5(&self) -> Usart5R {
        Usart5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&self) -> Acmp1R {
        Acmp1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp2(&self) -> Acmp2R {
        Acmp2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog Comparator 3 Clock Enable"]
    #[inline(always)]
    pub fn acmp3(&self) -> Acmp3R {
        Acmp3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C 1 Clock Enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C 2 Clock Enable"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CRYOTIMER Clock Enable"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CryotimerR {
        CryotimerR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&self) -> Idac0R {
        Idac0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - True Random Number Generator 0 Clock Enable"]
    #[inline(always)]
    pub fn trng0(&self) -> Trng0R {
        Trng0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&mut self) -> Timer0W<'_, Hfperclken0Spec> {
        Timer0W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&mut self) -> Timer1W<'_, Hfperclken0Spec> {
        Timer1W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&mut self) -> Timer2W<'_, Hfperclken0Spec> {
        Timer2W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn timer3(&mut self) -> Timer3W<'_, Hfperclken0Spec> {
        Timer3W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer 4 Clock Enable"]
    #[inline(always)]
    pub fn timer4(&mut self) -> Timer4W<'_, Hfperclken0Spec> {
        Timer4W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 5 Clock Enable"]
    #[inline(always)]
    pub fn timer5(&mut self) -> Timer5W<'_, Hfperclken0Spec> {
        Timer5W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 6 Clock Enable"]
    #[inline(always)]
    pub fn timer6(&mut self) -> Timer6W<'_, Hfperclken0Spec> {
        Timer6W::new(self, 6)
    }
    #[doc = "Bit 7 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&mut self) -> Usart0W<'_, Hfperclken0Spec> {
        Usart0W::new(self, 7)
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<'_, Hfperclken0Spec> {
        Usart1W::new(self, 8)
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
    #[inline(always)]
    pub fn usart2(&mut self) -> Usart2W<'_, Hfperclken0Spec> {
        Usart2W::new(self, 9)
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<'_, Hfperclken0Spec> {
        Usart3W::new(self, 10)
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable"]
    #[inline(always)]
    pub fn usart4(&mut self) -> Usart4W<'_, Hfperclken0Spec> {
        Usart4W::new(self, 11)
    }
    #[doc = "Bit 12 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable"]
    #[inline(always)]
    pub fn usart5(&mut self) -> Usart5W<'_, Hfperclken0Spec> {
        Usart5W::new(self, 12)
    }
    #[doc = "Bit 13 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<'_, Hfperclken0Spec> {
        Acmp0W::new(self, 13)
    }
    #[doc = "Bit 14 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> Acmp1W<'_, Hfperclken0Spec> {
        Acmp1W::new(self, 14)
    }
    #[doc = "Bit 15 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp2(&mut self) -> Acmp2W<'_, Hfperclken0Spec> {
        Acmp2W::new(self, 15)
    }
    #[doc = "Bit 16 - Analog Comparator 3 Clock Enable"]
    #[inline(always)]
    pub fn acmp3(&mut self) -> Acmp3W<'_, Hfperclken0Spec> {
        Acmp3W::new(self, 16)
    }
    #[doc = "Bit 17 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, Hfperclken0Spec> {
        I2c0W::new(self, 17)
    }
    #[doc = "Bit 18 - I2C 1 Clock Enable"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Hfperclken0Spec> {
        I2c1W::new(self, 18)
    }
    #[doc = "Bit 19 - I2C 2 Clock Enable"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, Hfperclken0Spec> {
        I2c2W::new(self, 19)
    }
    #[doc = "Bit 20 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<'_, Hfperclken0Spec> {
        Adc0W::new(self, 20)
    }
    #[doc = "Bit 21 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Hfperclken0Spec> {
        Adc1W::new(self, 21)
    }
    #[doc = "Bit 22 - CRYOTIMER Clock Enable"]
    #[inline(always)]
    pub fn cryotimer(&mut self) -> CryotimerW<'_, Hfperclken0Spec> {
        CryotimerW::new(self, 22)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&mut self) -> Idac0W<'_, Hfperclken0Spec> {
        Idac0W::new(self, 23)
    }
    #[doc = "Bit 24 - True Random Number Generator 0 Clock Enable"]
    #[inline(always)]
    pub fn trng0(&mut self) -> Trng0W<'_, Hfperclken0Spec> {
        Trng0W::new(self, 24)
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfperclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfperclken0Spec;
impl crate::RegisterSpec for Hfperclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken0::R`](R) reader structure"]
impl crate::Readable for Hfperclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`hfperclken0::W`](W) writer structure"]
impl crate::Writable for Hfperclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFPERCLKEN0 to value 0"]
impl crate::Resettable for Hfperclken0Spec {}
