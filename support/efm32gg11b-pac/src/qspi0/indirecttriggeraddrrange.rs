#[doc = "Register `INDIRECTTRIGGERADDRRANGE` reader"]
pub type R = crate::R<IndirecttriggeraddrrangeSpec>;
#[doc = "Register `INDIRECTTRIGGERADDRRANGE` writer"]
pub type W = crate::W<IndirecttriggeraddrrangeSpec>;
#[doc = "Field `INDRANGEWIDTH` reader - Indirect Trigger Address Width"]
pub type IndrangewidthR = crate::FieldReader;
#[doc = "Field `INDRANGEWIDTH` writer - Indirect Trigger Address Width"]
pub type IndrangewidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&self) -> IndrangewidthR {
        IndrangewidthR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&mut self) -> IndrangewidthW<'_, IndirecttriggeraddrrangeSpec> {
        IndrangewidthW::new(self, 0)
    }
}
#[doc = "Indirect Trigger Address Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirecttriggeraddrrange::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirecttriggeraddrrange::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirecttriggeraddrrangeSpec;
impl crate::RegisterSpec for IndirecttriggeraddrrangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirecttriggeraddrrange::R`](R) reader structure"]
impl crate::Readable for IndirecttriggeraddrrangeSpec {}
#[doc = "`write(|w| ..)` method takes [`indirecttriggeraddrrange::W`](W) writer structure"]
impl crate::Writable for IndirecttriggeraddrrangeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTTRIGGERADDRRANGE to value 0x04"]
impl crate::Resettable for IndirecttriggeraddrrangeSpec {
    const RESET_VALUE: u32 = 0x04;
}
