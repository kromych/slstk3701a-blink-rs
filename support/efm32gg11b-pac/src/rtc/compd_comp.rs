#[doc = "Register `COMPD_COMP` reader"]
pub type R = crate::R<CompdCompSpec>;
#[doc = "Register `COMPD_COMP` writer"]
pub type W = crate::W<CompdCompSpec>;
#[doc = "Field `COMP` reader - Compare Value"]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, CompdCompSpec> {
        CompW::new(self, 0)
    }
}
#[doc = "Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compd_comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compd_comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompdCompSpec;
impl crate::RegisterSpec for CompdCompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compd_comp::R`](R) reader structure"]
impl crate::Readable for CompdCompSpec {}
#[doc = "`write(|w| ..)` method takes [`compd_comp::W`](W) writer structure"]
impl crate::Writable for CompdCompSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPD_COMP to value 0"]
impl crate::Resettable for CompdCompSpec {}
