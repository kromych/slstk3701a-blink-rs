#[doc = "Register `INPUTDATA` reader"]
pub type R = crate::R<InputdataSpec>;
#[doc = "Register `INPUTDATA` writer"]
pub type W = crate::W<InputdataSpec>;
#[doc = "Field `INPUTDATA` reader - Input Data for 32-bit"]
pub type InputdataR = crate::FieldReader<u32>;
#[doc = "Field `INPUTDATA` writer - Input Data for 32-bit"]
pub type InputdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&self) -> InputdataR {
        InputdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&mut self) -> InputdataW<'_, InputdataSpec> {
        InputdataW::new(self, 0)
    }
}
#[doc = "Input 32-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inputdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputdataSpec;
impl crate::RegisterSpec for InputdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdata::R`](R) reader structure"]
impl crate::Readable for InputdataSpec {}
#[doc = "`write(|w| ..)` method takes [`inputdata::W`](W) writer structure"]
impl crate::Writable for InputdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTDATA to value 0"]
impl crate::Resettable for InputdataSpec {}
