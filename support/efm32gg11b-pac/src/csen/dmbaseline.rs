#[doc = "Register `DMBASELINE` reader"]
pub type R = crate::R<DMBASELINE_SPEC>;
#[doc = "Register `DMBASELINE` writer"]
pub type W = crate::W<DMBASELINE_SPEC>;
#[doc = "Field `BASELINEUP` reader - Delta Modulator Integrator Initial Value"]
pub type BASELINEUP_R = crate::FieldReader<u16>;
#[doc = "Field `BASELINEUP` writer - Delta Modulator Integrator Initial Value"]
pub type BASELINEUP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BASELINEDN` reader - Delta Modulator Integrator Initial Value"]
pub type BASELINEDN_R = crate::FieldReader<u16>;
#[doc = "Field `BASELINEDN` writer - Delta Modulator Integrator Initial Value"]
pub type BASELINEDN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselineup(&self) -> BASELINEUP_R {
        BASELINEUP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselinedn(&self) -> BASELINEDN_R {
        BASELINEDN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn baselineup(&mut self) -> BASELINEUP_W<DMBASELINE_SPEC> {
        BASELINEUP_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn baselinedn(&mut self) -> BASELINEDN_W<DMBASELINE_SPEC> {
        BASELINEDN_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Delta Modulation Baseline\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmbaseline::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmbaseline::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMBASELINE_SPEC;
impl crate::RegisterSpec for DMBASELINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmbaseline::R`](R) reader structure"]
impl crate::Readable for DMBASELINE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmbaseline::W`](W) writer structure"]
impl crate::Writable for DMBASELINE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMBASELINE to value 0"]
impl crate::Resettable for DMBASELINE_SPEC {
    const RESET_VALUE: u32 = 0;
}
