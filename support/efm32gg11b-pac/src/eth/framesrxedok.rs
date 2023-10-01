#[doc = "Register `FRAMESRXEDOK` reader"]
pub type R = crate::R<FRAMESRXEDOK_SPEC>;
#[doc = "Register `FRAMESRXEDOK` writer"]
pub type W = crate::W<FRAMESRXEDOK_SPEC>;
#[doc = "Field `COUNT` reader - Frames received without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Frames received without error"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frames received without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<FRAMESRXEDOK_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxedok::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxedok::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMESRXEDOK_SPEC;
impl crate::RegisterSpec for FRAMESRXEDOK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxedok::R`](R) reader structure"]
impl crate::Readable for FRAMESRXEDOK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framesrxedok::W`](W) writer structure"]
impl crate::Writable for FRAMESRXEDOK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMESRXEDOK to value 0"]
impl crate::Resettable for FRAMESRXEDOK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
