#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel0 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    Ulfrco = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    Lfrco = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    Lfxo = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    Hfxo = 6,
    #[doc = "7: HFEXPCLK"]
    Hfexpclk = 7,
    #[doc = "9: ULFRCO (qualified)"]
    Ulfrcoq = 9,
    #[doc = "10: LFRCO (qualified)"]
    Lfrcoq = 10,
    #[doc = "11: LFXO (qualified)"]
    Lfxoq = 11,
    #[doc = "12: HFRCO (qualified)"]
    Hfrcoq = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    Auxhfrcoq = 13,
    #[doc = "14: HFXO (qualified)"]
    Hfxoq = 14,
    #[doc = "15: HFSRCCLK"]
    Hfsrcclk = 15,
    #[doc = "18: USHFRCO (qualified)"]
    Ushfrcoq = 18,
}
impl From<Clkoutsel0> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel0 {}
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type Clkoutsel0R = crate::FieldReader<Clkoutsel0>;
impl Clkoutsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel0> {
        match self.bits {
            0 => Some(Clkoutsel0::Disabled),
            1 => Some(Clkoutsel0::Ulfrco),
            2 => Some(Clkoutsel0::Lfrco),
            3 => Some(Clkoutsel0::Lfxo),
            6 => Some(Clkoutsel0::Hfxo),
            7 => Some(Clkoutsel0::Hfexpclk),
            9 => Some(Clkoutsel0::Ulfrcoq),
            10 => Some(Clkoutsel0::Lfrcoq),
            11 => Some(Clkoutsel0::Lfxoq),
            12 => Some(Clkoutsel0::Hfrcoq),
            13 => Some(Clkoutsel0::Auxhfrcoq),
            14 => Some(Clkoutsel0::Hfxoq),
            15 => Some(Clkoutsel0::Hfsrcclk),
            18 => Some(Clkoutsel0::Ushfrcoq),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutsel0::Disabled
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel0::Ulfrco
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel0::Lfrco
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel0::Lfxo
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel0::Hfxo
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == Clkoutsel0::Hfexpclk
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == Clkoutsel0::Ulfrcoq
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == Clkoutsel0::Lfrcoq
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == Clkoutsel0::Lfxoq
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == Clkoutsel0::Hfrcoq
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == Clkoutsel0::Auxhfrcoq
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == Clkoutsel0::Hfxoq
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == Clkoutsel0::Hfsrcclk
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == Clkoutsel0::Ushfrcoq
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type Clkoutsel0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Clkoutsel0>;
impl<'a, REG> Clkoutsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Disabled)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Ulfrco)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Lfrco)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Lfxo)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfxo)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfexpclk)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Ulfrcoq)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Lfrcoq)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Lfxoq)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfrcoq)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Auxhfrcoq)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfxoq)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfsrcclk)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Ushfrcoq)
    }
}
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel1 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    Ulfrco = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    Lfrco = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    Lfxo = 3,
    #[doc = "6: HFXO (directly from oscillator)"]
    Hfxo = 6,
    #[doc = "7: HFEXPCLK"]
    Hfexpclk = 7,
    #[doc = "9: ULFRCO (qualified)"]
    Ulfrcoq = 9,
    #[doc = "10: LFRCO (qualified)"]
    Lfrcoq = 10,
    #[doc = "11: LFXO (qualified)"]
    Lfxoq = 11,
    #[doc = "12: HFRCO (qualified)"]
    Hfrcoq = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    Auxhfrcoq = 13,
    #[doc = "14: HFXO (qualified)"]
    Hfxoq = 14,
    #[doc = "15: HFSRCCLK"]
    Hfsrcclk = 15,
    #[doc = "18: USHFRCO (qualified)"]
    Ushfrcoq = 18,
}
impl From<Clkoutsel1> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel1 {}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type Clkoutsel1R = crate::FieldReader<Clkoutsel1>;
impl Clkoutsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel1> {
        match self.bits {
            0 => Some(Clkoutsel1::Disabled),
            1 => Some(Clkoutsel1::Ulfrco),
            2 => Some(Clkoutsel1::Lfrco),
            3 => Some(Clkoutsel1::Lfxo),
            6 => Some(Clkoutsel1::Hfxo),
            7 => Some(Clkoutsel1::Hfexpclk),
            9 => Some(Clkoutsel1::Ulfrcoq),
            10 => Some(Clkoutsel1::Lfrcoq),
            11 => Some(Clkoutsel1::Lfxoq),
            12 => Some(Clkoutsel1::Hfrcoq),
            13 => Some(Clkoutsel1::Auxhfrcoq),
            14 => Some(Clkoutsel1::Hfxoq),
            15 => Some(Clkoutsel1::Hfsrcclk),
            18 => Some(Clkoutsel1::Ushfrcoq),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutsel1::Disabled
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel1::Ulfrco
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel1::Lfrco
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel1::Lfxo
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel1::Hfxo
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == Clkoutsel1::Hfexpclk
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == Clkoutsel1::Ulfrcoq
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == Clkoutsel1::Lfrcoq
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == Clkoutsel1::Lfxoq
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == Clkoutsel1::Hfrcoq
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == Clkoutsel1::Auxhfrcoq
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == Clkoutsel1::Hfxoq
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == Clkoutsel1::Hfsrcclk
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == Clkoutsel1::Ushfrcoq
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type Clkoutsel1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Clkoutsel1>;
impl<'a, REG> Clkoutsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Disabled)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Ulfrco)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfrco)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfxo)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfxo)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfexpclk)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Ulfrcoq)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfrcoq)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfxoq)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfrcoq)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Auxhfrcoq)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfxoq)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfsrcclk)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Ushfrcoq)
    }
}
#[doc = "Clock Output Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel2 {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: ULFRCO (directly from oscillator)"]
    Ulfrco = 1,
    #[doc = "2: LFRCO (directly from oscillator)"]
    Lfrco = 2,
    #[doc = "3: LFXO (directly from oscillator)"]
    Lfxo = 3,
    #[doc = "5: HFXO divided by two (qualified)"]
    Hfxodiv2q = 5,
    #[doc = "6: HFXO (directly from oscillator)"]
    Hfxo = 6,
    #[doc = "7: HFEXPCLK"]
    Hfexpclk = 7,
    #[doc = "8: HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    Hfxox2q = 8,
    #[doc = "9: ULFRCO (qualified)"]
    Ulfrcoq = 9,
    #[doc = "10: LFRCO (qualified)"]
    Lfrcoq = 10,
    #[doc = "11: LFXO (qualified)"]
    Lfxoq = 11,
    #[doc = "12: HFRCO (qualified)"]
    Hfrcoq = 12,
    #[doc = "13: AUXHFRCO (qualified)"]
    Auxhfrcoq = 13,
    #[doc = "14: HFXO (qualified)"]
    Hfxoq = 14,
    #[doc = "15: HFSRCCLK"]
    Hfsrcclk = 15,
    #[doc = "18: USHFRCO (qualified)"]
    Ushfrcoq = 18,
}
impl From<Clkoutsel2> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel2 {}
#[doc = "Field `CLKOUTSEL2` reader - Clock Output Select 2"]
pub type Clkoutsel2R = crate::FieldReader<Clkoutsel2>;
impl Clkoutsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel2> {
        match self.bits {
            0 => Some(Clkoutsel2::Disabled),
            1 => Some(Clkoutsel2::Ulfrco),
            2 => Some(Clkoutsel2::Lfrco),
            3 => Some(Clkoutsel2::Lfxo),
            5 => Some(Clkoutsel2::Hfxodiv2q),
            6 => Some(Clkoutsel2::Hfxo),
            7 => Some(Clkoutsel2::Hfexpclk),
            8 => Some(Clkoutsel2::Hfxox2q),
            9 => Some(Clkoutsel2::Ulfrcoq),
            10 => Some(Clkoutsel2::Lfrcoq),
            11 => Some(Clkoutsel2::Lfxoq),
            12 => Some(Clkoutsel2::Hfrcoq),
            13 => Some(Clkoutsel2::Auxhfrcoq),
            14 => Some(Clkoutsel2::Hfxoq),
            15 => Some(Clkoutsel2::Hfsrcclk),
            18 => Some(Clkoutsel2::Ushfrcoq),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutsel2::Disabled
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel2::Ulfrco
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel2::Lfrco
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel2::Lfxo
    }
    #[doc = "HFXO divided by two (qualified)"]
    #[inline(always)]
    pub fn is_hfxodiv2q(&self) -> bool {
        *self == Clkoutsel2::Hfxodiv2q
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel2::Hfxo
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == Clkoutsel2::Hfexpclk
    }
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    #[inline(always)]
    pub fn is_hfxox2q(&self) -> bool {
        *self == Clkoutsel2::Hfxox2q
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == Clkoutsel2::Ulfrcoq
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == Clkoutsel2::Lfrcoq
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == Clkoutsel2::Lfxoq
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == Clkoutsel2::Hfrcoq
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == Clkoutsel2::Auxhfrcoq
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == Clkoutsel2::Hfxoq
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == Clkoutsel2::Hfsrcclk
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == Clkoutsel2::Ushfrcoq
    }
}
#[doc = "Field `CLKOUTSEL2` writer - Clock Output Select 2"]
pub type Clkoutsel2W<'a, REG> = crate::FieldWriter<'a, REG, 5, Clkoutsel2>;
impl<'a, REG> Clkoutsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Disabled)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Ulfrco)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Lfrco)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Lfxo)
    }
    #[doc = "HFXO divided by two (qualified)"]
    #[inline(always)]
    pub fn hfxodiv2q(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfxodiv2q)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfxo)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfexpclk)
    }
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    #[inline(always)]
    pub fn hfxox2q(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfxox2q)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Ulfrcoq)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Lfrcoq)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Lfxoq)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfrcoq)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Auxhfrcoq)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfxoq)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfsrcclk)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Ushfrcoq)
    }
}
#[doc = "Field `WSHFLE` reader - Wait State for High-Frequency LE Interface"]
pub type WshfleR = crate::BitReader;
#[doc = "Field `WSHFLE` writer - Wait State for High-Frequency LE Interface"]
pub type WshfleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFPERCLKEN` reader - HFPERCLK Enable"]
pub type HfperclkenR = crate::BitReader;
#[doc = "Field `HFPERCLKEN` writer - HFPERCLK Enable"]
pub type HfperclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> Clkoutsel0R {
        Clkoutsel0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> Clkoutsel1R {
        Clkoutsel1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&self) -> Clkoutsel2R {
        Clkoutsel2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&self) -> WshfleR {
        WshfleR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HfperclkenR {
        HfperclkenR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&mut self) -> Clkoutsel0W<'_, CtrlSpec> {
        Clkoutsel0W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&mut self) -> Clkoutsel1W<'_, CtrlSpec> {
        Clkoutsel1W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&mut self) -> Clkoutsel2W<'_, CtrlSpec> {
        Clkoutsel2W::new(self, 10)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&mut self) -> WshfleW<'_, CtrlSpec> {
        WshfleW::new(self, 16)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&mut self) -> HfperclkenW<'_, CtrlSpec> {
        HfperclkenW::new(self, 20)
    }
}
#[doc = "CMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x0010_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
