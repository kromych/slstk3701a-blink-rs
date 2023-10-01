#[doc = "Register `DPLLCTRL` reader"]
pub type R = crate::R<DPLLCTRL_SPEC>;
#[doc = "Register `DPLLCTRL` writer"]
pub type W = crate::W<DPLLCTRL_SPEC>;
#[doc = "Field `MODE` reader - Operating Mode Control"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Operating Mode Control"]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDGESEL` reader - Reference Edge Select"]
pub type EDGESEL_R = crate::BitReader;
#[doc = "Field `EDGESEL` writer - Reference Edge Select"]
pub type EDGESEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTORECOVER` reader - Automatic Recovery Ctrl"]
pub type AUTORECOVER_R = crate::BitReader;
#[doc = "Field `AUTORECOVER` writer - Automatic Recovery Ctrl"]
pub type AUTORECOVER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REFSEL` reader - Reference Clock Selection Control"]
pub type REFSEL_R = crate::FieldReader<REFSEL_A>;
#[doc = "Reference Clock Selection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: HFXO selected"]
    HFXO = 0,
    #[doc = "1: LFXO selected"]
    LFXO = 1,
    #[doc = "2: USHFRCO selected"]
    USHFRCO = 2,
    #[doc = "3: CLKIN0 selected"]
    CLKIN0 = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFSEL_A {
    type Ux = u8;
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::HFXO,
            1 => REFSEL_A::LFXO,
            2 => REFSEL_A::USHFRCO,
            3 => REFSEL_A::CLKIN0,
            _ => unreachable!(),
        }
    }
    #[doc = "HFXO selected"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == REFSEL_A::HFXO
    }
    #[doc = "LFXO selected"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == REFSEL_A::LFXO
    }
    #[doc = "USHFRCO selected"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == REFSEL_A::USHFRCO
    }
    #[doc = "CLKIN0 selected"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == REFSEL_A::CLKIN0
    }
}
#[doc = "Field `REFSEL` writer - Reference Clock Selection Control"]
pub type REFSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, REFSEL_A>;
impl<'a, REG, const O: u8> REFSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFXO selected"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::HFXO)
    }
    #[doc = "LFXO selected"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::LFXO)
    }
    #[doc = "USHFRCO selected"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::USHFRCO)
    }
    #[doc = "CLKIN0 selected"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::CLKIN0)
    }
}
#[doc = "Field `DITHEN` reader - Dither Enable Control"]
pub type DITHEN_R = crate::BitReader;
#[doc = "Field `DITHEN` writer - Dither Enable Control"]
pub type DITHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Recovery Ctrl"]
    #[inline(always)]
    pub fn autorecover(&self) -> AUTORECOVER_R {
        AUTORECOVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reference Clock Selection Control"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<DPLLCTRL_SPEC, 0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edgesel(&mut self) -> EDGESEL_W<DPLLCTRL_SPEC, 1> {
        EDGESEL_W::new(self)
    }
    #[doc = "Bit 2 - Automatic Recovery Ctrl"]
    #[inline(always)]
    #[must_use]
    pub fn autorecover(&mut self) -> AUTORECOVER_W<DPLLCTRL_SPEC, 2> {
        AUTORECOVER_W::new(self)
    }
    #[doc = "Bits 3:4 - Reference Clock Selection Control"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<DPLLCTRL_SPEC, 3> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn dithen(&mut self) -> DITHEN_W<DPLLCTRL_SPEC, 6> {
        DITHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DPLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpllctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPLLCTRL_SPEC;
impl crate::RegisterSpec for DPLLCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrl::R`](R) reader structure"]
impl crate::Readable for DPLLCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpllctrl::W`](W) writer structure"]
impl crate::Writable for DPLLCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLCTRL to value 0"]
impl crate::Resettable for DPLLCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
