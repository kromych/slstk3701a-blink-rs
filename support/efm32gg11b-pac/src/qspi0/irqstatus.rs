#[doc = "Register `IRQSTATUS` reader"]
pub type R = crate::R<IrqstatusSpec>;
#[doc = "Register `IRQSTATUS` writer"]
pub type W = crate::W<IrqstatusSpec>;
#[doc = "Field `MODEMFAIL` reader - Mode M Failure"]
pub type ModemfailR = crate::BitReader;
#[doc = "Field `MODEMFAIL` writer - Mode M Failure"]
pub type ModemfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOWDET` reader - Underflow Detected"]
pub type UnderflowdetR = crate::BitReader;
#[doc = "Field `UNDERFLOWDET` writer - Underflow Detected"]
pub type UnderflowdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTOPDONE` reader - Indirect Operation Complete"]
pub type IndirectopdoneR = crate::BitReader;
#[doc = "Field `INDIRECTOPDONE` writer - Indirect Operation Complete"]
pub type IndirectopdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTREADREJECT` reader - Indirect Operation Was Requested but Could Not Be Accepted"]
pub type IndirectreadrejectR = crate::BitReader;
#[doc = "Field `INDIRECTREADREJECT` writer - Indirect Operation Was Requested but Could Not Be Accepted"]
pub type IndirectreadrejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTWRATTEMPT` reader - Write to Protected Area Was Attempted and Rejected"]
pub type ProtwrattemptR = crate::BitReader;
#[doc = "Field `PROTWRATTEMPT` writer - Write to Protected Area Was Attempted and Rejected"]
pub type ProtwrattemptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLEGALACCESSDET` reader - Illegal Memory Access Has Been Detected"]
pub type IllegalaccessdetR = crate::BitReader;
#[doc = "Field `ILLEGALACCESSDET` writer - Illegal Memory Access Has Been Detected"]
pub type IllegalaccessdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTXFERLEVELBREACH` reader - Indirect Transfer Watermark Level Breached"]
pub type IndirectxferlevelbreachR = crate::BitReader;
#[doc = "Field `INDIRECTXFERLEVELBREACH` writer - Indirect Transfer Watermark Level Breached"]
pub type IndirectxferlevelbreachW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECVOVERFLOW` reader - Receive Overflow"]
pub type RecvoverflowR = crate::BitReader;
#[doc = "Field `RECVOVERFLOW` writer - Receive Overflow"]
pub type RecvoverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFONOTFULL` reader - Small TX FIFO Not Full"]
pub type TxfifonotfullR = crate::BitReader;
#[doc = "Field `TXFIFONOTFULL` writer - Small TX FIFO Not Full"]
pub type TxfifonotfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOFULL` reader - Small TX FIFO Full"]
pub type TxfifofullR = crate::BitReader;
#[doc = "Field `TXFIFOFULL` writer - Small TX FIFO Full"]
pub type TxfifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFONOTEMPTY` reader - Small RX FIFO Not Empty"]
pub type RxfifonotemptyR = crate::BitReader;
#[doc = "Field `RXFIFONOTEMPTY` writer - Small RX FIFO Not Empty"]
pub type RxfifonotemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFULL` reader - Small RX FIFO Full"]
pub type RxfifofullR = crate::BitReader;
#[doc = "Field `RXFIFOFULL` writer - Small RX FIFO Full"]
pub type RxfifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDRDSRAMFULL` reader - Indirect Read Partition Overflow"]
pub type IndrdsramfullR = crate::BitReader;
#[doc = "Field `INDRDSRAMFULL` writer - Indirect Read Partition Overflow"]
pub type IndrdsramfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLEXPINT` reader - The Maximum Number of Programmed Polls Cycles is Expired"]
pub type PollexpintR = crate::BitReader;
#[doc = "Field `POLLEXPINT` writer - The Maximum Number of Programmed Polls Cycles is Expired"]
pub type PollexpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIGREQINT` reader - The Controller is Ready for Getting Another STIG Request"]
pub type StigreqintR = crate::BitReader;
#[doc = "Field `STIGREQINT` writer - The Controller is Ready for Getting Another STIG Request"]
pub type StigreqintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAERR` reader - RX CRC Data Error"]
pub type RxcrcdataerrR = crate::BitReader;
#[doc = "Field `RXCRCDATAERR` writer - RX CRC Data Error"]
pub type RxcrcdataerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAVAL` reader - RX CRC Data Valid"]
pub type RxcrcdatavalR = crate::BitReader;
#[doc = "Field `RXCRCDATAVAL` writer - RX CRC Data Valid"]
pub type RxcrcdatavalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCRCCHUNKBRK` reader - TX CRC Chunk Was Broken"]
pub type TxcrcchunkbrkR = crate::BitReader;
#[doc = "Field `TXCRCCHUNKBRK` writer - TX CRC Chunk Was Broken"]
pub type TxcrcchunkbrkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode M Failure"]
    #[inline(always)]
    pub fn modemfail(&self) -> ModemfailR {
        ModemfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected"]
    #[inline(always)]
    pub fn underflowdet(&self) -> UnderflowdetR {
        UnderflowdetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect Operation Complete"]
    #[inline(always)]
    pub fn indirectopdone(&self) -> IndirectopdoneR {
        IndirectopdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect Operation Was Requested but Could Not Be Accepted"]
    #[inline(always)]
    pub fn indirectreadreject(&self) -> IndirectreadrejectR {
        IndirectreadrejectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write to Protected Area Was Attempted and Rejected"]
    #[inline(always)]
    pub fn protwrattempt(&self) -> ProtwrattemptR {
        ProtwrattemptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal Memory Access Has Been Detected"]
    #[inline(always)]
    pub fn illegalaccessdet(&self) -> IllegalaccessdetR {
        IllegalaccessdetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    pub fn indirectxferlevelbreach(&self) -> IndirectxferlevelbreachR {
        IndirectxferlevelbreachR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow"]
    #[inline(always)]
    pub fn recvoverflow(&self) -> RecvoverflowR {
        RecvoverflowR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full"]
    #[inline(always)]
    pub fn txfifonotfull(&self) -> TxfifonotfullR {
        TxfifonotfullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full"]
    #[inline(always)]
    pub fn txfifofull(&self) -> TxfifofullR {
        TxfifofullR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rxfifonotempty(&self) -> RxfifonotemptyR {
        RxfifonotemptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full"]
    #[inline(always)]
    pub fn rxfifofull(&self) -> RxfifofullR {
        RxfifofullR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow"]
    #[inline(always)]
    pub fn indrdsramfull(&self) -> IndrdsramfullR {
        IndrdsramfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The Maximum Number of Programmed Polls Cycles is Expired"]
    #[inline(always)]
    pub fn pollexpint(&self) -> PollexpintR {
        PollexpintR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The Controller is Ready for Getting Another STIG Request"]
    #[inline(always)]
    pub fn stigreqint(&self) -> StigreqintR {
        StigreqintR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error"]
    #[inline(always)]
    pub fn rxcrcdataerr(&self) -> RxcrcdataerrR {
        RxcrcdataerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid"]
    #[inline(always)]
    pub fn rxcrcdataval(&self) -> RxcrcdatavalR {
        RxcrcdatavalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken"]
    #[inline(always)]
    pub fn txcrcchunkbrk(&self) -> TxcrcchunkbrkR {
        TxcrcchunkbrkR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure"]
    #[inline(always)]
    pub fn modemfail(&mut self) -> ModemfailW<'_, IrqstatusSpec> {
        ModemfailW::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Detected"]
    #[inline(always)]
    pub fn underflowdet(&mut self) -> UnderflowdetW<'_, IrqstatusSpec> {
        UnderflowdetW::new(self, 1)
    }
    #[doc = "Bit 2 - Indirect Operation Complete"]
    #[inline(always)]
    pub fn indirectopdone(&mut self) -> IndirectopdoneW<'_, IrqstatusSpec> {
        IndirectopdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Indirect Operation Was Requested but Could Not Be Accepted"]
    #[inline(always)]
    pub fn indirectreadreject(&mut self) -> IndirectreadrejectW<'_, IrqstatusSpec> {
        IndirectreadrejectW::new(self, 3)
    }
    #[doc = "Bit 4 - Write to Protected Area Was Attempted and Rejected"]
    #[inline(always)]
    pub fn protwrattempt(&mut self) -> ProtwrattemptW<'_, IrqstatusSpec> {
        ProtwrattemptW::new(self, 4)
    }
    #[doc = "Bit 5 - Illegal Memory Access Has Been Detected"]
    #[inline(always)]
    pub fn illegalaccessdet(&mut self) -> IllegalaccessdetW<'_, IrqstatusSpec> {
        IllegalaccessdetW::new(self, 5)
    }
    #[doc = "Bit 6 - Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    pub fn indirectxferlevelbreach(&mut self) -> IndirectxferlevelbreachW<'_, IrqstatusSpec> {
        IndirectxferlevelbreachW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Overflow"]
    #[inline(always)]
    pub fn recvoverflow(&mut self) -> RecvoverflowW<'_, IrqstatusSpec> {
        RecvoverflowW::new(self, 7)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full"]
    #[inline(always)]
    pub fn txfifonotfull(&mut self) -> TxfifonotfullW<'_, IrqstatusSpec> {
        TxfifonotfullW::new(self, 8)
    }
    #[doc = "Bit 9 - Small TX FIFO Full"]
    #[inline(always)]
    pub fn txfifofull(&mut self) -> TxfifofullW<'_, IrqstatusSpec> {
        TxfifofullW::new(self, 9)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rxfifonotempty(&mut self) -> RxfifonotemptyW<'_, IrqstatusSpec> {
        RxfifonotemptyW::new(self, 10)
    }
    #[doc = "Bit 11 - Small RX FIFO Full"]
    #[inline(always)]
    pub fn rxfifofull(&mut self) -> RxfifofullW<'_, IrqstatusSpec> {
        RxfifofullW::new(self, 11)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow"]
    #[inline(always)]
    pub fn indrdsramfull(&mut self) -> IndrdsramfullW<'_, IrqstatusSpec> {
        IndrdsramfullW::new(self, 12)
    }
    #[doc = "Bit 13 - The Maximum Number of Programmed Polls Cycles is Expired"]
    #[inline(always)]
    pub fn pollexpint(&mut self) -> PollexpintW<'_, IrqstatusSpec> {
        PollexpintW::new(self, 13)
    }
    #[doc = "Bit 14 - The Controller is Ready for Getting Another STIG Request"]
    #[inline(always)]
    pub fn stigreqint(&mut self) -> StigreqintW<'_, IrqstatusSpec> {
        StigreqintW::new(self, 14)
    }
    #[doc = "Bit 16 - RX CRC Data Error"]
    #[inline(always)]
    pub fn rxcrcdataerr(&mut self) -> RxcrcdataerrW<'_, IrqstatusSpec> {
        RxcrcdataerrW::new(self, 16)
    }
    #[doc = "Bit 17 - RX CRC Data Valid"]
    #[inline(always)]
    pub fn rxcrcdataval(&mut self) -> RxcrcdatavalW<'_, IrqstatusSpec> {
        RxcrcdatavalW::new(self, 17)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken"]
    #[inline(always)]
    pub fn txcrcchunkbrk(&mut self) -> TxcrcchunkbrkW<'_, IrqstatusSpec> {
        TxcrcchunkbrkW::new(self, 18)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqstatusSpec;
impl crate::RegisterSpec for IrqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqstatus::R`](R) reader structure"]
impl crate::Readable for IrqstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`irqstatus::W`](W) writer structure"]
impl crate::Writable for IrqstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQSTATUS to value 0"]
impl crate::Resettable for IrqstatusSpec {}
