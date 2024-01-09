#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Set RXFULL Interrupt Flag"]
pub type RXFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Set RXOF Interrupt Flag"]
pub type RXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` writer - Set TXUF Interrupt Flag"]
pub type TXUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Set PERR Interrupt Flag"]
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Set FERR Interrupt Flag"]
pub type FERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Set MPAF Interrupt Flag"]
pub type MPAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` writer - Set SSM Interrupt Flag"]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` writer - Set CCF Interrupt Flag"]
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` writer - Set TXIDLE Interrupt Flag"]
pub type TXIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP0` writer - Set TCMP0 Interrupt Flag"]
pub type TCMP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP1` writer - Set TCMP1 Interrupt Flag"]
pub type TCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP2` writer - Set TCMP2 Interrupt Flag"]
pub type TCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TXC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFS_SPEC> {
        TXC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Set RXFULL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IFS_SPEC> {
        RXFULL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set RXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<IFS_SPEC> {
        RXOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFS_SPEC> {
        RXUF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFS_SPEC> {
        TXOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set TXUF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TXUF_W<IFS_SPEC> {
        TXUF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set PERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<IFS_SPEC> {
        PERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set FERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IFS_SPEC> {
        FERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set MPAF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<IFS_SPEC> {
        MPAF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set SSM Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<IFS_SPEC> {
        SSM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set CCF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<IFS_SPEC> {
        CCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set TXIDLE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TXIDLE_W<IFS_SPEC> {
        TXIDLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set TCMP0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> TCMP0_W<IFS_SPEC> {
        TCMP0_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set TCMP1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> TCMP1_W<IFS_SPEC> {
        TCMP1_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set TCMP2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> TCMP2_W<IFS_SPEC> {
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
