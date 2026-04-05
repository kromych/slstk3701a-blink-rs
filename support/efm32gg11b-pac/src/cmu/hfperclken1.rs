#[doc = "Register `HFPERCLKEN1` reader"]
pub type R = crate::R<Hfperclken1Spec>;
#[doc = "Register `HFPERCLKEN1` writer"]
pub type W = crate::W<Hfperclken1Spec>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 Clock Enable"]
pub type Wtimer0R = crate::BitReader;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 Clock Enable"]
pub type Wtimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER1` reader - Wide Timer 0 Clock Enable"]
pub type Wtimer1R = crate::BitReader;
#[doc = "Field `WTIMER1` writer - Wide Timer 0 Clock Enable"]
pub type Wtimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER2` reader - Wide Timer 2 Clock Enable"]
pub type Wtimer2R = crate::BitReader;
#[doc = "Field `WTIMER2` writer - Wide Timer 2 Clock Enable"]
pub type Wtimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER3` reader - Wide Timer 3 Clock Enable"]
pub type Wtimer3R = crate::BitReader;
#[doc = "Field `WTIMER3` writer - Wide Timer 3 Clock Enable"]
pub type Wtimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0` reader - CAN 0 Clock Enable"]
pub type Can0R = crate::BitReader;
#[doc = "Field `CAN0` writer - CAN 0 Clock Enable"]
pub type Can0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN 1 Clock Enable"]
pub type Can1R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN 1 Clock Enable"]
pub type Can1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 Clock Enable"]
pub type Vdac0R = crate::BitReader;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 Clock Enable"]
pub type Vdac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module Clock Enable"]
pub type CsenR = crate::BitReader;
#[doc = "Field `CSEN` writer - Capacitive touch sense module Clock Enable"]
pub type CsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&self) -> Wtimer0R {
        Wtimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&self) -> Wtimer1R {
        Wtimer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn wtimer2(&self) -> Wtimer2R {
        Wtimer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn wtimer3(&self) -> Wtimer3R {
        Wtimer3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&self) -> Can0R {
        Can0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN 1 Clock Enable"]
    #[inline(always)]
    pub fn can1(&self) -> Can1R {
        Can1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&self) -> Vdac0R {
        Vdac0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CsenR {
        CsenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> Wtimer0W<'_, Hfperclken1Spec> {
        Wtimer0W::new(self, 0)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> Wtimer1W<'_, Hfperclken1Spec> {
        Wtimer1W::new(self, 1)
    }
    #[doc = "Bit 2 - Wide Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn wtimer2(&mut self) -> Wtimer2W<'_, Hfperclken1Spec> {
        Wtimer2W::new(self, 2)
    }
    #[doc = "Bit 3 - Wide Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn wtimer3(&mut self) -> Wtimer3W<'_, Hfperclken1Spec> {
        Wtimer3W::new(self, 3)
    }
    #[doc = "Bit 4 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<'_, Hfperclken1Spec> {
        Uart0W::new(self, 4)
    }
    #[doc = "Bit 5 - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Hfperclken1Spec> {
        Uart1W::new(self, 5)
    }
    #[doc = "Bit 6 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&mut self) -> Can0W<'_, Hfperclken1Spec> {
        Can0W::new(self, 6)
    }
    #[doc = "Bit 7 - CAN 1 Clock Enable"]
    #[inline(always)]
    pub fn can1(&mut self) -> Can1W<'_, Hfperclken1Spec> {
        Can1W::new(self, 7)
    }
    #[doc = "Bit 8 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> Vdac0W<'_, Hfperclken1Spec> {
        Vdac0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&mut self) -> CsenW<'_, Hfperclken1Spec> {
        CsenW::new(self, 9)
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hfperclken1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperclken1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfperclken1Spec;
impl crate::RegisterSpec for Hfperclken1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken1::R`](R) reader structure"]
impl crate::Readable for Hfperclken1Spec {}
#[doc = "`write(|w| ..)` method takes [`hfperclken1::W`](W) writer structure"]
impl crate::Writable for Hfperclken1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFPERCLKEN1 to value 0"]
impl crate::Resettable for Hfperclken1Spec {}
