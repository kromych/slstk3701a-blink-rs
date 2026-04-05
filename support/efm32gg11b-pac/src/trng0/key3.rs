#[doc = "Register `KEY3` reader"]
pub type R = crate::R<Key3Spec>;
#[doc = "Register `KEY3` writer"]
pub type W = crate::W<Key3Spec>;
#[doc = "Field `VALUE` reader - Key 3"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Key 3"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, Key3Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Key Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3Spec;
impl crate::RegisterSpec for Key3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3::R`](R) reader structure"]
impl crate::Readable for Key3Spec {}
#[doc = "`write(|w| ..)` method takes [`key3::W`](W) writer structure"]
impl crate::Writable for Key3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for Key3Spec {}
