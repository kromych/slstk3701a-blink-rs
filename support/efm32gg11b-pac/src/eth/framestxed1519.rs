#[doc = "Register `FRAMESTXED1519` reader"]
pub type R = crate::R<FRAMESTXED1519_SPEC>;
#[doc = "Register `FRAMESTXED1519` writer"]
pub type W = crate::W<FRAMESTXED1519_SPEC>;
#[doc = "Field `COUNT` reader - Greater than 1518 byte frames transmitted without error"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Greater than 1518 byte frames transmitted without error"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 byte frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Greater than 1518 byte frames transmitted without error"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<FRAMESTXED1519_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Greater Than 1518 Byte Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`framestxed1519::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`framestxed1519::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAMESTXED1519_SPEC;
impl crate::RegisterSpec for FRAMESTXED1519_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxed1519::R`](R) reader structure"]
impl crate::Readable for FRAMESTXED1519_SPEC {}
#[doc = "`write(|w| ..)` method takes [`framestxed1519::W`](W) writer structure"]
impl crate::Writable for FRAMESTXED1519_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMESTXED1519 to value 0"]
impl crate::Resettable for FRAMESTXED1519_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
