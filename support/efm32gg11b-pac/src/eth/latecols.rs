#[doc = "Register `LATECOLS` reader"]
pub type R = crate::R<LatecolsSpec>;
#[doc = "Register `LATECOLS` writer"]
pub type W = crate::W<LatecolsSpec>;
#[doc = "Field `COUNT` reader - Late collisions"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Late collisions"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Late collisions"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Late collisions"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, LatecolsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Late Collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`latecols::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`latecols::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LatecolsSpec;
impl crate::RegisterSpec for LatecolsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`latecols::R`](R) reader structure"]
impl crate::Readable for LatecolsSpec {}
#[doc = "`write(|w| ..)` method takes [`latecols::W`](W) writer structure"]
impl crate::Writable for LatecolsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LATECOLS to value 0"]
impl crate::Resettable for LatecolsSpec {}
