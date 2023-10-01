#[doc = "Register `SEGD5L` reader"]
pub type R = crate::R<SEGD5L_SPEC>;
#[doc = "Register `SEGD5L` writer"]
pub type W = crate::W<SEGD5L_SPEC>;
#[doc = "Field `SEGD5L` reader - COM5 Segment Data"]
pub type SEGD5L_R = crate::FieldReader<u32>;
#[doc = "Field `SEGD5L` writer - COM5 Segment Data"]
pub type SEGD5L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&self) -> SEGD5L_R {
        SEGD5L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    #[must_use]
    pub fn segd5l(&mut self) -> SEGD5L_W<SEGD5L_SPEC, 0> {
        SEGD5L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Segment Data Low Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`segd5l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`segd5l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEGD5L_SPEC;
impl crate::RegisterSpec for SEGD5L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd5l::R`](R) reader structure"]
impl crate::Readable for SEGD5L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`segd5l::W`](W) writer structure"]
impl crate::Writable for SEGD5L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD5L to value 0"]
impl crate::Resettable for SEGD5L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
