#[doc = "Register `TESTDATA` reader"]
pub type R = crate::R<TESTDATA_SPEC>;
#[doc = "Register `TESTDATA` writer"]
pub type W = crate::W<TESTDATA_SPEC>;
#[doc = "Field `VALUE` reader - Test data input to conditioning function or to the continuous tests"]
pub type VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Test data input to conditioning function or to the continuous tests"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Test data input to conditioning function or to the continuous tests"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Test data input to conditioning function or to the continuous tests"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<TESTDATA_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "Test Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TESTDATA_SPEC;
impl crate::RegisterSpec for TESTDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testdata::R`](R) reader structure"]
impl crate::Readable for TESTDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`testdata::W`](W) writer structure"]
impl crate::Writable for TESTDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTDATA to value 0"]
impl crate::Resettable for TESTDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
