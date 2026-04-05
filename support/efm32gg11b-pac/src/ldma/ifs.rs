#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `DONE` writer - Set DONE Interrupt Flag"]
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ERROR` writer - Set ERROR Interrupt Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - Set DONE Interrupt Flag"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IfsSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 31 - Set ERROR Interrupt Flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IfsSpec> {
        ErrorW::new(self, 31)
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
