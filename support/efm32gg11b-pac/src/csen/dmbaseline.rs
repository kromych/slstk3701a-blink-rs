#[doc = "Register `DMBASELINE` reader"]
pub type R = crate::R<DmbaselineSpec>;
#[doc = "Register `DMBASELINE` writer"]
pub type W = crate::W<DmbaselineSpec>;
#[doc = "Field `BASELINEUP` reader - Delta Modulator Integrator Initial Value"]
pub type BaselineupR = crate::FieldReader<u16>;
#[doc = "Field `BASELINEUP` writer - Delta Modulator Integrator Initial Value"]
pub type BaselineupW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BASELINEDN` reader - Delta Modulator Integrator Initial Value"]
pub type BaselinednR = crate::FieldReader<u16>;
#[doc = "Field `BASELINEDN` writer - Delta Modulator Integrator Initial Value"]
pub type BaselinednW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselineup(&self) -> BaselineupR {
        BaselineupR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselinedn(&self) -> BaselinednR {
        BaselinednR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselineup(&mut self) -> BaselineupW<'_, DmbaselineSpec> {
        BaselineupW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselinedn(&mut self) -> BaselinednW<'_, DmbaselineSpec> {
        BaselinednW::new(self, 16)
    }
}
#[doc = "Delta Modulation Baseline\n\nYou can [`read`](crate::Reg::read) this register and get [`dmbaseline::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmbaseline::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmbaselineSpec;
impl crate::RegisterSpec for DmbaselineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmbaseline::R`](R) reader structure"]
impl crate::Readable for DmbaselineSpec {}
#[doc = "`write(|w| ..)` method takes [`dmbaseline::W`](W) writer structure"]
impl crate::Writable for DmbaselineSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMBASELINE to value 0"]
impl crate::Resettable for DmbaselineSpec {}
