#[doc = "Register `CACHECMD` writer"]
pub type W = crate::W<CACHECMD_SPEC>;
#[doc = "Field `INVCACHE` writer - Invalidate Instruction Cache"]
pub type INVCACHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type STARTPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type STOPPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    #[must_use]
    pub fn invcache(&mut self) -> INVCACHE_W<CACHECMD_SPEC, 0> {
        INVCACHE_W::new(self)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn startpc(&mut self) -> STARTPC_W<CACHECMD_SPEC, 1> {
        STARTPC_W::new(self)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    #[must_use]
    pub fn stoppc(&mut self) -> STOPPC_W<CACHECMD_SPEC, 2> {
        STOPPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHECMD to value 0"]
impl crate::Resettable for CACHECMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
