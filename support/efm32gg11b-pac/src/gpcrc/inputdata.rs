#[doc = "Register `INPUTDATA` reader"]
pub type R = crate::R<INPUTDATA_SPEC>;
#[doc = "Register `INPUTDATA` writer"]
pub type W = crate::W<INPUTDATA_SPEC>;
#[doc = "Field `INPUTDATA` reader - Input Data for 32-bit"]
pub type INPUTDATA_R = crate::FieldReader<u32>;
#[doc = "Field `INPUTDATA` writer - Input Data for 32-bit"]
pub type INPUTDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&self) -> INPUTDATA_R {
        INPUTDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdata(&mut self) -> INPUTDATA_W<INPUTDATA_SPEC, 0> {
        INPUTDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input 32-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTDATA_SPEC;
impl crate::RegisterSpec for INPUTDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdata::R`](R) reader structure"]
impl crate::Readable for INPUTDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputdata::W`](W) writer structure"]
impl crate::Writable for INPUTDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTDATA to value 0"]
impl crate::Resettable for INPUTDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
