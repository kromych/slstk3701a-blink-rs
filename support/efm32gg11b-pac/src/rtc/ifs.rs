#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` writer - Set COMP Interrupt Flag"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfsSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bits 1:6 - Set COMP Interrupt Flag"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, IfsSpec> {
        CompW::new(self, 1)
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
