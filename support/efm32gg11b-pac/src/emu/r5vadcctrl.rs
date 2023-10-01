#[doc = "Register `R5VADCCTRL` reader"]
pub type R = crate::R<R5VADCCTRL_SPEC>;
#[doc = "Register `R5VADCCTRL` writer"]
pub type W = crate::W<R5VADCCTRL_SPEC>;
#[doc = "Field `ENAMUX` reader - Enable the 5V Subsystem ADC MUX"]
pub type ENAMUX_R = crate::BitReader;
#[doc = "Field `ENAMUX` writer - Enable the 5V Subsystem ADC MUX"]
pub type ENAMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMUXSEL` reader - ADC Mux Selection"]
pub type AMUXSEL_R = crate::FieldReader<AMUXSEL_A>;
#[doc = "ADC Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMUXSEL_A {
    #[doc = "0: VBUS divided by 10"]
    VBUSDIV10 = 0,
    #[doc = "1: VREGI divided by 10"]
    VREGIDIV10 = 1,
    #[doc = "2: VREGO divided by 6"]
    VREGODIV6 = 2,
    #[doc = "3: VREGI current monitor"]
    VREGIIMON = 3,
    #[doc = "4: VBUS current monitor"]
    VBUSIMON = 4,
}
impl From<AMUXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: AMUXSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AMUXSEL_A {
    type Ux = u8;
}
impl AMUXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMUXSEL_A> {
        match self.bits {
            0 => Some(AMUXSEL_A::VBUSDIV10),
            1 => Some(AMUXSEL_A::VREGIDIV10),
            2 => Some(AMUXSEL_A::VREGODIV6),
            3 => Some(AMUXSEL_A::VREGIIMON),
            4 => Some(AMUXSEL_A::VBUSIMON),
            _ => None,
        }
    }
    #[doc = "VBUS divided by 10"]
    #[inline(always)]
    pub fn is_vbusdiv10(&self) -> bool {
        *self == AMUXSEL_A::VBUSDIV10
    }
    #[doc = "VREGI divided by 10"]
    #[inline(always)]
    pub fn is_vregidiv10(&self) -> bool {
        *self == AMUXSEL_A::VREGIDIV10
    }
    #[doc = "VREGO divided by 6"]
    #[inline(always)]
    pub fn is_vregodiv6(&self) -> bool {
        *self == AMUXSEL_A::VREGODIV6
    }
    #[doc = "VREGI current monitor"]
    #[inline(always)]
    pub fn is_vregiimon(&self) -> bool {
        *self == AMUXSEL_A::VREGIIMON
    }
    #[doc = "VBUS current monitor"]
    #[inline(always)]
    pub fn is_vbusimon(&self) -> bool {
        *self == AMUXSEL_A::VBUSIMON
    }
}
#[doc = "Field `AMUXSEL` writer - ADC Mux Selection"]
pub type AMUXSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, AMUXSEL_A>;
impl<'a, REG, const O: u8> AMUXSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VBUS divided by 10"]
    #[inline(always)]
    pub fn vbusdiv10(self) -> &'a mut crate::W<REG> {
        self.variant(AMUXSEL_A::VBUSDIV10)
    }
    #[doc = "VREGI divided by 10"]
    #[inline(always)]
    pub fn vregidiv10(self) -> &'a mut crate::W<REG> {
        self.variant(AMUXSEL_A::VREGIDIV10)
    }
    #[doc = "VREGO divided by 6"]
    #[inline(always)]
    pub fn vregodiv6(self) -> &'a mut crate::W<REG> {
        self.variant(AMUXSEL_A::VREGODIV6)
    }
    #[doc = "VREGI current monitor"]
    #[inline(always)]
    pub fn vregiimon(self) -> &'a mut crate::W<REG> {
        self.variant(AMUXSEL_A::VREGIIMON)
    }
    #[doc = "VBUS current monitor"]
    #[inline(always)]
    pub fn vbusimon(self) -> &'a mut crate::W<REG> {
        self.variant(AMUXSEL_A::VBUSIMON)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&self) -> ENAMUX_R {
        ENAMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&self) -> AMUXSEL_R {
        AMUXSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    #[must_use]
    pub fn enamux(&mut self) -> ENAMUX_W<R5VADCCTRL_SPEC, 0> {
        ENAMUX_W::new(self)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn amuxsel(&mut self) -> AMUXSEL_W<R5VADCCTRL_SPEC, 12> {
        AMUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "5V Regulator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r5vadcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r5vadcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5VADCCTRL_SPEC;
impl crate::RegisterSpec for R5VADCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5vadcctrl::R`](R) reader structure"]
impl crate::Readable for R5VADCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r5vadcctrl::W`](W) writer structure"]
impl crate::Writable for R5VADCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R5VADCCTRL to value 0"]
impl crate::Resettable for R5VADCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
