#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CURSTABLE` reader - CURSTABLE Interrupt Enable"]
pub type CurstableR = crate::BitReader;
#[doc = "Field `CURSTABLE` writer - CURSTABLE Interrupt Enable"]
pub type CurstableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` reader - APORTCONFLICT Interrupt Enable"]
pub type AportconflictR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` writer - APORTCONFLICT Interrupt Enable"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CURSTABLE Interrupt Enable"]
    #[inline(always)]
    pub fn curstable(&self) -> CurstableR {
        CurstableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CURSTABLE Interrupt Enable"]
    #[inline(always)]
    pub fn curstable(&mut self) -> CurstableW<'_, IenSpec> {
        CurstableW::new(self, 0)
    }
    #[doc = "Bit 1 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> AportconflictW<'_, IenSpec> {
        AportconflictW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
