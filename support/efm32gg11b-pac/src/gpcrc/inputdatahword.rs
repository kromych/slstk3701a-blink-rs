#[doc = "Register `INPUTDATAHWORD` reader"]
pub type R = crate::R<INPUTDATAHWORD_SPEC>;
#[doc = "Register `INPUTDATAHWORD` writer"]
pub type W = crate::W<INPUTDATAHWORD_SPEC>;
#[doc = "Field `INPUTDATAHWORD` reader - Input Data for 16-bit"]
pub type INPUTDATAHWORD_R = crate::FieldReader<u16>;
#[doc = "Field `INPUTDATAHWORD` writer - Input Data for 16-bit"]
pub type INPUTDATAHWORD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    pub fn inputdatahword(&self) -> INPUTDATAHWORD_R {
        INPUTDATAHWORD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdatahword(&mut self) -> INPUTDATAHWORD_W<INPUTDATAHWORD_SPEC, 0> {
        INPUTDATAHWORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input 16-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdatahword::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdatahword::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTDATAHWORD_SPEC;
impl crate::RegisterSpec for INPUTDATAHWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdatahword::R`](R) reader structure"]
impl crate::Readable for INPUTDATAHWORD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputdatahword::W`](W) writer structure"]
impl crate::Writable for INPUTDATAHWORD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTDATAHWORD to value 0"]
impl crate::Resettable for INPUTDATAHWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
