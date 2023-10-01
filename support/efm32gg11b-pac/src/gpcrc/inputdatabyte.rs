#[doc = "Register `INPUTDATABYTE` reader"]
pub type R = crate::R<INPUTDATABYTE_SPEC>;
#[doc = "Register `INPUTDATABYTE` writer"]
pub type W = crate::W<INPUTDATABYTE_SPEC>;
#[doc = "Field `INPUTDATABYTE` reader - Input Data for 8-bit"]
pub type INPUTDATABYTE_R = crate::FieldReader;
#[doc = "Field `INPUTDATABYTE` writer - Input Data for 8-bit"]
pub type INPUTDATABYTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&self) -> INPUTDATABYTE_R {
        INPUTDATABYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    #[must_use]
    pub fn inputdatabyte(&mut self) -> INPUTDATABYTE_W<INPUTDATABYTE_SPEC, 0> {
        INPUTDATABYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input 8-bit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputdatabyte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputdatabyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTDATABYTE_SPEC;
impl crate::RegisterSpec for INPUTDATABYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdatabyte::R`](R) reader structure"]
impl crate::Readable for INPUTDATABYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputdatabyte::W`](W) writer structure"]
impl crate::Writable for INPUTDATABYTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTDATABYTE to value 0"]
impl crate::Resettable for INPUTDATABYTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
