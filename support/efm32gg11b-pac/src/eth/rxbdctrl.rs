#[doc = "Register `RXBDCTRL` reader"]
pub type R = crate::R<RXBDCTRL_SPEC>;
#[doc = "Register `RXBDCTRL` writer"]
pub type W = crate::W<RXBDCTRL_SPEC>;
#[doc = "Field `RXBDTSMODE` reader - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RXBDTSMODE_R = crate::FieldReader;
#[doc = "Field `RXBDTSMODE` writer - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type RXBDTSMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn rxbdtsmode(&self) -> RXBDTSMODE_R {
        RXBDTSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - RX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn rxbdtsmode(&mut self) -> RXBDTSMODE_W<RXBDCTRL_SPEC, 4> {
        RXBDTSMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RX BD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbdctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbdctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBDCTRL_SPEC;
impl crate::RegisterSpec for RXBDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbdctrl::R`](R) reader structure"]
impl crate::Readable for RXBDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxbdctrl::W`](W) writer structure"]
impl crate::Writable for RXBDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXBDCTRL to value 0"]
impl crate::Resettable for RXBDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
