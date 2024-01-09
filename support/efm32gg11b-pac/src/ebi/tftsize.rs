#[doc = "Register `TFTSIZE` reader"]
pub type R = crate::R<TFTSIZE_SPEC>;
#[doc = "Register `TFTSIZE` writer"]
pub type W = crate::W<TFTSIZE_SPEC>;
#[doc = "Field `HSZ` reader - Horizontal Size (excluding Porches)"]
pub type HSZ_R = crate::FieldReader<u16>;
#[doc = "Field `HSZ` writer - Horizontal Size (excluding Porches)"]
pub type HSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VSZ` reader - Vertical Size (excluding Porches)"]
pub type VSZ_R = crate::FieldReader<u16>;
#[doc = "Field `VSZ` writer - Vertical Size (excluding Porches)"]
pub type VSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Horizontal Size (excluding Porches)"]
    #[inline(always)]
    pub fn hsz(&self) -> HSZ_R {
        HSZ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding Porches)"]
    #[inline(always)]
    pub fn vsz(&self) -> VSZ_R {
        VSZ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Horizontal Size (excluding Porches)"]
    #[inline(always)]
    #[must_use]
    pub fn hsz(&mut self) -> HSZ_W<TFTSIZE_SPEC> {
        HSZ_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding Porches)"]
    #[inline(always)]
    #[must_use]
    pub fn vsz(&mut self) -> VSZ_W<TFTSIZE_SPEC> {
        VSZ_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTSIZE_SPEC;
impl crate::RegisterSpec for TFTSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftsize::R`](R) reader structure"]
impl crate::Readable for TFTSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftsize::W`](W) writer structure"]
impl crate::Writable for TFTSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFTSIZE to value 0"]
impl crate::Resettable for TFTSIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
