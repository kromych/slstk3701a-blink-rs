#[doc = "Register `TXDATAX` reader"]
pub type R = crate::R<TxdataxSpec>;
#[doc = "Register `TXDATAX` writer"]
pub type W = crate::W<TxdataxSpec>;
#[doc = "Field `TXDATAX` reader - TX Data"]
pub type TxdataxR = crate::FieldReader<u16>;
#[doc = "Field `TXDATAX` writer - TX Data"]
pub type TxdataxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT` reader - Unblock RX After Transmission"]
pub type UbrxatR = crate::BitReader;
#[doc = "Field `UBRXAT` writer - Unblock RX After Transmission"]
pub type UbrxatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT` reader - Set TXTRI After Transmission"]
pub type TxtriatR = crate::BitReader;
#[doc = "Field `TXTRIAT` writer - Set TXTRI After Transmission"]
pub type TxtriatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK` reader - Transmit Data as Break"]
pub type TxbreakR = crate::BitReader;
#[doc = "Field `TXBREAK` writer - Transmit Data as Break"]
pub type TxbreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT` reader - Clear TXEN After Transmission"]
pub type TxdisatR = crate::BitReader;
#[doc = "Field `TXDISAT` writer - Clear TXEN After Transmission"]
pub type TxdisatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT` reader - Enable RX After Transmission"]
pub type RxenatR = crate::BitReader;
#[doc = "Field `RXENAT` writer - Enable RX After Transmission"]
pub type RxenatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdatax(&self) -> TxdataxR {
        TxdataxR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&self) -> UbrxatR {
        UbrxatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat(&self) -> TxtriatR {
        TxtriatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&self) -> TxbreakR {
        TxbreakR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat(&self) -> TxdisatR {
        TxdisatR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&self) -> RxenatR {
        RxenatR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdatax(&mut self) -> TxdataxW<'_, TxdataxSpec> {
        TxdataxW::new(self, 0)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&mut self) -> UbrxatW<'_, TxdataxSpec> {
        UbrxatW::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat(&mut self) -> TxtriatW<'_, TxdataxSpec> {
        TxtriatW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&mut self) -> TxbreakW<'_, TxdataxSpec> {
        TxbreakW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat(&mut self) -> TxdisatW<'_, TxdataxSpec> {
        TxdisatW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&mut self) -> RxenatW<'_, TxdataxSpec> {
        RxenatW::new(self, 15)
    }
}
#[doc = "TX Buffer Data Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdatax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataxSpec;
impl crate::RegisterSpec for TxdataxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdatax::R`](R) reader structure"]
impl crate::Readable for TxdataxSpec {}
#[doc = "`write(|w| ..)` method takes [`txdatax::W`](W) writer structure"]
impl crate::Writable for TxdataxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATAX to value 0"]
impl crate::Resettable for TxdataxSpec {}
