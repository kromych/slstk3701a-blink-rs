#[doc = "Register `PFRAMESTXED` reader"]
pub type R = crate::R<PFRAMESTXED_SPEC>;
#[doc = "Register `PFRAMESTXED` writer"]
pub type W = crate::W<PFRAMESTXED_SPEC>;
#[doc = "Field `COUNT` reader - Transmitted pause frames"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Transmitted pause frames"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted pause frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitted pause frames"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<PFRAMESTXED_SPEC> {
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
#[doc = "Pause Frames Transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pframestxed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pframestxed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRAMESTXED_SPEC;
impl crate::RegisterSpec for PFRAMESTXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pframestxed::R`](R) reader structure"]
impl crate::Readable for PFRAMESTXED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pframestxed::W`](W) writer structure"]
impl crate::Writable for PFRAMESTXED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRAMESTXED to value 0"]
impl crate::Resettable for PFRAMESTXED_SPEC {
    const RESET_VALUE: u32 = 0;
}
