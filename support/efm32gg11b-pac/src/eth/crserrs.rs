#[doc = "Register `CRSERRS` reader"]
pub type R = crate::R<CRSERRS_SPEC>;
#[doc = "Register `CRSERRS` writer"]
pub type W = crate::W<CRSERRS_SPEC>;
#[doc = "Field `COUNT` reader - Carrier sense errors"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Carrier sense errors"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Carrier sense errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Carrier sense errors"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<CRSERRS_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Carrier Sense Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crserrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crserrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRSERRS_SPEC;
impl crate::RegisterSpec for CRSERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crserrs::R`](R) reader structure"]
impl crate::Readable for CRSERRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crserrs::W`](W) writer structure"]
impl crate::Writable for CRSERRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRSERRS to value 0"]
impl crate::Resettable for CRSERRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
