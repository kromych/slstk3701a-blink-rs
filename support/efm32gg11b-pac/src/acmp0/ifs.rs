#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `EDGE` writer - Set EDGE Interrupt Flag"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARMUP` writer - Set WARMUP Interrupt Flag"]
pub type WarmupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set EDGE Interrupt Flag"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<'_, IfsSpec> {
        EdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - Set WARMUP Interrupt Flag"]
    #[inline(always)]
    pub fn warmup(&mut self) -> WarmupW<'_, IfsSpec> {
        WarmupW::new(self, 1)
    }
    #[doc = "Bit 2 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> AportconflictW<'_, IfsSpec> {
        AportconflictW::new(self, 2)
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
