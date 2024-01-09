#[doc = "Register `NETWORKCFG` reader"]
pub type R = crate::R<NETWORKCFG_SPEC>;
#[doc = "Register `NETWORKCFG` writer"]
pub type W = crate::W<NETWORKCFG_SPEC>;
#[doc = "Field `SPEED` reader - Speed"]
pub type SPEED_R = crate::BitReader;
#[doc = "Field `SPEED` writer - Speed"]
pub type SPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLDUPLEX` reader - Full duplex"]
pub type FULLDUPLEX_R = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - Full duplex"]
pub type FULLDUPLEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCRDNONVLANFRAMES` reader - Discard non-VLAN frames"]
pub type DISCRDNONVLANFRAMES_R = crate::BitReader;
#[doc = "Field `DISCRDNONVLANFRAMES` writer - Discard non-VLAN frames"]
pub type DISCRDNONVLANFRAMES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JUMBOFRAMES` reader - Jumbo frames enable"]
pub type JUMBOFRAMES_R = crate::BitReader;
#[doc = "Field `JUMBOFRAMES` writer - Jumbo frames enable"]
pub type JUMBOFRAMES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPYALLFRAMES` reader - Copy all frames"]
pub type COPYALLFRAMES_R = crate::BitReader;
#[doc = "Field `COPYALLFRAMES` writer - Copy all frames"]
pub type COPYALLFRAMES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBROADCAST` reader - No broadcast"]
pub type NOBROADCAST_R = crate::BitReader;
#[doc = "Field `NOBROADCAST` writer - No broadcast"]
pub type NOBROADCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICASTHASHEN` reader - Multicast hash enable"]
pub type MULTICASTHASHEN_R = crate::BitReader;
#[doc = "Field `MULTICASTHASHEN` writer - Multicast hash enable"]
pub type MULTICASTHASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNICASTHASHEN` reader - Unicast hash enable"]
pub type UNICASTHASHEN_R = crate::BitReader;
#[doc = "Field `UNICASTHASHEN` writer - Unicast hash enable"]
pub type UNICASTHASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1536BYTEFRAMES` reader - Receive 1536 byte frames"]
pub type RX1536BYTEFRAMES_R = crate::BitReader;
#[doc = "Field `RX1536BYTEFRAMES` writer - Receive 1536 byte frames"]
pub type RX1536BYTEFRAMES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRYTEST` reader - Retry test"]
pub type RETRYTEST_R = crate::BitReader;
#[doc = "Field `RETRYTEST` writer - Retry test"]
pub type RETRYTEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSEEN` reader - Pause enable"]
pub type PAUSEEN_R = crate::BitReader;
#[doc = "Field `PAUSEEN` writer - Pause enable"]
pub type PAUSEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFFOFFSET` reader - Receive buffer offset"]
pub type RXBUFFOFFSET_R = crate::FieldReader;
#[doc = "Field `RXBUFFOFFSET` writer - Receive buffer offset"]
pub type RXBUFFOFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LENFIELDERRFRMDISCRD` reader - Length field error frame discard"]
pub type LENFIELDERRFRMDISCRD_R = crate::BitReader;
#[doc = "Field `LENFIELDERRFRMDISCRD` writer - Length field error frame discard"]
pub type LENFIELDERRFRMDISCRD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSREMOVE` reader - FCS remove"]
pub type FCSREMOVE_R = crate::BitReader;
#[doc = "Field `FCSREMOVE` writer - FCS remove"]
pub type FCSREMOVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDCCLKDIV` reader - MDC clock division"]
pub type MDCCLKDIV_R = crate::FieldReader<MDCCLKDIV_A>;
#[doc = "MDC clock division\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDCCLKDIV_A {
    #[doc = "0: divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    DIVBY8 = 0,
    #[doc = "1: divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    DIVBY16 = 1,
    #[doc = "2: divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    DIVBY32 = 2,
    #[doc = "3: divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    DIVBY48 = 3,
    #[doc = "4: divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    DIVBY64 = 4,
    #[doc = "5: divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    DIVBY96 = 5,
    #[doc = "6: divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    DIVBY128 = 6,
    #[doc = "7: divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    DIVBY224 = 7,
}
impl From<MDCCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MDCCLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDCCLKDIV_A {
    type Ux = u8;
}
impl MDCCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDCCLKDIV_A {
        match self.bits {
            0 => MDCCLKDIV_A::DIVBY8,
            1 => MDCCLKDIV_A::DIVBY16,
            2 => MDCCLKDIV_A::DIVBY32,
            3 => MDCCLKDIV_A::DIVBY48,
            4 => MDCCLKDIV_A::DIVBY64,
            5 => MDCCLKDIV_A::DIVBY96,
            6 => MDCCLKDIV_A::DIVBY128,
            7 => MDCCLKDIV_A::DIVBY224,
            _ => unreachable!(),
        }
    }
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY8
    }
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY16
    }
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY32
    }
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    #[inline(always)]
    pub fn is_divby48(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY48
    }
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY64
    }
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    #[inline(always)]
    pub fn is_divby96(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY96
    }
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY128
    }
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    #[inline(always)]
    pub fn is_divby224(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY224
    }
}
#[doc = "Field `MDCCLKDIV` writer - MDC clock division"]
pub type MDCCLKDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MDCCLKDIV_A>;
impl<'a, REG> MDCCLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY8)
    }
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY16)
    }
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY32)
    }
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    #[inline(always)]
    pub fn divby48(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY48)
    }
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY64)
    }
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    #[inline(always)]
    pub fn divby96(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY96)
    }
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY128)
    }
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    #[inline(always)]
    pub fn divby224(self) -> &'a mut crate::W<REG> {
        self.variant(MDCCLKDIV_A::DIVBY224)
    }
}
#[doc = "Field `DISCOPYOFPFRAMES` reader - Disable copy of pause frames"]
pub type DISCOPYOFPFRAMES_R = crate::BitReader;
#[doc = "Field `DISCOPYOFPFRAMES` writer - Disable copy of pause frames"]
pub type DISCOPYOFPFRAMES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCHKSUMOFFLOADEN` reader - Receive checksum offload enable"]
pub type RXCHKSUMOFFLOADEN_R = crate::BitReader;
#[doc = "Field `RXCHKSUMOFFLOADEN` writer - Receive checksum offload enable"]
pub type RXCHKSUMOFFLOADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENHALFDUPLEXRX` reader - Enable frames to be received in half-duplex mode while transmitting."]
pub type ENHALFDUPLEXRX_R = crate::BitReader;
#[doc = "Field `ENHALFDUPLEXRX` writer - Enable frames to be received in half-duplex mode while transmitting."]
pub type ENHALFDUPLEXRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORERXFCS` reader - Ignore RX FCS"]
pub type IGNORERXFCS_R = crate::BitReader;
#[doc = "Field `IGNORERXFCS` writer - Ignore RX FCS"]
pub type IGNORERXFCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPGSTRTCHEN` reader - IPG stretch enable"]
pub type IPGSTRTCHEN_R = crate::BitReader;
#[doc = "Field `IPGSTRTCHEN` writer - IPG stretch enable"]
pub type IPGSTRTCHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPCHANGE` reader - Receive bad preamble."]
pub type NSPCHANGE_R = crate::BitReader;
#[doc = "Field `NSPCHANGE` writer - Receive bad preamble."]
pub type NSPCHANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREIPGRXER` reader - Ignore IPG rx_er."]
pub type IGNOREIPGRXER_R = crate::BitReader;
#[doc = "Field `IGNOREIPGRXER` writer - Ignore IPG rx_er."]
pub type IGNOREIPGRXER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline(always)]
    pub fn discrdnonvlanframes(&self) -> DISCRDNONVLANFRAMES_R {
        DISCRDNONVLANFRAMES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline(always)]
    pub fn jumboframes(&self) -> JUMBOFRAMES_R {
        JUMBOFRAMES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline(always)]
    pub fn copyallframes(&self) -> COPYALLFRAMES_R {
        COPYALLFRAMES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline(always)]
    pub fn nobroadcast(&self) -> NOBROADCAST_R {
        NOBROADCAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline(always)]
    pub fn multicasthashen(&self) -> MULTICASTHASHEN_R {
        MULTICASTHASHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline(always)]
    pub fn unicasthashen(&self) -> UNICASTHASHEN_R {
        UNICASTHASHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline(always)]
    pub fn rx1536byteframes(&self) -> RX1536BYTEFRAMES_R {
        RX1536BYTEFRAMES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn retrytest(&self) -> RETRYTEST_R {
        RETRYTEST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PAUSEEN_R {
        PAUSEEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline(always)]
    pub fn rxbuffoffset(&self) -> RXBUFFOFFSET_R {
        RXBUFFOFFSET_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline(always)]
    pub fn lenfielderrfrmdiscrd(&self) -> LENFIELDERRFRMDISCRD_R {
        LENFIELDERRFRMDISCRD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline(always)]
    pub fn fcsremove(&self) -> FCSREMOVE_R {
        FCSREMOVE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline(always)]
    pub fn mdcclkdiv(&self) -> MDCCLKDIV_R {
        MDCCLKDIV_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline(always)]
    pub fn discopyofpframes(&self) -> DISCOPYOFPFRAMES_R {
        DISCOPYOFPFRAMES_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline(always)]
    pub fn rxchksumoffloaden(&self) -> RXCHKSUMOFFLOADEN_R {
        RXCHKSUMOFFLOADEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    pub fn enhalfduplexrx(&self) -> ENHALFDUPLEXRX_R {
        ENHALFDUPLEXRX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn ignorerxfcs(&self) -> IGNORERXFCS_R {
        IGNORERXFCS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline(always)]
    pub fn ipgstrtchen(&self) -> IPGSTRTCHEN_R {
        IPGSTRTCHEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline(always)]
    pub fn nspchange(&self) -> NSPCHANGE_R {
        NSPCHANGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline(always)]
    pub fn ignoreipgrxer(&self) -> IGNOREIPGRXER_R {
        IGNOREIPGRXER_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<NETWORKCFG_SPEC> {
        SPEED_W::new(self, 0)
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W<NETWORKCFG_SPEC> {
        FULLDUPLEX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline(always)]
    #[must_use]
    pub fn discrdnonvlanframes(&mut self) -> DISCRDNONVLANFRAMES_W<NETWORKCFG_SPEC> {
        DISCRDNONVLANFRAMES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline(always)]
    #[must_use]
    pub fn jumboframes(&mut self) -> JUMBOFRAMES_W<NETWORKCFG_SPEC> {
        JUMBOFRAMES_W::new(self, 3)
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline(always)]
    #[must_use]
    pub fn copyallframes(&mut self) -> COPYALLFRAMES_W<NETWORKCFG_SPEC> {
        COPYALLFRAMES_W::new(self, 4)
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn nobroadcast(&mut self) -> NOBROADCAST_W<NETWORKCFG_SPEC> {
        NOBROADCAST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline(always)]
    #[must_use]
    pub fn multicasthashen(&mut self) -> MULTICASTHASHEN_W<NETWORKCFG_SPEC> {
        MULTICASTHASHEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline(always)]
    #[must_use]
    pub fn unicasthashen(&mut self) -> UNICASTHASHEN_W<NETWORKCFG_SPEC> {
        UNICASTHASHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline(always)]
    #[must_use]
    pub fn rx1536byteframes(&mut self) -> RX1536BYTEFRAMES_W<NETWORKCFG_SPEC> {
        RX1536BYTEFRAMES_W::new(self, 8)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    #[must_use]
    pub fn retrytest(&mut self) -> RETRYTEST_W<NETWORKCFG_SPEC> {
        RETRYTEST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline(always)]
    #[must_use]
    pub fn pauseen(&mut self) -> PAUSEEN_W<NETWORKCFG_SPEC> {
        PAUSEEN_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuffoffset(&mut self) -> RXBUFFOFFSET_W<NETWORKCFG_SPEC> {
        RXBUFFOFFSET_W::new(self, 14)
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline(always)]
    #[must_use]
    pub fn lenfielderrfrmdiscrd(&mut self) -> LENFIELDERRFRMDISCRD_W<NETWORKCFG_SPEC> {
        LENFIELDERRFRMDISCRD_W::new(self, 16)
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline(always)]
    #[must_use]
    pub fn fcsremove(&mut self) -> FCSREMOVE_W<NETWORKCFG_SPEC> {
        FCSREMOVE_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline(always)]
    #[must_use]
    pub fn mdcclkdiv(&mut self) -> MDCCLKDIV_W<NETWORKCFG_SPEC> {
        MDCCLKDIV_W::new(self, 18)
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline(always)]
    #[must_use]
    pub fn discopyofpframes(&mut self) -> DISCOPYOFPFRAMES_W<NETWORKCFG_SPEC> {
        DISCOPYOFPFRAMES_W::new(self, 23)
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxchksumoffloaden(&mut self) -> RXCHKSUMOFFLOADEN_W<NETWORKCFG_SPEC> {
        RXCHKSUMOFFLOADEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    #[must_use]
    pub fn enhalfduplexrx(&mut self) -> ENHALFDUPLEXRX_W<NETWORKCFG_SPEC> {
        ENHALFDUPLEXRX_W::new(self, 25)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    #[must_use]
    pub fn ignorerxfcs(&mut self) -> IGNORERXFCS_W<NETWORKCFG_SPEC> {
        IGNORERXFCS_W::new(self, 26)
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipgstrtchen(&mut self) -> IPGSTRTCHEN_W<NETWORKCFG_SPEC> {
        IPGSTRTCHEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline(always)]
    #[must_use]
    pub fn nspchange(&mut self) -> NSPCHANGE_W<NETWORKCFG_SPEC> {
        NSPCHANGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline(always)]
    #[must_use]
    pub fn ignoreipgrxer(&mut self) -> IGNOREIPGRXER_W<NETWORKCFG_SPEC> {
        IGNOREIPGRXER_W::new(self, 30)
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
#[doc = "Network configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`networkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`networkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NETWORKCFG_SPEC;
impl crate::RegisterSpec for NETWORKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`networkcfg::R`](R) reader structure"]
impl crate::Readable for NETWORKCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`networkcfg::W`](W) writer structure"]
impl crate::Writable for NETWORKCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NETWORKCFG to value 0x0008_0000"]
impl crate::Resettable for NETWORKCFG_SPEC {
    const RESET_VALUE: u32 = 0x0008_0000;
}
