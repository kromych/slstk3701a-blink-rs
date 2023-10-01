#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `DONE` writer - Clear DONE Interrupt Flag"]
pub type DONE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `ERROR` writer - Clear ERROR Interrupt Flag"]
pub type ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:23 - Clear DONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<IFC_SPEC, 0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 31 - Clear ERROR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<IFC_SPEC, 31> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
