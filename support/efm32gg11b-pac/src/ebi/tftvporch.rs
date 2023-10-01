#[doc = "Register `TFTVPORCH` reader"]
pub type R = crate::R<TFTVPORCH_SPEC>;
#[doc = "Register `TFTVPORCH` writer"]
pub type W = crate::W<TFTVPORCH_SPEC>;
#[doc = "Field `VSYNC` reader - Vertical Synchronization Pulse Width"]
pub type VSYNC_R = crate::FieldReader;
#[doc = "Field `VSYNC` writer - Vertical Synchronization Pulse Width"]
pub type VSYNC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `VFPORCH` reader - Vertical Front Porch Size"]
pub type VFPORCH_R = crate::FieldReader<u16>;
#[doc = "Field `VFPORCH` writer - Vertical Front Porch Size"]
pub type VFPORCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `VBPORCH` reader - Vertical Back Porch Size"]
pub type VBPORCH_R = crate::FieldReader<u16>;
#[doc = "Field `VBPORCH` writer - Vertical Back Porch Size"]
pub type VBPORCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:19 - Vertical Front Porch Size"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Vertical Back Porch Size"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<TFTVPORCH_SPEC, 0> {
        VSYNC_W::new(self)
    }
    #[doc = "Bits 8:19 - Vertical Front Porch Size"]
    #[inline(always)]
    #[must_use]
    pub fn vfporch(&mut self) -> VFPORCH_W<TFTVPORCH_SPEC, 8> {
        VFPORCH_W::new(self)
    }
    #[doc = "Bits 20:31 - Vertical Back Porch Size"]
    #[inline(always)]
    #[must_use]
    pub fn vbporch(&mut self) -> VBPORCH_W<TFTVPORCH_SPEC, 20> {
        VBPORCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Vertical Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftvporch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftvporch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTVPORCH_SPEC;
impl crate::RegisterSpec for TFTVPORCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftvporch::R`](R) reader structure"]
impl crate::Readable for TFTVPORCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftvporch::W`](W) writer structure"]
impl crate::Writable for TFTVPORCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFTVPORCH to value 0"]
impl crate::Resettable for TFTVPORCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
