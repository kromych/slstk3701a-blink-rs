#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `TXC` reader - TXC Interrupt Enable"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TXC Interrupt Enable"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBL` reader - TXBL Interrupt Enable"]
pub type TxblR = crate::BitReader;
#[doc = "Field `TXBL` writer - TXBL Interrupt Enable"]
pub type TxblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAV` reader - RXDATAV Interrupt Enable"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXDATAV` writer - RXDATAV Interrupt Enable"]
pub type RxdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RXFULL Interrupt Enable"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RXFULL Interrupt Enable"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` reader - RXOF Interrupt Enable"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXOF` writer - RXOF Interrupt Enable"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RXUF Interrupt Enable"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - RXUF Interrupt Enable"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TXOF Interrupt Enable"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - TXOF Interrupt Enable"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` reader - TXUF Interrupt Enable"]
pub type TxufR = crate::BitReader;
#[doc = "Field `TXUF` writer - TXUF Interrupt Enable"]
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - PERR Interrupt Enable"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - PERR Interrupt Enable"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - FERR Interrupt Enable"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - FERR Interrupt Enable"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` reader - MPAF Interrupt Enable"]
pub type MpafR = crate::BitReader;
#[doc = "Field `MPAF` writer - MPAF Interrupt Enable"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - SSM Interrupt Enable"]
pub type SsmR = crate::BitReader;
#[doc = "Field `SSM` writer - SSM Interrupt Enable"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` reader - CCF Interrupt Enable"]
pub type CcfR = crate::BitReader;
#[doc = "Field `CCF` writer - CCF Interrupt Enable"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` reader - TXIDLE Interrupt Enable"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` writer - TXIDLE Interrupt Enable"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP0` reader - TCMP0 Interrupt Enable"]
pub type Tcmp0R = crate::BitReader;
#[doc = "Field `TCMP0` writer - TCMP0 Interrupt Enable"]
pub type Tcmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP1` reader - TCMP1 Interrupt Enable"]
pub type Tcmp1R = crate::BitReader;
#[doc = "Field `TCMP1` writer - TCMP1 Interrupt Enable"]
pub type Tcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP2` reader - TCMP2 Interrupt Enable"]
pub type Tcmp2R = crate::BitReader;
#[doc = "Field `TCMP2` writer - TCMP2 Interrupt Enable"]
pub type Tcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXOF Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXUF Interrupt Enable"]
    #[inline(always)]
    pub fn txuf(&self) -> TxufR {
        TxufR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FERR Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPAF Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSM Interrupt Enable"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CCF Interrupt Enable"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXIDLE Interrupt Enable"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TCMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp0(&self) -> Tcmp0R {
        Tcmp0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp1(&self) -> Tcmp1R {
        Tcmp1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TCMP2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp2(&self) -> Tcmp2R {
        Tcmp2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IenSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TxblW<'_, IenSpec> {
        TxblW::new(self, 1)
    }
    #[doc = "Bit 2 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RxdatavW<'_, IenSpec> {
        RxdatavW::new(self, 2)
    }
    #[doc = "Bit 3 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IenSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - RXOF Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IenSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IenSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IenSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - TXUF Interrupt Enable"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<'_, IenSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IenSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - FERR Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IenSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - MPAF Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IenSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - SSM Interrupt Enable"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<'_, IenSpec> {
        SsmW::new(self, 11)
    }
    #[doc = "Bit 12 - CCF Interrupt Enable"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<'_, IenSpec> {
        CcfW::new(self, 12)
    }
    #[doc = "Bit 13 - TXIDLE Interrupt Enable"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TxidleW<'_, IenSpec> {
        TxidleW::new(self, 13)
    }
    #[doc = "Bit 14 - TCMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp0(&mut self) -> Tcmp0W<'_, IenSpec> {
        Tcmp0W::new(self, 14)
    }
    #[doc = "Bit 15 - TCMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp1(&mut self) -> Tcmp1W<'_, IenSpec> {
        Tcmp1W::new(self, 15)
    }
    #[doc = "Bit 16 - TCMP2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp2(&mut self) -> Tcmp2W<'_, IenSpec> {
        Tcmp2W::new(self, 16)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
