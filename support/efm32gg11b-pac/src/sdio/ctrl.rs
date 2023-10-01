#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `ITAPDLYEN` reader - Selective Tap Delay Line Enable on Rxclk_in"]
pub type ITAPDLYEN_R = crate::BitReader;
#[doc = "Field `ITAPDLYEN` writer - Selective Tap Delay Line Enable on Rxclk_in"]
pub type ITAPDLYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ITAPDLYSEL` reader - Selects One of 32 Taps on the Rxclk_in Line"]
pub type ITAPDLYSEL_R = crate::FieldReader;
#[doc = "Field `ITAPDLYSEL` writer - Selects One of 32 Taps on the Rxclk_in Line"]
pub type ITAPDLYSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `ITAPCHGWIN` reader - Gating Signal for Tap Delay Change"]
pub type ITAPCHGWIN_R = crate::BitReader;
#[doc = "Field `ITAPCHGWIN` writer - Gating Signal for Tap Delay Change"]
pub type ITAPCHGWIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTAPDLYEN` reader - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
pub type OTAPDLYEN_R = crate::BitReader;
#[doc = "Field `OTAPDLYEN` writer - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
pub type OTAPDLYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTAPDLYSEL` reader - Selects One of 32 Taps on the SDIO_CLK Pin"]
pub type OTAPDLYSEL_R = crate::FieldReader;
#[doc = "Field `OTAPDLYSEL` writer - Selects One of 32 Taps on the SDIO_CLK Pin"]
pub type OTAPDLYSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TXDLYMUXSEL` reader - TX Delay Mux Selection"]
pub type TXDLYMUXSEL_R = crate::FieldReader;
#[doc = "Field `TXDLYMUXSEL` writer - TX Delay Mux Selection"]
pub type TXDLYMUXSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&self) -> ITAPDLYEN_R {
        ITAPDLYEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&self) -> ITAPDLYSEL_R {
        ITAPDLYSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&self) -> ITAPCHGWIN_R {
        ITAPCHGWIN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&self) -> OTAPDLYEN_R {
        OTAPDLYEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&self) -> OTAPDLYSEL_R {
        OTAPDLYSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&self) -> TXDLYMUXSEL_R {
        TXDLYMUXSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    #[must_use]
    pub fn itapdlyen(&mut self) -> ITAPDLYEN_W<CTRL_SPEC, 0> {
        ITAPDLYEN_W::new(self)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    #[must_use]
    pub fn itapdlysel(&mut self) -> ITAPDLYSEL_W<CTRL_SPEC, 1> {
        ITAPDLYSEL_W::new(self)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    #[must_use]
    pub fn itapchgwin(&mut self) -> ITAPCHGWIN_W<CTRL_SPEC, 6> {
        ITAPCHGWIN_W::new(self)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    #[must_use]
    pub fn otapdlyen(&mut self) -> OTAPDLYEN_W<CTRL_SPEC, 7> {
        OTAPDLYEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    #[must_use]
    pub fn otapdlysel(&mut self) -> OTAPDLYSEL_W<CTRL_SPEC, 8> {
        OTAPDLYSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn txdlymuxsel(&mut self) -> TXDLYMUXSEL_W<CTRL_SPEC, 16> {
        TXDLYMUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core Control Signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
