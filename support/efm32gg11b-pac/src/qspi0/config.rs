#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `ENBSPI` reader - QSPI Enable"]
pub type EnbspiR = crate::BitReader;
#[doc = "Field `ENBSPI` writer - QSPI Enable"]
pub type EnbspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELCLKPOL` reader - Clock Polarity, CPOL"]
pub type SelclkpolR = crate::BitReader;
#[doc = "Field `SELCLKPOL` writer - Clock Polarity, CPOL"]
pub type SelclkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELCLKPHASE` reader - Clock Phase, CPHA"]
pub type SelclkphaseR = crate::BitReader;
#[doc = "Field `SELCLKPHASE` writer - Clock Phase, CPHA"]
pub type SelclkphaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYMODEENABLE` reader - PHY Mode Enable"]
pub type PhymodeenableR = crate::BitReader;
#[doc = "Field `PHYMODEENABLE` writer - PHY Mode Enable"]
pub type PhymodeenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBDIRACCCTLR` reader - Enable Direct Access Controller"]
pub type EnbdiraccctlrR = crate::BitReader;
#[doc = "Field `ENBDIRACCCTLR` writer - Enable Direct Access Controller"]
pub type EnbdiraccctlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBLEGACYIPMODE` reader - Legacy IP Mode Enable"]
pub type EnblegacyipmodeR = crate::BitReader;
#[doc = "Field `ENBLEGACYIPMODE` writer - Legacy IP Mode Enable"]
pub type EnblegacyipmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIPHSELDEC` reader - Peripheral Select Decode"]
pub type PeriphseldecR = crate::BitReader;
#[doc = "Field `PERIPHSELDEC` writer - Peripheral Select Decode"]
pub type PeriphseldecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIPHCSLINES` reader - Peripheral Chip Select Lines"]
pub type PeriphcslinesR = crate::FieldReader;
#[doc = "Field `PERIPHCSLINES` writer - Peripheral Chip Select Lines"]
pub type PeriphcslinesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRPROTFLASH` reader - Write Protect Flash Pin"]
pub type WrprotflashR = crate::BitReader;
#[doc = "Field `WRPROTFLASH` writer - Write Protect Flash Pin"]
pub type WrprotflashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBAHBADDRREMAP` reader - Enable Address Remapping"]
pub type EnbahbaddrremapR = crate::BitReader;
#[doc = "Field `ENBAHBADDRREMAP` writer - Enable Address Remapping"]
pub type EnbahbaddrremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTERXIPMODE` reader - Enter XIP Mode on Next READ"]
pub type EnterxipmodeR = crate::BitReader;
#[doc = "Field `ENTERXIPMODE` writer - Enter XIP Mode on Next READ"]
pub type EnterxipmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTERXIPMODEIMM` reader - Enter XIP Mode Immediately"]
pub type EnterxipmodeimmR = crate::BitReader;
#[doc = "Field `ENTERXIPMODEIMM` writer - Enter XIP Mode Immediately"]
pub type EnterxipmodeimmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTRBAUDDIV` reader - Master Mode Baud Rate Divisor"]
pub type MstrbauddivR = crate::FieldReader;
#[doc = "Field `MSTRBAUDDIV` writer - Master Mode Baud Rate Divisor"]
pub type MstrbauddivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENABLEAHBDECODER` reader - Enable Address Decoder"]
pub type EnableahbdecoderR = crate::BitReader;
#[doc = "Field `ENABLEAHBDECODER` writer - Enable Address Decoder"]
pub type EnableahbdecoderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEDTRPROTOCOL` reader - Enable DTR Protocol"]
pub type EnabledtrprotocolR = crate::BitReader;
#[doc = "Field `ENABLEDTRPROTOCOL` writer - Enable DTR Protocol"]
pub type EnabledtrprotocolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPELINEPHY` reader - Pipeline PHY Mode Enable"]
pub type PipelinephyR = crate::BitReader;
#[doc = "Field `PIPELINEPHY` writer - Pipeline PHY Mode Enable"]
pub type PipelinephyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCENABLE` reader - CRC Enable Bit"]
pub type CrcenableR = crate::BitReader;
#[doc = "Field `CRCENABLE` writer - CRC Enable Bit"]
pub type CrcenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALBYTEOPCODEEN` reader - Dual-byte Opcode Mode Enable Bit"]
pub type DualbyteopcodeenR = crate::BitReader;
#[doc = "Field `DUALBYTEOPCODEEN` writer - Dual-byte Opcode Mode Enable Bit"]
pub type DualbyteopcodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - Serial Interface and Low Level SPI Pipeline is IDLE"]
pub type IdleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn enbspi(&self) -> EnbspiR {
        EnbspiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    pub fn selclkpol(&self) -> SelclkpolR {
        SelclkpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    pub fn selclkphase(&self) -> SelclkphaseR {
        SelclkphaseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    pub fn phymodeenable(&self) -> PhymodeenableR {
        PhymodeenableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn enbdiraccctlr(&self) -> EnbdiraccctlrR {
        EnbdiraccctlrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn enblegacyipmode(&self) -> EnblegacyipmodeR {
        EnblegacyipmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    pub fn periphseldec(&self) -> PeriphseldecR {
        PeriphseldecR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    pub fn periphcslines(&self) -> PeriphcslinesR {
        PeriphcslinesR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    pub fn wrprotflash(&self) -> WrprotflashR {
        WrprotflashR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    pub fn enbahbaddrremap(&self) -> EnbahbaddrremapR {
        EnbahbaddrremapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    pub fn enterxipmode(&self) -> EnterxipmodeR {
        EnterxipmodeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    pub fn enterxipmodeimm(&self) -> EnterxipmodeimmR {
        EnterxipmodeimmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    pub fn mstrbauddiv(&self) -> MstrbauddivR {
        MstrbauddivR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    pub fn enableahbdecoder(&self) -> EnableahbdecoderR {
        EnableahbdecoderR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn enabledtrprotocol(&self) -> EnabledtrprotocolR {
        EnabledtrprotocolR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    pub fn pipelinephy(&self) -> PipelinephyR {
        PipelinephyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    pub fn crcenable(&self) -> CrcenableR {
        CrcenableR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    pub fn dualbyteopcodeen(&self) -> DualbyteopcodeenR {
        DualbyteopcodeenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Serial Interface and Low Level SPI Pipeline is IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn enbspi(&mut self) -> EnbspiW<'_, ConfigSpec> {
        EnbspiW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    pub fn selclkpol(&mut self) -> SelclkpolW<'_, ConfigSpec> {
        SelclkpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    pub fn selclkphase(&mut self) -> SelclkphaseW<'_, ConfigSpec> {
        SelclkphaseW::new(self, 2)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    pub fn phymodeenable(&mut self) -> PhymodeenableW<'_, ConfigSpec> {
        PhymodeenableW::new(self, 3)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn enbdiraccctlr(&mut self) -> EnbdiraccctlrW<'_, ConfigSpec> {
        EnbdiraccctlrW::new(self, 7)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn enblegacyipmode(&mut self) -> EnblegacyipmodeW<'_, ConfigSpec> {
        EnblegacyipmodeW::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    pub fn periphseldec(&mut self) -> PeriphseldecW<'_, ConfigSpec> {
        PeriphseldecW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    pub fn periphcslines(&mut self) -> PeriphcslinesW<'_, ConfigSpec> {
        PeriphcslinesW::new(self, 10)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    pub fn wrprotflash(&mut self) -> WrprotflashW<'_, ConfigSpec> {
        WrprotflashW::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    pub fn enbahbaddrremap(&mut self) -> EnbahbaddrremapW<'_, ConfigSpec> {
        EnbahbaddrremapW::new(self, 16)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    pub fn enterxipmode(&mut self) -> EnterxipmodeW<'_, ConfigSpec> {
        EnterxipmodeW::new(self, 17)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    pub fn enterxipmodeimm(&mut self) -> EnterxipmodeimmW<'_, ConfigSpec> {
        EnterxipmodeimmW::new(self, 18)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    pub fn mstrbauddiv(&mut self) -> MstrbauddivW<'_, ConfigSpec> {
        MstrbauddivW::new(self, 19)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    pub fn enableahbdecoder(&mut self) -> EnableahbdecoderW<'_, ConfigSpec> {
        EnableahbdecoderW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn enabledtrprotocol(&mut self) -> EnabledtrprotocolW<'_, ConfigSpec> {
        EnabledtrprotocolW::new(self, 24)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    pub fn pipelinephy(&mut self) -> PipelinephyW<'_, ConfigSpec> {
        PipelinephyW::new(self, 25)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    pub fn crcenable(&mut self) -> CrcenableW<'_, ConfigSpec> {
        CrcenableW::new(self, 29)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    pub fn dualbyteopcodeen(&mut self) -> DualbyteopcodeenW<'_, ConfigSpec> {
        DualbyteopcodeenW::new(self, 30)
    }
}
#[doc = "Octal-SPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x8078_0081"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x8078_0081;
}
