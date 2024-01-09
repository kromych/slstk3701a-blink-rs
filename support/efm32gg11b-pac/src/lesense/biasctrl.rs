#[doc = "Register `BIASCTRL` reader"]
pub type R = crate::R<BIASCTRL_SPEC>;
#[doc = "Register `BIASCTRL` writer"]
pub type W = crate::W<BIASCTRL_SPEC>;
#[doc = "Field `BIASMODE` reader - Select Bias Mode"]
pub type BIASMODE_R = crate::FieldReader<BIASMODE_A>;
#[doc = "Select Bias Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIASMODE_A {
    #[doc = "0: Bias module is controlled by the EMU and is not affected by LESENSE"]
    DONTTOUCH = 0,
    #[doc = "1: Bias module duty cycled between low power and high accuracy mode"]
    DUTYCYCLE = 1,
    #[doc = "2: Bias module always in high accuracy mode"]
    HIGHACC = 2,
}
impl From<BIASMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BIASMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIASMODE_A {
    type Ux = u8;
}
impl BIASMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BIASMODE_A> {
        match self.bits {
            0 => Some(BIASMODE_A::DONTTOUCH),
            1 => Some(BIASMODE_A::DUTYCYCLE),
            2 => Some(BIASMODE_A::HIGHACC),
            _ => None,
        }
    }
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE"]
    #[inline(always)]
    pub fn is_donttouch(&self) -> bool {
        *self == BIASMODE_A::DONTTOUCH
    }
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline(always)]
    pub fn is_dutycycle(&self) -> bool {
        *self == BIASMODE_A::DUTYCYCLE
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline(always)]
    pub fn is_highacc(&self) -> bool {
        *self == BIASMODE_A::HIGHACC
    }
}
#[doc = "Field `BIASMODE` writer - Select Bias Mode"]
pub type BIASMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BIASMODE_A>;
impl<'a, REG> BIASMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE"]
    #[inline(always)]
    pub fn donttouch(self) -> &'a mut crate::W<REG> {
        self.variant(BIASMODE_A::DONTTOUCH)
    }
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline(always)]
    pub fn dutycycle(self) -> &'a mut crate::W<REG> {
        self.variant(BIASMODE_A::DUTYCYCLE)
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline(always)]
    pub fn highacc(self) -> &'a mut crate::W<REG> {
        self.variant(BIASMODE_A::HIGHACC)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select Bias Mode"]
    #[inline(always)]
    pub fn biasmode(&self) -> BIASMODE_R {
        BIASMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select Bias Mode"]
    #[inline(always)]
    #[must_use]
    pub fn biasmode(&mut self) -> BIASMODE_W<BIASCTRL_SPEC> {
        BIASMODE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bias Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASCTRL_SPEC;
impl crate::RegisterSpec for BIASCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasctrl::R`](R) reader structure"]
impl crate::Readable for BIASCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`biasctrl::W`](W) writer structure"]
impl crate::Writable for BIASCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIASCTRL to value 0"]
impl crate::Resettable for BIASCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
