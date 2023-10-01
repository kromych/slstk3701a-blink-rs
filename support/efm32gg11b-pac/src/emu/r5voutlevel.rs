#[doc = "Register `R5VOUTLEVEL` reader"]
pub type R = crate::R<R5VOUTLEVEL_SPEC>;
#[doc = "Register `R5VOUTLEVEL` writer"]
pub type W = crate::W<R5VOUTLEVEL_SPEC>;
#[doc = "Field `OUTLEVEL` reader - 5V Regulator Voltage"]
pub type OUTLEVEL_R = crate::FieldReader;
#[doc = "Field `OUTLEVEL` writer - 5V Regulator Voltage"]
pub type OUTLEVEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&self) -> OUTLEVEL_R {
        OUTLEVEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn outlevel(&mut self) -> OUTLEVEL_W<R5VOUTLEVEL_SPEC, 0> {
        OUTLEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "5V Regulator Voltage Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5voutlevel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5voutlevel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5VOUTLEVEL_SPEC;
impl crate::RegisterSpec for R5VOUTLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5voutlevel::R`](R) reader structure"]
impl crate::Readable for R5VOUTLEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r5voutlevel::W`](W) writer structure"]
impl crate::Writable for R5VOUTLEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R5VOUTLEVEL to value 0x01"]
impl crate::Resettable for R5VOUTLEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
