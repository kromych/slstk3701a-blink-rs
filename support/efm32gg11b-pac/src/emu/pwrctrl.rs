#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PWRCTRL_SPEC>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PWRCTRL_SPEC>;
#[doc = "Field `ANASW` reader - Analog Switch Selection"]
pub type ANASW_R = crate::BitReader;
#[doc = "Field `ANASW` writer - Analog Switch Selection"]
pub type ANASW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REGPWRSEL` reader - This Field Selects the Input Supply Pin for the Digital LDO"]
pub type REGPWRSEL_R = crate::BitReader;
#[doc = "Field `REGPWRSEL` writer - This Field Selects the Input Supply Pin for the Digital LDO"]
pub type REGPWRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMMEDIATEPWRSWITCH` reader - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
pub type IMMEDIATEPWRSWITCH_R = crate::BitReader;
#[doc = "Field `IMMEDIATEPWRSWITCH` writer - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
pub type IMMEDIATEPWRSWITCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&self) -> ANASW_R {
        ANASW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - This Field Selects the Input Supply Pin for the Digital LDO"]
    #[inline(always)]
    pub fn regpwrsel(&self) -> REGPWRSEL_R {
        REGPWRSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
    #[inline(always)]
    pub fn immediatepwrswitch(&self) -> IMMEDIATEPWRSWITCH_R {
        IMMEDIATEPWRSWITCH_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    #[must_use]
    pub fn anasw(&mut self) -> ANASW_W<PWRCTRL_SPEC, 5> {
        ANASW_W::new(self)
    }
    #[doc = "Bit 10 - This Field Selects the Input Supply Pin for the Digital LDO"]
    #[inline(always)]
    #[must_use]
    pub fn regpwrsel(&mut self) -> REGPWRSEL_W<PWRCTRL_SPEC, 10> {
        REGPWRSEL_W::new(self)
    }
    #[doc = "Bit 13 - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
    #[inline(always)]
    #[must_use]
    pub fn immediatepwrswitch(&mut self) -> IMMEDIATEPWRSWITCH_W<PWRCTRL_SPEC, 13> {
        IMMEDIATEPWRSWITCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PWRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
