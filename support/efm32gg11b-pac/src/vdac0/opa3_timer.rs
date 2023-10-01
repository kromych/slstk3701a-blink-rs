#[doc = "Register `OPA3_TIMER` reader"]
pub type R = crate::R<OPA3_TIMER_SPEC>;
#[doc = "Register `OPA3_TIMER` writer"]
pub type W = crate::W<OPA3_TIMER_SPEC>;
#[doc = "Field `STARTUPDLY` reader - OPAx Startup Delay Count Value"]
pub type STARTUPDLY_R = crate::FieldReader;
#[doc = "Field `STARTUPDLY` writer - OPAx Startup Delay Count Value"]
pub type STARTUPDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `WARMUPTIME` reader - OPAx Warmup Time Count Value"]
pub type WARMUPTIME_R = crate::FieldReader;
#[doc = "Field `WARMUPTIME` writer - OPAx Warmup Time Count Value"]
pub type WARMUPTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SETTLETIME` reader - OPAx Output Settling Timeout Value"]
pub type SETTLETIME_R = crate::FieldReader<u16>;
#[doc = "Field `SETTLETIME` writer - OPAx Output Settling Timeout Value"]
pub type SETTLETIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    pub fn startupdly(&self) -> STARTUPDLY_R {
        STARTUPDLY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WARMUPTIME_R {
        WARMUPTIME_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    pub fn settletime(&self) -> SETTLETIME_R {
        SETTLETIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn startupdly(&mut self) -> STARTUPDLY_W<OPA3_TIMER_SPEC, 0> {
        STARTUPDLY_W::new(self)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn warmuptime(&mut self) -> WARMUPTIME_W<OPA3_TIMER_SPEC, 8> {
        WARMUPTIME_W::new(self)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn settletime(&mut self) -> SETTLETIME_W<OPA3_TIMER_SPEC, 16> {
        SETTLETIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Operational Amplifier Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa3_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa3_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPA3_TIMER_SPEC;
impl crate::RegisterSpec for OPA3_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa3_timer::R`](R) reader structure"]
impl crate::Readable for OPA3_TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opa3_timer::W`](W) writer structure"]
impl crate::Writable for OPA3_TIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA3_TIMER to value 0x0001_0700"]
impl crate::Resettable for OPA3_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0700;
}
