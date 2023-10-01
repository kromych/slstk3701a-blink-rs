#[doc = "Register `ECCCTRL` reader"]
pub type R = crate::R<ECCCTRL_SPEC>;
#[doc = "Register `ECCCTRL` writer"]
pub type W = crate::W<ECCCTRL_SPEC>;
#[doc = "Field `RAMECCEWEN` reader - RAM ECC Write Enable"]
pub type RAMECCEWEN_R = crate::BitReader;
#[doc = "Field `RAMECCEWEN` writer - RAM ECC Write Enable"]
pub type RAMECCEWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMECCCHKEN` reader - RAM ECC Check Enable"]
pub type RAMECCCHKEN_R = crate::BitReader;
#[doc = "Field `RAMECCCHKEN` writer - RAM ECC Check Enable"]
pub type RAMECCCHKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAM1ECCEWEN` reader - RAM1 ECC Write Enable"]
pub type RAM1ECCEWEN_R = crate::BitReader;
#[doc = "Field `RAM1ECCEWEN` writer - RAM1 ECC Write Enable"]
pub type RAM1ECCEWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAM1ECCCHKEN` reader - RAM1 ECC Check Enable"]
pub type RAM1ECCCHKEN_R = crate::BitReader;
#[doc = "Field `RAM1ECCCHKEN` writer - RAM1 ECC Check Enable"]
pub type RAM1ECCCHKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&self) -> RAMECCEWEN_R {
        RAMECCEWEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&self) -> RAMECCCHKEN_R {
        RAMECCCHKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&self) -> RAM1ECCEWEN_R {
        RAM1ECCEWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&self) -> RAM1ECCCHKEN_R {
        RAM1ECCCHKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rameccewen(&mut self) -> RAMECCEWEN_W<ECCCTRL_SPEC, 0> {
        RAMECCEWEN_W::new(self)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rameccchken(&mut self) -> RAMECCCHKEN_W<ECCCTRL_SPEC, 1> {
        RAMECCCHKEN_W::new(self)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram1eccewen(&mut self) -> RAM1ECCEWEN_W<ECCCTRL_SPEC, 2> {
        RAM1ECCEWEN_W::new(self)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram1eccchken(&mut self) -> RAM1ECCCHKEN_W<ECCCTRL_SPEC, 3> {
        RAM1ECCCHKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAM ECC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCCTRL_SPEC;
impl crate::RegisterSpec for ECCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccctrl::R`](R) reader structure"]
impl crate::Readable for ECCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccctrl::W`](W) writer structure"]
impl crate::Writable for ECCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCCTRL to value 0"]
impl crate::Resettable for ECCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
