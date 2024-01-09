#[doc = "Register `ETMTRACEIDR` reader"]
pub type R = crate::R<ETMTRACEIDR_SPEC>;
#[doc = "Register `ETMTRACEIDR` writer"]
pub type W = crate::W<ETMTRACEIDR_SPEC>;
#[doc = "Field `TRACEID` reader - Trace ID"]
pub type TRACEID_R = crate::FieldReader;
#[doc = "Field `TRACEID` writer - Trace ID"]
pub type TRACEID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&self) -> TRACEID_R {
        TRACEID_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    #[must_use]
    pub fn traceid(&mut self) -> TRACEID_W<ETMTRACEIDR_SPEC> {
        TRACEID_W::new(self, 0)
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
#[doc = "CoreSight Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtraceidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtraceidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMTRACEIDR_SPEC;
impl crate::RegisterSpec for ETMTRACEIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtraceidr::R`](R) reader structure"]
impl crate::Readable for ETMTRACEIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmtraceidr::W`](W) writer structure"]
impl crate::Writable for ETMTRACEIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMTRACEIDR to value 0"]
impl crate::Resettable for ETMTRACEIDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
