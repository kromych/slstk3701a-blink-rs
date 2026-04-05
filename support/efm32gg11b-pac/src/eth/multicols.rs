#[doc = "Register `MULTICOLS` reader"]
pub type R = crate::R<MulticolsSpec>;
#[doc = "Register `MULTICOLS` writer"]
pub type W = crate::W<MulticolsSpec>;
#[doc = "Field `COUNT` reader - Multiple collision frames"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Multiple collision frames"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Multiple collision frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Multiple collision frames"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, MulticolsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Multiple Collision Frames\n\nYou can [`read`](crate::Reg::read) this register and get [`multicols::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multicols::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MulticolsSpec;
impl crate::RegisterSpec for MulticolsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`multicols::R`](R) reader structure"]
impl crate::Readable for MulticolsSpec {}
#[doc = "`write(|w| ..)` method takes [`multicols::W`](W) writer structure"]
impl crate::Writable for MulticolsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULTICOLS to value 0"]
impl crate::Resettable for MulticolsSpec {}
