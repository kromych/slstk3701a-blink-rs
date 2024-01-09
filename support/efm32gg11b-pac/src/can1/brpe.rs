#[doc = "Register `BRPE` reader"]
pub type R = crate::R<BRPE_SPEC>;
#[doc = "Register `BRPE` writer"]
pub type W = crate::W<BRPE_SPEC>;
#[doc = "Field `BRPE` reader - Baud Rate Prescaler Extension"]
pub type BRPE_R = crate::FieldReader;
#[doc = "Field `BRPE` writer - Baud Rate Prescaler Extension"]
pub type BRPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Baud Rate Prescaler Extension"]
    #[inline(always)]
    pub fn brpe(&self) -> BRPE_R {
        BRPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Prescaler Extension"]
    #[inline(always)]
    #[must_use]
    pub fn brpe(&mut self) -> BRPE_W<BRPE_SPEC> {
        BRPE_W::new(self, 0)
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
#[doc = "BRP Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brpe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brpe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRPE_SPEC;
impl crate::RegisterSpec for BRPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brpe::R`](R) reader structure"]
impl crate::Readable for BRPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brpe::W`](W) writer structure"]
impl crate::Writable for BRPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRPE to value 0"]
impl crate::Resettable for BRPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
