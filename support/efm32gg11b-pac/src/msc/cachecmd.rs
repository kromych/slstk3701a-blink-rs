#[doc = "Register `CACHECMD` writer"]
pub type W = crate::W<CachecmdSpec>;
#[doc = "Field `INVCACHE` writer - Invalidate Instruction Cache"]
pub type InvcacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type StartpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type StoppcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    pub fn invcache(&mut self) -> InvcacheW<'_, CachecmdSpec> {
        InvcacheW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    pub fn startpc(&mut self) -> StartpcW<'_, CachecmdSpec> {
        StartpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    pub fn stoppc(&mut self) -> StoppcW<'_, CachecmdSpec> {
        StoppcW::new(self, 2)
    }
}
#[doc = "Flash Cache Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CachecmdSpec;
impl crate::RegisterSpec for CachecmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cachecmd::W`](W) writer structure"]
impl crate::Writable for CachecmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHECMD to value 0"]
impl crate::Resettable for CachecmdSpec {}
