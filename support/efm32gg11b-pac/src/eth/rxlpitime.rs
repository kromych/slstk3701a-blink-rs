#[doc = "Register `RXLPITIME` reader"]
pub type R = crate::R<RXLPITIME_SPEC>;
#[doc = "Register `RXLPITIME` writer"]
pub type W = crate::W<RXLPITIME_SPEC>;
#[doc = "Field `LPITIME` reader - Time in LPI"]
pub type LPITIME_R = crate::FieldReader<u32>;
#[doc = "Field `LPITIME` writer - Time in LPI"]
pub type LPITIME_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    pub fn lpitime(&mut self) -> LPITIME_W<RXLPITIME_SPEC> {
        LPITIME_W::new(self, 0)
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
#[doc = "Received LPI time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlpitime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxlpitime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLPITIME_SPEC;
impl crate::RegisterSpec for RXLPITIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpitime::R`](R) reader structure"]
impl crate::Readable for RXLPITIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxlpitime::W`](W) writer structure"]
impl crate::Writable for RXLPITIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXLPITIME to value 0"]
impl crate::Resettable for RXLPITIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
