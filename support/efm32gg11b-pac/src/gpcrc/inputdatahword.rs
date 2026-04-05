#[doc = "Register `INPUTDATAHWORD` reader"]
pub type R = crate::R<InputdatahwordSpec>;
#[doc = "Register `INPUTDATAHWORD` writer"]
pub type W = crate::W<InputdatahwordSpec>;
#[doc = "Field `INPUTDATAHWORD` reader - Input Data for 16-bit"]
pub type InputdatahwordR = crate::FieldReader<u16>;
#[doc = "Field `INPUTDATAHWORD` writer - Input Data for 16-bit"]
pub type InputdatahwordW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    pub fn inputdatahword(&self) -> InputdatahwordR {
        InputdatahwordR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    pub fn inputdatahword(&mut self) -> InputdatahwordW<'_, InputdatahwordSpec> {
        InputdatahwordW::new(self, 0)
    }
}
#[doc = "Input 16-bit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inputdatahword::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatahword::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputdatahwordSpec;
impl crate::RegisterSpec for InputdatahwordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputdatahword::R`](R) reader structure"]
impl crate::Readable for InputdatahwordSpec {}
#[doc = "`write(|w| ..)` method takes [`inputdatahword::W`](W) writer structure"]
impl crate::Writable for InputdatahwordSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTDATAHWORD to value 0"]
impl crate::Resettable for InputdatahwordSpec {}
