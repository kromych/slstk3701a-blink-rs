#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `ENBSPI` reader - QSPI Enable"]
pub type ENBSPI_R = crate::BitReader;
#[doc = "Field `ENBSPI` writer - QSPI Enable"]
pub type ENBSPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SELCLKPOL` reader - Clock Polarity, CPOL"]
pub type SELCLKPOL_R = crate::BitReader;
#[doc = "Field `SELCLKPOL` writer - Clock Polarity, CPOL"]
pub type SELCLKPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SELCLKPHASE` reader - Clock Phase, CPHA"]
pub type SELCLKPHASE_R = crate::BitReader;
#[doc = "Field `SELCLKPHASE` writer - Clock Phase, CPHA"]
pub type SELCLKPHASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PHYMODEENABLE` reader - PHY Mode Enable"]
pub type PHYMODEENABLE_R = crate::BitReader;
#[doc = "Field `PHYMODEENABLE` writer - PHY Mode Enable"]
pub type PHYMODEENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENBDIRACCCTLR` reader - Enable Direct Access Controller"]
pub type ENBDIRACCCTLR_R = crate::BitReader;
#[doc = "Field `ENBDIRACCCTLR` writer - Enable Direct Access Controller"]
pub type ENBDIRACCCTLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENBLEGACYIPMODE` reader - Legacy IP Mode Enable"]
pub type ENBLEGACYIPMODE_R = crate::BitReader;
#[doc = "Field `ENBLEGACYIPMODE` writer - Legacy IP Mode Enable"]
pub type ENBLEGACYIPMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERIPHSELDEC` reader - Peripheral Select Decode"]
pub type PERIPHSELDEC_R = crate::BitReader;
#[doc = "Field `PERIPHSELDEC` writer - Peripheral Select Decode"]
pub type PERIPHSELDEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERIPHCSLINES` reader - Peripheral Chip Select Lines"]
pub type PERIPHCSLINES_R = crate::FieldReader;
#[doc = "Field `PERIPHCSLINES` writer - Peripheral Chip Select Lines"]
pub type PERIPHCSLINES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WRPROTFLASH` reader - Write Protect Flash Pin"]
pub type WRPROTFLASH_R = crate::BitReader;
#[doc = "Field `WRPROTFLASH` writer - Write Protect Flash Pin"]
pub type WRPROTFLASH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENBAHBADDRREMAP` reader - Enable Address Remapping"]
pub type ENBAHBADDRREMAP_R = crate::BitReader;
#[doc = "Field `ENBAHBADDRREMAP` writer - Enable Address Remapping"]
pub type ENBAHBADDRREMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENTERXIPMODE` reader - Enter XIP Mode on Next READ"]
pub type ENTERXIPMODE_R = crate::BitReader;
#[doc = "Field `ENTERXIPMODE` writer - Enter XIP Mode on Next READ"]
pub type ENTERXIPMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENTERXIPMODEIMM` reader - Enter XIP Mode Immediately"]
pub type ENTERXIPMODEIMM_R = crate::BitReader;
#[doc = "Field `ENTERXIPMODEIMM` writer - Enter XIP Mode Immediately"]
pub type ENTERXIPMODEIMM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSTRBAUDDIV` reader - Master Mode Baud Rate Divisor"]
pub type MSTRBAUDDIV_R = crate::FieldReader;
#[doc = "Field `MSTRBAUDDIV` writer - Master Mode Baud Rate Divisor"]
pub type MSTRBAUDDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ENABLEAHBDECODER` reader - Enable Address Decoder"]
pub type ENABLEAHBDECODER_R = crate::BitReader;
#[doc = "Field `ENABLEAHBDECODER` writer - Enable Address Decoder"]
pub type ENABLEAHBDECODER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLEDTRPROTOCOL` reader - Enable DTR Protocol"]
pub type ENABLEDTRPROTOCOL_R = crate::BitReader;
#[doc = "Field `ENABLEDTRPROTOCOL` writer - Enable DTR Protocol"]
pub type ENABLEDTRPROTOCOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PIPELINEPHY` reader - Pipeline PHY Mode Enable"]
pub type PIPELINEPHY_R = crate::BitReader;
#[doc = "Field `PIPELINEPHY` writer - Pipeline PHY Mode Enable"]
pub type PIPELINEPHY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCENABLE` reader - CRC Enable Bit"]
pub type CRCENABLE_R = crate::BitReader;
#[doc = "Field `CRCENABLE` writer - CRC Enable Bit"]
pub type CRCENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUALBYTEOPCODEEN` reader - Dual-byte Opcode Mode Enable Bit"]
pub type DUALBYTEOPCODEEN_R = crate::BitReader;
#[doc = "Field `DUALBYTEOPCODEEN` writer - Dual-byte Opcode Mode Enable Bit"]
pub type DUALBYTEOPCODEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDLE` reader - Serial Interface and Low Level SPI Pipeline is IDLE"]
pub type IDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn enbspi(&self) -> ENBSPI_R {
        ENBSPI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    pub fn selclkpol(&self) -> SELCLKPOL_R {
        SELCLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    pub fn selclkphase(&self) -> SELCLKPHASE_R {
        SELCLKPHASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    pub fn phymodeenable(&self) -> PHYMODEENABLE_R {
        PHYMODEENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn enbdiraccctlr(&self) -> ENBDIRACCCTLR_R {
        ENBDIRACCCTLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn enblegacyipmode(&self) -> ENBLEGACYIPMODE_R {
        ENBLEGACYIPMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    pub fn periphseldec(&self) -> PERIPHSELDEC_R {
        PERIPHSELDEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    pub fn periphcslines(&self) -> PERIPHCSLINES_R {
        PERIPHCSLINES_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    pub fn wrprotflash(&self) -> WRPROTFLASH_R {
        WRPROTFLASH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    pub fn enbahbaddrremap(&self) -> ENBAHBADDRREMAP_R {
        ENBAHBADDRREMAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    pub fn enterxipmode(&self) -> ENTERXIPMODE_R {
        ENTERXIPMODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    pub fn enterxipmodeimm(&self) -> ENTERXIPMODEIMM_R {
        ENTERXIPMODEIMM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    pub fn mstrbauddiv(&self) -> MSTRBAUDDIV_R {
        MSTRBAUDDIV_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    pub fn enableahbdecoder(&self) -> ENABLEAHBDECODER_R {
        ENABLEAHBDECODER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn enabledtrprotocol(&self) -> ENABLEDTRPROTOCOL_R {
        ENABLEDTRPROTOCOL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    pub fn pipelinephy(&self) -> PIPELINEPHY_R {
        PIPELINEPHY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    pub fn crcenable(&self) -> CRCENABLE_R {
        CRCENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    pub fn dualbyteopcodeen(&self) -> DUALBYTEOPCODEEN_R {
        DUALBYTEOPCODEEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Serial Interface and Low Level SPI Pipeline is IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbspi(&mut self) -> ENBSPI_W<CONFIG_SPEC, 0> {
        ENBSPI_W::new(self)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    #[must_use]
    pub fn selclkpol(&mut self) -> SELCLKPOL_W<CONFIG_SPEC, 1> {
        SELCLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    #[must_use]
    pub fn selclkphase(&mut self) -> SELCLKPHASE_W<CONFIG_SPEC, 2> {
        SELCLKPHASE_W::new(self)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phymodeenable(&mut self) -> PHYMODEENABLE_W<CONFIG_SPEC, 3> {
        PHYMODEENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    #[must_use]
    pub fn enbdiraccctlr(&mut self) -> ENBDIRACCCTLR_W<CONFIG_SPEC, 7> {
        ENBDIRACCCTLR_W::new(self)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enblegacyipmode(&mut self) -> ENBLEGACYIPMODE_W<CONFIG_SPEC, 8> {
        ENBLEGACYIPMODE_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    #[must_use]
    pub fn periphseldec(&mut self) -> PERIPHSELDEC_W<CONFIG_SPEC, 9> {
        PERIPHSELDEC_W::new(self)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    #[must_use]
    pub fn periphcslines(&mut self) -> PERIPHCSLINES_W<CONFIG_SPEC, 10> {
        PERIPHCSLINES_W::new(self)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    #[must_use]
    pub fn wrprotflash(&mut self) -> WRPROTFLASH_W<CONFIG_SPEC, 14> {
        WRPROTFLASH_W::new(self)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    #[must_use]
    pub fn enbahbaddrremap(&mut self) -> ENBAHBADDRREMAP_W<CONFIG_SPEC, 16> {
        ENBAHBADDRREMAP_W::new(self)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    #[must_use]
    pub fn enterxipmode(&mut self) -> ENTERXIPMODE_W<CONFIG_SPEC, 17> {
        ENTERXIPMODE_W::new(self)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn enterxipmodeimm(&mut self) -> ENTERXIPMODEIMM_W<CONFIG_SPEC, 18> {
        ENTERXIPMODEIMM_W::new(self)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn mstrbauddiv(&mut self) -> MSTRBAUDDIV_W<CONFIG_SPEC, 19> {
        MSTRBAUDDIV_W::new(self)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    #[must_use]
    pub fn enableahbdecoder(&mut self) -> ENABLEAHBDECODER_W<CONFIG_SPEC, 23> {
        ENABLEAHBDECODER_W::new(self)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    #[must_use]
    pub fn enabledtrprotocol(&mut self) -> ENABLEDTRPROTOCOL_W<CONFIG_SPEC, 24> {
        ENABLEDTRPROTOCOL_W::new(self)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pipelinephy(&mut self) -> PIPELINEPHY_W<CONFIG_SPEC, 25> {
        PIPELINEPHY_W::new(self)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcenable(&mut self) -> CRCENABLE_W<CONFIG_SPEC, 29> {
        CRCENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dualbyteopcodeen(&mut self) -> DUALBYTEOPCODEEN_W<CONFIG_SPEC, 30> {
        DUALBYTEOPCODEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Octal-SPI Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x8078_0081"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8078_0081;
}
