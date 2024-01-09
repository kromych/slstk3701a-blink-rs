#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `EXT` writer - Clear EXT Interrupt Flag"]
pub type EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM4WU` writer - Clear EM4WU Interrupt Flag"]
pub type EM4WU_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Clear EXT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> EXT_W<IFC_SPEC> {
        EXT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Clear EM4WU Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> EM4WU_W<IFC_SPEC> {
        EM4WU_W::new(self, 16)
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
