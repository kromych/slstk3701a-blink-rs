#[doc = "Register `ETMCNTRLDVR1` reader"]
pub type R = crate::R<ETMCNTRLDVR1_SPEC>;
#[doc = "Register `ETMCNTRLDVR1` writer"]
pub type W = crate::W<ETMCNTRLDVR1_SPEC>;
#[doc = "Field `COUNT` reader - Free running counter reload value"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Free running counter reload value"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Free running counter reload value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Free running counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<ETMCNTRLDVR1_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Counter Reload Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmcntrldvr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmcntrldvr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMCNTRLDVR1_SPEC;
impl crate::RegisterSpec for ETMCNTRLDVR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcntrldvr1::R`](R) reader structure"]
impl crate::Readable for ETMCNTRLDVR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmcntrldvr1::W`](W) writer structure"]
impl crate::Writable for ETMCNTRLDVR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETMCNTRLDVR1 to value 0"]
impl crate::Resettable for ETMCNTRLDVR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
