#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `CMP` writer - Set CMP Interrupt Flag"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV` writer - Set CONV Interrupt Flag"]
pub type ConvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` writer - Set EOS Interrupt Flag"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOF` writer - Set DMAOF Interrupt Flag"]
pub type DmaofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTCONFLICT` writer - Set APORTCONFLICT Interrupt Flag"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set CMP Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, IfsSpec> {
        CmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Set CONV Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&mut self) -> ConvW<'_, IfsSpec> {
        ConvW::new(self, 1)
    }
    #[doc = "Bit 2 - Set EOS Interrupt Flag"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IfsSpec> {
        EosW::new(self, 2)
    }
    #[doc = "Bit 3 - Set DMAOF Interrupt Flag"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DmaofW<'_, IfsSpec> {
        DmaofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> AportconflictW<'_, IfsSpec> {
        AportconflictW::new(self, 4)
    }
}
#[doc = "Interrupt Flag Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
