#[doc = "Register `OCTETSTXEDBOTTOM` reader"]
pub type R = crate::R<OCTETSTXEDBOTTOM_SPEC>;
#[doc = "Register `OCTETSTXEDBOTTOM` writer"]
pub type W = crate::W<OCTETSTXEDBOTTOM_SPEC>;
#[doc = "Field `COUNT` reader - Transmitted octets in frame without errors \\[31:0\\]"]
pub type COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Transmitted octets in frame without errors \\[31:0\\]"]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<OCTETSTXEDBOTTOM_SPEC, 0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Octets transmitted 31:0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octetstxedbottom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octetstxedbottom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCTETSTXEDBOTTOM_SPEC;
impl crate::RegisterSpec for OCTETSTXEDBOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octetstxedbottom::R`](R) reader structure"]
impl crate::Readable for OCTETSTXEDBOTTOM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`octetstxedbottom::W`](W) writer structure"]
impl crate::Writable for OCTETSTXEDBOTTOM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTETSTXEDBOTTOM to value 0"]
impl crate::Resettable for OCTETSTXEDBOTTOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
