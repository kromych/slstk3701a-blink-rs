#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type CLKOUTSEL0_R = crate::FieldReader<CLKOUTSEL0_A>;
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    ULFRCO = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    LFRCO = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    LFXO = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    HFXO = 6,
    #[doc = "7: HFEXPCLK"]
    HFEXPCLK = 7,
    #[doc = "9: ULFRCO (qualified)"]
    ULFRCOQ = 9,
    #[doc = "10: LFRCO (qualified)"]
    LFRCOQ = 10,
    #[doc = "11: LFXO (qualified)"]
    LFXOQ = 11,
    #[doc = "12: HFRCO (qualified)"]
    HFRCOQ = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    AUXHFRCOQ = 13,
    #[doc = "14: HFXO (qualified)"]
    HFXOQ = 14,
    #[doc = "15: HFSRCCLK"]
    HFSRCCLK = 15,
    #[doc = "18: USHFRCO (qualified)"]
    USHFRCOQ = 18,
}
impl From<CLKOUTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUTSEL0_A {
    type Ux = u8;
}
impl CLKOUTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL0_A> {
        match self.bits {
            0 => Some(CLKOUTSEL0_A::DISABLED),
            1 => Some(CLKOUTSEL0_A::ULFRCO),
            2 => Some(CLKOUTSEL0_A::LFRCO),
            3 => Some(CLKOUTSEL0_A::LFXO),
            6 => Some(CLKOUTSEL0_A::HFXO),
            7 => Some(CLKOUTSEL0_A::HFEXPCLK),
            9 => Some(CLKOUTSEL0_A::ULFRCOQ),
            10 => Some(CLKOUTSEL0_A::LFRCOQ),
            11 => Some(CLKOUTSEL0_A::LFXOQ),
            12 => Some(CLKOUTSEL0_A::HFRCOQ),
            13 => Some(CLKOUTSEL0_A::AUXHFRCOQ),
            14 => Some(CLKOUTSEL0_A::HFXOQ),
            15 => Some(CLKOUTSEL0_A::HFSRCCLK),
            18 => Some(CLKOUTSEL0_A::USHFRCOQ),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL0_A::DISABLED
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCO
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCO
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXO
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXO
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFEXPCLK
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCOQ
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCOQ
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXOQ
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCOQ
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::AUXHFRCOQ
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXOQ
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFSRCCLK
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::USHFRCOQ
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type CLKOUTSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, CLKOUTSEL0_A>;
impl<'a, REG, const O: u8> CLKOUTSEL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::HFSRCCLK)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL0_A::USHFRCOQ)
    }
}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type CLKOUTSEL1_R = crate::FieldReader<CLKOUTSEL1_A>;
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    ULFRCO = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    LFRCO = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    LFXO = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    HFXO = 6,
    #[doc = "7: HFEXPCLK"]
    HFEXPCLK = 7,
    #[doc = "9: ULFRCO (qualified)"]
    ULFRCOQ = 9,
    #[doc = "10: LFRCO (qualified)"]
    LFRCOQ = 10,
    #[doc = "11: LFXO (qualified)"]
    LFXOQ = 11,
    #[doc = "12: HFRCO (qualified)"]
    HFRCOQ = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    AUXHFRCOQ = 13,
    #[doc = "14: HFXO (qualified)"]
    HFXOQ = 14,
    #[doc = "15: HFSRCCLK"]
    HFSRCCLK = 15,
    #[doc = "18: USHFRCO (qualified)"]
    USHFRCOQ = 18,
}
impl From<CLKOUTSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUTSEL1_A {
    type Ux = u8;
}
impl CLKOUTSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL1_A> {
        match self.bits {
            0 => Some(CLKOUTSEL1_A::DISABLED),
            1 => Some(CLKOUTSEL1_A::ULFRCO),
            2 => Some(CLKOUTSEL1_A::LFRCO),
            3 => Some(CLKOUTSEL1_A::LFXO),
            6 => Some(CLKOUTSEL1_A::HFXO),
            7 => Some(CLKOUTSEL1_A::HFEXPCLK),
            9 => Some(CLKOUTSEL1_A::ULFRCOQ),
            10 => Some(CLKOUTSEL1_A::LFRCOQ),
            11 => Some(CLKOUTSEL1_A::LFXOQ),
            12 => Some(CLKOUTSEL1_A::HFRCOQ),
            13 => Some(CLKOUTSEL1_A::AUXHFRCOQ),
            14 => Some(CLKOUTSEL1_A::HFXOQ),
            15 => Some(CLKOUTSEL1_A::HFSRCCLK),
            18 => Some(CLKOUTSEL1_A::USHFRCOQ),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL1_A::DISABLED
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCO
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCO
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXO
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXO
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFEXPCLK
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCOQ
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCOQ
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXOQ
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFRCOQ
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::AUXHFRCOQ
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXOQ
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFSRCCLK
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::USHFRCOQ
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type CLKOUTSEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, CLKOUTSEL1_A>;
impl<'a, REG, const O: u8> CLKOUTSEL1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::HFSRCCLK)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL1_A::USHFRCOQ)
    }
}
#[doc = "Field `CLKOUTSEL2` reader - Clock Output Select 2"]
pub type CLKOUTSEL2_R = crate::FieldReader<CLKOUTSEL2_A>;
#[doc = "Clock Output Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    ULFRCO = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    LFRCO = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    LFXO = 3,
    #[doc = "5: HFXO divided by two (qualified)"]
    HFXODIV2Q = 5,
    #[doc = "6: HFXO (directly from oscillator)"]
    HFXO = 6,
    #[doc = "7: HFEXPCLK"]
    HFEXPCLK = 7,
    #[doc = "8: HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    HFXOX2Q = 8,
    #[doc = "9: ULFRCO (qualified)"]
    ULFRCOQ = 9,
    #[doc = "10: LFRCO (qualified)"]
    LFRCOQ = 10,
    #[doc = "11: LFXO (qualified)"]
    LFXOQ = 11,
    #[doc = "12: HFRCO (qualified)"]
    HFRCOQ = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    AUXHFRCOQ = 13,
    #[doc = "14: HFXO (qualified)"]
    HFXOQ = 14,
    #[doc = "15: HFSRCCLK"]
    HFSRCCLK = 15,
    #[doc = "18: USHFRCO (qualified)"]
    USHFRCOQ = 18,
}
impl From<CLKOUTSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUTSEL2_A {
    type Ux = u8;
}
impl CLKOUTSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL2_A> {
        match self.bits {
            0 => Some(CLKOUTSEL2_A::DISABLED),
            1 => Some(CLKOUTSEL2_A::ULFRCO),
            2 => Some(CLKOUTSEL2_A::LFRCO),
            3 => Some(CLKOUTSEL2_A::LFXO),
            5 => Some(CLKOUTSEL2_A::HFXODIV2Q),
            6 => Some(CLKOUTSEL2_A::HFXO),
            7 => Some(CLKOUTSEL2_A::HFEXPCLK),
            8 => Some(CLKOUTSEL2_A::HFXOX2Q),
            9 => Some(CLKOUTSEL2_A::ULFRCOQ),
            10 => Some(CLKOUTSEL2_A::LFRCOQ),
            11 => Some(CLKOUTSEL2_A::LFXOQ),
            12 => Some(CLKOUTSEL2_A::HFRCOQ),
            13 => Some(CLKOUTSEL2_A::AUXHFRCOQ),
            14 => Some(CLKOUTSEL2_A::HFXOQ),
            15 => Some(CLKOUTSEL2_A::HFSRCCLK),
            18 => Some(CLKOUTSEL2_A::USHFRCOQ),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL2_A::DISABLED
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCO
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCO
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXO
    }
    #[doc = "HFXO divided by two (qualified)"]
    #[inline(always)]
    pub fn is_hfxodiv2q(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXODIV2Q
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXO
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFEXPCLK
    }
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    #[inline(always)]
    pub fn is_hfxox2q(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXOX2Q
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCOQ
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCOQ
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXOQ
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::HFRCOQ
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::AUXHFRCOQ
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXOQ
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFSRCCLK
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::USHFRCOQ
    }
}
#[doc = "Field `CLKOUTSEL2` writer - Clock Output Select 2"]
pub type CLKOUTSEL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, CLKOUTSEL2_A>;
impl<'a, REG, const O: u8> CLKOUTSEL2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::LFXO)
    }
    #[doc = "HFXO divided by two (qualified)"]
    #[inline(always)]
    pub fn hfxodiv2q(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFXODIV2Q)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFEXPCLK)
    }
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    #[inline(always)]
    pub fn hfxox2q(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFXOX2Q)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::HFSRCCLK)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL2_A::USHFRCOQ)
    }
}
#[doc = "Field `WSHFLE` reader - Wait State for High-Frequency LE Interface"]
pub type WSHFLE_R = crate::BitReader;
#[doc = "Field `WSHFLE` writer - Wait State for High-Frequency LE Interface"]
pub type WSHFLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HFPERCLKEN_R = crate::BitReader;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HFPERCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&self) -> CLKOUTSEL2_R {
        CLKOUTSEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&self) -> WSHFLE_R {
        WSHFLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock Output Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W<CTRL_SPEC, 0> {
        CLKOUTSEL0_W::new(self)
    }
    #[doc = "Bits 5:9 - Clock Output Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W<CTRL_SPEC, 5> {
        CLKOUTSEL1_W::new(self)
    }
    #[doc = "Bits 10:14 - Clock Output Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel2(&mut self) -> CLKOUTSEL2_W<CTRL_SPEC, 10> {
        CLKOUTSEL2_W::new(self)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    #[must_use]
    pub fn wshfle(&mut self) -> WSHFLE_W<CTRL_SPEC, 16> {
        WSHFLE_W::new(self)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W<CTRL_SPEC, 20> {
        HFPERCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CMU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0010_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
