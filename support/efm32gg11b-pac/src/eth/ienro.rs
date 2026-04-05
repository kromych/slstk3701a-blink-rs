#[doc = "Register `IENRO` reader"]
pub type R = crate::R<IenroSpec>;
#[doc = "Register `IENRO` writer"]
pub type W = crate::W<IenroSpec>;
#[doc = "Field `MNGMNTDONE` reader - management done interrupt mask"]
pub type MngmntdoneR = crate::BitReader;
#[doc = "Field `MNGMNTDONE` writer - management done interrupt mask"]
pub type MngmntdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCMPLT` reader - receive complete interrupt mask"]
pub type RxcmpltR = crate::BitReader;
#[doc = "Field `RXCMPLT` writer - receive complete interrupt mask"]
pub type RxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSEDBITREAD` reader - receive used bit read interrupt mask"]
pub type RxusedbitreadR = crate::BitReader;
#[doc = "Field `RXUSEDBITREAD` writer - receive used bit read interrupt mask"]
pub type RxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUSEDBITREAD` reader - transmit used bit read interrupt mask"]
pub type TxusedbitreadR = crate::BitReader;
#[doc = "Field `TXUSEDBITREAD` writer - transmit used bit read interrupt mask"]
pub type TxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUN` reader - transmit buffer under run interrupt mask"]
pub type TxunderrunR = crate::BitReader;
#[doc = "Field `TXUNDERRUN` writer - transmit buffer under run interrupt mask"]
pub type TxunderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRYLMTORLATECOL` reader - Retry limit exceeded or late collision (gigabit mode only) interrupt mask"]
pub type RtrylmtorlatecolR = crate::BitReader;
#[doc = "Field `RTRYLMTORLATECOL` writer - Retry limit exceeded or late collision (gigabit mode only) interrupt mask"]
pub type RtrylmtorlatecolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMBAERR` reader - Transmit frame corruption due to AMBA (AHB) error interrupt mask"]
pub type AmbaerrR = crate::BitReader;
#[doc = "Field `AMBAERR` writer - Transmit frame corruption due to AMBA (AHB) error interrupt mask"]
pub type AmbaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCMPLT` reader - Transmit complete interrupt mask"]
pub type TxcmpltR = crate::BitReader;
#[doc = "Field `TXCMPLT` writer - Transmit complete interrupt mask"]
pub type TxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNUSED` reader - Unused"]
pub type UnusedR = crate::BitReader;
#[doc = "Field `UNUSED` writer - Unused"]
pub type UnusedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUN` reader - Receive overrun interrupt mask"]
pub type RxoverrunR = crate::BitReader;
#[doc = "Field `RXOVERRUN` writer - Receive overrun interrupt mask"]
pub type RxoverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK interrupt mask"]
pub type RespnotokR = crate::BitReader;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK interrupt mask"]
pub type RespnotokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONZEROPFRMQUANT` reader - Pause frame with non-zero pause quantum interrupt mask"]
pub type NonzeropfrmquantR = crate::BitReader;
#[doc = "Field `NONZEROPFRMQUANT` writer - Pause frame with non-zero pause quantum interrupt mask"]
pub type NonzeropfrmquantW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSETIMEZERO` reader - pause time zero interrupt mask"]
pub type PausetimezeroR = crate::BitReader;
#[doc = "Field `PAUSETIMEZERO` writer - pause time zero interrupt mask"]
pub type PausetimezeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRMTX` reader - pause frame transmitted interrupt mask"]
pub type PfrmtxR = crate::BitReader;
#[doc = "Field `PFRMTX` writer - pause frame transmitted interrupt mask"]
pub type PfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMRX` reader - PTP delay_req frame received mask"]
pub type PtpdlyreqfrmrxR = crate::BitReader;
#[doc = "Field `PTPDLYREQFRMRX` writer - PTP delay_req frame received mask"]
pub type PtpdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMRX` reader - PTP sync frame received mask"]
pub type PtpsyncfrmrxR = crate::BitReader;
#[doc = "Field `PTPSYNCFRMRX` writer - PTP sync frame received mask"]
pub type PtpsyncfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMTX` reader - PTP delay_req frame transmitted mask"]
pub type PtpdlyreqfrmtxR = crate::BitReader;
#[doc = "Field `PTPDLYREQFRMTX` writer - PTP delay_req frame transmitted mask"]
pub type PtpdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMTX` reader - PTP sync frame transmitted mask"]
pub type PtpsyncfrmtxR = crate::BitReader;
#[doc = "Field `PTPSYNCFRMTX` writer - PTP sync frame transmitted mask"]
pub type PtpsyncfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMRX` reader - PTP pdelay_req frame received mask"]
pub type PtppdlyreqfrmrxR = crate::BitReader;
#[doc = "Field `PTPPDLYREQFRMRX` writer - PTP pdelay_req frame received mask"]
pub type PtppdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMRX` reader - PTP pdelay_resp frame received mask"]
pub type PtppdlyrespfrmrxR = crate::BitReader;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - PTP pdelay_resp frame received mask"]
pub type PtppdlyrespfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMTX` reader - PTP pdelay_req frame transmitted mask"]
pub type PtppdlyreqfrmtxR = crate::BitReader;
#[doc = "Field `PTPPDLYREQFRMTX` writer - PTP pdelay_req frame transmitted mask"]
pub type PtppdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMTX` reader - PTP pdelay_resp frame transmitted mask"]
pub type PtppdlyrespfrmtxR = crate::BitReader;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - PTP pdelay_resp frame transmitted mask"]
pub type PtppdlyrespfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUSECREGINCR` reader - TSU seconds register increment mask"]
pub type TsusecregincrR = crate::BitReader;
#[doc = "Field `TSUSECREGINCR` writer - TSU seconds register increment mask"]
pub type TsusecregincrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIINDC` reader - RX LPI indication mask"]
pub type RxlpiindcR = crate::BitReader;
#[doc = "Field `RXLPIINDC` writer - RX LPI indication mask"]
pub type RxlpiindcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLEVNTRX` reader - WOL event received mask"]
pub type WolevntrxR = crate::BitReader;
#[doc = "Field `WOLEVNTRX` writer - WOL event received mask"]
pub type WolevntrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUTIMERCOMP` reader - TSU timer comparison interrupt mask."]
pub type TsutimercompR = crate::BitReader;
#[doc = "Field `TSUTIMERCOMP` writer - TSU timer comparison interrupt mask."]
pub type TsutimercompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - management done interrupt mask"]
    #[inline(always)]
    pub fn mngmntdone(&self) -> MngmntdoneR {
        MngmntdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - receive complete interrupt mask"]
    #[inline(always)]
    pub fn rxcmplt(&self) -> RxcmpltR {
        RxcmpltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - receive used bit read interrupt mask"]
    #[inline(always)]
    pub fn rxusedbitread(&self) -> RxusedbitreadR {
        RxusedbitreadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transmit used bit read interrupt mask"]
    #[inline(always)]
    pub fn txusedbitread(&self) -> TxusedbitreadR {
        TxusedbitreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - transmit buffer under run interrupt mask"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TxunderrunR {
        TxunderrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision (gigabit mode only) interrupt mask"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&self) -> RtrylmtorlatecolR {
        RtrylmtorlatecolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error interrupt mask"]
    #[inline(always)]
    pub fn ambaerr(&self) -> AmbaerrR {
        AmbaerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete interrupt mask"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TxcmpltR {
        TxcmpltR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Unused"]
    #[inline(always)]
    pub fn unused(&self) -> UnusedR {
        UnusedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RxoverrunR {
        RxoverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - bresp/hresp not OK interrupt mask"]
    #[inline(always)]
    pub fn respnotok(&self) -> RespnotokR {
        RespnotokR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum interrupt mask"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&self) -> NonzeropfrmquantR {
        NonzeropfrmquantR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pause time zero interrupt mask"]
    #[inline(always)]
    pub fn pausetimezero(&self) -> PausetimezeroR {
        PausetimezeroR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pause frame transmitted interrupt mask"]
    #[inline(always)]
    pub fn pfrmtx(&self) -> PfrmtxR {
        PfrmtxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP delay_req frame received mask"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&self) -> PtpdlyreqfrmrxR {
        PtpdlyreqfrmrxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP sync frame received mask"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&self) -> PtpsyncfrmrxR {
        PtpsyncfrmrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted mask"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&self) -> PtpdlyreqfrmtxR {
        PtpdlyreqfrmtxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted mask"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&self) -> PtpsyncfrmtxR {
        PtpsyncfrmtxR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received mask"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&self) -> PtppdlyreqfrmrxR {
        PtppdlyreqfrmrxR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received mask"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&self) -> PtppdlyrespfrmrxR {
        PtppdlyrespfrmrxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted mask"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&self) -> PtppdlyreqfrmtxR {
        PtppdlyreqfrmtxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted mask"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&self) -> PtppdlyrespfrmtxR {
        PtppdlyrespfrmtxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSU seconds register increment mask"]
    #[inline(always)]
    pub fn tsusecregincr(&self) -> TsusecregincrR {
        TsusecregincrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RX LPI indication mask"]
    #[inline(always)]
    pub fn rxlpiindc(&self) -> RxlpiindcR {
        RxlpiindcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WOL event received mask"]
    #[inline(always)]
    pub fn wolevntrx(&self) -> WolevntrxR {
        WolevntrxR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt mask."]
    #[inline(always)]
    pub fn tsutimercomp(&self) -> TsutimercompR {
        TsutimercompR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - management done interrupt mask"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MngmntdoneW<'_, IenroSpec> {
        MngmntdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - receive complete interrupt mask"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RxcmpltW<'_, IenroSpec> {
        RxcmpltW::new(self, 1)
    }
    #[doc = "Bit 2 - receive used bit read interrupt mask"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RxusedbitreadW<'_, IenroSpec> {
        RxusedbitreadW::new(self, 2)
    }
    #[doc = "Bit 3 - transmit used bit read interrupt mask"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TxusedbitreadW<'_, IenroSpec> {
        TxusedbitreadW::new(self, 3)
    }
    #[doc = "Bit 4 - transmit buffer under run interrupt mask"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TxunderrunW<'_, IenroSpec> {
        TxunderrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision (gigabit mode only) interrupt mask"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RtrylmtorlatecolW<'_, IenroSpec> {
        RtrylmtorlatecolW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error interrupt mask"]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AmbaerrW<'_, IenroSpec> {
        AmbaerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit complete interrupt mask"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TxcmpltW<'_, IenroSpec> {
        TxcmpltW::new(self, 7)
    }
    #[doc = "Bit 8 - Unused"]
    #[inline(always)]
    pub fn unused(&mut self) -> UnusedW<'_, IenroSpec> {
        UnusedW::new(self, 8)
    }
    #[doc = "Bit 10 - Receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RxoverrunW<'_, IenroSpec> {
        RxoverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - bresp/hresp not OK interrupt mask"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RespnotokW<'_, IenroSpec> {
        RespnotokW::new(self, 11)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum interrupt mask"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NonzeropfrmquantW<'_, IenroSpec> {
        NonzeropfrmquantW::new(self, 12)
    }
    #[doc = "Bit 13 - pause time zero interrupt mask"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PausetimezeroW<'_, IenroSpec> {
        PausetimezeroW::new(self, 13)
    }
    #[doc = "Bit 14 - pause frame transmitted interrupt mask"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PfrmtxW<'_, IenroSpec> {
        PfrmtxW::new(self, 14)
    }
    #[doc = "Bit 18 - PTP delay_req frame received mask"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PtpdlyreqfrmrxW<'_, IenroSpec> {
        PtpdlyreqfrmrxW::new(self, 18)
    }
    #[doc = "Bit 19 - PTP sync frame received mask"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PtpsyncfrmrxW<'_, IenroSpec> {
        PtpsyncfrmrxW::new(self, 19)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted mask"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PtpdlyreqfrmtxW<'_, IenroSpec> {
        PtpdlyreqfrmtxW::new(self, 20)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted mask"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PtpsyncfrmtxW<'_, IenroSpec> {
        PtpsyncfrmtxW::new(self, 21)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received mask"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PtppdlyreqfrmrxW<'_, IenroSpec> {
        PtppdlyreqfrmrxW::new(self, 22)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received mask"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PtppdlyrespfrmrxW<'_, IenroSpec> {
        PtppdlyrespfrmrxW::new(self, 23)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted mask"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PtppdlyreqfrmtxW<'_, IenroSpec> {
        PtppdlyreqfrmtxW::new(self, 24)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted mask"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PtppdlyrespfrmtxW<'_, IenroSpec> {
        PtppdlyrespfrmtxW::new(self, 25)
    }
    #[doc = "Bit 26 - TSU seconds register increment mask"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TsusecregincrW<'_, IenroSpec> {
        TsusecregincrW::new(self, 26)
    }
    #[doc = "Bit 27 - RX LPI indication mask"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RxlpiindcW<'_, IenroSpec> {
        RxlpiindcW::new(self, 27)
    }
    #[doc = "Bit 28 - WOL event received mask"]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WolevntrxW<'_, IenroSpec> {
        WolevntrxW::new(self, 28)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt mask."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TsutimercompW<'_, IenroSpec> {
        TsutimercompW::new(self, 29)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`ienro::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ienro::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenroSpec;
impl crate::RegisterSpec for IenroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ienro::R`](R) reader structure"]
impl crate::Readable for IenroSpec {}
#[doc = "`write(|w| ..)` method takes [`ienro::W`](W) writer structure"]
impl crate::Writable for IenroSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IENRO to value 0x3ffc_7dff"]
impl crate::Resettable for IenroSpec {
    const RESET_VALUE: u32 = 0x3ffc_7dff;
}
