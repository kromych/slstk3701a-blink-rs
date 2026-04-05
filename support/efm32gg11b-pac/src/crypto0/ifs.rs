#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `INSTRDONE` writer - Set INSTRDONE Interrupt Flag"]
pub type InstrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQDONE` writer - Set SEQDONE Interrupt Flag"]
pub type SeqdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set INSTRDONE Interrupt Flag"]
    #[inline(always)]
    pub fn instrdone(&mut self) -> InstrdoneW<'_, IfsSpec> {
        InstrdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set SEQDONE Interrupt Flag"]
    #[inline(always)]
    pub fn seqdone(&mut self) -> SeqdoneW<'_, IfsSpec> {
        SeqdoneW::new(self, 1)
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
