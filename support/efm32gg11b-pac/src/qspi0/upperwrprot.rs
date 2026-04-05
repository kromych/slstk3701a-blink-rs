#[doc = "Register `UPPERWRPROT` reader"]
pub type R = crate::R<UpperwrprotSpec>;
#[doc = "Register `UPPERWRPROT` writer"]
pub type W = crate::W<UpperwrprotSpec>;
#[doc = "Field `SUBSECTOR` reader - Upper Block Number"]
pub type SubsectorR = crate::FieldReader<u32>;
#[doc = "Field `SUBSECTOR` writer - Upper Block Number"]
pub type SubsectorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&self) -> SubsectorR {
        SubsectorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&mut self) -> SubsectorW<'_, UpperwrprotSpec> {
        SubsectorW::new(self, 0)
    }
}
#[doc = "Upper Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`upperwrprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upperwrprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpperwrprotSpec;
impl crate::RegisterSpec for UpperwrprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`upperwrprot::R`](R) reader structure"]
impl crate::Readable for UpperwrprotSpec {}
#[doc = "`write(|w| ..)` method takes [`upperwrprot::W`](W) writer structure"]
impl crate::Writable for UpperwrprotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPPERWRPROT to value 0"]
impl crate::Resettable for UpperwrprotSpec {}
