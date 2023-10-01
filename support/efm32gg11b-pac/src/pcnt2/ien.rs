#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UF_R = crate::BitReader;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIRCNG` reader - DIRCNG Interrupt Enable"]
pub type DIRCNG_R = crate::BitReader;
#[doc = "Field `DIRCNG` writer - DIRCNG Interrupt Enable"]
pub type DIRCNG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUXOF` reader - AUXOF Interrupt Enable"]
pub type AUXOF_R = crate::BitReader;
#[doc = "Field `AUXOF` writer - AUXOF Interrupt Enable"]
pub type AUXOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC` reader - TCC Interrupt Enable"]
pub type TCC_R = crate::BitReader;
#[doc = "Field `TCC` writer - TCC Interrupt Enable"]
pub type TCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OQSTERR` reader - OQSTERR Interrupt Enable"]
pub type OQSTERR_R = crate::BitReader;
#[doc = "Field `OQSTERR` writer - OQSTERR Interrupt Enable"]
pub type OQSTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIRCNG Interrupt Enable"]
    #[inline(always)]
    pub fn dircng(&self) -> DIRCNG_R {
        DIRCNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AUXOF Interrupt Enable"]
    #[inline(always)]
    pub fn auxof(&self) -> AUXOF_R {
        AUXOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC Interrupt Enable"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OQSTERR Interrupt Enable"]
    #[inline(always)]
    pub fn oqsterr(&self) -> OQSTERR_R {
        OQSTERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IEN_SPEC, 0> {
        UF_W::new(self)
    }
    #[doc = "Bit 1 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IEN_SPEC, 1> {
        OF_W::new(self)
    }
    #[doc = "Bit 2 - DIRCNG Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DIRCNG_W<IEN_SPEC, 2> {
        DIRCNG_W::new(self)
    }
    #[doc = "Bit 3 - AUXOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AUXOF_W<IEN_SPEC, 3> {
        AUXOF_W::new(self)
    }
    #[doc = "Bit 4 - TCC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<IEN_SPEC, 4> {
        TCC_W::new(self)
    }
    #[doc = "Bit 5 - OQSTERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OQSTERR_W<IEN_SPEC, 5> {
        OQSTERR_W::new(self)
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
