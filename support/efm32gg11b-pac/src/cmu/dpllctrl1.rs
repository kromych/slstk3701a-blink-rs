#[doc = "Register `DPLLCTRL1` reader"]
pub type R = crate::R<DPLLCTRL1_SPEC>;
#[doc = "Register `DPLLCTRL1` writer"]
pub type W = crate::W<DPLLCTRL1_SPEC>;
#[doc = "Field `M` reader - Factor M"]
pub type M_R = crate::FieldReader<u16>;
#[doc = "Field `M` writer - Factor M"]
pub type M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `N` reader - Factor N"]
pub type N_R = crate::FieldReader<u16>;
#[doc = "Field `N` writer - Factor N"]
pub type N_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<DPLLCTRL1_SPEC, 0> {
        M_W::new(self)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> N_W<DPLLCTRL1_SPEC, 16> {
        N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DPLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPLLCTRL1_SPEC;
impl crate::RegisterSpec for DPLLCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrl1::R`](R) reader structure"]
impl crate::Readable for DPLLCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpllctrl1::W`](W) writer structure"]
impl crate::Writable for DPLLCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLCTRL1 to value 0"]
impl crate::Resettable for DPLLCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
