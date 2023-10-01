#[doc = "Register `SINGLECOLS` reader"]
pub type R = crate::R<SINGLECOLS_SPEC>;
#[doc = "Register `SINGLECOLS` writer"]
pub type W = crate::W<SINGLECOLS_SPEC>;
#[doc = "Field `COUNT` reader - Single collision frames"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Single collision frames"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
impl R {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<SINGLECOLS_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Single Collision Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlecols::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlecols::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLECOLS_SPEC;
impl crate::RegisterSpec for SINGLECOLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlecols::R`](R) reader structure"]
impl crate::Readable for SINGLECOLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`singlecols::W`](W) writer structure"]
impl crate::Writable for SINGLECOLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINGLECOLS to value 0"]
impl crate::Resettable for SINGLECOLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
