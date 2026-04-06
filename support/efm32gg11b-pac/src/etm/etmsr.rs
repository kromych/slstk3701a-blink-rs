#[doc = "Register `ETMSR` reader"]
pub type R = crate::R<EtmsrSpec>;
#[doc = "Register `ETMSR` writer"]
pub type W = crate::W<EtmsrSpec>;
#[doc = "Field `ETHOF` reader - ETM Overflow"]
pub type EthofR = crate::BitReader;
#[doc = "Field `ETMPROGBIT` reader - ETM Programming Bit Status"]
pub type EtmprogbitR = crate::BitReader;
#[doc = "Field `TRACESTAT` reader - Trace Start/Stop Status"]
pub type TracestatR = crate::BitReader;
#[doc = "Field `TRACESTAT` writer - Trace Start/Stop Status"]
pub type TracestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGBIT` reader - Trigger Bit"]
pub type TrigbitR = crate::BitReader;
#[doc = "Field `TRIGBIT` writer - Trigger Bit"]
pub type TrigbitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETM Overflow"]
    #[inline(always)]
    pub fn ethof(&self) -> EthofR {
        EthofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETM Programming Bit Status"]
    #[inline(always)]
    pub fn etmprogbit(&self) -> EtmprogbitR {
        EtmprogbitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&self) -> TracestatR {
        TracestatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&self) -> TrigbitR {
        TrigbitR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Trace Start/Stop Status"]
    #[inline(always)]
    pub fn tracestat(&mut self) -> TracestatW<'_, EtmsrSpec> {
        TracestatW::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger Bit"]
    #[inline(always)]
    pub fn trigbit(&mut self) -> TrigbitW<'_, EtmsrSpec> {
        TrigbitW::new(self, 3)
    }
}
#[doc = "ETM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmsrSpec;
impl crate::RegisterSpec for EtmsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmsr::R`](R) reader structure"]
impl crate::Readable for EtmsrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmsr::W`](W) writer structure"]
impl crate::Writable for EtmsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMSR to value 0x02"]
impl crate::Resettable for EtmsrSpec {
    const RESET_VALUE: u32 = 0x02;
}
