#[doc = "Register `OCTETSTXEDTOP` reader"]
pub type R = crate::R<OCTETSTXEDTOP_SPEC>;
#[doc = "Register `OCTETSTXEDTOP` writer"]
pub type W = crate::W<OCTETSTXEDTOP_SPEC>;
#[doc = "Field `COUNT` reader - Transmitted octets in frame without errors \\[47:32\\]"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Transmitted octets in frame without errors \\[47:32\\]"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<OCTETSTXEDTOP_SPEC> {
        COUNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Octets Transmitted 47:32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octetstxedtop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octetstxedtop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCTETSTXEDTOP_SPEC;
impl crate::RegisterSpec for OCTETSTXEDTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octetstxedtop::R`](R) reader structure"]
impl crate::Readable for OCTETSTXEDTOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`octetstxedtop::W`](W) writer structure"]
impl crate::Writable for OCTETSTXEDTOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCTETSTXEDTOP to value 0"]
impl crate::Resettable for OCTETSTXEDTOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
