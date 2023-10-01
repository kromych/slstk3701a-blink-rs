#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `TOUT` reader - TOUT Interrupt Enable"]
pub type TOUT_R = crate::BitReader;
#[doc = "Field `TOUT` writer - TOUT Interrupt Enable"]
pub type TOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WARN` reader - WARN Interrupt Enable"]
pub type WARN_R = crate::BitReader;
#[doc = "Field `WARN` writer - WARN Interrupt Enable"]
pub type WARN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIN` reader - WIN Interrupt Enable"]
pub type WIN_R = crate::BitReader;
#[doc = "Field `WIN` writer - WIN Interrupt Enable"]
pub type WIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEM0` reader - PEM0 Interrupt Enable"]
pub type PEM0_R = crate::BitReader;
#[doc = "Field `PEM0` writer - PEM0 Interrupt Enable"]
pub type PEM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEM1` reader - PEM1 Interrupt Enable"]
pub type PEM1_R = crate::BitReader;
#[doc = "Field `PEM1` writer - PEM1 Interrupt Enable"]
pub type PEM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<IEN_SPEC, 0> {
        TOUT_W::new(self)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WARN_W<IEN_SPEC, 1> {
        WARN_W::new(self)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<IEN_SPEC, 2> {
        WIN_W::new(self)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pem0(&mut self) -> PEM0_W<IEN_SPEC, 3> {
        PEM0_W::new(self)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pem1(&mut self) -> PEM1_W<IEN_SPEC, 4> {
        PEM1_W::new(self)
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
