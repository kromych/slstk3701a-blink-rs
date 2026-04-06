#[doc = "Register `COMPE_COMP` reader"]
pub type R = crate::R<CompeCompSpec>;
#[doc = "Register `COMPE_COMP` writer"]
pub type W = crate::W<CompeCompSpec>;
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
    pub fn comp(&mut self) -> CompW<'_, CompeCompSpec> {
        CompW::new(self, 0)
    }
}
#[doc = "Compare Value Register X\n\nYou can [`read`](crate::Reg::read) this register and get [`compe_comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compe_comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompeCompSpec;
impl crate::RegisterSpec for CompeCompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compe_comp::R`](R) reader structure"]
impl crate::Readable for CompeCompSpec {}
#[doc = "`write(|w| ..)` method takes [`compe_comp::W`](W) writer structure"]
impl crate::Writable for CompeCompSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPE_COMP to value 0"]
impl crate::Resettable for CompeCompSpec {}
