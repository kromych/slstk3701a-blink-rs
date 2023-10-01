#[doc = "Register `SEGEN2` reader"]
pub type R = crate::R<SEGEN2_SPEC>;
#[doc = "Register `SEGEN2` writer"]
pub type W = crate::W<SEGEN2_SPEC>;
#[doc = "Field `SEGEN2` reader - Segment Enable (second Group)"]
pub type SEGEN2_R = crate::FieldReader;
#[doc = "Field `SEGEN2` writer - Segment Enable (second Group)"]
pub type SEGEN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&self) -> SEGEN2_R {
        SEGEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Segment Enable (second Group)"]
    #[inline(always)]
    #[must_use]
    pub fn segen2(&mut self) -> SEGEN2_W<SEGEN2_SPEC, 0> {
        SEGEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Enable (32 to 39)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segen2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segen2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGEN2_SPEC;
impl crate::RegisterSpec for SEGEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segen2::R`](R) reader structure"]
impl crate::Readable for SEGEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segen2::W`](W) writer structure"]
impl crate::Writable for SEGEN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGEN2 to value 0"]
impl crate::Resettable for SEGEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
