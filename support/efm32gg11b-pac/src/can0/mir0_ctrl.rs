#[doc = "Register `MIR0_CTRL` reader"]
pub type R = crate::R<MIR0_CTRL_SPEC>;
#[doc = "Register `MIR0_CTRL` writer"]
pub type W = crate::W<MIR0_CTRL_SPEC>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EOB` reader - End of Buffer"]
pub type EOB_R = crate::BitReader;
#[doc = "Field `EOB` writer - End of Buffer"]
pub type EOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRQST` reader - Transmit Request"]
pub type TXRQST_R = crate::BitReader;
#[doc = "Field `TXRQST` writer - Transmit Request"]
pub type TXRQST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RMTEN` reader - Remote Enable"]
pub type RMTEN_R = crate::BitReader;
#[doc = "Field `RMTEN` writer - Remote Enable"]
pub type RMTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RXIE_R = crate::BitReader;
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TXIE_R = crate::BitReader;
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UMASK` reader - Use Acceptance Mask"]
pub type UMASK_R = crate::BitReader;
#[doc = "Field `UMASK` writer - Use Acceptance Mask"]
pub type UMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTPND` reader - Interrupt Pending"]
pub type INTPND_R = crate::BitReader;
#[doc = "Field `INTPND` writer - Interrupt Pending"]
pub type INTPND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MESSAGEOF` reader - Message Lost (only Valid for Message Objects With Direction = Receive)"]
pub type MESSAGEOF_R = crate::BitReader;
#[doc = "Field `MESSAGEOF` writer - Message Lost (only Valid for Message Objects With Direction = Receive)"]
pub type MESSAGEOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAVALID` reader - New Data"]
pub type DATAVALID_R = crate::BitReader;
#[doc = "Field `DATAVALID` writer - New Data"]
pub type DATAVALID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn eob(&self) -> EOB_R {
        EOB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrqst(&self) -> TXRQST_R {
        TXRQST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn rmten(&self) -> RMTEN_R {
        RMTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn umask(&self) -> UMASK_R {
        UMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Lost (only Valid for Message Objects With Direction = Receive)"]
    #[inline(always)]
    pub fn messageof(&self) -> MESSAGEOF_R {
        MESSAGEOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn datavalid(&self) -> DATAVALID_R {
        DATAVALID_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<MIR0_CTRL_SPEC, 0> {
        DLC_W::new(self)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn eob(&mut self) -> EOB_W<MIR0_CTRL_SPEC, 7> {
        EOB_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn txrqst(&mut self) -> TXRQST_W<MIR0_CTRL_SPEC, 8> {
        TXRQST_W::new(self)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmten(&mut self) -> RMTEN_W<MIR0_CTRL_SPEC, 9> {
        RMTEN_W::new(self)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<MIR0_CTRL_SPEC, 10> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<MIR0_CTRL_SPEC, 11> {
        TXIE_W::new(self)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    #[must_use]
    pub fn umask(&mut self) -> UMASK_W<MIR0_CTRL_SPEC, 12> {
        UMASK_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn intpnd(&mut self) -> INTPND_W<MIR0_CTRL_SPEC, 13> {
        INTPND_W::new(self)
    }
    #[doc = "Bit 14 - Message Lost (only Valid for Message Objects With Direction = Receive)"]
    #[inline(always)]
    #[must_use]
    pub fn messageof(&mut self) -> MESSAGEOF_W<MIR0_CTRL_SPEC, 14> {
        MESSAGEOF_W::new(self)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    #[must_use]
    pub fn datavalid(&mut self) -> DATAVALID_W<MIR0_CTRL_SPEC, 15> {
        DATAVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interface Message Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR0_CTRL_SPEC;
impl crate::RegisterSpec for MIR0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_ctrl::R`](R) reader structure"]
impl crate::Readable for MIR0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir0_ctrl::W`](W) writer structure"]
impl crate::Writable for MIR0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIR0_CTRL to value 0"]
impl crate::Resettable for MIR0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
