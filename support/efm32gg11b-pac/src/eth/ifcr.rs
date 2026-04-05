#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IfcrSpec>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `MNGMNTDONE` reader - Management frame sent"]
pub type MngmntdoneR = crate::BitReader;
#[doc = "Field `MNGMNTDONE` writer - Management frame sent"]
pub type MngmntdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCMPLT` reader - Receive complete"]
pub type RxcmpltR = crate::BitReader;
#[doc = "Field `RXCMPLT` writer - Receive complete"]
pub type RxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSEDBITREAD` reader - RX used bit read"]
pub type RxusedbitreadR = crate::BitReader;
#[doc = "Field `RXUSEDBITREAD` writer - RX used bit read"]
pub type RxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUSEDBITREAD` reader - TX used bit read"]
pub type TxusedbitreadR = crate::BitReader;
#[doc = "Field `TXUSEDBITREAD` writer - TX used bit read"]
pub type TxusedbitreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUN` reader - Transmit under run"]
pub type TxunderrunR = crate::BitReader;
#[doc = "Field `TXUNDERRUN` writer - Transmit under run"]
pub type TxunderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRYLMTORLATECOL` reader - Retry limit exceeded or late collision"]
pub type RtrylmtorlatecolR = crate::BitReader;
#[doc = "Field `RTRYLMTORLATECOL` writer - Retry limit exceeded or late collision"]
pub type RtrylmtorlatecolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMBAERR` reader - Transmit frame corruption due to AMBA (AHB) error."]
pub type AmbaerrR = crate::BitReader;
#[doc = "Field `AMBAERR` writer - Transmit frame corruption due to AMBA (AHB) error."]
pub type AmbaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCMPLT` reader - Transmit complete"]
pub type TxcmpltR = crate::BitReader;
#[doc = "Field `TXCMPLT` writer - Transmit complete"]
pub type TxcmpltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUN` reader - Receive overrun"]
pub type RxoverrunR = crate::BitReader;
#[doc = "Field `RXOVERRUN` writer - Receive overrun"]
pub type RxoverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` reader - Hresp not OK"]
pub type RespnotokR = crate::BitReader;
#[doc = "Field `RESPNOTOK` writer - Hresp not OK"]
pub type RespnotokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONZEROPFRMQUANT` reader - Pause frame with non-zero pause quantum received"]
pub type NonzeropfrmquantR = crate::BitReader;
#[doc = "Field `NONZEROPFRMQUANT` writer - Pause frame with non-zero pause quantum received"]
pub type NonzeropfrmquantW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSETIMEZERO` reader - Pause Time zero"]
pub type PausetimezeroR = crate::BitReader;
#[doc = "Field `PAUSETIMEZERO` writer - Pause Time zero"]
pub type PausetimezeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRMTX` reader - Pause frame transmitted"]
pub type PfrmtxR = crate::BitReader;
#[doc = "Field `PFRMTX` writer - Pause frame transmitted"]
pub type PfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMRX` reader - PTP delay_req frame received"]
pub type PtpdlyreqfrmrxR = crate::BitReader;
#[doc = "Field `PTPDLYREQFRMRX` writer - PTP delay_req frame received"]
pub type PtpdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMRX` reader - PTP sync frame received"]
pub type PtpsyncfrmrxR = crate::BitReader;
#[doc = "Field `PTPSYNCFRMRX` writer - PTP sync frame received"]
pub type PtpsyncfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMTX` reader - PTP delay_req frame transmitted"]
pub type PtpdlyreqfrmtxR = crate::BitReader;
#[doc = "Field `PTPDLYREQFRMTX` writer - PTP delay_req frame transmitted"]
pub type PtpdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMTX` reader - PTP sync frame transmitted"]
pub type PtpsyncfrmtxR = crate::BitReader;
#[doc = "Field `PTPSYNCFRMTX` writer - PTP sync frame transmitted"]
pub type PtpsyncfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMRX` reader - PTP pdelay_req frame received"]
pub type PtppdlyreqfrmrxR = crate::BitReader;
#[doc = "Field `PTPPDLYREQFRMRX` writer - PTP pdelay_req frame received"]
pub type PtppdlyreqfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMRX` reader - PTP pdelay_resp frame received"]
pub type PtppdlyrespfrmrxR = crate::BitReader;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - PTP pdelay_resp frame received"]
pub type PtppdlyrespfrmrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMTX` reader - PTP pdelay_req frame transmitted"]
pub type PtppdlyreqfrmtxR = crate::BitReader;
#[doc = "Field `PTPPDLYREQFRMTX` writer - PTP pdelay_req frame transmitted"]
pub type PtppdlyreqfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMTX` reader - PTP pdelay_resp frame transmitted"]
pub type PtppdlyrespfrmtxR = crate::BitReader;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - PTP pdelay_resp frame transmitted"]
pub type PtppdlyrespfrmtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUSECREGINCR` reader - TSU seconds register increment"]
pub type TsusecregincrR = crate::BitReader;
#[doc = "Field `TSUSECREGINCR` writer - TSU seconds register increment"]
pub type TsusecregincrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIINDC` reader - Receive LPI indication status bit change"]
pub type RxlpiindcR = crate::BitReader;
#[doc = "Field `RXLPIINDC` writer - Receive LPI indication status bit change"]
pub type RxlpiindcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLEVNTRX` reader - WOL event received interrupt."]
pub type WolevntrxR = crate::BitReader;
#[doc = "Field `WOLEVNTRX` writer - WOL event received interrupt."]
pub type WolevntrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUTIMERCOMP` reader - TSU timer comparison interrupt."]
pub type TsutimercompR = crate::BitReader;
#[doc = "Field `TSUTIMERCOMP` writer - TSU timer comparison interrupt."]
pub type TsutimercompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Management frame sent"]
    #[inline(always)]
    pub fn mngmntdone(&self) -> MngmntdoneR {
        MngmntdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn rxcmplt(&self) -> RxcmpltR {
        RxcmpltR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rxusedbitread(&self) -> RxusedbitreadR {
        RxusedbitreadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX used bit read"]
    #[inline(always)]
    pub fn txusedbitread(&self) -> TxusedbitreadR {
        TxusedbitreadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TxunderrunR {
        TxunderrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&self) -> RtrylmtorlatecolR {
        RtrylmtorlatecolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AmbaerrR {
        AmbaerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TxcmpltR {
        TxcmpltR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RxoverrunR {
        RxoverrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RespnotokR {
        RespnotokR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&self) -> NonzeropfrmquantR {
        NonzeropfrmquantR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time zero"]
    #[inline(always)]
    pub fn pausetimezero(&self) -> PausetimezeroR {
        PausetimezeroR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pause frame transmitted"]
    #[inline(always)]
    pub fn pfrmtx(&self) -> PfrmtxR {
        PfrmtxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP delay_req frame received"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&self) -> PtpdlyreqfrmrxR {
        PtpdlyreqfrmrxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP sync frame received"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&self) -> PtpsyncfrmrxR {
        PtpsyncfrmrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&self) -> PtpdlyreqfrmtxR {
        PtpdlyreqfrmtxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&self) -> PtpsyncfrmtxR {
        PtpsyncfrmtxR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&self) -> PtppdlyreqfrmrxR {
        PtppdlyreqfrmrxR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&self) -> PtppdlyrespfrmrxR {
        PtppdlyrespfrmrxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&self) -> PtppdlyreqfrmtxR {
        PtppdlyreqfrmtxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&self) -> PtppdlyrespfrmtxR {
        PtppdlyrespfrmtxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSU seconds register increment"]
    #[inline(always)]
    pub fn tsusecregincr(&self) -> TsusecregincrR {
        TsusecregincrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    pub fn rxlpiindc(&self) -> RxlpiindcR {
        RxlpiindcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WOL event received interrupt."]
    #[inline(always)]
    pub fn wolevntrx(&self) -> WolevntrxR {
        WolevntrxR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&self) -> TsutimercompR {
        TsutimercompR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management frame sent"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MngmntdoneW<'_, IfcrSpec> {
        MngmntdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RxcmpltW<'_, IfcrSpec> {
        RxcmpltW::new(self, 1)
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RxusedbitreadW<'_, IfcrSpec> {
        RxusedbitreadW::new(self, 2)
    }
    #[doc = "Bit 3 - TX used bit read"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TxusedbitreadW<'_, IfcrSpec> {
        TxusedbitreadW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TxunderrunW<'_, IfcrSpec> {
        TxunderrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RtrylmtorlatecolW<'_, IfcrSpec> {
        RtrylmtorlatecolW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AmbaerrW<'_, IfcrSpec> {
        AmbaerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TxcmpltW<'_, IfcrSpec> {
        TxcmpltW::new(self, 7)
    }
    #[doc = "Bit 10 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RxoverrunW<'_, IfcrSpec> {
        RxoverrunW::new(self, 10)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RespnotokW<'_, IfcrSpec> {
        RespnotokW::new(self, 11)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NonzeropfrmquantW<'_, IfcrSpec> {
        NonzeropfrmquantW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Time zero"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PausetimezeroW<'_, IfcrSpec> {
        PausetimezeroW::new(self, 13)
    }
    #[doc = "Bit 14 - Pause frame transmitted"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PfrmtxW<'_, IfcrSpec> {
        PfrmtxW::new(self, 14)
    }
    #[doc = "Bit 18 - PTP delay_req frame received"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PtpdlyreqfrmrxW<'_, IfcrSpec> {
        PtpdlyreqfrmrxW::new(self, 18)
    }
    #[doc = "Bit 19 - PTP sync frame received"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PtpsyncfrmrxW<'_, IfcrSpec> {
        PtpsyncfrmrxW::new(self, 19)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PtpdlyreqfrmtxW<'_, IfcrSpec> {
        PtpdlyreqfrmtxW::new(self, 20)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PtpsyncfrmtxW<'_, IfcrSpec> {
        PtpsyncfrmtxW::new(self, 21)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PtppdlyreqfrmrxW<'_, IfcrSpec> {
        PtppdlyreqfrmrxW::new(self, 22)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PtppdlyrespfrmrxW<'_, IfcrSpec> {
        PtppdlyrespfrmrxW::new(self, 23)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PtppdlyreqfrmtxW<'_, IfcrSpec> {
        PtppdlyreqfrmtxW::new(self, 24)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PtppdlyrespfrmtxW<'_, IfcrSpec> {
        PtppdlyrespfrmtxW::new(self, 25)
    }
    #[doc = "Bit 26 - TSU seconds register increment"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TsusecregincrW<'_, IfcrSpec> {
        TsusecregincrW::new(self, 26)
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RxlpiindcW<'_, IfcrSpec> {
        RxlpiindcW::new(self, 27)
    }
    #[doc = "Bit 28 - WOL event received interrupt."]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WolevntrxW<'_, IfcrSpec> {
        WolevntrxW::new(self, 28)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TsutimercompW<'_, IfcrSpec> {
        TsutimercompW::new(self, 29)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}
