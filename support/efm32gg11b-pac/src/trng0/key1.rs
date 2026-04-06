#[doc = "Register `KEY1` reader"]
pub type R = crate::R<Key1Spec>;
#[doc = "Register `KEY1` writer"]
pub type W = crate::W<Key1Spec>;
#[doc = "Field `VALUE` reader - Key 1"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Key 1"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key 1"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key 1"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, Key1Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Key Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1Spec;
impl crate::RegisterSpec for Key1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1::R`](R) reader structure"]
impl crate::Readable for Key1Spec {}
#[doc = "`write(|w| ..)` method takes [`key1::W`](W) writer structure"]
impl crate::Writable for Key1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for Key1Spec {}
