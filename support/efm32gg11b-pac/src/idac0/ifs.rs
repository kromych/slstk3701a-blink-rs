#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `CURSTABLE` writer - Set CURSTABLE Interrupt Flag"]
pub type CurstableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set CURSTABLE Interrupt Flag"]
    #[inline(always)]
    pub fn curstable(&mut self) -> CurstableW<'_, IfsSpec> {
        CurstableW::new(self, 0)
    }
    #[doc = "Bit 1 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> AportconflictW<'_, IfsSpec> {
        AportconflictW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
