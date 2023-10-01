#[doc = "Register `SENSORSTATE` reader"]
pub type R = crate::R<SENSORSTATE_SPEC>;
#[doc = "Register `SENSORSTATE` writer"]
pub type W = crate::W<SENSORSTATE_SPEC>;
#[doc = "Field `SENSORSTATE` reader - Decoder Input Register"]
pub type SENSORSTATE_R = crate::FieldReader;
#[doc = "Field `SENSORSTATE` writer - Decoder Input Register"]
pub type SENSORSTATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Decoder Input Register"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SENSORSTATE_R {
        SENSORSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decoder Input Register"]
    #[inline(always)]
    #[must_use]
    pub fn sensorstate(&mut self) -> SENSORSTATE_W<SENSORSTATE_SPEC, 0> {
        SENSORSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Decoder Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sensorstate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sensorstate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SENSORSTATE_SPEC;
impl crate::RegisterSpec for SENSORSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensorstate::R`](R) reader structure"]
impl crate::Readable for SENSORSTATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sensorstate::W`](W) writer structure"]
impl crate::Writable for SENSORSTATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SENSORSTATE to value 0"]
impl crate::Resettable for SENSORSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
