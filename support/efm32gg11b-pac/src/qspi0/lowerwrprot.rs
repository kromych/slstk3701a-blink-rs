#[doc = "Register `LOWERWRPROT` reader"]
pub type R = crate::R<LowerwrprotSpec>;
#[doc = "Register `LOWERWRPROT` writer"]
pub type W = crate::W<LowerwrprotSpec>;
#[doc = "Field `SUBSECTOR` reader - Lower Block Number"]
pub type SubsectorR = crate::FieldReader<u32>;
#[doc = "Field `SUBSECTOR` writer - Lower Block Number"]
pub type SubsectorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower Block Number"]
    #[inline(always)]
    pub fn subsector(&self) -> SubsectorR {
        SubsectorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower Block Number"]
    #[inline(always)]
    pub fn subsector(&mut self) -> SubsectorW<'_, LowerwrprotSpec> {
        SubsectorW::new(self, 0)
    }
}
#[doc = "Lower Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lowerwrprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowerwrprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowerwrprotSpec;
impl crate::RegisterSpec for LowerwrprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowerwrprot::R`](R) reader structure"]
impl crate::Readable for LowerwrprotSpec {}
#[doc = "`write(|w| ..)` method takes [`lowerwrprot::W`](W) writer structure"]
impl crate::Writable for LowerwrprotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOWERWRPROT to value 0"]
impl crate::Resettable for LowerwrprotSpec {}
