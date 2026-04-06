#[doc = "Register `IENS` writer"]
pub type W = crate::W<IensSpec>;
#[doc = "Field `MNGMNTDONE` writer - Enable management done interrupt"]
pub type MngmntdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCMPLT` writer - Enable receive complete interrupt"]
pub type RxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSEDBITREAD` writer - Enable receive used bit read interrupt"]
pub type RxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUSEDBITREAD` writer - Enable transmit used bit read interrupt"]
pub type TxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUN` writer - Enable transmit buffer under run interrupt"]
pub type TxunderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Enable retry limit exceeded or late collision interrupt"]
pub type RtrylmtorlatecolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMBAERR` writer - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
pub type AmbaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCMPLT` writer - Enable transmit complete interrupt"]
pub type TxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUN` writer - Enable receive overrun interrupt"]
pub type RxoverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` writer - Enable bresp/hresp not OK interrupt"]
pub type RespnotokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Enable pause frame with non-zero pause quantum interrupt"]
pub type NonzeropfrmquantW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSETIMEZERO` writer - Enable pause time zero interrupt"]
pub type PausetimezeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRMTX` writer - Enable pause frame transmitted interrupt"]
pub type PfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMRX` writer - Enable PTP delay_req frame received interrupt"]
pub type PtpdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMRX` writer - Enable PTP sync frame received interrupt"]
pub type PtpsyncfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMTX` writer - Enable PTP delay_req frame transmitted interrupt"]
pub type PtpdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMTX` writer - Enable PTP sync frame transmitted interrupt"]
pub type PtpsyncfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - Enable PTP pdelay_req frame received interrupt"]
pub type PtppdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - Enable PTP pdelay_resp frame received interrupt"]
pub type PtppdlyrespfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - Enable PTP pdelay_req frame transmitted interrupt"]
pub type PtppdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - Enable PTP pdelay_resp frame transmitted interrupt"]
pub type PtppdlyrespfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUSECREGINCR` writer - Enable TSU seconds register increment interrupt"]
pub type TsusecregincrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIINDC` writer - Enable RX LPI indication interrupt"]
pub type RxlpiindcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLEVNTRX` writer - Enable WOL event received interrupt"]
pub type WolevntrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUTIMERCOMP` writer - Enable TSU timer comparison interrupt."]
pub type TsutimercompW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable management done interrupt"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MngmntdoneW<'_, IensSpec> {
        MngmntdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable receive complete interrupt"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RxcmpltW<'_, IensSpec> {
        RxcmpltW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable receive used bit read interrupt"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RxusedbitreadW<'_, IensSpec> {
        RxusedbitreadW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable transmit used bit read interrupt"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TxusedbitreadW<'_, IensSpec> {
        TxusedbitreadW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable transmit buffer under run interrupt"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TxunderrunW<'_, IensSpec> {
        TxunderrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RtrylmtorlatecolW<'_, IensSpec> {
        RtrylmtorlatecolW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AmbaerrW<'_, IensSpec> {
        AmbaerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable transmit complete interrupt"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TxcmpltW<'_, IensSpec> {
        TxcmpltW::new(self, 7)
    }
    #[doc = "Bit 10 - Enable receive overrun interrupt"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RxoverrunW<'_, IensSpec> {
        RxoverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable bresp/hresp not OK interrupt"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RespnotokW<'_, IensSpec> {
        RespnotokW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NonzeropfrmquantW<'_, IensSpec> {
        NonzeropfrmquantW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable pause time zero interrupt"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PausetimezeroW<'_, IensSpec> {
        PausetimezeroW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable pause frame transmitted interrupt"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PfrmtxW<'_, IensSpec> {
        PfrmtxW::new(self, 14)
    }
    #[doc = "Bit 18 - Enable PTP delay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PtpdlyreqfrmrxW<'_, IensSpec> {
        PtpdlyreqfrmrxW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable PTP sync frame received interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PtpsyncfrmrxW<'_, IensSpec> {
        PtpsyncfrmrxW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PtpdlyreqfrmtxW<'_, IensSpec> {
        PtpdlyreqfrmtxW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PtpsyncfrmtxW<'_, IensSpec> {
        PtpsyncfrmtxW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PtppdlyreqfrmrxW<'_, IensSpec> {
        PtppdlyreqfrmrxW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PtppdlyrespfrmrxW<'_, IensSpec> {
        PtppdlyrespfrmrxW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PtppdlyreqfrmtxW<'_, IensSpec> {
        PtppdlyreqfrmtxW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PtppdlyrespfrmtxW<'_, IensSpec> {
        PtppdlyrespfrmtxW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable TSU seconds register increment interrupt"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TsusecregincrW<'_, IensSpec> {
        TsusecregincrW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable RX LPI indication interrupt"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RxlpiindcW<'_, IensSpec> {
        RxlpiindcW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable WOL event received interrupt"]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WolevntrxW<'_, IensSpec> {
        WolevntrxW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TsutimercompW<'_, IensSpec> {
        TsutimercompW::new(self, 29)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iens::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IensSpec;
impl crate::RegisterSpec for IensSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iens::W`](W) writer structure"]
impl crate::Writable for IensSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IENS to value 0"]
impl crate::Resettable for IensSpec {}
