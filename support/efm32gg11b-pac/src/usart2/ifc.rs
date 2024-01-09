#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `TXC` writer - Clear TXC Interrupt Flag"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Clear RXFULL Interrupt Flag"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Clear RXOF Interrupt Flag"]
pub type RXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Clear RXUF Interrupt Flag"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Clear TXOF Interrupt Flag"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` writer - Clear TXUF Interrupt Flag"]
pub type TXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Clear PERR Interrupt Flag"]
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Clear FERR Interrupt Flag"]
pub type FERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Clear MPAF Interrupt Flag"]
pub type MPAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` writer - Clear SSM Interrupt Flag"]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` writer - Clear CCF Interrupt Flag"]
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` writer - Clear TXIDLE Interrupt Flag"]
pub type TXIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP0` writer - Clear TCMP0 Interrupt Flag"]
pub type TCMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP1` writer - Clear TCMP1 Interrupt Flag"]
pub type TCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP2` writer - Clear TCMP2 Interrupt Flag"]
pub type TCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFC_SPEC> {
        TXC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Clear RXFULL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IFC_SPEC> {
        RXFULL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear RXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<IFC_SPEC> {
        RXOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFC_SPEC> {
        RXUF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFC_SPEC> {
        TXOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear TXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TXUF_W<IFC_SPEC> {
        TXUF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear PERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<IFC_SPEC> {
        PERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear FERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IFC_SPEC> {
        FERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear MPAF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<IFC_SPEC> {
        MPAF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear SSM Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<IFC_SPEC> {
        SSM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear CCF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<IFC_SPEC> {
        CCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear TXIDLE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TXIDLE_W<IFC_SPEC> {
        TXIDLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear TCMP0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> TCMP0_W<IFC_SPEC> {
        TCMP0_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear TCMP1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> TCMP1_W<IFC_SPEC> {
        TCMP1_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear TCMP2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> TCMP2_W<IFC_SPEC> {
        TCMP2_W::new(self, 16)
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
