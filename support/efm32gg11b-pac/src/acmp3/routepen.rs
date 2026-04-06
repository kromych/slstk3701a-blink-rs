#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `OUTPEN` reader - ACMP Output Pin Enable"]
pub type OutpenR = crate::BitReader;
#[doc = "Field `OUTPEN` writer - ACMP Output Pin Enable"]
pub type OutpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn outpen(&self) -> OutpenR {
        OutpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn outpen(&mut self) -> OutpenW<'_, RoutepenSpec> {
        OutpenW::new(self, 0)
    }
}
#[doc = "I/O Routing Pine Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoutepenSpec;
impl crate::RegisterSpec for RoutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for RoutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for RoutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for RoutepenSpec {}
