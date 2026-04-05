#[doc = "Register `NETWORKCTRL` reader"]
pub type R = crate::R<NetworkctrlSpec>;
#[doc = "Register `NETWORKCTRL` writer"]
pub type W = crate::W<NetworkctrlSpec>;
#[doc = "Field `LOOPBACKLOCAL` reader - Loopback local"]
pub type LoopbacklocalR = crate::BitReader;
#[doc = "Field `LOOPBACKLOCAL` writer - Loopback local"]
pub type LoopbacklocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBRX` reader - Receive enable"]
pub type EnbrxR = crate::BitReader;
#[doc = "Field `ENBRX` writer - Receive enable"]
pub type EnbrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBTX` reader - Transmit enable"]
pub type EnbtxR = crate::BitReader;
#[doc = "Field `ENBTX` writer - Transmit enable"]
pub type EnbtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANPORTEN` reader - Management port enable"]
pub type ManportenR = crate::BitReader;
#[doc = "Field `MANPORTEN` writer - Management port enable"]
pub type ManportenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRALLSTATSREGS` reader - Clear statistics registers"]
pub type ClrallstatsregsR = crate::BitReader;
#[doc = "Field `CLRALLSTATSREGS` writer - Clear statistics registers"]
pub type ClrallstatsregsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCALLSTATSREGS` reader - Incremental statistics registers"]
pub type IncallstatsregsR = crate::BitReader;
#[doc = "Field `INCALLSTATSREGS` writer - Incremental statistics registers"]
pub type IncallstatsregsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATSWREN` reader - Write enable for statistics registers"]
pub type StatswrenR = crate::BitReader;
#[doc = "Field `STATSWREN` writer - Write enable for statistics registers"]
pub type StatswrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKPRESSURE` reader - Back pressure will force collisions on all received frames"]
pub type BackpressureR = crate::BitReader;
#[doc = "Field `BACKPRESSURE` writer - Back pressure will force collisions on all received frames"]
pub type BackpressureW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTRT` reader - Start transmission"]
pub type TxstrtR = crate::BitReader;
#[doc = "Field `TXSTRT` writer - Start transmission"]
pub type TxstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXHALT` reader - Transmit halt"]
pub type TxhaltR = crate::BitReader;
#[doc = "Field `TXHALT` writer - Transmit halt"]
pub type TxhaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPFRMREQ` reader - Transmit pause frame"]
pub type TxpfrmreqR = crate::BitReader;
#[doc = "Field `TXPFRMREQ` writer - Transmit pause frame"]
pub type TxpfrmreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPFRMZERO` reader - Transmit zero quantum pause frame"]
pub type TxpfrmzeroR = crate::BitReader;
#[doc = "Field `TXPFRMZERO` writer - Transmit zero quantum pause frame"]
pub type TxpfrmzeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STORERXTS` reader - Store receive time stamp to memory."]
pub type StorerxtsR = crate::BitReader;
#[doc = "Field `STORERXTS` writer - Store receive time stamp to memory."]
pub type StorerxtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCENB` reader - Enable PFC Priority Based Pause Reception capabilities."]
pub type PfcenbR = crate::BitReader;
#[doc = "Field `PFCENB` writer - Enable PFC Priority Based Pause Reception capabilities."]
pub type PfcenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPFCPRIORPFRM` reader - Write a one to transmit PFC priority based pause frame."]
pub type TxpfcpriorpfrmR = crate::BitReader;
#[doc = "Field `TXPFCPRIORPFRM` writer - Write a one to transmit PFC priority based pause frame."]
pub type TxpfcpriorpfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHRXPKT` reader - Flush the next packet from the external RX DPRAM."]
pub type FlushrxpktR = crate::BitReader;
#[doc = "Field `FLUSHRXPKT` writer - Flush the next packet from the external RX DPRAM."]
pub type FlushrxpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLPIEN` reader - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
pub type TxlpienR = crate::BitReader;
#[doc = "Field `TXLPIEN` writer - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
pub type TxlpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTPUNICASTEN` reader - Enable detection of unicast PTP unicast frames."]
pub type PtpunicastenR = crate::BitReader;
#[doc = "Field `PTPUNICASTEN` writer - Enable detection of unicast PTP unicast frames."]
pub type PtpunicastenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOREUDPOFFSET` reader - Store UDP / TCP offset to memory."]
pub type StoreudpoffsetR = crate::BitReader;
#[doc = "Field `STOREUDPOFFSET` writer - Store UDP / TCP offset to memory."]
pub type StoreudpoffsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESTEPSYNCMODE` reader - 1588 One Step Sync Mode."]
pub type OnestepsyncmodeR = crate::BitReader;
#[doc = "Field `ONESTEPSYNCMODE` writer - 1588 One Step Sync Mode."]
pub type OnestepsyncmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCCTRL` reader - Enable multiple PFC pause quantums, one per pause priority"]
pub type PfcctrlR = crate::BitReader;
#[doc = "Field `PFCCTRL` writer - Enable multiple PFC pause quantums, one per pause priority"]
pub type PfcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn loopbacklocal(&self) -> LoopbacklocalR {
        LoopbacklocalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn enbrx(&self) -> EnbrxR {
        EnbrxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn enbtx(&self) -> EnbtxR {
        EnbtxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn manporten(&self) -> ManportenR {
        ManportenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrallstatsregs(&self) -> ClrallstatsregsR {
        ClrallstatsregsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Incremental statistics registers"]
    #[inline(always)]
    pub fn incallstatsregs(&self) -> IncallstatsregsR {
        IncallstatsregsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn statswren(&self) -> StatswrenR {
        StatswrenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure will force collisions on all received frames"]
    #[inline(always)]
    pub fn backpressure(&self) -> BackpressureR {
        BackpressureR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn txstrt(&self) -> TxstrtR {
        TxstrtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn txhalt(&self) -> TxhaltR {
        TxhaltR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit pause frame"]
    #[inline(always)]
    pub fn txpfrmreq(&self) -> TxpfrmreqR {
        TxpfrmreqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame"]
    #[inline(always)]
    pub fn txpfrmzero(&self) -> TxpfrmzeroR {
        TxpfrmzeroR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory."]
    #[inline(always)]
    pub fn storerxts(&self) -> StorerxtsR {
        StorerxtsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    pub fn pfcenb(&self) -> PfcenbR {
        PfcenbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    pub fn txpfcpriorpfrm(&self) -> TxpfcpriorpfrmR {
        TxpfcpriorpfrmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    pub fn flushrxpkt(&self) -> FlushrxpktR {
        FlushrxpktR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    pub fn txlpien(&self) -> TxlpienR {
        TxlpienR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub fn ptpunicasten(&self) -> PtpunicastenR {
        PtpunicastenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Store UDP / TCP offset to memory."]
    #[inline(always)]
    pub fn storeudpoffset(&self) -> StoreudpoffsetR {
        StoreudpoffsetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode."]
    #[inline(always)]
    pub fn onestepsyncmode(&self) -> OnestepsyncmodeR {
        OnestepsyncmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable multiple PFC pause quantums, one per pause priority"]
    #[inline(always)]
    pub fn pfcctrl(&self) -> PfcctrlR {
        PfcctrlR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn loopbacklocal(&mut self) -> LoopbacklocalW<'_, NetworkctrlSpec> {
        LoopbacklocalW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn enbrx(&mut self) -> EnbrxW<'_, NetworkctrlSpec> {
        EnbrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn enbtx(&mut self) -> EnbtxW<'_, NetworkctrlSpec> {
        EnbtxW::new(self, 3)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn manporten(&mut self) -> ManportenW<'_, NetworkctrlSpec> {
        ManportenW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrallstatsregs(&mut self) -> ClrallstatsregsW<'_, NetworkctrlSpec> {
        ClrallstatsregsW::new(self, 5)
    }
    #[doc = "Bit 6 - Incremental statistics registers"]
    #[inline(always)]
    pub fn incallstatsregs(&mut self) -> IncallstatsregsW<'_, NetworkctrlSpec> {
        IncallstatsregsW::new(self, 6)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn statswren(&mut self) -> StatswrenW<'_, NetworkctrlSpec> {
        StatswrenW::new(self, 7)
    }
    #[doc = "Bit 8 - Back pressure will force collisions on all received frames"]
    #[inline(always)]
    pub fn backpressure(&mut self) -> BackpressureW<'_, NetworkctrlSpec> {
        BackpressureW::new(self, 8)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn txstrt(&mut self) -> TxstrtW<'_, NetworkctrlSpec> {
        TxstrtW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn txhalt(&mut self) -> TxhaltW<'_, NetworkctrlSpec> {
        TxhaltW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit pause frame"]
    #[inline(always)]
    pub fn txpfrmreq(&mut self) -> TxpfrmreqW<'_, NetworkctrlSpec> {
        TxpfrmreqW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame"]
    #[inline(always)]
    pub fn txpfrmzero(&mut self) -> TxpfrmzeroW<'_, NetworkctrlSpec> {
        TxpfrmzeroW::new(self, 12)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory."]
    #[inline(always)]
    pub fn storerxts(&mut self) -> StorerxtsW<'_, NetworkctrlSpec> {
        StorerxtsW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    pub fn pfcenb(&mut self) -> PfcenbW<'_, NetworkctrlSpec> {
        PfcenbW::new(self, 16)
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    pub fn txpfcpriorpfrm(&mut self) -> TxpfcpriorpfrmW<'_, NetworkctrlSpec> {
        TxpfcpriorpfrmW::new(self, 17)
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    pub fn flushrxpkt(&mut self) -> FlushrxpktW<'_, NetworkctrlSpec> {
        FlushrxpktW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    pub fn txlpien(&mut self) -> TxlpienW<'_, NetworkctrlSpec> {
        TxlpienW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub fn ptpunicasten(&mut self) -> PtpunicastenW<'_, NetworkctrlSpec> {
        PtpunicastenW::new(self, 20)
    }
    #[doc = "Bit 22 - Store UDP / TCP offset to memory."]
    #[inline(always)]
    pub fn storeudpoffset(&mut self) -> StoreudpoffsetW<'_, NetworkctrlSpec> {
        StoreudpoffsetW::new(self, 22)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode."]
    #[inline(always)]
    pub fn onestepsyncmode(&mut self) -> OnestepsyncmodeW<'_, NetworkctrlSpec> {
        OnestepsyncmodeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable multiple PFC pause quantums, one per pause priority"]
    #[inline(always)]
    pub fn pfcctrl(&mut self) -> PfcctrlW<'_, NetworkctrlSpec> {
        PfcctrlW::new(self, 25)
    }
}
#[doc = "Network control register\n\nYou can [`read`](crate::Reg::read) this register and get [`networkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`networkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NetworkctrlSpec;
impl crate::RegisterSpec for NetworkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`networkctrl::R`](R) reader structure"]
impl crate::Readable for NetworkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`networkctrl::W`](W) writer structure"]
impl crate::Writable for NetworkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NETWORKCTRL to value 0"]
impl crate::Resettable for NetworkctrlSpec {}
