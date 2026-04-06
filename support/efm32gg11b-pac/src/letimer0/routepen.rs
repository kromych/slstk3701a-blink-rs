#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `OUT0PEN` reader - Output 0 Pin Enable"]
pub type Out0penR = crate::BitReader;
#[doc = "Field `OUT0PEN` writer - Output 0 Pin Enable"]
pub type Out0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1PEN` reader - Output 1 Pin Enable"]
pub type Out1penR = crate::BitReader;
#[doc = "Field `OUT1PEN` writer - Output 1 Pin Enable"]
pub type Out1penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output 0 Pin Enable"]
    #[inline(always)]
    pub fn out0pen(&self) -> Out0penR {
        Out0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output 1 Pin Enable"]
    #[inline(always)]
    pub fn out1pen(&self) -> Out1penR {
        Out1penR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output 0 Pin Enable"]
    #[inline(always)]
    pub fn out0pen(&mut self) -> Out0penW<'_, RoutepenSpec> {
        Out0penW::new(self, 0)
    }
    #[doc = "Bit 1 - Output 1 Pin Enable"]
    #[inline(always)]
    pub fn out1pen(&mut self) -> Out1penW<'_, RoutepenSpec> {
        Out1penW::new(self, 1)
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
