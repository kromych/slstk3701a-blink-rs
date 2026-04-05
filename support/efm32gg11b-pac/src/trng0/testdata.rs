#[doc = "Register `TESTDATA` reader"]
pub type R = crate::R<TestdataSpec>;
#[doc = "Register `TESTDATA` writer"]
pub type W = crate::W<TestdataSpec>;
#[doc = "Field `VALUE` reader - Test data input to conditioning function or to the continuous tests"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Test data input to conditioning function or to the continuous tests"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Test data input to conditioning function or to the continuous tests"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Test data input to conditioning function or to the continuous tests"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, TestdataSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Test Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`testdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestdataSpec;
impl crate::RegisterSpec for TestdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testdata::R`](R) reader structure"]
impl crate::Readable for TestdataSpec {}
#[doc = "`write(|w| ..)` method takes [`testdata::W`](W) writer structure"]
impl crate::Writable for TestdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TESTDATA to value 0"]
impl crate::Resettable for TestdataSpec {}
