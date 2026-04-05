#[doc = "Register `HFBUSCLKEN0` reader"]
pub type R = crate::R<Hfbusclken0Spec>;
#[doc = "Register `HFBUSCLKEN0` writer"]
pub type W = crate::W<Hfbusclken0Spec>;
#[doc = "Field `LE` reader - Low Energy Peripheral Interface Clock Enable"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - Low Energy Peripheral Interface Clock Enable"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO0` reader - Advanced Encryption Standard Accelerator Clock Enable"]
pub type Crypto0R = crate::BitReader;
#[doc = "Field `CRYPTO0` writer - Advanced Encryption Standard Accelerator Clock Enable"]
pub type Crypto0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBI` reader - External Bus Interface Clock Enable"]
pub type EbiR = crate::BitReader;
#[doc = "Field `EBI` writer - External Bus Interface Clock Enable"]
pub type EbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH` reader - Ethernet Controller Clock Enable"]
pub type EthR = crate::BitReader;
#[doc = "Field `ETH` writer - Ethernet Controller Clock Enable"]
pub type EthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO` reader - SDIO Controller Clock Enable"]
pub type SdioR = crate::BitReader;
#[doc = "Field `SDIO` writer - SDIO Controller Clock Enable"]
pub type SdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - General purpose Input/Output Clock Enable"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - General purpose Input/Output Clock Enable"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - Peripheral Reflex System Clock Enable"]
pub type PrsR = crate::BitReader;
#[doc = "Field `PRS` writer - Peripheral Reflex System Clock Enable"]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - Linked Direct Memory Access Controller Clock Enable"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - Linked Direct Memory Access Controller Clock Enable"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPCRC` reader - General Purpose CRC Clock Enable"]
pub type GpcrcR = crate::BitReader;
#[doc = "Field `GPCRC` writer - General Purpose CRC Clock Enable"]
pub type GpcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI0` reader - Quad-SPI Clock Enable"]
pub type Qspi0R = crate::BitReader;
#[doc = "Field `QSPI0` writer - Quad-SPI Clock Enable"]
pub type Qspi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - Universal Serial Bus Interface Clock Enable"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - Universal Serial Bus Interface Clock Enable"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn crypto0(&self) -> Crypto0R {
        Crypto0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn ebi(&self) -> EbiR {
        EbiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ethernet Controller Clock Enable"]
    #[inline(always)]
    pub fn eth(&self) -> EthR {
        EthR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDIO Controller Clock Enable"]
    #[inline(always)]
    pub fn sdio(&self) -> SdioR {
        SdioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GpcrcR {
        GpcrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quad-SPI Clock Enable"]
    #[inline(always)]
    pub fn qspi0(&self) -> Qspi0R {
        Qspi0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<'_, Hfbusclken0Spec> {
        LeW::new(self, 0)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn crypto0(&mut self) -> Crypto0W<'_, Hfbusclken0Spec> {
        Crypto0W::new(self, 1)
    }
    #[doc = "Bit 2 - External Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EbiW<'_, Hfbusclken0Spec> {
        EbiW::new(self, 2)
    }
    #[doc = "Bit 3 - Ethernet Controller Clock Enable"]
    #[inline(always)]
    pub fn eth(&mut self) -> EthW<'_, Hfbusclken0Spec> {
        EthW::new(self, 3)
    }
    #[doc = "Bit 4 - SDIO Controller Clock Enable"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SdioW<'_, Hfbusclken0Spec> {
        SdioW::new(self, 4)
    }
    #[doc = "Bit 5 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<'_, Hfbusclken0Spec> {
        GpioW::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, Hfbusclken0Spec> {
        PrsW::new(self, 6)
    }
    #[doc = "Bit 7 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<'_, Hfbusclken0Spec> {
        LdmaW::new(self, 7)
    }
    #[doc = "Bit 8 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    pub fn gpcrc(&mut self) -> GpcrcW<'_, Hfbusclken0Spec> {
        GpcrcW::new(self, 8)
    }
    #[doc = "Bit 9 - Quad-SPI Clock Enable"]
    #[inline(always)]
    pub fn qspi0(&mut self) -> Qspi0W<'_, Hfbusclken0Spec> {
        Qspi0W::new(self, 9)
    }
    #[doc = "Bit 10 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, Hfbusclken0Spec> {
        UsbW::new(self, 10)
    }
}
#[doc = "High Frequency Bus Clock Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hfbusclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfbusclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfbusclken0Spec;
impl crate::RegisterSpec for Hfbusclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfbusclken0::R`](R) reader structure"]
impl crate::Readable for Hfbusclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`hfbusclken0::W`](W) writer structure"]
impl crate::Writable for Hfbusclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFBUSCLKEN0 to value 0"]
impl crate::Resettable for Hfbusclken0Spec {}
