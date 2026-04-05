#[doc = "Register `INPUTDATABYTE` reader"]
pub type R = crate::R<InputdatabyteSpec>;
#[doc = "Register `INPUTDATABYTE` writer"]
pub type W = crate::W<InputdatabyteSpec>;
#[doc = "Field `INPUTDATABYTE` reader - Input Data for 8-bit"]
pub type InputdatabyteR = crate::FieldReader;
#[doc = "Field `INPUTDATABYTE` writer - Input Data for 8-bit"]
pub type InputdatabyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&self) -> InputdatabyteR {
        InputdatabyteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&mut self) -> InputdatabyteW<'_, InputdatabyteSpec> {
        InputdatabyteW::new(self, 0)
    }
}
#[doc = "Input 8-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inputdatabyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatabyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputdatabyteSpec;
impl crate::RegisterSpec for InputdatabyteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdatabyte::R`](R) reader structure"]
impl crate::Readable for InputdatabyteSpec {}
#[doc = "`write(|w| ..)` method takes [`inputdatabyte::W`](W) writer structure"]
impl crate::Writable for InputdatabyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTDATABYTE to value 0"]
impl crate::Resettable for InputdatabyteSpec {}
