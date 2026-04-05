#[doc = "Register `IENC` writer"]
pub type W = crate::W<IencSpec>;
#[doc = "Field `MNGMNTDONE` writer - Disable management done interrupt"]
pub type MngmntdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCMPLT` writer - Disable receive complete interrupt"]
pub type RxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSEDBITREAD` writer - Disable receive used bit read interrupt"]
pub type RxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUSEDBITREAD` writer - Disable transmit used bit read interrupt"]
pub type TxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUN` writer - Disable transmit buffer under run interrupt"]
pub type TxunderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Disable retry limit exceeded or late collision interrupt"]
pub type RtrylmtorlatecolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMBAERR` writer - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
pub type AmbaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCMPLT` writer - Disable transmit complete interrupt"]
pub type TxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUN` writer - Disable receive overrun interrupt"]
pub type RxoverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` writer - Disable bresp/hresp not OK interrupt"]
pub type RespnotokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Disable pause frame with non-zero pause quantum interrupt"]
pub type NonzeropfrmquantW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSETIMEZERO` writer - Disable pause time zero interrupt"]
pub type PausetimezeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRMTX` writer - Disable pause frame transmitted interrupt"]
pub type PfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMRX` writer - Disable PTP delay_req frame received interrupt"]
pub type PtpdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMRX` writer - Disable PTP sync frame received interrupt"]
pub type PtpsyncfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMTX` writer - Disable PTP delay_req frame transmitted interrupt"]
pub type PtpdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMTX` writer - Disable PTP sync frame transmitted interrupt"]
pub type PtpsyncfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - Disable PTP pdelay_req frame received interrupt"]
pub type PtppdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - Disable PTP pdelay_resp frame received interrupt"]
pub type PtppdlyrespfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - Disable PTP pdelay_req frame transmitted interrupt"]
pub type PtppdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - Disable PTP pdelay_resp frame transmitted interrupt"]
pub type PtppdlyrespfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUSECREGINCR` writer - Disable TSU seconds register increment interrupt"]
pub type TsusecregincrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIINDC` writer - Disable RX LPI indication interrupt"]
pub type RxlpiindcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLEVNTRX` writer - Disable WOL event received interrupt"]
pub type WolevntrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUTIMERCOMP` writer - Disable TSU timer comparison interrupt."]
pub type TsutimercompW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable management done interrupt"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MngmntdoneW<'_, IencSpec> {
        MngmntdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable receive complete interrupt"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RxcmpltW<'_, IencSpec> {
        RxcmpltW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable receive used bit read interrupt"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RxusedbitreadW<'_, IencSpec> {
        RxusedbitreadW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable transmit used bit read interrupt"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TxusedbitreadW<'_, IencSpec> {
        TxusedbitreadW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable transmit buffer under run interrupt"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TxunderrunW<'_, IencSpec> {
        TxunderrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RtrylmtorlatecolW<'_, IencSpec> {
        RtrylmtorlatecolW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AmbaerrW<'_, IencSpec> {
        AmbaerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable transmit complete interrupt"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TxcmpltW<'_, IencSpec> {
        TxcmpltW::new(self, 7)
    }
    #[doc = "Bit 10 - Disable receive overrun interrupt"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RxoverrunW<'_, IencSpec> {
        RxoverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable bresp/hresp not OK interrupt"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RespnotokW<'_, IencSpec> {
        RespnotokW::new(self, 11)
    }
    #[doc = "Bit 12 - Disable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NonzeropfrmquantW<'_, IencSpec> {
        NonzeropfrmquantW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable pause time zero interrupt"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PausetimezeroW<'_, IencSpec> {
        PausetimezeroW::new(self, 13)
    }
    #[doc = "Bit 14 - Disable pause frame transmitted interrupt"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PfrmtxW<'_, IencSpec> {
        PfrmtxW::new(self, 14)
    }
    #[doc = "Bit 18 - Disable PTP delay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PtpdlyreqfrmrxW<'_, IencSpec> {
        PtpdlyreqfrmrxW::new(self, 18)
    }
    #[doc = "Bit 19 - Disable PTP sync frame received interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PtpsyncfrmrxW<'_, IencSpec> {
        PtpsyncfrmrxW::new(self, 19)
    }
    #[doc = "Bit 20 - Disable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PtpdlyreqfrmtxW<'_, IencSpec> {
        PtpdlyreqfrmtxW::new(self, 20)
    }
    #[doc = "Bit 21 - Disable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PtpsyncfrmtxW<'_, IencSpec> {
        PtpsyncfrmtxW::new(self, 21)
    }
    #[doc = "Bit 22 - Disable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PtppdlyreqfrmrxW<'_, IencSpec> {
        PtppdlyreqfrmrxW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PtppdlyrespfrmrxW<'_, IencSpec> {
        PtppdlyrespfrmrxW::new(self, 23)
    }
    #[doc = "Bit 24 - Disable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PtppdlyreqfrmtxW<'_, IencSpec> {
        PtppdlyreqfrmtxW::new(self, 24)
    }
    #[doc = "Bit 25 - Disable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PtppdlyrespfrmtxW<'_, IencSpec> {
        PtppdlyrespfrmtxW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable TSU seconds register increment interrupt"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TsusecregincrW<'_, IencSpec> {
        TsusecregincrW::new(self, 26)
    }
    #[doc = "Bit 27 - Disable RX LPI indication interrupt"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RxlpiindcW<'_, IencSpec> {
        RxlpiindcW::new(self, 27)
    }
    #[doc = "Bit 28 - Disable WOL event received interrupt"]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WolevntrxW<'_, IencSpec> {
        WolevntrxW::new(self, 28)
    }
    #[doc = "Bit 29 - Disable TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TsutimercompW<'_, IencSpec> {
        TsutimercompW::new(self, 29)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IencSpec;
impl crate::RegisterSpec for IencSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ienc::W`](W) writer structure"]
impl crate::Writable for IencSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IENC to value 0"]
impl crate::Resettable for IencSpec {}
