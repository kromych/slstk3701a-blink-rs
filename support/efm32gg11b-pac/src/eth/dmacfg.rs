#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DMACFG_SPEC>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DMACFG_SPEC>;
#[doc = "Field `AMBABRSTLEN` reader - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
pub type AMBABRSTLEN_R = crate::FieldReader;
#[doc = "Field `AMBABRSTLEN` writer - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
pub type AMBABRSTLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `HDRDATASPLITEN` reader - Enable header data Splitting."]
pub type HDRDATASPLITEN_R = crate::BitReader;
#[doc = "Field `HDRDATASPLITEN` writer - Enable header data Splitting."]
pub type HDRDATASPLITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXPBUFSIZE` reader - Receiver packet buffer memory size select."]
pub type RXPBUFSIZE_R = crate::FieldReader<RXPBUFSIZE_A>;
#[doc = "Receiver packet buffer memory size select.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPBUFSIZE_A {
    #[doc = "0: Do not use top three address bits (0.5 Kb)"]
    SIZE0 = 0,
    #[doc = "1: Do not use top two address bits (1 Kb)"]
    SIZE1 = 1,
    #[doc = "2: Do not use top address bit (2 Kb)"]
    SIZE2 = 2,
    #[doc = "3: Use full configured addressable space (4 Kb)"]
    SIZE3 = 3,
}
impl From<RXPBUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPBUFSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXPBUFSIZE_A {
    type Ux = u8;
}
impl RXPBUFSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPBUFSIZE_A {
        match self.bits {
            0 => RXPBUFSIZE_A::SIZE0,
            1 => RXPBUFSIZE_A::SIZE1,
            2 => RXPBUFSIZE_A::SIZE2,
            3 => RXPBUFSIZE_A::SIZE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline(always)]
    pub fn is_size0(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE0
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE1
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE2
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline(always)]
    pub fn is_size3(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE3
    }
}
#[doc = "Field `RXPBUFSIZE` writer - Receiver packet buffer memory size select."]
pub type RXPBUFSIZE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RXPBUFSIZE_A>;
impl<'a, REG, const O: u8> RXPBUFSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline(always)]
    pub fn size0(self) -> &'a mut crate::W<REG> {
        self.variant(RXPBUFSIZE_A::SIZE0)
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline(always)]
    pub fn size1(self) -> &'a mut crate::W<REG> {
        self.variant(RXPBUFSIZE_A::SIZE1)
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline(always)]
    pub fn size2(self) -> &'a mut crate::W<REG> {
        self.variant(RXPBUFSIZE_A::SIZE2)
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline(always)]
    pub fn size3(self) -> &'a mut crate::W<REG> {
        self.variant(RXPBUFSIZE_A::SIZE3)
    }
}
#[doc = "Field `TXPBUFSIZE` reader - Transmitter packet buffer memory size select."]
pub type TXPBUFSIZE_R = crate::BitReader;
#[doc = "Field `TXPBUFSIZE` writer - Transmitter packet buffer memory size select."]
pub type TXPBUFSIZE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPBUFTCPEN` reader - Transmitter IP, TCP and UDP checksum generation offload enable"]
pub type TXPBUFTCPEN_R = crate::BitReader;
#[doc = "Field `TXPBUFTCPEN` writer - Transmitter IP, TCP and UDP checksum generation offload enable"]
pub type TXPBUFTCPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INFLASTDBUFSIZEEN` reader - Forces the DMA"]
pub type INFLASTDBUFSIZEEN_R = crate::BitReader;
#[doc = "Field `INFLASTDBUFSIZEEN` writer - Forces the DMA"]
pub type INFLASTDBUFSIZEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUFSIZE` reader - DMA receive buffer size in external AMBA (AHB) system memory."]
pub type RXBUFSIZE_R = crate::FieldReader;
#[doc = "Field `RXBUFSIZE` writer - DMA receive buffer size in external AMBA (AHB) system memory."]
pub type RXBUFSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FRCDISCARDONERR` reader - Auto Discard RX pkts during lack of resource."]
pub type FRCDISCARDONERR_R = crate::BitReader;
#[doc = "Field `FRCDISCARDONERR` writer - Auto Discard RX pkts during lack of resource."]
pub type FRCDISCARDONERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRCMAXAMBABRSTRX` reader - Force max length bursts on RX."]
pub type FRCMAXAMBABRSTRX_R = crate::BitReader;
#[doc = "Field `FRCMAXAMBABRSTRX` writer - Force max length bursts on RX."]
pub type FRCMAXAMBABRSTRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRCMAXAMBABRSTTX` reader - Force max length bursts on TX."]
pub type FRCMAXAMBABRSTTX_R = crate::BitReader;
#[doc = "Field `FRCMAXAMBABRSTTX` writer - Force max length bursts on TX."]
pub type FRCMAXAMBABRSTTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBDEXTNDMODEEN` reader - Enable RX extended BD mode."]
pub type RXBDEXTNDMODEEN_R = crate::BitReader;
#[doc = "Field `RXBDEXTNDMODEEN` writer - Enable RX extended BD mode."]
pub type RXBDEXTNDMODEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBDEXTENDMODEEN` reader - Enable TX extended BD mode."]
pub type TXBDEXTENDMODEEN_R = crate::BitReader;
#[doc = "Field `TXBDEXTENDMODEEN` writer - Enable TX extended BD mode."]
pub type TXBDEXTENDMODEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&self) -> AMBABRSTLEN_R {
        AMBABRSTLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&self) -> HDRDATASPLITEN_R {
        HDRDATASPLITEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&self) -> RXPBUFSIZE_R {
        RXPBUFSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&self) -> TXPBUFSIZE_R {
        TXPBUFSIZE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&self) -> TXPBUFTCPEN_R {
        TXPBUFTCPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&self) -> INFLASTDBUFSIZEEN_R {
        INFLASTDBUFSIZEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&self) -> RXBUFSIZE_R {
        RXBUFSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&self) -> FRCDISCARDONERR_R {
        FRCDISCARDONERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&self) -> FRCMAXAMBABRSTRX_R {
        FRCMAXAMBABRSTRX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&self) -> FRCMAXAMBABRSTTX_R {
        FRCMAXAMBABRSTTX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&self) -> RXBDEXTNDMODEEN_R {
        RXBDEXTNDMODEEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&self) -> TXBDEXTENDMODEEN_R {
        TXBDEXTENDMODEEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    #[must_use]
    pub fn ambabrstlen(&mut self) -> AMBABRSTLEN_W<DMACFG_SPEC, 0> {
        AMBABRSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    #[must_use]
    pub fn hdrdataspliten(&mut self) -> HDRDATASPLITEN_W<DMACFG_SPEC, 5> {
        HDRDATASPLITEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    #[must_use]
    pub fn rxpbufsize(&mut self) -> RXPBUFSIZE_W<DMACFG_SPEC, 8> {
        RXPBUFSIZE_W::new(self)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    #[must_use]
    pub fn txpbufsize(&mut self) -> TXPBUFSIZE_W<DMACFG_SPEC, 10> {
        TXPBUFSIZE_W::new(self)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    #[must_use]
    pub fn txpbuftcpen(&mut self) -> TXPBUFTCPEN_W<DMACFG_SPEC, 11> {
        TXPBUFTCPEN_W::new(self)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    #[must_use]
    pub fn inflastdbufsizeen(&mut self) -> INFLASTDBUFSIZEEN_W<DMACFG_SPEC, 12> {
        INFLASTDBUFSIZEEN_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    #[must_use]
    pub fn rxbufsize(&mut self) -> RXBUFSIZE_W<DMACFG_SPEC, 16> {
        RXBUFSIZE_W::new(self)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    #[must_use]
    pub fn frcdiscardonerr(&mut self) -> FRCDISCARDONERR_W<DMACFG_SPEC, 24> {
        FRCDISCARDONERR_W::new(self)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    #[must_use]
    pub fn frcmaxambabrstrx(&mut self) -> FRCMAXAMBABRSTRX_W<DMACFG_SPEC, 25> {
        FRCMAXAMBABRSTRX_W::new(self)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    #[must_use]
    pub fn frcmaxambabrsttx(&mut self) -> FRCMAXAMBABRSTTX_W<DMACFG_SPEC, 26> {
        FRCMAXAMBABRSTTX_W::new(self)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxbdextndmodeen(&mut self) -> RXBDEXTNDMODEEN_W<DMACFG_SPEC, 28> {
        RXBDEXTNDMODEEN_W::new(self)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    #[must_use]
    pub fn txbdextendmodeen(&mut self) -> TXBDEXTENDMODEEN_W<DMACFG_SPEC, 29> {
        TXBDEXTENDMODEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DMACFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACFG to value 0x0002_0704"]
impl crate::Resettable for DMACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0704;
}
