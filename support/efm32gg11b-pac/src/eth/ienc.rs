#[doc = "Register `IENC` writer"]
pub type W = crate::W<IENC_SPEC>;
#[doc = "Field `MNGMNTDONE` writer - Disable management done interrupt"]
pub type MNGMNTDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXCMPLT` writer - Disable receive complete interrupt"]
pub type RXCMPLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUSEDBITREAD` writer - Disable receive used bit read interrupt"]
pub type RXUSEDBITREAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUSEDBITREAD` writer - Disable transmit used bit read interrupt"]
pub type TXUSEDBITREAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNDERRUN` writer - Disable transmit buffer under run interrupt"]
pub type TXUNDERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Disable retry limit exceeded or late collision interrupt"]
pub type RTRYLMTORLATECOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMBAERR` writer - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
pub type AMBAERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXCMPLT` writer - Disable transmit complete interrupt"]
pub type TXCMPLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOVERRUN` writer - Disable receive overrun interrupt"]
pub type RXOVERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESPNOTOK` writer - Disable bresp/hresp not OK interrupt"]
pub type RESPNOTOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Disable pause frame with non-zero pause quantum interrupt"]
pub type NONZEROPFRMQUANT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAUSETIMEZERO` writer - Disable pause time zero interrupt"]
pub type PAUSETIMEZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFRMTX` writer - Disable pause frame transmitted interrupt"]
pub type PFRMTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPDLYREQFRMRX` writer - Disable PTP delay_req frame received interrupt"]
pub type PTPDLYREQFRMRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPSYNCFRMRX` writer - Disable PTP sync frame received interrupt"]
pub type PTPSYNCFRMRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPDLYREQFRMTX` writer - Disable PTP delay_req frame transmitted interrupt"]
pub type PTPDLYREQFRMTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPSYNCFRMTX` writer - Disable PTP sync frame transmitted interrupt"]
pub type PTPSYNCFRMTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - Disable PTP pdelay_req frame received interrupt"]
pub type PTPPDLYREQFRMRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - Disable PTP pdelay_resp frame received interrupt"]
pub type PTPPDLYRESPFRMRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - Disable PTP pdelay_req frame transmitted interrupt"]
pub type PTPPDLYREQFRMTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - Disable PTP pdelay_resp frame transmitted interrupt"]
pub type PTPPDLYRESPFRMTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSUSECREGINCR` writer - Disable TSU seconds register increment interrupt"]
pub type TSUSECREGINCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXLPIINDC` writer - Disable RX LPI indication interrupt"]
pub type RXLPIINDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WOLEVNTRX` writer - Disable WOL event received interrupt"]
pub type WOLEVNTRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSUTIMERCOMP` writer - Disable TSU timer comparison interrupt."]
pub type TSUTIMERCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Disable management done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W<IENC_SPEC, 0> {
        MNGMNTDONE_W::new(self)
    }
    #[doc = "Bit 1 - Disable receive complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W<IENC_SPEC, 1> {
        RXCMPLT_W::new(self)
    }
    #[doc = "Bit 2 - Disable receive used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W<IENC_SPEC, 2> {
        RXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 3 - Disable transmit used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W<IENC_SPEC, 3> {
        TXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 4 - Disable transmit buffer under run interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W<IENC_SPEC, 4> {
        TXUNDERRUN_W::new(self)
    }
    #[doc = "Bit 5 - Disable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W<IENC_SPEC, 5> {
        RTRYLMTORLATECOL_W::new(self)
    }
    #[doc = "Bit 6 - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ambaerr(&mut self) -> AMBAERR_W<IENC_SPEC, 6> {
        AMBAERR_W::new(self)
    }
    #[doc = "Bit 7 - Disable transmit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txcmplt(&mut self) -> TXCMPLT_W<IENC_SPEC, 7> {
        TXCMPLT_W::new(self)
    }
    #[doc = "Bit 10 - Disable receive overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W<IENC_SPEC, 10> {
        RXOVERRUN_W::new(self)
    }
    #[doc = "Bit 11 - Disable bresp/hresp not OK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn respnotok(&mut self) -> RESPNOTOK_W<IENC_SPEC, 11> {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Bit 12 - Disable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W<IENC_SPEC, 12> {
        NONZEROPFRMQUANT_W::new(self)
    }
    #[doc = "Bit 13 - Disable pause time zero interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W<IENC_SPEC, 13> {
        PAUSETIMEZERO_W::new(self)
    }
    #[doc = "Bit 14 - Disable pause frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pfrmtx(&mut self) -> PFRMTX_W<IENC_SPEC, 14> {
        PFRMTX_W::new(self)
    }
    #[doc = "Bit 18 - Disable PTP delay_req frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W<IENC_SPEC, 18> {
        PTPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 19 - Disable PTP sync frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W<IENC_SPEC, 19> {
        PTPSYNCFRMRX_W::new(self)
    }
    #[doc = "Bit 20 - Disable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W<IENC_SPEC, 20> {
        PTPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 21 - Disable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W<IENC_SPEC, 21> {
        PTPSYNCFRMTX_W::new(self)
    }
    #[doc = "Bit 22 - Disable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W<IENC_SPEC, 22> {
        PTPPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 23 - Disable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W<IENC_SPEC, 23> {
        PTPPDLYRESPFRMRX_W::new(self)
    }
    #[doc = "Bit 24 - Disable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W<IENC_SPEC, 24> {
        PTPPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 25 - Disable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W<IENC_SPEC, 25> {
        PTPPDLYRESPFRMTX_W::new(self)
    }
    #[doc = "Bit 26 - Disable TSU seconds register increment interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W<IENC_SPEC, 26> {
        TSUSECREGINCR_W::new(self)
    }
    #[doc = "Bit 27 - Disable RX LPI indication interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W<IENC_SPEC, 27> {
        RXLPIINDC_W::new(self)
    }
    #[doc = "Bit 28 - Disable WOL event received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W<IENC_SPEC, 28> {
        WOLEVNTRX_W::new(self)
    }
    #[doc = "Bit 29 - Disable TSU timer comparison interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tsutimercomp(&mut self) -> TSUTIMERCOMP_W<IENC_SPEC, 29> {
        TSUTIMERCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENC_SPEC;
impl crate::RegisterSpec for IENC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ienc::W`](W) writer structure"]
impl crate::Writable for IENC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENC to value 0"]
impl crate::Resettable for IENC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
