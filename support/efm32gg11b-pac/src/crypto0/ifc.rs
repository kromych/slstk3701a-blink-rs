#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `INSTRDONE` writer - Clear INSTRDONE Interrupt Flag"]
pub type INSTRDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEQDONE` writer - Clear SEQDONE Interrupt Flag"]
pub type SEQDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear INSTRDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn instrdone(&mut self) -> INSTRDONE_W<IFC_SPEC, 0> {
        INSTRDONE_W::new(self)
    }
    #[doc = "Bit 1 - Clear SEQDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn seqdone(&mut self) -> SEQDONE_W<IFC_SPEC, 1> {
        SEQDONE_W::new(self)
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
