#[doc = "Register `SWPULSE` writer"]
pub type W = crate::W<SWPULSE_SPEC>;
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type CH0PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type CH1PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type CH2PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type CH3PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type CH4PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type CH5PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PULSE` writer - Channel 6 Pulse Generation"]
pub type CH6PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PULSE` writer - Channel 7 Pulse Generation"]
pub type CH7PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PULSE` writer - Channel 8 Pulse Generation"]
pub type CH8PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PULSE` writer - Channel 9 Pulse Generation"]
pub type CH9PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PULSE` writer - Channel 10 Pulse Generation"]
pub type CH10PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PULSE` writer - Channel 11 Pulse Generation"]
pub type CH11PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12PULSE` writer - Channel 12 Pulse Generation"]
pub type CH12PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13PULSE` writer - Channel 13 Pulse Generation"]
pub type CH13PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14PULSE` writer - Channel 14 Pulse Generation"]
pub type CH14PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15PULSE` writer - Channel 15 Pulse Generation"]
pub type CH15PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH16PULSE` writer - Channel 16 Pulse Generation"]
pub type CH16PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH17PULSE` writer - Channel 17 Pulse Generation"]
pub type CH17PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH18PULSE` writer - Channel 18 Pulse Generation"]
pub type CH18PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH19PULSE` writer - Channel 19 Pulse Generation"]
pub type CH19PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH20PULSE` writer - Channel 20 Pulse Generation"]
pub type CH20PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH21PULSE` writer - Channel 21 Pulse Generation"]
pub type CH21PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH22PULSE` writer - Channel 22 Pulse Generation"]
pub type CH22PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH23PULSE` writer - Channel 23 Pulse Generation"]
pub type CH23PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W<SWPULSE_SPEC> {
        CH0PULSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W<SWPULSE_SPEC> {
        CH1PULSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W<SWPULSE_SPEC> {
        CH2PULSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W<SWPULSE_SPEC> {
        CH3PULSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W<SWPULSE_SPEC> {
        CH4PULSE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W<SWPULSE_SPEC> {
        CH5PULSE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pulse(&mut self) -> CH6PULSE_W<SWPULSE_SPEC> {
        CH6PULSE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pulse(&mut self) -> CH7PULSE_W<SWPULSE_SPEC> {
        CH7PULSE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pulse(&mut self) -> CH8PULSE_W<SWPULSE_SPEC> {
        CH8PULSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pulse(&mut self) -> CH9PULSE_W<SWPULSE_SPEC> {
        CH9PULSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pulse(&mut self) -> CH10PULSE_W<SWPULSE_SPEC> {
        CH10PULSE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pulse(&mut self) -> CH11PULSE_W<SWPULSE_SPEC> {
        CH11PULSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 12 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch12pulse(&mut self) -> CH12PULSE_W<SWPULSE_SPEC> {
        CH12PULSE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 13 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch13pulse(&mut self) -> CH13PULSE_W<SWPULSE_SPEC> {
        CH13PULSE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 14 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch14pulse(&mut self) -> CH14PULSE_W<SWPULSE_SPEC> {
        CH14PULSE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 15 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch15pulse(&mut self) -> CH15PULSE_W<SWPULSE_SPEC> {
        CH15PULSE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel 16 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch16pulse(&mut self) -> CH16PULSE_W<SWPULSE_SPEC> {
        CH16PULSE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 17 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch17pulse(&mut self) -> CH17PULSE_W<SWPULSE_SPEC> {
        CH17PULSE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 18 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch18pulse(&mut self) -> CH18PULSE_W<SWPULSE_SPEC> {
        CH18PULSE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 19 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch19pulse(&mut self) -> CH19PULSE_W<SWPULSE_SPEC> {
        CH19PULSE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel 20 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch20pulse(&mut self) -> CH20PULSE_W<SWPULSE_SPEC> {
        CH20PULSE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel 21 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch21pulse(&mut self) -> CH21PULSE_W<SWPULSE_SPEC> {
        CH21PULSE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel 22 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch22pulse(&mut self) -> CH22PULSE_W<SWPULSE_SPEC> {
        CH22PULSE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 23 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch23pulse(&mut self) -> CH23PULSE_W<SWPULSE_SPEC> {
        CH23PULSE_W::new(self, 23)
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
#[doc = "Software Pulse Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWPULSE_SPEC;
impl crate::RegisterSpec for SWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpulse::W`](W) writer structure"]
impl crate::Writable for SWPULSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SWPULSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
