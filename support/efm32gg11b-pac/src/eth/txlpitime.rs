#[doc = "Register `TXLPITIME` reader"]
pub type R = crate::R<TXLPITIME_SPEC>;
#[doc = "Register `TXLPITIME` writer"]
pub type W = crate::W<TXLPITIME_SPEC>;
#[doc = "Field `LPITIME` reader - Time in LPI"]
pub type LPITIME_R = crate::FieldReader<u32>;
#[doc = "Field `LPITIME` writer - Time in LPI"]
pub type LPITIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&self) -> LPITIME_R {
        LPITIME_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    #[must_use]
    pub fn lpitime(&mut self) -> LPITIME_W<TXLPITIME_SPEC, 0> {
        LPITIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit LPI time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlpitime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txlpitime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLPITIME_SPEC;
impl crate::RegisterSpec for TXLPITIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpitime::R`](R) reader structure"]
impl crate::Readable for TXLPITIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txlpitime::W`](W) writer structure"]
impl crate::Writable for TXLPITIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXLPITIME to value 0"]
impl crate::Resettable for TXLPITIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
