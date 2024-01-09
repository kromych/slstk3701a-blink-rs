#[doc = "Register `IENS` writer"]
pub type W = crate::W<IENS_SPEC>;
#[doc = "Field `MNGMNTDONE` writer - Enable management done interrupt"]
pub type MNGMNTDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCMPLT` writer - Enable receive complete interrupt"]
pub type RXCMPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSEDBITREAD` writer - Enable receive used bit read interrupt"]
pub type RXUSEDBITREAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUSEDBITREAD` writer - Enable transmit used bit read interrupt"]
pub type TXUSEDBITREAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRUN` writer - Enable transmit buffer under run interrupt"]
pub type TXUNDERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Enable retry limit exceeded or late collision interrupt"]
pub type RTRYLMTORLATECOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMBAERR` writer - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
pub type AMBAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCMPLT` writer - Enable transmit complete interrupt"]
pub type TXCMPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRUN` writer - Enable receive overrun interrupt"]
pub type RXOVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPNOTOK` writer - Enable bresp/hresp not OK interrupt"]
pub type RESPNOTOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Enable pause frame with non-zero pause quantum interrupt"]
pub type NONZEROPFRMQUANT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSETIMEZERO` writer - Enable pause time zero interrupt"]
pub type PAUSETIMEZERO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRMTX` writer - Enable pause frame transmitted interrupt"]
pub type PFRMTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMRX` writer - Enable PTP delay_req frame received interrupt"]
pub type PTPDLYREQFRMRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMRX` writer - Enable PTP sync frame received interrupt"]
pub type PTPSYNCFRMRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPDLYREQFRMTX` writer - Enable PTP delay_req frame transmitted interrupt"]
pub type PTPDLYREQFRMTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPSYNCFRMTX` writer - Enable PTP sync frame transmitted interrupt"]
pub type PTPSYNCFRMTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - Enable PTP pdelay_req frame received interrupt"]
pub type PTPPDLYREQFRMRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - Enable PTP pdelay_resp frame received interrupt"]
pub type PTPPDLYRESPFRMRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - Enable PTP pdelay_req frame transmitted interrupt"]
pub type PTPPDLYREQFRMTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - Enable PTP pdelay_resp frame transmitted interrupt"]
pub type PTPPDLYRESPFRMTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUSECREGINCR` writer - Enable TSU seconds register increment interrupt"]
pub type TSUSECREGINCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIINDC` writer - Enable RX LPI indication interrupt"]
pub type RXLPIINDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WOLEVNTRX` writer - Enable WOL event received interrupt"]
pub type WOLEVNTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUTIMERCOMP` writer - Enable TSU timer comparison interrupt."]
pub type TSUTIMERCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable management done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W<IENS_SPEC> {
        MNGMNTDONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable receive complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W<IENS_SPEC> {
        RXCMPLT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable receive used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W<IENS_SPEC> {
        RXUSEDBITREAD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable transmit used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W<IENS_SPEC> {
        TXUSEDBITREAD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable transmit buffer under run interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W<IENS_SPEC> {
        TXUNDERRUN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W<IENS_SPEC> {
        RTRYLMTORLATECOL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ambaerr(&mut self) -> AMBAERR_W<IENS_SPEC> {
        AMBAERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable transmit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txcmplt(&mut self) -> TXCMPLT_W<IENS_SPEC> {
        TXCMPLT_W::new(self, 7)
    }
    #[doc = "Bit 10 - Enable receive overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W<IENS_SPEC> {
        RXOVERRUN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable bresp/hresp not OK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn respnotok(&mut self) -> RESPNOTOK_W<IENS_SPEC> {
        RESPNOTOK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W<IENS_SPEC> {
        NONZEROPFRMQUANT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable pause time zero interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W<IENS_SPEC> {
        PAUSETIMEZERO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable pause frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pfrmtx(&mut self) -> PFRMTX_W<IENS_SPEC> {
        PFRMTX_W::new(self, 14)
    }
    #[doc = "Bit 18 - Enable PTP delay_req frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W<IENS_SPEC> {
        PTPDLYREQFRMRX_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable PTP sync frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W<IENS_SPEC> {
        PTPSYNCFRMRX_W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W<IENS_SPEC> {
        PTPDLYREQFRMTX_W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W<IENS_SPEC> {
        PTPSYNCFRMTX_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W<IENS_SPEC> {
        PTPPDLYREQFRMRX_W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W<IENS_SPEC> {
        PTPPDLYRESPFRMRX_W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W<IENS_SPEC> {
        PTPPDLYREQFRMTX_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W<IENS_SPEC> {
        PTPPDLYRESPFRMTX_W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable TSU seconds register increment interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W<IENS_SPEC> {
        TSUSECREGINCR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable RX LPI indication interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W<IENS_SPEC> {
        RXLPIINDC_W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable WOL event received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W<IENS_SPEC> {
        WOLEVNTRX_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable TSU timer comparison interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tsutimercomp(&mut self) -> TSUTIMERCOMP_W<IENS_SPEC> {
        TSUTIMERCOMP_W::new(self, 29)
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
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iens::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENS_SPEC;
impl crate::RegisterSpec for IENS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iens::W`](W) writer structure"]
impl crate::Writable for IENS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IENS to value 0"]
impl crate::Resettable for IENS_SPEC {
    const RESET_VALUE: u32 = 0;
}
