#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `TOUT` writer - Set TOUT Interrupt Flag"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - Set WARN Interrupt Flag"]
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN` writer - Set WIN Interrupt Flag"]
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM0` writer - Set PEM0 Interrupt Flag"]
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM1` writer - Set PEM1 Interrupt Flag"]
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TOUT Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<'_, IfsSpec> {
        ToutW::new(self, 0)
    }
    #[doc = "Bit 1 - Set WARN Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&mut self) -> WarnW<'_, IfsSpec> {
        WarnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set WIN Interrupt Flag"]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<'_, IfsSpec> {
        WinW::new(self, 2)
    }
    #[doc = "Bit 3 - Set PEM0 Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&mut self) -> Pem0W<'_, IfsSpec> {
        Pem0W::new(self, 3)
    }
    #[doc = "Bit 4 - Set PEM1 Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&mut self) -> Pem1W<'_, IfsSpec> {
        Pem1W::new(self, 4)
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
