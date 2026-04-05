#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Last Error Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lec {
    #[doc = "0: No error occurred during last CAN bus event."]
    None = 0,
    #[doc = "1: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    Stuff = 1,
    #[doc = "2: A fixed format part of a received frame has the wrong format."]
    Form = 2,
    #[doc = "3: The message this CAN Core transmitted was not acknowledged by another node."]
    Ack = 3,
    #[doc = "4: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    Bit1 = 4,
    #[doc = "5: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    Bit0 = 5,
    #[doc = "6: The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    Crc = 6,
    #[doc = "7: When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    Unused = 7,
}
impl From<Lec> for u8 {
    #[inline(always)]
    fn from(variant: Lec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lec {
    type Ux = u8;
}
impl crate::IsEnum for Lec {}
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LecR = crate::FieldReader<Lec>;
impl LecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lec {
        match self.bits {
            0 => Lec::None,
            1 => Lec::Stuff,
            2 => Lec::Form,
            3 => Lec::Ack,
            4 => Lec::Bit1,
            5 => Lec::Bit0,
            6 => Lec::Crc,
            7 => Lec::Unused,
            _ => unreachable!(),
        }
    }
    #[doc = "No error occurred during last CAN bus event."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lec::None
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == Lec::Stuff
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == Lec::Form
    }
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Lec::Ack
    }
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == Lec::Bit1
    }
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == Lec::Bit0
    }
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == Lec::Crc
    }
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    #[inline(always)]
    pub fn is_unused(&self) -> bool {
        *self == Lec::Unused
    }
}
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lec, crate::Safe>;
impl<'a, REG> LecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No error occurred during last CAN bus event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::None)
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Stuff)
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn form(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Form)
    }
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Ack)
    }
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Bit1)
    }
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn bit0(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Bit0)
    }
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Crc)
    }
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    #[inline(always)]
    pub fn unused(self) -> &'a mut crate::W<REG> {
        self.variant(Lec::Unused)
    }
}
#[doc = "Field `TXOK` reader - Transmitted a Message Successfully"]
pub type TxokR = crate::BitReader;
#[doc = "Field `TXOK` writer - Transmitted a Message Successfully"]
pub type TxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOK` reader - Received a Message Successfully"]
pub type RxokR = crate::BitReader;
#[doc = "Field `RXOK` writer - Received a Message Successfully"]
pub type RxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPASS` reader - Error Passive"]
pub type EpassR = crate::BitReader;
#[doc = "Field `EWARN` reader - Warning Status"]
pub type EwarnR = crate::BitReader;
#[doc = "Field `BOFF` reader - Bus Off Status"]
pub type BoffR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TxokR {
        TxokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RxokR {
        RxokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn epass(&self) -> EpassR {
        EpassR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ewarn(&self) -> EwarnR {
        EwarnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LecW<'_, StatusSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn txok(&mut self) -> TxokW<'_, StatusSpec> {
        TxokW::new(self, 3)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rxok(&mut self) -> RxokW<'_, StatusSpec> {
        RxokW::new(self, 4)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
