#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `TXC` writer - Clear TXC Interrupt Flag"]
pub type TXC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOF` writer - Clear RXOF Interrupt Flag"]
pub type RXOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUF` writer - Clear RXUF Interrupt Flag"]
pub type RXUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOF` writer - Clear TXOF Interrupt Flag"]
pub type TXOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERR` writer - Clear PERR Interrupt Flag"]
pub type PERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FERR` writer - Clear FERR Interrupt Flag"]
pub type FERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPAF` writer - Clear MPAF Interrupt Flag"]
pub type MPAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTF` writer - Clear STARTF Interrupt Flag"]
pub type STARTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SIGF` writer - Clear SIGF Interrupt Flag"]
pub type SIGF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFC_SPEC, 0> {
        TXC_W::new(self)
    }
    #[doc = "Bit 3 - Clear RXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<IFC_SPEC, 3> {
        RXOF_W::new(self)
    }
    #[doc = "Bit 4 - Clear RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFC_SPEC, 4> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 5 - Clear TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFC_SPEC, 5> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 6 - Clear PERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<IFC_SPEC, 6> {
        PERR_W::new(self)
    }
    #[doc = "Bit 7 - Clear FERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IFC_SPEC, 7> {
        FERR_W::new(self)
    }
    #[doc = "Bit 8 - Clear MPAF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<IFC_SPEC, 8> {
        MPAF_W::new(self)
    }
    #[doc = "Bit 9 - Clear STARTF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn startf(&mut self) -> STARTF_W<IFC_SPEC, 9> {
        STARTF_W::new(self)
    }
    #[doc = "Bit 10 - Clear SIGF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sigf(&mut self) -> SIGF_W<IFC_SPEC, 10> {
        SIGF_W::new(self)
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
