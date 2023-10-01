#[doc = "Register `R5VCTRL` reader"]
pub type R = crate::R<R5VCTRL_SPEC>;
#[doc = "Register `R5VCTRL` writer"]
pub type W = crate::W<R5VCTRL_SPEC>;
#[doc = "Field `BYPASS` reader - 5V Regulator Bypass"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - 5V Regulator Bypass"]
pub type BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WUEN` reader - Enable EM4 Wakeup Due to VBUS Detection"]
pub type EM4WUEN_R = crate::BitReader;
#[doc = "Field `EM4WUEN` writer - Enable EM4 Wakeup Due to VBUS Detection"]
pub type EM4WUEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMONEN` reader - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
pub type IMONEN_R = crate::BitReader;
#[doc = "Field `IMONEN` writer - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
pub type IMONEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INPUTMODE` reader - 5V Input Mode"]
pub type INPUTMODE_R = crate::FieldReader<INPUTMODE_A>;
#[doc = "5V Input Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTMODE_A {
    #[doc = "0: Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    AUTO = 0,
    #[doc = "1: Force VBUS pin as the regulator input"]
    VBUS = 1,
    #[doc = "2: Force VREGI pin as the regulator input"]
    VREGI = 2,
}
impl From<INPUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUTMODE_A {
    type Ux = u8;
}
impl INPUTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPUTMODE_A> {
        match self.bits {
            0 => Some(INPUTMODE_A::AUTO),
            1 => Some(INPUTMODE_A::VBUS),
            2 => Some(INPUTMODE_A::VREGI),
            _ => None,
        }
    }
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == INPUTMODE_A::AUTO
    }
    #[doc = "Force VBUS pin as the regulator input"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == INPUTMODE_A::VBUS
    }
    #[doc = "Force VREGI pin as the regulator input"]
    #[inline(always)]
    pub fn is_vregi(&self) -> bool {
        *self == INPUTMODE_A::VREGI
    }
}
#[doc = "Field `INPUTMODE` writer - 5V Input Mode"]
pub type INPUTMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, INPUTMODE_A>;
impl<'a, REG, const O: u8> INPUTMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::AUTO)
    }
    #[doc = "Force VBUS pin as the regulator input"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::VBUS)
    }
    #[doc = "Force VREGI pin as the regulator input"]
    #[inline(always)]
    pub fn vregi(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTMODE_A::VREGI)
    }
}
impl R {
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline(always)]
    pub fn imonen(&self) -> IMONEN_R {
        IMONEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline(always)]
    pub fn inputmode(&self) -> INPUTMODE_R {
        INPUTMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<R5VCTRL_SPEC, 0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline(always)]
    #[must_use]
    pub fn em4wuen(&mut self) -> EM4WUEN_W<R5VCTRL_SPEC, 1> {
        EM4WUEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn imonen(&mut self) -> IMONEN_W<R5VCTRL_SPEC, 2> {
        IMONEN_W::new(self)
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inputmode(&mut self) -> INPUTMODE_W<R5VCTRL_SPEC, 8> {
        INPUTMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "5V Regulator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5vctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5VCTRL_SPEC;
impl crate::RegisterSpec for R5VCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vctrl::R`](R) reader structure"]
impl crate::Readable for R5VCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r5vctrl::W`](W) writer structure"]
impl crate::Writable for R5VCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R5VCTRL to value 0"]
impl crate::Resettable for R5VCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
