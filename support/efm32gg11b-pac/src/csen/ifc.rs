#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `CMP` writer - Clear CMP Interrupt Flag"]
pub type CMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV` writer - Clear CONV Interrupt Flag"]
pub type CONV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` writer - Clear EOS Interrupt Flag"]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOF` writer - Clear DMAOF Interrupt Flag"]
pub type DMAOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Clear APORTCONFLICT Interrupt Flag"]
pub type APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear CMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<IFC_SPEC> {
        CMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CONV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn conv(&mut self) -> CONV_W<IFC_SPEC> {
        CONV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear EOS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<IFC_SPEC> {
        EOS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear DMAOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaof(&mut self) -> DMAOF_W<IFC_SPEC> {
        DMAOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W<IFC_SPEC> {
        APORTCONFLICT_W::new(self, 4)
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
#[doc = "Interrupt Flag Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
