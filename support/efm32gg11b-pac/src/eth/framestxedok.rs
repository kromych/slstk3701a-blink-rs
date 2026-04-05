#[doc = "Register `FRAMESTXEDOK` reader"]
pub type R = crate::R<FramestxedokSpec>;
#[doc = "Register `FRAMESTXEDOK` writer"]
pub type W = crate::W<FramestxedokSpec>;
#[doc = "Field `COUNT` reader - Frames transmitted without error"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Frames transmitted without error"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frames transmitted without error"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, FramestxedokSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Frames Transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`framestxedok::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framestxedok::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramestxedokSpec;
impl crate::RegisterSpec for FramestxedokSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framestxedok::R`](R) reader structure"]
impl crate::Readable for FramestxedokSpec {}
#[doc = "`write(|w| ..)` method takes [`framestxedok::W`](W) writer structure"]
impl crate::Writable for FramestxedokSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMESTXEDOK to value 0"]
impl crate::Resettable for FramestxedokSpec {}
