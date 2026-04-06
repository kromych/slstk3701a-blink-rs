#[doc = "Register `REMAPADDR` reader"]
pub type R = crate::R<RemapaddrSpec>;
#[doc = "Register `REMAPADDR` writer"]
pub type W = crate::W<RemapaddrSpec>;
#[doc = "Field `VALUE` reader - Remap Address Value"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Remap Address Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remap Address Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remap Address Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, RemapaddrSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Remap Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remapaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remapaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemapaddrSpec;
impl crate::RegisterSpec for RemapaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remapaddr::R`](R) reader structure"]
impl crate::Readable for RemapaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`remapaddr::W`](W) writer structure"]
impl crate::Writable for RemapaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAPADDR to value 0"]
impl crate::Resettable for RemapaddrSpec {}
