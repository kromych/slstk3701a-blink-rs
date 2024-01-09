#[doc = "Register `REQCLEAR` writer"]
pub type W = crate::W<REQCLEAR_SPEC>;
#[doc = "Field `REQCLEAR` writer - DMA Request Clear"]
pub type REQCLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - DMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn reqclear(&mut self) -> REQCLEAR_W<REQCLEAR_SPEC> {
        REQCLEAR_W::new(self, 0)
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
#[doc = "DMA Channel Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQCLEAR_SPEC;
impl crate::RegisterSpec for REQCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reqclear::W`](W) writer structure"]
impl crate::Writable for REQCLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQCLEAR to value 0"]
impl crate::Resettable for REQCLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
