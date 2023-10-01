#[doc = "Register `TFTPOLARITY` reader"]
pub type R = crate::R<TFTPOLARITY_SPEC>;
#[doc = "Register `TFTPOLARITY` writer"]
pub type W = crate::W<TFTPOLARITY_SPEC>;
#[doc = "Field `CSPOL` reader - TFT Chip Select Polarity"]
pub type CSPOL_R = crate::BitReader;
#[doc = "Field `CSPOL` writer - TFT Chip Select Polarity"]
pub type CSPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCLKPOL` reader - TFT DCLK Polarity"]
pub type DCLKPOL_R = crate::BitReader;
#[doc = "Field `DCLKPOL` writer - TFT DCLK Polarity"]
pub type DCLKPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAENPOL` reader - TFT DATAEN Polarity"]
pub type DATAENPOL_R = crate::BitReader;
#[doc = "Field `DATAENPOL` writer - TFT DATAEN Polarity"]
pub type DATAENPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSYNCPOL` reader - Address Latch Polarity"]
pub type HSYNCPOL_R = crate::BitReader;
#[doc = "Field `HSYNCPOL` writer - Address Latch Polarity"]
pub type HSYNCPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSYNCPOL` reader - VSYNC Polarity"]
pub type VSYNCPOL_R = crate::BitReader;
#[doc = "Field `VSYNCPOL` writer - VSYNC Polarity"]
pub type VSYNCPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    pub fn dclkpol(&self) -> DCLKPOL_R {
        DCLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    pub fn dataenpol(&self) -> DATAENPOL_R {
        DATAENPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HSYNCPOL_R {
        HSYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VSYNCPOL_R {
        VSYNCPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<TFTPOLARITY_SPEC, 0> {
        CSPOL_W::new(self)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dclkpol(&mut self) -> DCLKPOL_W<TFTPOLARITY_SPEC, 1> {
        DCLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dataenpol(&mut self) -> DATAENPOL_W<TFTPOLARITY_SPEC, 2> {
        DATAENPOL_W::new(self)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsyncpol(&mut self) -> HSYNCPOL_W<TFTPOLARITY_SPEC, 3> {
        HSYNCPOL_W::new(self)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsyncpol(&mut self) -> VSYNCPOL_W<TFTPOLARITY_SPEC, 4> {
        VSYNCPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftpolarity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftpolarity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTPOLARITY_SPEC;
impl crate::RegisterSpec for TFTPOLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftpolarity::R`](R) reader structure"]
impl crate::Readable for TFTPOLARITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftpolarity::W`](W) writer structure"]
impl crate::Writable for TFTPOLARITY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTPOLARITY to value 0"]
impl crate::Resettable for TFTPOLARITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
