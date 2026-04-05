#[doc = "Register `EXCESSCOLS` reader"]
pub type R = crate::R<ExcesscolsSpec>;
#[doc = "Register `EXCESSCOLS` writer"]
pub type W = crate::W<ExcesscolsSpec>;
#[doc = "Field `COUNT` reader - Excessive collisions"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Excessive collisions"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Excessive collisions"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Excessive collisions"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, ExcesscolsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Excessive Collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`excesscols::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`excesscols::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExcesscolsSpec;
impl crate::RegisterSpec for ExcesscolsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`excesscols::R`](R) reader structure"]
impl crate::Readable for ExcesscolsSpec {}
#[doc = "`write(|w| ..)` method takes [`excesscols::W`](W) writer structure"]
impl crate::Writable for ExcesscolsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXCESSCOLS to value 0"]
impl crate::Resettable for ExcesscolsSpec {}
