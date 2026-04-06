#[doc = "Register `TXBDCTRL` reader"]
pub type R = crate::R<TxbdctrlSpec>;
#[doc = "Register `TXBDCTRL` writer"]
pub type W = crate::W<TxbdctrlSpec>;
#[doc = "Field `TXBDTSMODE` reader - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TxbdtsmodeR = crate::FieldReader;
#[doc = "Field `TXBDTSMODE` writer - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TxbdtsmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&self) -> TxbdtsmodeR {
        TxbdtsmodeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&mut self) -> TxbdtsmodeW<'_, TxbdctrlSpec> {
        TxbdtsmodeW::new(self, 4)
    }
}
#[doc = "TX BD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbdctrlSpec;
impl crate::RegisterSpec for TxbdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbdctrl::R`](R) reader structure"]
impl crate::Readable for TxbdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`txbdctrl::W`](W) writer structure"]
impl crate::Writable for TxbdctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBDCTRL to value 0"]
impl crate::Resettable for TxbdctrlSpec {}
