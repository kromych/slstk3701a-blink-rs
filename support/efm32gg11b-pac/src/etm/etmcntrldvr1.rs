#[doc = "Register `ETMCNTRLDVR1` reader"]
pub type R = crate::R<Etmcntrldvr1Spec>;
#[doc = "Register `ETMCNTRLDVR1` writer"]
pub type W = crate::W<Etmcntrldvr1Spec>;
#[doc = "Field `COUNT` reader - Free running counter reload value"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Free running counter reload value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Free running counter reload value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Free running counter reload value"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Etmcntrldvr1Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "Counter Reload Value\n\nYou can [`read`](crate::Reg::read) this register and get [`etmcntrldvr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmcntrldvr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmcntrldvr1Spec;
impl crate::RegisterSpec for Etmcntrldvr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmcntrldvr1::R`](R) reader structure"]
impl crate::Readable for Etmcntrldvr1Spec {}
#[doc = "`write(|w| ..)` method takes [`etmcntrldvr1::W`](W) writer structure"]
impl crate::Writable for Etmcntrldvr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMCNTRLDVR1 to value 0"]
impl crate::Resettable for Etmcntrldvr1Spec {}
