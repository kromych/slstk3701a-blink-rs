#[doc = "Register `INDIRECTREADXFERNUMBYTES` reader"]
pub type R = crate::R<IndirectreadxfernumbytesSpec>;
#[doc = "Register `INDIRECTREADXFERNUMBYTES` writer"]
pub type W = crate::W<IndirectreadxfernumbytesSpec>;
#[doc = "Field `VALUE` reader - Indirect Read Transfer Number Bytes"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Indirect Read Transfer Number Bytes"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indirect Read Transfer Number Bytes"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Read Transfer Number Bytes"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, IndirectreadxfernumbytesSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Indirect Read Transfer Number Bytes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxfernumbytes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxfernumbytes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectreadxfernumbytesSpec;
impl crate::RegisterSpec for IndirectreadxfernumbytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectreadxfernumbytes::R`](R) reader structure"]
impl crate::Readable for IndirectreadxfernumbytesSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectreadxfernumbytes::W`](W) writer structure"]
impl crate::Writable for IndirectreadxfernumbytesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTREADXFERNUMBYTES to value 0"]
impl crate::Resettable for IndirectreadxfernumbytesSpec {}
