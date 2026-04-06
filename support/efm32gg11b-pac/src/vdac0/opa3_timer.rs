#[doc = "Register `OPA3_TIMER` reader"]
pub type R = crate::R<Opa3TimerSpec>;
#[doc = "Register `OPA3_TIMER` writer"]
pub type W = crate::W<Opa3TimerSpec>;
#[doc = "Field `STARTUPDLY` reader - OPAx Startup Delay Count Value"]
pub type StartupdlyR = crate::FieldReader;
#[doc = "Field `STARTUPDLY` writer - OPAx Startup Delay Count Value"]
pub type StartupdlyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WARMUPTIME` reader - OPAx Warmup Time Count Value"]
pub type WarmuptimeR = crate::FieldReader;
#[doc = "Field `WARMUPTIME` writer - OPAx Warmup Time Count Value"]
pub type WarmuptimeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SETTLETIME` reader - OPAx Output Settling Timeout Value"]
pub type SettletimeR = crate::FieldReader<u16>;
#[doc = "Field `SETTLETIME` writer - OPAx Output Settling Timeout Value"]
pub type SettletimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    pub fn startupdly(&self) -> StartupdlyR {
        StartupdlyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WarmuptimeR {
        WarmuptimeR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    pub fn settletime(&self) -> SettletimeR {
        SettletimeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    pub fn startupdly(&mut self) -> StartupdlyW<'_, Opa3TimerSpec> {
        StartupdlyW::new(self, 0)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    pub fn warmuptime(&mut self) -> WarmuptimeW<'_, Opa3TimerSpec> {
        WarmuptimeW::new(self, 8)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    pub fn settletime(&mut self) -> SettletimeW<'_, Opa3TimerSpec> {
        SettletimeW::new(self, 16)
    }
}
#[doc = "Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opa3TimerSpec;
impl crate::RegisterSpec for Opa3TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa3_timer::R`](R) reader structure"]
impl crate::Readable for Opa3TimerSpec {}
#[doc = "`write(|w| ..)` method takes [`opa3_timer::W`](W) writer structure"]
impl crate::Writable for Opa3TimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPA3_TIMER to value 0x0001_0700"]
impl crate::Resettable for Opa3TimerSpec {
    const RESET_VALUE: u32 = 0x0001_0700;
}
