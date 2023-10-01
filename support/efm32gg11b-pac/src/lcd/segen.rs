#[doc = "Register `SEGEN` reader"]
pub type R = crate::R<SEGEN_SPEC>;
#[doc = "Register `SEGEN` writer"]
pub type W = crate::W<SEGEN_SPEC>;
#[doc = "Field `SEGEN` reader - Segment Enable"]
pub type SEGEN_R = crate::FieldReader<u32>;
#[doc = "Field `SEGEN` writer - Segment Enable"]
pub type SEGEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Segment Enable"]
    #[inline(always)]
    pub fn segen(&self) -> SEGEN_R {
        SEGEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Segment Enable"]
    #[inline(always)]
    #[must_use]
    pub fn segen(&mut self) -> SEGEN_W<SEGEN_SPEC, 0> {
        SEGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGEN_SPEC;
impl crate::RegisterSpec for SEGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segen::R`](R) reader structure"]
impl crate::Readable for SEGEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segen::W`](W) writer structure"]
impl crate::Writable for SEGEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGEN to value 0"]
impl crate::Resettable for SEGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
