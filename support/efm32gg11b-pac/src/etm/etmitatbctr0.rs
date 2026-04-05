#[doc = "Register `ETMITATBCTR0` reader"]
pub type R = crate::R<Etmitatbctr0Spec>;
#[doc = "Register `ETMITATBCTR0` writer"]
pub type W = crate::W<Etmitatbctr0Spec>;
#[doc = "Field `ATVALID` reader - ATVALID Output Value"]
pub type AtvalidR = crate::BitReader;
#[doc = "Field `ATVALID` writer - ATVALID Output Value"]
pub type AtvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ATVALID Output Value"]
    #[inline(always)]
    pub fn atvalid(&self) -> AtvalidR {
        AtvalidR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATVALID Output Value"]
    #[inline(always)]
    pub fn atvalid(&mut self) -> AtvalidW<'_, Etmitatbctr0Spec> {
        AtvalidW::new(self, 0)
    }
}
#[doc = "ETM Integration Test ATB Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmitatbctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmitatbctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Etmitatbctr0Spec;
impl crate::RegisterSpec for Etmitatbctr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmitatbctr0::R`](R) reader structure"]
impl crate::Readable for Etmitatbctr0Spec {}
#[doc = "`write(|w| ..)` method takes [`etmitatbctr0::W`](W) writer structure"]
impl crate::Writable for Etmitatbctr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMITATBCTR0 to value 0"]
impl crate::Resettable for Etmitatbctr0Spec {}
