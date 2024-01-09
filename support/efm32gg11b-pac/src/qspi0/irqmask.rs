#[doc = "Register `IRQMASK` reader"]
pub type R = crate::R<IRQMASK_SPEC>;
#[doc = "Register `IRQMASK` writer"]
pub type W = crate::W<IRQMASK_SPEC>;
#[doc = "Field `MODEMFAILMASK` reader - Mode M Failure Mask"]
pub type MODEMFAILMASK_R = crate::BitReader;
#[doc = "Field `MODEMFAILMASK` writer - Mode M Failure Mask"]
pub type MODEMFAILMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOWDETMASK` reader - Underflow Detected Mask"]
pub type UNDERFLOWDETMASK_R = crate::BitReader;
#[doc = "Field `UNDERFLOWDETMASK` writer - Underflow Detected Mask"]
pub type UNDERFLOWDETMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTOPDONEMASK` reader - Indirect Complete Mask"]
pub type INDIRECTOPDONEMASK_R = crate::BitReader;
#[doc = "Field `INDIRECTOPDONEMASK` writer - Indirect Complete Mask"]
pub type INDIRECTOPDONEMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTREADREJECTMASK` reader - Indirect Read Reject Mask"]
pub type INDIRECTREADREJECTMASK_R = crate::BitReader;
#[doc = "Field `INDIRECTREADREJECTMASK` writer - Indirect Read Reject Mask"]
pub type INDIRECTREADREJECTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTWRATTEMPTMASK` reader - Protected Area Write Attempt Mask"]
pub type PROTWRATTEMPTMASK_R = crate::BitReader;
#[doc = "Field `PROTWRATTEMPTMASK` writer - Protected Area Write Attempt Mask"]
pub type PROTWRATTEMPTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLEGALACCESSDETMASK` reader - Illegal Access Detected Mask"]
pub type ILLEGALACCESSDETMASK_R = crate::BitReader;
#[doc = "Field `ILLEGALACCESSDETMASK` writer - Illegal Access Detected Mask"]
pub type ILLEGALACCESSDETMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECTXFERLEVELBREACHMASK` reader - Transfer Watermark Breach Mask"]
pub type INDIRECTXFERLEVELBREACHMASK_R = crate::BitReader;
#[doc = "Field `INDIRECTXFERLEVELBREACHMASK` writer - Transfer Watermark Breach Mask"]
pub type INDIRECTXFERLEVELBREACHMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECVOVERFLOWMASK` reader - Receive Overflow Mask"]
pub type RECVOVERFLOWMASK_R = crate::BitReader;
#[doc = "Field `RECVOVERFLOWMASK` writer - Receive Overflow Mask"]
pub type RECVOVERFLOWMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFONOTFULLMASK` reader - Small TX FIFO Not Full Mask"]
pub type TXFIFONOTFULLMASK_R = crate::BitReader;
#[doc = "Field `TXFIFONOTFULLMASK` writer - Small TX FIFO Not Full Mask"]
pub type TXFIFONOTFULLMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOFULLMASK` reader - Small TX FIFO Full Mask"]
pub type TXFIFOFULLMASK_R = crate::BitReader;
#[doc = "Field `TXFIFOFULLMASK` writer - Small TX FIFO Full Mask"]
pub type TXFIFOFULLMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFONOTEMPTYMASK` reader - Small RX FIFO Not Empty Mask"]
pub type RXFIFONOTEMPTYMASK_R = crate::BitReader;
#[doc = "Field `RXFIFONOTEMPTYMASK` writer - Small RX FIFO Not Empty Mask"]
pub type RXFIFONOTEMPTYMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFULLMASK` reader - Small RX FIFO Full Mask"]
pub type RXFIFOFULLMASK_R = crate::BitReader;
#[doc = "Field `RXFIFOFULLMASK` writer - Small RX FIFO Full Mask"]
pub type RXFIFOFULLMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDRDSRAMFULLMASK` reader - Indirect Read Partition Overflow Mask"]
pub type INDRDSRAMFULLMASK_R = crate::BitReader;
#[doc = "Field `INDRDSRAMFULLMASK` writer - Indirect Read Partition Overflow Mask"]
pub type INDRDSRAMFULLMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLEXPINTMASK` reader - Polling Expiration Detected Mask"]
pub type POLLEXPINTMASK_R = crate::BitReader;
#[doc = "Field `POLLEXPINTMASK` writer - Polling Expiration Detected Mask"]
pub type POLLEXPINTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIGREQMASK` reader - STIG Request Completion Mask"]
pub type STIGREQMASK_R = crate::BitReader;
#[doc = "Field `STIGREQMASK` writer - STIG Request Completion Mask"]
pub type STIGREQMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAERRMASK` reader - RX CRC Data Error Mask"]
pub type RXCRCDATAERRMASK_R = crate::BitReader;
#[doc = "Field `RXCRCDATAERRMASK` writer - RX CRC Data Error Mask"]
pub type RXCRCDATAERRMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCDATAVALMASK` reader - RX CRC Data Valid Mask"]
pub type RXCRCDATAVALMASK_R = crate::BitReader;
#[doc = "Field `RXCRCDATAVALMASK` writer - RX CRC Data Valid Mask"]
pub type RXCRCDATAVALMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCRCCHUNKBRKMASK` reader - TX CRC Chunk Was Broken Mask"]
pub type TXCRCCHUNKBRKMASK_R = crate::BitReader;
#[doc = "Field `TXCRCCHUNKBRKMASK` writer - TX CRC Chunk Was Broken Mask"]
pub type TXCRCCHUNKBRKMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&self) -> MODEMFAILMASK_R {
        MODEMFAILMASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&self) -> UNDERFLOWDETMASK_R {
        UNDERFLOWDETMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&self) -> INDIRECTOPDONEMASK_R {
        INDIRECTOPDONEMASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&self) -> INDIRECTREADREJECTMASK_R {
        INDIRECTREADREJECTMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&self) -> PROTWRATTEMPTMASK_R {
        PROTWRATTEMPTMASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&self) -> ILLEGALACCESSDETMASK_R {
        ILLEGALACCESSDETMASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&self) -> INDIRECTXFERLEVELBREACHMASK_R {
        INDIRECTXFERLEVELBREACHMASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&self) -> RECVOVERFLOWMASK_R {
        RECVOVERFLOWMASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&self) -> TXFIFONOTFULLMASK_R {
        TXFIFONOTFULLMASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&self) -> TXFIFOFULLMASK_R {
        TXFIFOFULLMASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&self) -> RXFIFONOTEMPTYMASK_R {
        RXFIFONOTEMPTYMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&self) -> RXFIFOFULLMASK_R {
        RXFIFOFULLMASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&self) -> INDRDSRAMFULLMASK_R {
        INDRDSRAMFULLMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&self) -> POLLEXPINTMASK_R {
        POLLEXPINTMASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&self) -> STIGREQMASK_R {
        STIGREQMASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&self) -> RXCRCDATAERRMASK_R {
        RXCRCDATAERRMASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&self) -> RXCRCDATAVALMASK_R {
        RXCRCDATAVALMASK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&self) -> TXCRCCHUNKBRKMASK_R {
        TXCRCCHUNKBRKMASK_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    #[must_use]
    pub fn modemfailmask(&mut self) -> MODEMFAILMASK_W<IRQMASK_SPEC> {
        MODEMFAILMASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    #[must_use]
    pub fn underflowdetmask(&mut self) -> UNDERFLOWDETMASK_W<IRQMASK_SPEC> {
        UNDERFLOWDETMASK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indirectopdonemask(&mut self) -> INDIRECTOPDONEMASK_W<IRQMASK_SPEC> {
        INDIRECTOPDONEMASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indirectreadrejectmask(&mut self) -> INDIRECTREADREJECTMASK_W<IRQMASK_SPEC> {
        INDIRECTREADREJECTMASK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn protwrattemptmask(&mut self) -> PROTWRATTEMPTMASK_W<IRQMASK_SPEC> {
        PROTWRATTEMPTMASK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    #[must_use]
    pub fn illegalaccessdetmask(&mut self) -> ILLEGALACCESSDETMASK_W<IRQMASK_SPEC> {
        ILLEGALACCESSDETMASK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indirectxferlevelbreachmask(&mut self) -> INDIRECTXFERLEVELBREACHMASK_W<IRQMASK_SPEC> {
        INDIRECTXFERLEVELBREACHMASK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    #[must_use]
    pub fn recvoverflowmask(&mut self) -> RECVOVERFLOWMASK_W<IRQMASK_SPEC> {
        RECVOVERFLOWMASK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifonotfullmask(&mut self) -> TXFIFONOTFULLMASK_W<IRQMASK_SPEC> {
        TXFIFONOTFULLMASK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifofullmask(&mut self) -> TXFIFOFULLMASK_W<IRQMASK_SPEC> {
        TXFIFOFULLMASK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifonotemptymask(&mut self) -> RXFIFONOTEMPTYMASK_W<IRQMASK_SPEC> {
        RXFIFONOTEMPTYMASK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofullmask(&mut self) -> RXFIFOFULLMASK_W<IRQMASK_SPEC> {
        RXFIFOFULLMASK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indrdsramfullmask(&mut self) -> INDRDSRAMFULLMASK_W<IRQMASK_SPEC> {
        INDRDSRAMFULLMASK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pollexpintmask(&mut self) -> POLLEXPINTMASK_W<IRQMASK_SPEC> {
        POLLEXPINTMASK_W::new(self, 13)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    #[must_use]
    pub fn stigreqmask(&mut self) -> STIGREQMASK_W<IRQMASK_SPEC> {
        STIGREQMASK_W::new(self, 14)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcdataerrmask(&mut self) -> RXCRCDATAERRMASK_W<IRQMASK_SPEC> {
        RXCRCDATAERRMASK_W::new(self, 16)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcdatavalmask(&mut self) -> RXCRCDATAVALMASK_W<IRQMASK_SPEC> {
        RXCRCDATAVALMASK_W::new(self, 17)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txcrcchunkbrkmask(&mut self) -> TXCRCCHUNKBRKMASK_W<IRQMASK_SPEC> {
        TXCRCCHUNKBRKMASK_W::new(self, 18)
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
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQMASK_SPEC;
impl crate::RegisterSpec for IRQMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqmask::R`](R) reader structure"]
impl crate::Readable for IRQMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irqmask::W`](W) writer structure"]
impl crate::Writable for IRQMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQMASK to value 0"]
impl crate::Resettable for IRQMASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
