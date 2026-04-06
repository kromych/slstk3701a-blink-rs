#[doc = "Register `SWPULSE` writer"]
pub type W = crate::W<SwpulseSpec>;
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type Ch0pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type Ch1pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type Ch2pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type Ch3pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type Ch4pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type Ch5pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PULSE` writer - Channel 6 Pulse Generation"]
pub type Ch6pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PULSE` writer - Channel 7 Pulse Generation"]
pub type Ch7pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PULSE` writer - Channel 8 Pulse Generation"]
pub type Ch8pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PULSE` writer - Channel 9 Pulse Generation"]
pub type Ch9pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PULSE` writer - Channel 10 Pulse Generation"]
pub type Ch10pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PULSE` writer - Channel 11 Pulse Generation"]
pub type Ch11pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12PULSE` writer - Channel 12 Pulse Generation"]
pub type Ch12pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13PULSE` writer - Channel 13 Pulse Generation"]
pub type Ch13pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14PULSE` writer - Channel 14 Pulse Generation"]
pub type Ch14pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15PULSE` writer - Channel 15 Pulse Generation"]
pub type Ch15pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH16PULSE` writer - Channel 16 Pulse Generation"]
pub type Ch16pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH17PULSE` writer - Channel 17 Pulse Generation"]
pub type Ch17pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH18PULSE` writer - Channel 18 Pulse Generation"]
pub type Ch18pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH19PULSE` writer - Channel 19 Pulse Generation"]
pub type Ch19pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH20PULSE` writer - Channel 20 Pulse Generation"]
pub type Ch20pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH21PULSE` writer - Channel 21 Pulse Generation"]
pub type Ch21pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH22PULSE` writer - Channel 22 Pulse Generation"]
pub type Ch22pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH23PULSE` writer - Channel 23 Pulse Generation"]
pub type Ch23pulseW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> Ch0pulseW<'_, SwpulseSpec> {
        Ch0pulseW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> Ch1pulseW<'_, SwpulseSpec> {
        Ch1pulseW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> Ch2pulseW<'_, SwpulseSpec> {
        Ch2pulseW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> Ch3pulseW<'_, SwpulseSpec> {
        Ch3pulseW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> Ch4pulseW<'_, SwpulseSpec> {
        Ch4pulseW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> Ch5pulseW<'_, SwpulseSpec> {
        Ch5pulseW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    pub fn ch6pulse(&mut self) -> Ch6pulseW<'_, SwpulseSpec> {
        Ch6pulseW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    pub fn ch7pulse(&mut self) -> Ch7pulseW<'_, SwpulseSpec> {
        Ch7pulseW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    pub fn ch8pulse(&mut self) -> Ch8pulseW<'_, SwpulseSpec> {
        Ch8pulseW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    pub fn ch9pulse(&mut self) -> Ch9pulseW<'_, SwpulseSpec> {
        Ch9pulseW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    pub fn ch10pulse(&mut self) -> Ch10pulseW<'_, SwpulseSpec> {
        Ch10pulseW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    pub fn ch11pulse(&mut self) -> Ch11pulseW<'_, SwpulseSpec> {
        Ch11pulseW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 12 Pulse Generation"]
    #[inline(always)]
    pub fn ch12pulse(&mut self) -> Ch12pulseW<'_, SwpulseSpec> {
        Ch12pulseW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 13 Pulse Generation"]
    #[inline(always)]
    pub fn ch13pulse(&mut self) -> Ch13pulseW<'_, SwpulseSpec> {
        Ch13pulseW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 14 Pulse Generation"]
    #[inline(always)]
    pub fn ch14pulse(&mut self) -> Ch14pulseW<'_, SwpulseSpec> {
        Ch14pulseW::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 15 Pulse Generation"]
    #[inline(always)]
    pub fn ch15pulse(&mut self) -> Ch15pulseW<'_, SwpulseSpec> {
        Ch15pulseW::new(self, 15)
    }
    #[doc = "Bit 16 - Channel 16 Pulse Generation"]
    #[inline(always)]
    pub fn ch16pulse(&mut self) -> Ch16pulseW<'_, SwpulseSpec> {
        Ch16pulseW::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 17 Pulse Generation"]
    #[inline(always)]
    pub fn ch17pulse(&mut self) -> Ch17pulseW<'_, SwpulseSpec> {
        Ch17pulseW::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 18 Pulse Generation"]
    #[inline(always)]
    pub fn ch18pulse(&mut self) -> Ch18pulseW<'_, SwpulseSpec> {
        Ch18pulseW::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 19 Pulse Generation"]
    #[inline(always)]
    pub fn ch19pulse(&mut self) -> Ch19pulseW<'_, SwpulseSpec> {
        Ch19pulseW::new(self, 19)
    }
    #[doc = "Bit 20 - Channel 20 Pulse Generation"]
    #[inline(always)]
    pub fn ch20pulse(&mut self) -> Ch20pulseW<'_, SwpulseSpec> {
        Ch20pulseW::new(self, 20)
    }
    #[doc = "Bit 21 - Channel 21 Pulse Generation"]
    #[inline(always)]
    pub fn ch21pulse(&mut self) -> Ch21pulseW<'_, SwpulseSpec> {
        Ch21pulseW::new(self, 21)
    }
    #[doc = "Bit 22 - Channel 22 Pulse Generation"]
    #[inline(always)]
    pub fn ch22pulse(&mut self) -> Ch22pulseW<'_, SwpulseSpec> {
        Ch22pulseW::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 23 Pulse Generation"]
    #[inline(always)]
    pub fn ch23pulse(&mut self) -> Ch23pulseW<'_, SwpulseSpec> {
        Ch23pulseW::new(self, 23)
    }
}
#[doc = "Software Pulse Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwpulseSpec;
impl crate::RegisterSpec for SwpulseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpulse::W`](W) writer structure"]
impl crate::Writable for SwpulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SwpulseSpec {}
