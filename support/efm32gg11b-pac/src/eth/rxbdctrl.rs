#[doc = "Register `RXBDCTRL` reader"]
pub type R = crate::R<RxbdctrlSpec>;
#[doc = "Register `RXBDCTRL` writer"]
pub type W = crate::W<RxbdctrlSpec>;
#[doc = "Field `RXBDTSMODE` reader - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RxbdtsmodeR = crate::FieldReader;
#[doc = "Field `RXBDTSMODE` writer - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RxbdtsmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn rxbdtsmode(&self) -> RxbdtsmodeR {
        RxbdtsmodeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn rxbdtsmode(&mut self) -> RxbdtsmodeW<'_, RxbdctrlSpec> {
        RxbdtsmodeW::new(self, 4)
    }
}
#[doc = "RX BD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbdctrlSpec;
impl crate::RegisterSpec for RxbdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbdctrl::R`](R) reader structure"]
impl crate::Readable for RxbdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxbdctrl::W`](W) writer structure"]
impl crate::Writable for RxbdctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXBDCTRL to value 0"]
impl crate::Resettable for RxbdctrlSpec {}
