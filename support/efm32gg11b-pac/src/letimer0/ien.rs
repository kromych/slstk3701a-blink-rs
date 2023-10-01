#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `COMP0` reader - COMP0 Interrupt Enable"]
pub type COMP0_R = crate::BitReader;
#[doc = "Field `COMP0` writer - COMP0 Interrupt Enable"]
pub type COMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP1` reader - COMP1 Interrupt Enable"]
pub type COMP1_R = crate::BitReader;
#[doc = "Field `COMP1` writer - COMP1 Interrupt Enable"]
pub type COMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UF_R = crate::BitReader;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REP0` reader - REP0 Interrupt Enable"]
pub type REP0_R = crate::BitReader;
#[doc = "Field `REP0` writer - REP0 Interrupt Enable"]
pub type REP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REP1` reader - REP1 Interrupt Enable"]
pub type REP1_R = crate::BitReader;
#[doc = "Field `REP1` writer - REP1 Interrupt Enable"]
pub type REP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - COMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REP0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REP1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> COMP0_W<IEN_SPEC, 0> {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - COMP1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> COMP1_W<IEN_SPEC, 1> {
        COMP1_W::new(self)
    }
    #[doc = "Bit 2 - UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IEN_SPEC, 2> {
        UF_W::new(self)
    }
    #[doc = "Bit 3 - REP0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rep0(&mut self) -> REP0_W<IEN_SPEC, 3> {
        REP0_W::new(self)
    }
    #[doc = "Bit 4 - REP1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rep1(&mut self) -> REP1_W<IEN_SPEC, 4> {
        REP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
