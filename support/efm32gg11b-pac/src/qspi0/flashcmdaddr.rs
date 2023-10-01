#[doc = "Register `FLASHCMDADDR` reader"]
pub type R = crate::R<FLASHCMDADDR_SPEC>;
#[doc = "Register `FLASHCMDADDR` writer"]
pub type W = crate::W<FLASHCMDADDR_SPEC>;
#[doc = "Field `ADDR` reader - Command Address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Command Address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<FLASHCMDADDR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash Command Address Register (STIG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcmdaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcmdaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHCMDADDR_SPEC;
impl crate::RegisterSpec for FLASHCMDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdaddr::R`](R) reader structure"]
impl crate::Readable for FLASHCMDADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flashcmdaddr::W`](W) writer structure"]
impl crate::Writable for FLASHCMDADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHCMDADDR to value 0"]
impl crate::Resettable for FLASHCMDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
