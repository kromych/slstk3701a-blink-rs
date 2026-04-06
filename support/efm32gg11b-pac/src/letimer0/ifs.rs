#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `COMP0` writer - Set COMP0 Interrupt Flag"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` writer - Set COMP1 Interrupt Flag"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Set UF Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP0` writer - Set REP0 Interrupt Flag"]
pub type Rep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP1` writer - Set REP1 Interrupt Flag"]
pub type Rep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set COMP0 Interrupt Flag"]
    #[inline(always)]
    pub fn comp0(&mut self) -> Comp0W<'_, IfsSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set COMP1 Interrupt Flag"]
    #[inline(always)]
    pub fn comp1(&mut self) -> Comp1W<'_, IfsSpec> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfsSpec> {
        UfW::new(self, 2)
    }
    #[doc = "Bit 3 - Set REP0 Interrupt Flag"]
    #[inline(always)]
    pub fn rep0(&mut self) -> Rep0W<'_, IfsSpec> {
        Rep0W::new(self, 3)
    }
    #[doc = "Bit 4 - Set REP1 Interrupt Flag"]
    #[inline(always)]
    pub fn rep1(&mut self) -> Rep1W<'_, IfsSpec> {
        Rep1W::new(self, 4)
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
