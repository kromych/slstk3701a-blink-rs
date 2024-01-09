#[doc = "Register `HFPERCLKEN1` reader"]
pub type R = crate::R<HFPERCLKEN1_SPEC>;
#[doc = "Register `HFPERCLKEN1` writer"]
pub type W = crate::W<HFPERCLKEN1_SPEC>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 Clock Enable"]
pub type WTIMER0_R = crate::BitReader;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 Clock Enable"]
pub type WTIMER0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER1` reader - Wide Timer 0 Clock Enable"]
pub type WTIMER1_R = crate::BitReader;
#[doc = "Field `WTIMER1` writer - Wide Timer 0 Clock Enable"]
pub type WTIMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER2` reader - Wide Timer 2 Clock Enable"]
pub type WTIMER2_R = crate::BitReader;
#[doc = "Field `WTIMER2` writer - Wide Timer 2 Clock Enable"]
pub type WTIMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTIMER3` reader - Wide Timer 3 Clock Enable"]
pub type WTIMER3_R = crate::BitReader;
#[doc = "Field `WTIMER3` writer - Wide Timer 3 Clock Enable"]
pub type WTIMER3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0` reader - CAN 0 Clock Enable"]
pub type CAN0_R = crate::BitReader;
#[doc = "Field `CAN0` writer - CAN 0 Clock Enable"]
pub type CAN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN 1 Clock Enable"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN 1 Clock Enable"]
pub type CAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 Clock Enable"]
pub type VDAC0_R = crate::BitReader;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 Clock Enable"]
pub type VDAC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module Clock Enable"]
pub type CSEN_R = crate::BitReader;
#[doc = "Field `CSEN` writer - Capacitive touch sense module Clock Enable"]
pub type CSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn wtimer2(&self) -> WTIMER2_R {
        WTIMER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn wtimer3(&self) -> WTIMER3_R {
        WTIMER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN 1 Clock Enable"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer0(&mut self) -> WTIMER0_W<HFPERCLKEN1_SPEC> {
        WTIMER0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer1(&mut self) -> WTIMER1_W<HFPERCLKEN1_SPEC> {
        WTIMER1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wide Timer 2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer2(&mut self) -> WTIMER2_W<HFPERCLKEN1_SPEC> {
        WTIMER2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Wide Timer 3 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wtimer3(&mut self) -> WTIMER3_W<HFPERCLKEN1_SPEC> {
        WTIMER3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<HFPERCLKEN1_SPEC> {
        UART0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<HFPERCLKEN1_SPEC> {
        UART1_W::new(self, 5)
    }
    #[doc = "Bit 6 - CAN 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can0(&mut self) -> CAN0_W<HFPERCLKEN1_SPEC> {
        CAN0_W::new(self, 6)
    }
    #[doc = "Bit 7 - CAN 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<HFPERCLKEN1_SPEC> {
        CAN1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vdac0(&mut self) -> VDAC0_W<HFPERCLKEN1_SPEC> {
        VDAC0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<HFPERCLKEN1_SPEC> {
        CSEN_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfperclken1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfperclken1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPERCLKEN1_SPEC;
impl crate::RegisterSpec for HFPERCLKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfperclken1::R`](R) reader structure"]
impl crate::Readable for HFPERCLKEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfperclken1::W`](W) writer structure"]
impl crate::Writable for HFPERCLKEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFPERCLKEN1 to value 0"]
impl crate::Resettable for HFPERCLKEN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
