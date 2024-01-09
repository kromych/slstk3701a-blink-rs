#[doc = "Register `CACHECMD` writer"]
pub type W = crate::W<CACHECMD_SPEC>;
#[doc = "Field `INVCACHE` writer - Invalidate Instruction Cache"]
pub type INVCACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type STARTPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type STOPPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    #[must_use]
    pub fn invcache(&mut self) -> INVCACHE_W<CACHECMD_SPEC> {
        INVCACHE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn startpc(&mut self) -> STARTPC_W<CACHECMD_SPEC> {
        STARTPC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn stoppc(&mut self) -> STOPPC_W<CACHECMD_SPEC> {
        STOPPC_W::new(self, 2)
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
#[doc = "Flash Cache Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cachecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHECMD_SPEC;
impl crate::RegisterSpec for CACHECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cachecmd::W`](W) writer structure"]
impl crate::Writable for CACHECMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHECMD to value 0"]
impl crate::Resettable for CACHECMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
