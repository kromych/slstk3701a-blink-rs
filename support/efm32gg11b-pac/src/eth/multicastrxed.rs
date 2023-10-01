#[doc = "Register `MULTICASTRXED` reader"]
pub type R = crate::R<MULTICASTRXED_SPEC>;
#[doc = "Register `MULTICASTRXED` writer"]
pub type W = crate::W<MULTICASTRXED_SPEC>;
#[doc = "Field `COUNT` reader - Multicast frames received without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Multicast frames received without error"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast frames received without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<MULTICASTRXED_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Multicast Frames Received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`multicastrxed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`multicastrxed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULTICASTRXED_SPEC;
impl crate::RegisterSpec for MULTICASTRXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`multicastrxed::R`](R) reader structure"]
impl crate::Readable for MULTICASTRXED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`multicastrxed::W`](W) writer structure"]
impl crate::Writable for MULTICASTRXED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULTICASTRXED to value 0"]
impl crate::Resettable for MULTICASTRXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
