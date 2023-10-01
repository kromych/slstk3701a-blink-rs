#[doc = "Register `USBCTRL` reader"]
pub type R = crate::R<USBCTRL_SPEC>;
#[doc = "Register `USBCTRL` writer"]
pub type W = crate::W<USBCTRL_SPEC>;
#[doc = "Field `USBCLKSEL` reader - USB Rate Clock Select"]
pub type USBCLKSEL_R = crate::FieldReader<USBCLKSEL_A>;
#[doc = "USB Rate Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBCLKSEL_A {
    #[doc = "0: USHFRCO (clock recovery) is clocking USB"]
    USHFRCO = 0,
    #[doc = "1: HFXO clock is used to clock USB"]
    HFXO = 1,
    #[doc = "2: HFXO clock doubler is used to clock USB"]
    HFXOX2 = 2,
    #[doc = "3: HFRCO clock is used to clock USB"]
    HFRCO = 3,
    #[doc = "4: LFXO clock is used to clock USB"]
    LFXO = 4,
    #[doc = "5: LFRCO clock is used to clock USB"]
    LFRCO = 5,
}
impl From<USBCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBCLKSEL_A {
    type Ux = u8;
}
impl USBCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USBCLKSEL_A> {
        match self.bits {
            0 => Some(USBCLKSEL_A::USHFRCO),
            1 => Some(USBCLKSEL_A::HFXO),
            2 => Some(USBCLKSEL_A::HFXOX2),
            3 => Some(USBCLKSEL_A::HFRCO),
            4 => Some(USBCLKSEL_A::LFXO),
            5 => Some(USBCLKSEL_A::LFRCO),
            _ => None,
        }
    }
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == USBCLKSEL_A::USHFRCO
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == USBCLKSEL_A::HFXO
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline(always)]
    pub fn is_hfxox2(&self) -> bool {
        *self == USBCLKSEL_A::HFXOX2
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == USBCLKSEL_A::HFRCO
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == USBCLKSEL_A::LFXO
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == USBCLKSEL_A::LFRCO
    }
}
#[doc = "Field `USBCLKSEL` writer - USB Rate Clock Select"]
pub type USBCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, USBCLKSEL_A>;
impl<'a, REG, const O: u8> USBCLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::USHFRCO)
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::HFXO)
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline(always)]
    pub fn hfxox2(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::HFXOX2)
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::HFRCO)
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::LFXO)
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::LFRCO)
    }
}
#[doc = "Field `USBCLKEN` reader - USB Rate Clock Enable"]
pub type USBCLKEN_R = crate::BitReader;
#[doc = "Field `USBCLKEN` writer - USB Rate Clock Enable"]
pub type USBCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&self) -> USBCLKSEL_R {
        USBCLKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&self) -> USBCLKEN_R {
        USBCLKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbclksel(&mut self) -> USBCLKSEL_W<USBCTRL_SPEC, 0> {
        USBCLKSEL_W::new(self)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbclken(&mut self) -> USBCLKEN_W<USBCTRL_SPEC, 7> {
        USBCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCTRL_SPEC;
impl crate::RegisterSpec for USBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbctrl::R`](R) reader structure"]
impl crate::Readable for USBCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbctrl::W`](W) writer structure"]
impl crate::Writable for USBCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCTRL to value 0"]
impl crate::Resettable for USBCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
