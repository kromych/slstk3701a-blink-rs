#[doc = "Register `INDIRECTWRITEXFERNUMBYTES` reader"]
pub type R = crate::R<INDIRECTWRITEXFERNUMBYTES_SPEC>;
#[doc = "Register `INDIRECTWRITEXFERNUMBYTES` writer"]
pub type W = crate::W<INDIRECTWRITEXFERNUMBYTES_SPEC>;
#[doc = "Field `VALUE` reader - Indirect Number of Bytes"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Indirect Number of Bytes"]
pub type VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Indirect Number of Bytes"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Number of Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<INDIRECTWRITEXFERNUMBYTES_SPEC, 0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indirect Write Transfer Number Bytes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirectwritexfernumbytes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirectwritexfernumbytes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECTWRITEXFERNUMBYTES_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERNUMBYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexfernumbytes::R`](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERNUMBYTES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexfernumbytes::W`](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERNUMBYTES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERNUMBYTES to value 0"]
impl crate::Resettable for INDIRECTWRITEXFERNUMBYTES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
