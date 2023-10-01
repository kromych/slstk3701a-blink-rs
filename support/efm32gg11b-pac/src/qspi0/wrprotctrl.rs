#[doc = "Register `WRPROTCTRL` reader"]
pub type R = crate::R<WRPROTCTRL_SPEC>;
#[doc = "Register `WRPROTCTRL` writer"]
pub type W = crate::W<WRPROTCTRL_SPEC>;
#[doc = "Field `INV` reader - Write Protection Inversion Bit"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Write Protection Inversion Bit"]
pub type INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENB` reader - Write Protection Enable Bit"]
pub type ENB_R = crate::BitReader;
#[doc = "Field `ENB` writer - Write Protection Enable Bit"]
pub type ENB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<WRPROTCTRL_SPEC, 0> {
        INV_W::new(self)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<WRPROTCTRL_SPEC, 1> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Write Protection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrprotctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrprotctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPROTCTRL_SPEC;
impl crate::RegisterSpec for WRPROTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrprotctrl::R`](R) reader structure"]
impl crate::Readable for WRPROTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrprotctrl::W`](W) writer structure"]
impl crate::Writable for WRPROTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPROTCTRL to value 0"]
impl crate::Resettable for WRPROTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
