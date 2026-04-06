#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `TXC` writer - Set TXC Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` writer - Set RXFULL Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` writer - Set RXOF Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` writer - Set RXUF Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` writer - Set TXOF Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` writer - Set TXUF Interrupt Flag"]
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` writer - Set PERR Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` writer - Set FERR Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` writer - Set MPAF Interrupt Flag"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` writer - Set SSM Interrupt Flag"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` writer - Set CCF Interrupt Flag"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` writer - Set TXIDLE Interrupt Flag"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP0` writer - Set TCMP0 Interrupt Flag"]
pub type Tcmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP1` writer - Set TCMP1 Interrupt Flag"]
pub type Tcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP2` writer - Set TCMP2 Interrupt Flag"]
pub type Tcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TXC Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IfsSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 3 - Set RXFULL Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IfsSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - Set RXOF Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IfsSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - Set RXUF Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IfsSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - Set TXOF Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IfsSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - Set TXUF Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<'_, IfsSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - Set PERR Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IfsSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Set FERR Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IfsSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Set MPAF Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IfsSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - Set SSM Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, IfsSpec> {
        SsmW::new(self, 11)
    }
    #[doc = "Bit 12 - Set CCF Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<'_, IfsSpec> {
        CcfW::new(self, 12)
    }
    #[doc = "Bit 13 - Set TXIDLE Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TxidleW<'_, IfsSpec> {
        TxidleW::new(self, 13)
    }
    #[doc = "Bit 14 - Set TCMP0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&mut self) -> Tcmp0W<'_, IfsSpec> {
        Tcmp0W::new(self, 14)
    }
    #[doc = "Bit 15 - Set TCMP1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&mut self) -> Tcmp1W<'_, IfsSpec> {
        Tcmp1W::new(self, 15)
    }
    #[doc = "Bit 16 - Set TCMP2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&mut self) -> Tcmp2W<'_, IfsSpec> {
        Tcmp2W::new(self, 16)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
