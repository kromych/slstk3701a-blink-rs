#[doc = "Register `LATECOLS` reader"]
pub type R = crate::R<LATECOLS_SPEC>;
#[doc = "Register `LATECOLS` writer"]
pub type W = crate::W<LATECOLS_SPEC>;
#[doc = "Field `COUNT` reader - Late collisions"]
pub type COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Late collisions"]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Late collisions"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Late collisions"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<LATECOLS_SPEC> {
        COUNT_W::new(self, 0)
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
#[doc = "Late Collisions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`latecols::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`latecols::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LATECOLS_SPEC;
impl crate::RegisterSpec for LATECOLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`latecols::R`](R) reader structure"]
impl crate::Readable for LATECOLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`latecols::W`](W) writer structure"]
impl crate::Writable for LATECOLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LATECOLS to value 0"]
impl crate::Resettable for LATECOLS_SPEC {
    const RESET_VALUE: u32 = 0;
}
