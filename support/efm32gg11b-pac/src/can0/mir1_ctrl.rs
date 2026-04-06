#[doc = "Register `MIR1_CTRL` reader"]
pub type R = crate::R<Mir1CtrlSpec>;
#[doc = "Register `MIR1_CTRL` writer"]
pub type W = crate::W<Mir1CtrlSpec>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EOB` reader - End of Buffer"]
pub type EobR = crate::BitReader;
#[doc = "Field `EOB` writer - End of Buffer"]
pub type EobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRQST` reader - Transmit Request"]
pub type TxrqstR = crate::BitReader;
#[doc = "Field `TXRQST` writer - Transmit Request"]
pub type TxrqstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMTEN` reader - Remote Enable"]
pub type RmtenR = crate::BitReader;
#[doc = "Field `RMTEN` writer - Remote Enable"]
pub type RmtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RxieR = crate::BitReader;
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TxieR = crate::BitReader;
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UMASK` reader - Use Acceptance Mask"]
pub type UmaskR = crate::BitReader;
#[doc = "Field `UMASK` writer - Use Acceptance Mask"]
pub type UmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTPND` reader - Interrupt Pending"]
pub type IntpndR = crate::BitReader;
#[doc = "Field `INTPND` writer - Interrupt Pending"]
pub type IntpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MESSAGEOF` reader - Message Lost (only Valid for Message Objects With Direction = Receive)"]
pub type MessageofR = crate::BitReader;
#[doc = "Field `MESSAGEOF` writer - Message Lost (only Valid for Message Objects With Direction = Receive)"]
pub type MessageofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAVALID` reader - New Data"]
pub type DatavalidR = crate::BitReader;
#[doc = "Field `DATAVALID` writer - New Data"]
pub type DatavalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn eob(&self) -> EobR {
        EobR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrqst(&self) -> TxrqstR {
        TxrqstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn rmten(&self) -> RmtenR {
        RmtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn umask(&self) -> UmaskR {
        UmaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn intpnd(&self) -> IntpndR {
        IntpndR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Lost (only Valid for Message Objects With Direction = Receive)"]
    #[inline(always)]
    pub fn messageof(&self) -> MessageofR {
        MessageofR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn datavalid(&self) -> DatavalidR {
        DatavalidR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, Mir1CtrlSpec> {
        DlcW::new(self, 0)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn eob(&mut self) -> EobW<'_, Mir1CtrlSpec> {
        EobW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrqst(&mut self) -> TxrqstW<'_, Mir1CtrlSpec> {
        TxrqstW::new(self, 8)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn rmten(&mut self) -> RmtenW<'_, Mir1CtrlSpec> {
        RmtenW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, Mir1CtrlSpec> {
        RxieW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&mut self) -> TxieW<'_, Mir1CtrlSpec> {
        TxieW::new(self, 11)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn umask(&mut self) -> UmaskW<'_, Mir1CtrlSpec> {
        UmaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn intpnd(&mut self) -> IntpndW<'_, Mir1CtrlSpec> {
        IntpndW::new(self, 13)
    }
    #[doc = "Bit 14 - Message Lost (only Valid for Message Objects With Direction = Receive)"]
    #[inline(always)]
    pub fn messageof(&mut self) -> MessageofW<'_, Mir1CtrlSpec> {
        MessageofW::new(self, 14)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn datavalid(&mut self) -> DatavalidW<'_, Mir1CtrlSpec> {
        DatavalidW::new(self, 15)
    }
}
#[doc = "Interface Message Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir1CtrlSpec;
impl crate::RegisterSpec for Mir1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_ctrl::R`](R) reader structure"]
impl crate::Readable for Mir1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mir1_ctrl::W`](W) writer structure"]
impl crate::Writable for Mir1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR1_CTRL to value 0"]
impl crate::Resettable for Mir1CtrlSpec {}
