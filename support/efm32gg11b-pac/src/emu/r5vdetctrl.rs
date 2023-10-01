#[doc = "Register `R5VDETCTRL` reader"]
pub type R = crate::R<R5VDETCTRL_SPEC>;
#[doc = "Register `R5VDETCTRL` writer"]
pub type W = crate::W<R5VDETCTRL_SPEC>;
#[doc = "Field `VREGIDETDIS` reader - VREGI Detector Disable"]
pub type VREGIDETDIS_R = crate::BitReader;
#[doc = "Field `VREGIDETDIS` writer - VREGI Detector Disable"]
pub type VREGIDETDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBUSDETDIS` reader - VBUS Detector Disable"]
pub type VBUSDETDIS_R = crate::BitReader;
#[doc = "Field `VBUSDETDIS` writer - VBUS Detector Disable"]
pub type VBUSDETDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREGODETDIS` reader - VREGO Detector Disable"]
pub type VREGODETDIS_R = crate::BitReader;
#[doc = "Field `VREGODETDIS` writer - VREGO Detector Disable"]
pub type VREGODETDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&self) -> VREGIDETDIS_R {
        VREGIDETDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&self) -> VBUSDETDIS_R {
        VBUSDETDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&self) -> VREGODETDIS_R {
        VREGODETDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vregidetdis(&mut self) -> VREGIDETDIS_W<R5VDETCTRL_SPEC, 0> {
        VREGIDETDIS_W::new(self)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdetdis(&mut self) -> VBUSDETDIS_W<R5VDETCTRL_SPEC, 1> {
        VBUSDETDIS_W::new(self)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vregodetdis(&mut self) -> VREGODETDIS_W<R5VDETCTRL_SPEC, 2> {
        VREGODETDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "5V Detector Enables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vdetctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5vdetctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5VDETCTRL_SPEC;
impl crate::RegisterSpec for R5VDETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vdetctrl::R`](R) reader structure"]
impl crate::Readable for R5VDETCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r5vdetctrl::W`](W) writer structure"]
impl crate::Writable for R5VDETCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R5VDETCTRL to value 0"]
impl crate::Resettable for R5VDETCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
