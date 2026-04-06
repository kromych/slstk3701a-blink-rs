#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ITAPDLYEN` reader - Selective Tap Delay Line Enable on Rxclk_in"]
pub type ItapdlyenR = crate::BitReader;
#[doc = "Field `ITAPDLYEN` writer - Selective Tap Delay Line Enable on Rxclk_in"]
pub type ItapdlyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAPDLYSEL` reader - Selects One of 32 Taps on the Rxclk_in Line"]
pub type ItapdlyselR = crate::FieldReader;
#[doc = "Field `ITAPDLYSEL` writer - Selects One of 32 Taps on the Rxclk_in Line"]
pub type ItapdlyselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ITAPCHGWIN` reader - Gating Signal for Tap Delay Change"]
pub type ItapchgwinR = crate::BitReader;
#[doc = "Field `ITAPCHGWIN` writer - Gating Signal for Tap Delay Change"]
pub type ItapchgwinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTAPDLYEN` reader - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
pub type OtapdlyenR = crate::BitReader;
#[doc = "Field `OTAPDLYEN` writer - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
pub type OtapdlyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTAPDLYSEL` reader - Selects One of 32 Taps on the SDIO_CLK Pin"]
pub type OtapdlyselR = crate::FieldReader;
#[doc = "Field `OTAPDLYSEL` writer - Selects One of 32 Taps on the SDIO_CLK Pin"]
pub type OtapdlyselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXDLYMUXSEL` reader - TX Delay Mux Selection"]
pub type TxdlymuxselR = crate::FieldReader;
#[doc = "Field `TXDLYMUXSEL` writer - TX Delay Mux Selection"]
pub type TxdlymuxselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&self) -> ItapdlyenR {
        ItapdlyenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&self) -> ItapdlyselR {
        ItapdlyselR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&self) -> ItapchgwinR {
        ItapchgwinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&self) -> OtapdlyenR {
        OtapdlyenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&self) -> OtapdlyselR {
        OtapdlyselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&self) -> TxdlymuxselR {
        TxdlymuxselR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&mut self) -> ItapdlyenW<'_, CtrlSpec> {
        ItapdlyenW::new(self, 0)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&mut self) -> ItapdlyselW<'_, CtrlSpec> {
        ItapdlyselW::new(self, 1)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&mut self) -> ItapchgwinW<'_, CtrlSpec> {
        ItapchgwinW::new(self, 6)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&mut self) -> OtapdlyenW<'_, CtrlSpec> {
        OtapdlyenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&mut self) -> OtapdlyselW<'_, CtrlSpec> {
        OtapdlyselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&mut self) -> TxdlymuxselW<'_, CtrlSpec> {
        TxdlymuxselW::new(self, 16)
    }
}
#[doc = "Core Control Signals\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
