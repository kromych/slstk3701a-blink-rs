#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LEC_R = crate::FieldReader<LEC_A>;
#[doc = "Last Error Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No error occurred during last CAN bus event."]
    NONE = 0,
    #[doc = "1: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF = 1,
    #[doc = "2: A fixed format part of a received frame has the wrong format."]
    FORM = 2,
    #[doc = "3: The message this CAN Core transmitted was not acknowledged by another node."]
    ACK = 3,
    #[doc = "4: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    BIT1 = 4,
    #[doc = "5: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    BIT0 = 5,
    #[doc = "6: The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    CRC = 6,
    #[doc = "7: When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    UNUSED = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEC_A {
    type Ux = u8;
}
impl LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NONE,
            1 => LEC_A::STUFF,
            2 => LEC_A::FORM,
            3 => LEC_A::ACK,
            4 => LEC_A::BIT1,
            5 => LEC_A::BIT0,
            6 => LEC_A::CRC,
            7 => LEC_A::UNUSED,
            _ => unreachable!(),
        }
    }
    #[doc = "No error occurred during last CAN bus event."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEC_A::NONE
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC_A::STUFF
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC_A::FORM
    }
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC_A::ACK
    }
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == LEC_A::BIT1
    }
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == LEC_A::BIT0
    }
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC_A::CRC
    }
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    #[inline(always)]
    pub fn is_unused(&self) -> bool {
        *self == LEC_A::UNUSED
    }
}
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LEC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, LEC_A>;
impl<'a, REG, const O: u8> LEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No error occurred during last CAN bus event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::NONE)
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::STUFF)
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn form(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::FORM)
    }
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::ACK)
    }
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::BIT1)
    }
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn bit0(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::BIT0)
    }
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::CRC)
    }
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    #[inline(always)]
    pub fn unused(self) -> &'a mut crate::W<REG> {
        self.variant(LEC_A::UNUSED)
    }
}
#[doc = "Field `TXOK` reader - Transmitted a Message Successfully"]
pub type TXOK_R = crate::BitReader;
#[doc = "Field `TXOK` writer - Transmitted a Message Successfully"]
pub type TXOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOK` reader - Received a Message Successfully"]
pub type RXOK_R = crate::BitReader;
#[doc = "Field `RXOK` writer - Received a Message Successfully"]
pub type RXOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPASS` reader - Error Passive"]
pub type EPASS_R = crate::BitReader;
#[doc = "Field `EWARN` reader - Warning Status"]
pub type EWARN_R = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus Off Status"]
pub type BOFF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<STATUS_SPEC, 0> {
        LEC_W::new(self)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn txok(&mut self) -> TXOK_W<STATUS_SPEC, 3> {
        TXOK_W::new(self)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    #[must_use]
    pub fn rxok(&mut self) -> RXOK_W<STATUS_SPEC, 4> {
        RXOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
