#[doc = "Register `TXBDCTRL` reader"]
pub type R = crate::R<TXBDCTRL_SPEC>;
#[doc = "Register `TXBDCTRL` writer"]
pub type W = crate::W<TXBDCTRL_SPEC>;
#[doc = "Field `TXBDTSMODE` reader - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TXBDTSMODE_R = crate::FieldReader;
#[doc = "Field `TXBDTSMODE` writer - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
pub type TXBDTSMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&self) -> TXBDTSMODE_R {
        TXBDTSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn txbdtsmode(&mut self) -> TXBDTSMODE_W<TXBDCTRL_SPEC> {
        TXBDTSMODE_W::new(self, 4)
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
#[doc = "TX BD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbdctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbdctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBDCTRL_SPEC;
impl crate::RegisterSpec for TXBDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbdctrl::R`](R) reader structure"]
impl crate::Readable for TXBDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbdctrl::W`](W) writer structure"]
impl crate::Writable for TXBDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBDCTRL to value 0"]
impl crate::Resettable for TXBDCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
