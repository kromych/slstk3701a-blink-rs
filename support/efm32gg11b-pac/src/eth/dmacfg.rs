#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DmacfgSpec>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DmacfgSpec>;
#[doc = "Field `AMBABRSTLEN` reader - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
pub type AmbabrstlenR = crate::FieldReader;
#[doc = "Field `AMBABRSTLEN` writer - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
pub type AmbabrstlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HDRDATASPLITEN` reader - Enable header data Splitting."]
pub type HdrdatasplitenR = crate::BitReader;
#[doc = "Field `HDRDATASPLITEN` writer - Enable header data Splitting."]
pub type HdrdatasplitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receiver packet buffer memory size select.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxpbufsize {
    #[doc = "0: Do not use top three address bits (0.5 Kb)"]
    Size0 = 0,
    #[doc = "1: Do not use top two address bits (1 Kb)"]
    Size1 = 1,
    #[doc = "2: Do not use top address bit (2 Kb)"]
    Size2 = 2,
    #[doc = "3: Use full configured addressable space (4 Kb)"]
    Size3 = 3,
}
impl From<Rxpbufsize> for u8 {
    #[inline(always)]
    fn from(variant: Rxpbufsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxpbufsize {
    type Ux = u8;
}
impl crate::IsEnum for Rxpbufsize {}
#[doc = "Field `RXPBUFSIZE` reader - Receiver packet buffer memory size select."]
pub type RxpbufsizeR = crate::FieldReader<Rxpbufsize>;
impl RxpbufsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxpbufsize {
        match self.bits {
            0 => Rxpbufsize::Size0,
            1 => Rxpbufsize::Size1,
            2 => Rxpbufsize::Size2,
            3 => Rxpbufsize::Size3,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline(always)]
    pub fn is_size0(&self) -> bool {
        *self == Rxpbufsize::Size0
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == Rxpbufsize::Size1
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == Rxpbufsize::Size2
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline(always)]
    pub fn is_size3(&self) -> bool {
        *self == Rxpbufsize::Size3
    }
}
#[doc = "Field `RXPBUFSIZE` writer - Receiver packet buffer memory size select."]
pub type RxpbufsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxpbufsize, crate::Safe>;
impl<'a, REG> RxpbufsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline(always)]
    pub fn size0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpbufsize::Size0)
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline(always)]
    pub fn size1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpbufsize::Size1)
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline(always)]
    pub fn size2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpbufsize::Size2)
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline(always)]
    pub fn size3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxpbufsize::Size3)
    }
}
#[doc = "Field `TXPBUFSIZE` reader - Transmitter packet buffer memory size select."]
pub type TxpbufsizeR = crate::BitReader;
#[doc = "Field `TXPBUFSIZE` writer - Transmitter packet buffer memory size select."]
pub type TxpbufsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPBUFTCPEN` reader - Transmitter IP, TCP and UDP checksum generation offload enable"]
pub type TxpbuftcpenR = crate::BitReader;
#[doc = "Field `TXPBUFTCPEN` writer - Transmitter IP, TCP and UDP checksum generation offload enable"]
pub type TxpbuftcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFLASTDBUFSIZEEN` reader - Forces the DMA"]
pub type InflastdbufsizeenR = crate::BitReader;
#[doc = "Field `INFLASTDBUFSIZEEN` writer - Forces the DMA"]
pub type InflastdbufsizeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFSIZE` reader - DMA receive buffer size in external AMBA (AHB) system memory."]
pub type RxbufsizeR = crate::FieldReader;
#[doc = "Field `RXBUFSIZE` writer - DMA receive buffer size in external AMBA (AHB) system memory."]
pub type RxbufsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FRCDISCARDONERR` reader - Auto Discard RX pkts during lack of resource."]
pub type FrcdiscardonerrR = crate::BitReader;
#[doc = "Field `FRCDISCARDONERR` writer - Auto Discard RX pkts during lack of resource."]
pub type FrcdiscardonerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCMAXAMBABRSTRX` reader - Force max length bursts on RX."]
pub type FrcmaxambabrstrxR = crate::BitReader;
#[doc = "Field `FRCMAXAMBABRSTRX` writer - Force max length bursts on RX."]
pub type FrcmaxambabrstrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCMAXAMBABRSTTX` reader - Force max length bursts on TX."]
pub type FrcmaxambabrsttxR = crate::BitReader;
#[doc = "Field `FRCMAXAMBABRSTTX` writer - Force max length bursts on TX."]
pub type FrcmaxambabrsttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBDEXTNDMODEEN` reader - Enable RX extended BD mode."]
pub type RxbdextndmodeenR = crate::BitReader;
#[doc = "Field `RXBDEXTNDMODEEN` writer - Enable RX extended BD mode."]
pub type RxbdextndmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBDEXTENDMODEEN` reader - Enable TX extended BD mode."]
pub type TxbdextendmodeenR = crate::BitReader;
#[doc = "Field `TXBDEXTENDMODEEN` writer - Enable TX extended BD mode."]
pub type TxbdextendmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&self) -> AmbabrstlenR {
        AmbabrstlenR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&self) -> HdrdatasplitenR {
        HdrdatasplitenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&self) -> RxpbufsizeR {
        RxpbufsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&self) -> TxpbufsizeR {
        TxpbufsizeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&self) -> TxpbuftcpenR {
        TxpbuftcpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&self) -> InflastdbufsizeenR {
        InflastdbufsizeenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&self) -> RxbufsizeR {
        RxbufsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&self) -> FrcdiscardonerrR {
        FrcdiscardonerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&self) -> FrcmaxambabrstrxR {
        FrcmaxambabrstrxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&self) -> FrcmaxambabrsttxR {
        FrcmaxambabrsttxR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&self) -> RxbdextndmodeenR {
        RxbdextndmodeenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&self) -> TxbdextendmodeenR {
        TxbdextendmodeenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&mut self) -> AmbabrstlenW<'_, DmacfgSpec> {
        AmbabrstlenW::new(self, 0)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&mut self) -> HdrdatasplitenW<'_, DmacfgSpec> {
        HdrdatasplitenW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&mut self) -> RxpbufsizeW<'_, DmacfgSpec> {
        RxpbufsizeW::new(self, 8)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&mut self) -> TxpbufsizeW<'_, DmacfgSpec> {
        TxpbufsizeW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&mut self) -> TxpbuftcpenW<'_, DmacfgSpec> {
        TxpbuftcpenW::new(self, 11)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&mut self) -> InflastdbufsizeenW<'_, DmacfgSpec> {
        InflastdbufsizeenW::new(self, 12)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&mut self) -> RxbufsizeW<'_, DmacfgSpec> {
        RxbufsizeW::new(self, 16)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&mut self) -> FrcdiscardonerrW<'_, DmacfgSpec> {
        FrcdiscardonerrW::new(self, 24)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&mut self) -> FrcmaxambabrstrxW<'_, DmacfgSpec> {
        FrcmaxambabrstrxW::new(self, 25)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&mut self) -> FrcmaxambabrsttxW<'_, DmacfgSpec> {
        FrcmaxambabrsttxW::new(self, 26)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&mut self) -> RxbdextndmodeenW<'_, DmacfgSpec> {
        RxbdextndmodeenW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&mut self) -> TxbdextendmodeenW<'_, DmacfgSpec> {
        TxbdextendmodeenW::new(self, 29)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgSpec;
impl crate::RegisterSpec for DmacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DmacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DmacfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACFG to value 0x0002_0704"]
impl crate::Resettable for DmacfgSpec {
    const RESET_VALUE: u32 = 0x0002_0704;
}
