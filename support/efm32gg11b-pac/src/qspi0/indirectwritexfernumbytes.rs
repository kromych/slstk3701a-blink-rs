#[doc = "Register `INDIRECTWRITEXFERNUMBYTES` reader"]
pub type R = crate::R<IndirectwritexfernumbytesSpec>;
#[doc = "Register `INDIRECTWRITEXFERNUMBYTES` writer"]
pub type W = crate::W<IndirectwritexfernumbytesSpec>;
#[doc = "Field `VALUE` reader - Indirect Number of Bytes"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Indirect Number of Bytes"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indirect Number of Bytes"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Number of Bytes"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, IndirectwritexfernumbytesSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Indirect Write Transfer Number Bytes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexfernumbytes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexfernumbytes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectwritexfernumbytesSpec;
impl crate::RegisterSpec for IndirectwritexfernumbytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexfernumbytes::R`](R) reader structure"]
impl crate::Readable for IndirectwritexfernumbytesSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexfernumbytes::W`](W) writer structure"]
impl crate::Writable for IndirectwritexfernumbytesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERNUMBYTES to value 0"]
impl crate::Resettable for IndirectwritexfernumbytesSpec {}
