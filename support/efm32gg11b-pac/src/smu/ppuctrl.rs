#[doc = "Register `PPUCTRL` reader"]
pub type R = crate::R<PPUCTRL_SPEC>;
#[doc = "Register `PPUCTRL` writer"]
pub type W = crate::W<PPUCTRL_SPEC>;
#[doc = "Field `ENABLE` reader - "]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - "]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<PPUCTRL_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppuctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppuctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPUCTRL_SPEC;
impl crate::RegisterSpec for PPUCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppuctrl::R`](R) reader structure"]
impl crate::Readable for PPUCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppuctrl::W`](W) writer structure"]
impl crate::Writable for PPUCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUCTRL to value 0"]
impl crate::Resettable for PPUCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
