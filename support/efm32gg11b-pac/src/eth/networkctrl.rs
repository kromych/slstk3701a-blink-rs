#[doc = "Register `NETWORKCTRL` reader"]
pub type R = crate::R<NETWORKCTRL_SPEC>;
#[doc = "Register `NETWORKCTRL` writer"]
pub type W = crate::W<NETWORKCTRL_SPEC>;
#[doc = "Field `LOOPBACKLOCAL` reader - Loopback local"]
pub type LOOPBACKLOCAL_R = crate::BitReader;
#[doc = "Field `LOOPBACKLOCAL` writer - Loopback local"]
pub type LOOPBACKLOCAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENBRX` reader - Receive enable"]
pub type ENBRX_R = crate::BitReader;
#[doc = "Field `ENBRX` writer - Receive enable"]
pub type ENBRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENBTX` reader - Transmit enable"]
pub type ENBTX_R = crate::BitReader;
#[doc = "Field `ENBTX` writer - Transmit enable"]
pub type ENBTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MANPORTEN` reader - Management port enable"]
pub type MANPORTEN_R = crate::BitReader;
#[doc = "Field `MANPORTEN` writer - Management port enable"]
pub type MANPORTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLRALLSTATSREGS` reader - Clear statistics registers"]
pub type CLRALLSTATSREGS_R = crate::BitReader;
#[doc = "Field `CLRALLSTATSREGS` writer - Clear statistics registers"]
pub type CLRALLSTATSREGS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCALLSTATSREGS` reader - Incremental statistics registers"]
pub type INCALLSTATSREGS_R = crate::BitReader;
#[doc = "Field `INCALLSTATSREGS` writer - Incremental statistics registers"]
pub type INCALLSTATSREGS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STATSWREN` reader - Write enable for statistics registers"]
pub type STATSWREN_R = crate::BitReader;
#[doc = "Field `STATSWREN` writer - Write enable for statistics registers"]
pub type STATSWREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BACKPRESSURE` reader - Back pressure will force collisions on all received frames"]
pub type BACKPRESSURE_R = crate::BitReader;
#[doc = "Field `BACKPRESSURE` writer - Back pressure will force collisions on all received frames"]
pub type BACKPRESSURE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSTRT` reader - Start transmission"]
pub type TXSTRT_R = crate::BitReader;
#[doc = "Field `TXSTRT` writer - Start transmission"]
pub type TXSTRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXHALT` reader - Transmit halt"]
pub type TXHALT_R = crate::BitReader;
#[doc = "Field `TXHALT` writer - Transmit halt"]
pub type TXHALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPFRMREQ` reader - Transmit pause frame"]
pub type TXPFRMREQ_R = crate::BitReader;
#[doc = "Field `TXPFRMREQ` writer - Transmit pause frame"]
pub type TXPFRMREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPFRMZERO` reader - Transmit zero quantum pause frame"]
pub type TXPFRMZERO_R = crate::BitReader;
#[doc = "Field `TXPFRMZERO` writer - Transmit zero quantum pause frame"]
pub type TXPFRMZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STORERXTS` reader - Store receive time stamp to memory."]
pub type STORERXTS_R = crate::BitReader;
#[doc = "Field `STORERXTS` writer - Store receive time stamp to memory."]
pub type STORERXTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFCENB` reader - Enable PFC Priority Based Pause Reception capabilities."]
pub type PFCENB_R = crate::BitReader;
#[doc = "Field `PFCENB` writer - Enable PFC Priority Based Pause Reception capabilities."]
pub type PFCENB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPFCPRIORPFRM` reader - Write a one to transmit PFC priority based pause frame."]
pub type TXPFCPRIORPFRM_R = crate::BitReader;
#[doc = "Field `TXPFCPRIORPFRM` writer - Write a one to transmit PFC priority based pause frame."]
pub type TXPFCPRIORPFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLUSHRXPKT` reader - Flush the next packet from the external RX DPRAM."]
pub type FLUSHRXPKT_R = crate::BitReader;
#[doc = "Field `FLUSHRXPKT` writer - Flush the next packet from the external RX DPRAM."]
pub type FLUSHRXPKT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXLPIEN` reader - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
pub type TXLPIEN_R = crate::BitReader;
#[doc = "Field `TXLPIEN` writer - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
pub type TXLPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTPUNICASTEN` reader - Enable detection of unicast PTP unicast frames."]
pub type PTPUNICASTEN_R = crate::BitReader;
#[doc = "Field `PTPUNICASTEN` writer - Enable detection of unicast PTP unicast frames."]
pub type PTPUNICASTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOREUDPOFFSET` reader - Store UDP / TCP offset to memory."]
pub type STOREUDPOFFSET_R = crate::BitReader;
#[doc = "Field `STOREUDPOFFSET` writer - Store UDP / TCP offset to memory."]
pub type STOREUDPOFFSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONESTEPSYNCMODE` reader - 1588 One Step Sync Mode."]
pub type ONESTEPSYNCMODE_R = crate::BitReader;
#[doc = "Field `ONESTEPSYNCMODE` writer - 1588 One Step Sync Mode."]
pub type ONESTEPSYNCMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFCCTRL` reader - Enable multiple PFC pause quantums, one per pause priority"]
pub type PFCCTRL_R = crate::BitReader;
#[doc = "Field `PFCCTRL` writer - Enable multiple PFC pause quantums, one per pause priority"]
pub type PFCCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn loopbacklocal(&self) -> LOOPBACKLOCAL_R {
        LOOPBACKLOCAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn enbrx(&self) -> ENBRX_R {
        ENBRX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn enbtx(&self) -> ENBTX_R {
        ENBTX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn manporten(&self) -> MANPORTEN_R {
        MANPORTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrallstatsregs(&self) -> CLRALLSTATSREGS_R {
        CLRALLSTATSREGS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Incremental statistics registers"]
    #[inline(always)]
    pub fn incallstatsregs(&self) -> INCALLSTATSREGS_R {
        INCALLSTATSREGS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn statswren(&self) -> STATSWREN_R {
        STATSWREN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure will force collisions on all received frames"]
    #[inline(always)]
    pub fn backpressure(&self) -> BACKPRESSURE_R {
        BACKPRESSURE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn txstrt(&self) -> TXSTRT_R {
        TXSTRT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn txhalt(&self) -> TXHALT_R {
        TXHALT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit pause frame"]
    #[inline(always)]
    pub fn txpfrmreq(&self) -> TXPFRMREQ_R {
        TXPFRMREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame"]
    #[inline(always)]
    pub fn txpfrmzero(&self) -> TXPFRMZERO_R {
        TXPFRMZERO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory."]
    #[inline(always)]
    pub fn storerxts(&self) -> STORERXTS_R {
        STORERXTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    pub fn pfcenb(&self) -> PFCENB_R {
        PFCENB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    pub fn txpfcpriorpfrm(&self) -> TXPFCPRIORPFRM_R {
        TXPFCPRIORPFRM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    pub fn flushrxpkt(&self) -> FLUSHRXPKT_R {
        FLUSHRXPKT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    pub fn txlpien(&self) -> TXLPIEN_R {
        TXLPIEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub fn ptpunicasten(&self) -> PTPUNICASTEN_R {
        PTPUNICASTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Store UDP / TCP offset to memory."]
    #[inline(always)]
    pub fn storeudpoffset(&self) -> STOREUDPOFFSET_R {
        STOREUDPOFFSET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode."]
    #[inline(always)]
    pub fn onestepsyncmode(&self) -> ONESTEPSYNCMODE_R {
        ONESTEPSYNCMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable multiple PFC pause quantums, one per pause priority"]
    #[inline(always)]
    pub fn pfcctrl(&self) -> PFCCTRL_R {
        PFCCTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    #[must_use]
    pub fn loopbacklocal(&mut self) -> LOOPBACKLOCAL_W<NETWORKCTRL_SPEC, 1> {
        LOOPBACKLOCAL_W::new(self)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbrx(&mut self) -> ENBRX_W<NETWORKCTRL_SPEC, 2> {
        ENBRX_W::new(self)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbtx(&mut self) -> ENBTX_W<NETWORKCTRL_SPEC, 3> {
        ENBTX_W::new(self)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    #[must_use]
    pub fn manporten(&mut self) -> MANPORTEN_W<NETWORKCTRL_SPEC, 4> {
        MANPORTEN_W::new(self)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn clrallstatsregs(&mut self) -> CLRALLSTATSREGS_W<NETWORKCTRL_SPEC, 5> {
        CLRALLSTATSREGS_W::new(self)
    }
    #[doc = "Bit 6 - Incremental statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn incallstatsregs(&mut self) -> INCALLSTATSREGS_W<NETWORKCTRL_SPEC, 6> {
        INCALLSTATSREGS_W::new(self)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    #[must_use]
    pub fn statswren(&mut self) -> STATSWREN_W<NETWORKCTRL_SPEC, 7> {
        STATSWREN_W::new(self)
    }
    #[doc = "Bit 8 - Back pressure will force collisions on all received frames"]
    #[inline(always)]
    #[must_use]
    pub fn backpressure(&mut self) -> BACKPRESSURE_W<NETWORKCTRL_SPEC, 8> {
        BACKPRESSURE_W::new(self)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txstrt(&mut self) -> TXSTRT_W<NETWORKCTRL_SPEC, 9> {
        TXSTRT_W::new(self)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    #[must_use]
    pub fn txhalt(&mut self) -> TXHALT_W<NETWORKCTRL_SPEC, 10> {
        TXHALT_W::new(self)
    }
    #[doc = "Bit 11 - Transmit pause frame"]
    #[inline(always)]
    #[must_use]
    pub fn txpfrmreq(&mut self) -> TXPFRMREQ_W<NETWORKCTRL_SPEC, 11> {
        TXPFRMREQ_W::new(self)
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame"]
    #[inline(always)]
    #[must_use]
    pub fn txpfrmzero(&mut self) -> TXPFRMZERO_W<NETWORKCTRL_SPEC, 12> {
        TXPFRMZERO_W::new(self)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory."]
    #[inline(always)]
    #[must_use]
    pub fn storerxts(&mut self) -> STORERXTS_W<NETWORKCTRL_SPEC, 15> {
        STORERXTS_W::new(self)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    #[must_use]
    pub fn pfcenb(&mut self) -> PFCENB_W<NETWORKCTRL_SPEC, 16> {
        PFCENB_W::new(self)
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    #[must_use]
    pub fn txpfcpriorpfrm(&mut self) -> TXPFCPRIORPFRM_W<NETWORKCTRL_SPEC, 17> {
        TXPFCPRIORPFRM_W::new(self)
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    #[must_use]
    pub fn flushrxpkt(&mut self) -> FLUSHRXPKT_W<NETWORKCTRL_SPEC, 18> {
        FLUSHRXPKT_W::new(self)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn txlpien(&mut self) -> TXLPIEN_W<NETWORKCTRL_SPEC, 19> {
        TXLPIEN_W::new(self)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    #[must_use]
    pub fn ptpunicasten(&mut self) -> PTPUNICASTEN_W<NETWORKCTRL_SPEC, 20> {
        PTPUNICASTEN_W::new(self)
    }
    #[doc = "Bit 22 - Store UDP / TCP offset to memory."]
    #[inline(always)]
    #[must_use]
    pub fn storeudpoffset(&mut self) -> STOREUDPOFFSET_W<NETWORKCTRL_SPEC, 22> {
        STOREUDPOFFSET_W::new(self)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode."]
    #[inline(always)]
    #[must_use]
    pub fn onestepsyncmode(&mut self) -> ONESTEPSYNCMODE_W<NETWORKCTRL_SPEC, 24> {
        ONESTEPSYNCMODE_W::new(self)
    }
    #[doc = "Bit 25 - Enable multiple PFC pause quantums, one per pause priority"]
    #[inline(always)]
    #[must_use]
    pub fn pfcctrl(&mut self) -> PFCCTRL_W<NETWORKCTRL_SPEC, 25> {
        PFCCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Network control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`networkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`networkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NETWORKCTRL_SPEC;
impl crate::RegisterSpec for NETWORKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`networkctrl::R`](R) reader structure"]
impl crate::Readable for NETWORKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`networkctrl::W`](W) writer structure"]
impl crate::Writable for NETWORKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NETWORKCTRL to value 0"]
impl crate::Resettable for NETWORKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
