#[doc = "Register `NETWORKCFG` reader"]
pub type R = crate::R<NetworkcfgSpec>;
#[doc = "Register `NETWORKCFG` writer"]
pub type W = crate::W<NetworkcfgSpec>;
#[doc = "Field `SPEED` reader - Speed"]
pub type SpeedR = crate::BitReader;
#[doc = "Field `SPEED` writer - Speed"]
pub type SpeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLDUPLEX` reader - Full duplex"]
pub type FullduplexR = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - Full duplex"]
pub type FullduplexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCRDNONVLANFRAMES` reader - Discard non-VLAN frames"]
pub type DiscrdnonvlanframesR = crate::BitReader;
#[doc = "Field `DISCRDNONVLANFRAMES` writer - Discard non-VLAN frames"]
pub type DiscrdnonvlanframesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JUMBOFRAMES` reader - Jumbo frames enable"]
pub type JumboframesR = crate::BitReader;
#[doc = "Field `JUMBOFRAMES` writer - Jumbo frames enable"]
pub type JumboframesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPYALLFRAMES` reader - Copy all frames"]
pub type CopyallframesR = crate::BitReader;
#[doc = "Field `COPYALLFRAMES` writer - Copy all frames"]
pub type CopyallframesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBROADCAST` reader - No broadcast"]
pub type NobroadcastR = crate::BitReader;
#[doc = "Field `NOBROADCAST` writer - No broadcast"]
pub type NobroadcastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICASTHASHEN` reader - Multicast hash enable"]
pub type MulticasthashenR = crate::BitReader;
#[doc = "Field `MULTICASTHASHEN` writer - Multicast hash enable"]
pub type MulticasthashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNICASTHASHEN` reader - Unicast hash enable"]
pub type UnicasthashenR = crate::BitReader;
#[doc = "Field `UNICASTHASHEN` writer - Unicast hash enable"]
pub type UnicasthashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1536BYTEFRAMES` reader - Receive 1536 byte frames"]
pub type Rx1536byteframesR = crate::BitReader;
#[doc = "Field `RX1536BYTEFRAMES` writer - Receive 1536 byte frames"]
pub type Rx1536byteframesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRYTEST` reader - Retry test"]
pub type RetrytestR = crate::BitReader;
#[doc = "Field `RETRYTEST` writer - Retry test"]
pub type RetrytestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSEEN` reader - Pause enable"]
pub type PauseenR = crate::BitReader;
#[doc = "Field `PAUSEEN` writer - Pause enable"]
pub type PauseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFFOFFSET` reader - Receive buffer offset"]
pub type RxbuffoffsetR = crate::FieldReader;
#[doc = "Field `RXBUFFOFFSET` writer - Receive buffer offset"]
pub type RxbuffoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LENFIELDERRFRMDISCRD` reader - Length field error frame discard"]
pub type LenfielderrfrmdiscrdR = crate::BitReader;
#[doc = "Field `LENFIELDERRFRMDISCRD` writer - Length field error frame discard"]
pub type LenfielderrfrmdiscrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSREMOVE` reader - FCS remove"]
pub type FcsremoveR = crate::BitReader;
#[doc = "Field `FCSREMOVE` writer - FCS remove"]
pub type FcsremoveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MDC clock division\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdcclkdiv {
    #[doc = "0: divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    Divby8 = 0,
    #[doc = "1: divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    Divby16 = 1,
    #[doc = "2: divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    Divby32 = 2,
    #[doc = "3: divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    Divby48 = 3,
    #[doc = "4: divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    Divby64 = 4,
    #[doc = "5: divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    Divby96 = 5,
    #[doc = "6: divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    Divby128 = 6,
    #[doc = "7: divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    Divby224 = 7,
}
impl From<Mdcclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Mdcclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdcclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Mdcclkdiv {}
#[doc = "Field `MDCCLKDIV` reader - MDC clock division"]
pub type MdcclkdivR = crate::FieldReader<Mdcclkdiv>;
impl MdcclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdcclkdiv {
        match self.bits {
            0 => Mdcclkdiv::Divby8,
            1 => Mdcclkdiv::Divby16,
            2 => Mdcclkdiv::Divby32,
            3 => Mdcclkdiv::Divby48,
            4 => Mdcclkdiv::Divby64,
            5 => Mdcclkdiv::Divby96,
            6 => Mdcclkdiv::Divby128,
            7 => Mdcclkdiv::Divby224,
            _ => unreachable!(),
        }
    }
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        *self == Mdcclkdiv::Divby8
    }
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        *self == Mdcclkdiv::Divby16
    }
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        *self == Mdcclkdiv::Divby32
    }
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    #[inline(always)]
    pub fn is_divby48(&self) -> bool {
        *self == Mdcclkdiv::Divby48
    }
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        *self == Mdcclkdiv::Divby64
    }
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    #[inline(always)]
    pub fn is_divby96(&self) -> bool {
        *self == Mdcclkdiv::Divby96
    }
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        *self == Mdcclkdiv::Divby128
    }
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    #[inline(always)]
    pub fn is_divby224(&self) -> bool {
        *self == Mdcclkdiv::Divby224
    }
}
#[doc = "Field `MDCCLKDIV` writer - MDC clock division"]
pub type MdcclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mdcclkdiv, crate::Safe>;
impl<'a, REG> MdcclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby8)
    }
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby16)
    }
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby32)
    }
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    #[inline(always)]
    pub fn divby48(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby48)
    }
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby64)
    }
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    #[inline(always)]
    pub fn divby96(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby96)
    }
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby128)
    }
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    #[inline(always)]
    pub fn divby224(self) -> &'a mut crate::W<REG> {
        self.variant(Mdcclkdiv::Divby224)
    }
}
#[doc = "Field `DISCOPYOFPFRAMES` reader - Disable copy of pause frames"]
pub type DiscopyofpframesR = crate::BitReader;
#[doc = "Field `DISCOPYOFPFRAMES` writer - Disable copy of pause frames"]
pub type DiscopyofpframesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCHKSUMOFFLOADEN` reader - Receive checksum offload enable"]
pub type RxchksumoffloadenR = crate::BitReader;
#[doc = "Field `RXCHKSUMOFFLOADEN` writer - Receive checksum offload enable"]
pub type RxchksumoffloadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENHALFDUPLEXRX` reader - Enable frames to be received in half-duplex mode while transmitting."]
pub type EnhalfduplexrxR = crate::BitReader;
#[doc = "Field `ENHALFDUPLEXRX` writer - Enable frames to be received in half-duplex mode while transmitting."]
pub type EnhalfduplexrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORERXFCS` reader - Ignore RX FCS"]
pub type IgnorerxfcsR = crate::BitReader;
#[doc = "Field `IGNORERXFCS` writer - Ignore RX FCS"]
pub type IgnorerxfcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPGSTRTCHEN` reader - IPG stretch enable"]
pub type IpgstrtchenR = crate::BitReader;
#[doc = "Field `IPGSTRTCHEN` writer - IPG stretch enable"]
pub type IpgstrtchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPCHANGE` reader - Receive bad preamble."]
pub type NspchangeR = crate::BitReader;
#[doc = "Field `NSPCHANGE` writer - Receive bad preamble."]
pub type NspchangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNOREIPGRXER` reader - Ignore IPG rx_er."]
pub type IgnoreipgrxerR = crate::BitReader;
#[doc = "Field `IGNOREIPGRXER` writer - Ignore IPG rx_er."]
pub type IgnoreipgrxerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline(always)]
    pub fn fullduplex(&self) -> FullduplexR {
        FullduplexR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline(always)]
    pub fn discrdnonvlanframes(&self) -> DiscrdnonvlanframesR {
        DiscrdnonvlanframesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline(always)]
    pub fn jumboframes(&self) -> JumboframesR {
        JumboframesR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline(always)]
    pub fn copyallframes(&self) -> CopyallframesR {
        CopyallframesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline(always)]
    pub fn nobroadcast(&self) -> NobroadcastR {
        NobroadcastR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline(always)]
    pub fn multicasthashen(&self) -> MulticasthashenR {
        MulticasthashenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline(always)]
    pub fn unicasthashen(&self) -> UnicasthashenR {
        UnicasthashenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline(always)]
    pub fn rx1536byteframes(&self) -> Rx1536byteframesR {
        Rx1536byteframesR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn retrytest(&self) -> RetrytestR {
        RetrytestR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PauseenR {
        PauseenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline(always)]
    pub fn rxbuffoffset(&self) -> RxbuffoffsetR {
        RxbuffoffsetR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline(always)]
    pub fn lenfielderrfrmdiscrd(&self) -> LenfielderrfrmdiscrdR {
        LenfielderrfrmdiscrdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline(always)]
    pub fn fcsremove(&self) -> FcsremoveR {
        FcsremoveR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline(always)]
    pub fn mdcclkdiv(&self) -> MdcclkdivR {
        MdcclkdivR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline(always)]
    pub fn discopyofpframes(&self) -> DiscopyofpframesR {
        DiscopyofpframesR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline(always)]
    pub fn rxchksumoffloaden(&self) -> RxchksumoffloadenR {
        RxchksumoffloadenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    pub fn enhalfduplexrx(&self) -> EnhalfduplexrxR {
        EnhalfduplexrxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn ignorerxfcs(&self) -> IgnorerxfcsR {
        IgnorerxfcsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline(always)]
    pub fn ipgstrtchen(&self) -> IpgstrtchenR {
        IpgstrtchenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline(always)]
    pub fn nspchange(&self) -> NspchangeR {
        NspchangeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline(always)]
    pub fn ignoreipgrxer(&self) -> IgnoreipgrxerR {
        IgnoreipgrxerR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, NetworkcfgSpec> {
        SpeedW::new(self, 0)
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FullduplexW<'_, NetworkcfgSpec> {
        FullduplexW::new(self, 1)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline(always)]
    pub fn discrdnonvlanframes(&mut self) -> DiscrdnonvlanframesW<'_, NetworkcfgSpec> {
        DiscrdnonvlanframesW::new(self, 2)
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline(always)]
    pub fn jumboframes(&mut self) -> JumboframesW<'_, NetworkcfgSpec> {
        JumboframesW::new(self, 3)
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline(always)]
    pub fn copyallframes(&mut self) -> CopyallframesW<'_, NetworkcfgSpec> {
        CopyallframesW::new(self, 4)
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline(always)]
    pub fn nobroadcast(&mut self) -> NobroadcastW<'_, NetworkcfgSpec> {
        NobroadcastW::new(self, 5)
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline(always)]
    pub fn multicasthashen(&mut self) -> MulticasthashenW<'_, NetworkcfgSpec> {
        MulticasthashenW::new(self, 6)
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline(always)]
    pub fn unicasthashen(&mut self) -> UnicasthashenW<'_, NetworkcfgSpec> {
        UnicasthashenW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline(always)]
    pub fn rx1536byteframes(&mut self) -> Rx1536byteframesW<'_, NetworkcfgSpec> {
        Rx1536byteframesW::new(self, 8)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn retrytest(&mut self) -> RetrytestW<'_, NetworkcfgSpec> {
        RetrytestW::new(self, 12)
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline(always)]
    pub fn pauseen(&mut self) -> PauseenW<'_, NetworkcfgSpec> {
        PauseenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline(always)]
    pub fn rxbuffoffset(&mut self) -> RxbuffoffsetW<'_, NetworkcfgSpec> {
        RxbuffoffsetW::new(self, 14)
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline(always)]
    pub fn lenfielderrfrmdiscrd(&mut self) -> LenfielderrfrmdiscrdW<'_, NetworkcfgSpec> {
        LenfielderrfrmdiscrdW::new(self, 16)
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline(always)]
    pub fn fcsremove(&mut self) -> FcsremoveW<'_, NetworkcfgSpec> {
        FcsremoveW::new(self, 17)
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline(always)]
    pub fn mdcclkdiv(&mut self) -> MdcclkdivW<'_, NetworkcfgSpec> {
        MdcclkdivW::new(self, 18)
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline(always)]
    pub fn discopyofpframes(&mut self) -> DiscopyofpframesW<'_, NetworkcfgSpec> {
        DiscopyofpframesW::new(self, 23)
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline(always)]
    pub fn rxchksumoffloaden(&mut self) -> RxchksumoffloadenW<'_, NetworkcfgSpec> {
        RxchksumoffloadenW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    pub fn enhalfduplexrx(&mut self) -> EnhalfduplexrxW<'_, NetworkcfgSpec> {
        EnhalfduplexrxW::new(self, 25)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn ignorerxfcs(&mut self) -> IgnorerxfcsW<'_, NetworkcfgSpec> {
        IgnorerxfcsW::new(self, 26)
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline(always)]
    pub fn ipgstrtchen(&mut self) -> IpgstrtchenW<'_, NetworkcfgSpec> {
        IpgstrtchenW::new(self, 28)
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline(always)]
    pub fn nspchange(&mut self) -> NspchangeW<'_, NetworkcfgSpec> {
        NspchangeW::new(self, 29)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline(always)]
    pub fn ignoreipgrxer(&mut self) -> IgnoreipgrxerW<'_, NetworkcfgSpec> {
        IgnoreipgrxerW::new(self, 30)
    }
}
#[doc = "Network configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`networkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`networkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NetworkcfgSpec;
impl crate::RegisterSpec for NetworkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`networkcfg::R`](R) reader structure"]
impl crate::Readable for NetworkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`networkcfg::W`](W) writer structure"]
impl crate::Writable for NetworkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NETWORKCFG to value 0x0008_0000"]
impl crate::Resettable for NetworkcfgSpec {
    const RESET_VALUE: u32 = 0x0008_0000;
}
