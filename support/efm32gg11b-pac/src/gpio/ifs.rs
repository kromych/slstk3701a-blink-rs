#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `EXT` writer - Set EXT Interrupt Flag"]
pub type ExtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM4WU` writer - Set EM4WU Interrupt Flag"]
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Set EXT Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&mut self) -> ExtW<'_, IfsSpec> {
        ExtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Set EM4WU Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> Em4wuW<'_, IfsSpec> {
        Em4wuW::new(self, 16)
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
