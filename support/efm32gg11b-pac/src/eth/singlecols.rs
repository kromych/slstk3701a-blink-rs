#[doc = "Register `SINGLECOLS` reader"]
pub type R = crate::R<SinglecolsSpec>;
#[doc = "Register `SINGLECOLS` writer"]
pub type W = crate::W<SinglecolsSpec>;
#[doc = "Field `COUNT` reader - Single collision frames"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Single collision frames"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, SinglecolsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Single Collision Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`singlecols::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlecols::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglecolsSpec;
impl crate::RegisterSpec for SinglecolsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlecols::R`](R) reader structure"]
impl crate::Readable for SinglecolsSpec {}
#[doc = "`write(|w| ..)` method takes [`singlecols::W`](W) writer structure"]
impl crate::Writable for SinglecolsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLECOLS to value 0"]
impl crate::Resettable for SinglecolsSpec {}
