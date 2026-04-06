#[doc = "Register `IRQMASK` reader"]
pub type R = crate::R<IrqmaskSpec>;
#[doc = "Register `IRQMASK` writer"]
pub type W = crate::W<IrqmaskSpec>;
#[doc = "Field `MODEMFAILMASK` reader - Mode M Failure Mask"]
pub type ModemfailmaskR = crate::BitReader;
#[doc = "Field `MODEMFAILMASK` writer - Mode M Failure Mask"]
pub type ModemfailmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOWDETMASK` reader - Underflow Detected Mask"]
pub type UnderflowdetmaskR = crate::BitReader;
#[doc = "Field `UNDERFLOWDETMASK` writer - Underflow Detected Mask"]
pub type UnderflowdetmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTOPDONEMASK` reader - Indirect Complete Mask"]
pub type IndirectopdonemaskR = crate::BitReader;
#[doc = "Field `INDIRECTOPDONEMASK` writer - Indirect Complete Mask"]
pub type IndirectopdonemaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTREADREJECTMASK` reader - Indirect Read Reject Mask"]
pub type IndirectreadrejectmaskR = crate::BitReader;
#[doc = "Field `INDIRECTREADREJECTMASK` writer - Indirect Read Reject Mask"]
pub type IndirectreadrejectmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTWRATTEMPTMASK` reader - Protected Area Write Attempt Mask"]
pub type ProtwrattemptmaskR = crate::BitReader;
#[doc = "Field `PROTWRATTEMPTMASK` writer - Protected Area Write Attempt Mask"]
pub type ProtwrattemptmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLEGALACCESSDETMASK` reader - Illegal Access Detected Mask"]
pub type IllegalaccessdetmaskR = crate::BitReader;
#[doc = "Field `ILLEGALACCESSDETMASK` writer - Illegal Access Detected Mask"]
pub type IllegalaccessdetmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTXFERLEVELBREACHMASK` reader - Transfer Watermark Breach Mask"]
pub type IndirectxferlevelbreachmaskR = crate::BitReader;
#[doc = "Field `INDIRECTXFERLEVELBREACHMASK` writer - Transfer Watermark Breach Mask"]
pub type IndirectxferlevelbreachmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECVOVERFLOWMASK` reader - Receive Overflow Mask"]
pub type RecvoverflowmaskR = crate::BitReader;
#[doc = "Field `RECVOVERFLOWMASK` writer - Receive Overflow Mask"]
pub type RecvoverflowmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFONOTFULLMASK` reader - Small TX FIFO Not Full Mask"]
pub type TxfifonotfullmaskR = crate::BitReader;
#[doc = "Field `TXFIFONOTFULLMASK` writer - Small TX FIFO Not Full Mask"]
pub type TxfifonotfullmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOFULLMASK` reader - Small TX FIFO Full Mask"]
pub type TxfifofullmaskR = crate::BitReader;
#[doc = "Field `TXFIFOFULLMASK` writer - Small TX FIFO Full Mask"]
pub type TxfifofullmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFONOTEMPTYMASK` reader - Small RX FIFO Not Empty Mask"]
pub type RxfifonotemptymaskR = crate::BitReader;
#[doc = "Field `RXFIFONOTEMPTYMASK` writer - Small RX FIFO Not Empty Mask"]
pub type RxfifonotemptymaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFULLMASK` reader - Small RX FIFO Full Mask"]
pub type RxfifofullmaskR = crate::BitReader;
#[doc = "Field `RXFIFOFULLMASK` writer - Small RX FIFO Full Mask"]
pub type RxfifofullmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDRDSRAMFULLMASK` reader - Indirect Read Partition Overflow Mask"]
pub type IndrdsramfullmaskR = crate::BitReader;
#[doc = "Field `INDRDSRAMFULLMASK` writer - Indirect Read Partition Overflow Mask"]
pub type IndrdsramfullmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLEXPINTMASK` reader - Polling Expiration Detected Mask"]
pub type PollexpintmaskR = crate::BitReader;
#[doc = "Field `POLLEXPINTMASK` writer - Polling Expiration Detected Mask"]
pub type PollexpintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIGREQMASK` reader - STIG Request Completion Mask"]
pub type StigreqmaskR = crate::BitReader;
#[doc = "Field `STIGREQMASK` writer - STIG Request Completion Mask"]
pub type StigreqmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAERRMASK` reader - RX CRC Data Error Mask"]
pub type RxcrcdataerrmaskR = crate::BitReader;
#[doc = "Field `RXCRCDATAERRMASK` writer - RX CRC Data Error Mask"]
pub type RxcrcdataerrmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAVALMASK` reader - RX CRC Data Valid Mask"]
pub type RxcrcdatavalmaskR = crate::BitReader;
#[doc = "Field `RXCRCDATAVALMASK` writer - RX CRC Data Valid Mask"]
pub type RxcrcdatavalmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCRCCHUNKBRKMASK` reader - TX CRC Chunk Was Broken Mask"]
pub type TxcrcchunkbrkmaskR = crate::BitReader;
#[doc = "Field `TXCRCCHUNKBRKMASK` writer - TX CRC Chunk Was Broken Mask"]
pub type TxcrcchunkbrkmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&self) -> ModemfailmaskR {
        ModemfailmaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&self) -> UnderflowdetmaskR {
        UnderflowdetmaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&self) -> IndirectopdonemaskR {
        IndirectopdonemaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&self) -> IndirectreadrejectmaskR {
        IndirectreadrejectmaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&self) -> ProtwrattemptmaskR {
        ProtwrattemptmaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&self) -> IllegalaccessdetmaskR {
        IllegalaccessdetmaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&self) -> IndirectxferlevelbreachmaskR {
        IndirectxferlevelbreachmaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&self) -> RecvoverflowmaskR {
        RecvoverflowmaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&self) -> TxfifonotfullmaskR {
        TxfifonotfullmaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&self) -> TxfifofullmaskR {
        TxfifofullmaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&self) -> RxfifonotemptymaskR {
        RxfifonotemptymaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&self) -> RxfifofullmaskR {
        RxfifofullmaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&self) -> IndrdsramfullmaskR {
        IndrdsramfullmaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&self) -> PollexpintmaskR {
        PollexpintmaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&self) -> StigreqmaskR {
        StigreqmaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&self) -> RxcrcdataerrmaskR {
        RxcrcdataerrmaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&self) -> RxcrcdatavalmaskR {
        RxcrcdatavalmaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&self) -> TxcrcchunkbrkmaskR {
        TxcrcchunkbrkmaskR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&mut self) -> ModemfailmaskW<'_, IrqmaskSpec> {
        ModemfailmaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&mut self) -> UnderflowdetmaskW<'_, IrqmaskSpec> {
        UnderflowdetmaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&mut self) -> IndirectopdonemaskW<'_, IrqmaskSpec> {
        IndirectopdonemaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&mut self) -> IndirectreadrejectmaskW<'_, IrqmaskSpec> {
        IndirectreadrejectmaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&mut self) -> ProtwrattemptmaskW<'_, IrqmaskSpec> {
        ProtwrattemptmaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&mut self) -> IllegalaccessdetmaskW<'_, IrqmaskSpec> {
        IllegalaccessdetmaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&mut self) -> IndirectxferlevelbreachmaskW<'_, IrqmaskSpec> {
        IndirectxferlevelbreachmaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&mut self) -> RecvoverflowmaskW<'_, IrqmaskSpec> {
        RecvoverflowmaskW::new(self, 7)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&mut self) -> TxfifonotfullmaskW<'_, IrqmaskSpec> {
        TxfifonotfullmaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&mut self) -> TxfifofullmaskW<'_, IrqmaskSpec> {
        TxfifofullmaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&mut self) -> RxfifonotemptymaskW<'_, IrqmaskSpec> {
        RxfifonotemptymaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&mut self) -> RxfifofullmaskW<'_, IrqmaskSpec> {
        RxfifofullmaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&mut self) -> IndrdsramfullmaskW<'_, IrqmaskSpec> {
        IndrdsramfullmaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&mut self) -> PollexpintmaskW<'_, IrqmaskSpec> {
        PollexpintmaskW::new(self, 13)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&mut self) -> StigreqmaskW<'_, IrqmaskSpec> {
        StigreqmaskW::new(self, 14)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&mut self) -> RxcrcdataerrmaskW<'_, IrqmaskSpec> {
        RxcrcdataerrmaskW::new(self, 16)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&mut self) -> RxcrcdatavalmaskW<'_, IrqmaskSpec> {
        RxcrcdatavalmaskW::new(self, 17)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&mut self) -> TxcrcchunkbrkmaskW<'_, IrqmaskSpec> {
        TxcrcchunkbrkmaskW::new(self, 18)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqmaskSpec;
impl crate::RegisterSpec for IrqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqmask::R`](R) reader structure"]
impl crate::Readable for IrqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`irqmask::W`](W) writer structure"]
impl crate::Writable for IrqmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQMASK to value 0"]
impl crate::Resettable for IrqmaskSpec {}
