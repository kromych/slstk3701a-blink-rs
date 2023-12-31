#[doc = "Register `FRAMESRXED64` reader"]
pub type R = crate::R<FRAMESRXED64_SPEC>;
#[doc = "Register `FRAMESRXED64` writer"]
pub type W = crate::W<FRAMESRXED64_SPEC>;
#[doc = "Field `COUNT` reader - 64 byte frames received without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 64 byte frames received without error"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 64 byte frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 64 byte frames received without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<FRAMESRXED64_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "64 Byte Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framesrxed64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framesrxed64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMESRXED64_SPEC;
impl crate::RegisterSpec for FRAMESRXED64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framesrxed64::R`](R) reader structure"]
impl crate::Readable for FRAMESRXED64_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framesrxed64::W`](W) writer structure"]
impl crate::Writable for FRAMESRXED64_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMESRXED64 to value 0"]
impl crate::Resettable for FRAMESRXED64_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
