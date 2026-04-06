#[doc = "Register `SCANINPUTSEL` reader"]
pub type R = crate::R<ScaninputselSpec>;
#[doc = "Register `SCANINPUTSEL` writer"]
pub type W = crate::W<ScaninputselSpec>;
#[doc = "Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input0to7sel {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<Input0to7sel> for u8 {
    #[inline(always)]
    fn from(variant: Input0to7sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input0to7sel {
    type Ux = u8;
}
impl crate::IsEnum for Input0to7sel {}
#[doc = "Field `INPUT0TO7SEL` reader - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
pub type Input0to7selR = crate::FieldReader<Input0to7sel>;
impl Input0to7selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input0to7sel> {
        match self.bits {
            0 => Some(Input0to7sel::Aport0ch0to7),
            1 => Some(Input0to7sel::Aport0ch8to15),
            4 => Some(Input0to7sel::Aport1ch0to7),
            5 => Some(Input0to7sel::Aport1ch8to15),
            6 => Some(Input0to7sel::Aport1ch16to23),
            7 => Some(Input0to7sel::Aport1ch24to31),
            8 => Some(Input0to7sel::Aport2ch0to7),
            9 => Some(Input0to7sel::Aport2ch8to15),
            10 => Some(Input0to7sel::Aport2ch16to23),
            11 => Some(Input0to7sel::Aport2ch24to31),
            12 => Some(Input0to7sel::Aport3ch0to7),
            13 => Some(Input0to7sel::Aport3ch8to15),
            14 => Some(Input0to7sel::Aport3ch16to23),
            15 => Some(Input0to7sel::Aport3ch24to31),
            16 => Some(Input0to7sel::Aport4ch0to7),
            17 => Some(Input0to7sel::Aport4ch8to15),
            18 => Some(Input0to7sel::Aport4ch16to23),
            19 => Some(Input0to7sel::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == Input0to7sel::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == Input0to7sel::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input0to7sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input0to7sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input0to7sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input0to7sel::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == Input0to7sel::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == Input0to7sel::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == Input0to7sel::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == Input0to7sel::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input0to7sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input0to7sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input0to7sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input0to7sel::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == Input0to7sel::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == Input0to7sel::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == Input0to7sel::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == Input0to7sel::Aport4ch24to31
    }
}
#[doc = "Field `INPUT0TO7SEL` writer - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
pub type Input0to7selW<'a, REG> = crate::FieldWriter<'a, REG, 5, Input0to7sel>;
impl<'a, REG> Input0to7selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input0to7sel::Aport4ch24to31)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input8to15sel {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<Input8to15sel> for u8 {
    #[inline(always)]
    fn from(variant: Input8to15sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input8to15sel {
    type Ux = u8;
}
impl crate::IsEnum for Input8to15sel {}
#[doc = "Field `INPUT8TO15SEL` reader - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
pub type Input8to15selR = crate::FieldReader<Input8to15sel>;
impl Input8to15selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input8to15sel> {
        match self.bits {
            0 => Some(Input8to15sel::Aport0ch0to7),
            1 => Some(Input8to15sel::Aport0ch8to15),
            4 => Some(Input8to15sel::Aport1ch0to7),
            5 => Some(Input8to15sel::Aport1ch8to15),
            6 => Some(Input8to15sel::Aport1ch16to23),
            7 => Some(Input8to15sel::Aport1ch24to31),
            8 => Some(Input8to15sel::Aport2ch0to7),
            9 => Some(Input8to15sel::Aport2ch8to15),
            10 => Some(Input8to15sel::Aport2ch16to23),
            11 => Some(Input8to15sel::Aport2ch24to31),
            12 => Some(Input8to15sel::Aport3ch0to7),
            13 => Some(Input8to15sel::Aport3ch8to15),
            14 => Some(Input8to15sel::Aport3ch16to23),
            15 => Some(Input8to15sel::Aport3ch24to31),
            16 => Some(Input8to15sel::Aport4ch0to7),
            17 => Some(Input8to15sel::Aport4ch8to15),
            18 => Some(Input8to15sel::Aport4ch16to23),
            19 => Some(Input8to15sel::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == Input8to15sel::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == Input8to15sel::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input8to15sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input8to15sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input8to15sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input8to15sel::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == Input8to15sel::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == Input8to15sel::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == Input8to15sel::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == Input8to15sel::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input8to15sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input8to15sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input8to15sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input8to15sel::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == Input8to15sel::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == Input8to15sel::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == Input8to15sel::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == Input8to15sel::Aport4ch24to31
    }
}
#[doc = "Field `INPUT8TO15SEL` writer - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
pub type Input8to15selW<'a, REG> = crate::FieldWriter<'a, REG, 5, Input8to15sel>;
impl<'a, REG> Input8to15selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input8to15sel::Aport4ch24to31)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input16to23sel {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<Input16to23sel> for u8 {
    #[inline(always)]
    fn from(variant: Input16to23sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input16to23sel {
    type Ux = u8;
}
impl crate::IsEnum for Input16to23sel {}
#[doc = "Field `INPUT16TO23SEL` reader - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
pub type Input16to23selR = crate::FieldReader<Input16to23sel>;
impl Input16to23selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input16to23sel> {
        match self.bits {
            0 => Some(Input16to23sel::Aport0ch0to7),
            1 => Some(Input16to23sel::Aport0ch8to15),
            4 => Some(Input16to23sel::Aport1ch0to7),
            5 => Some(Input16to23sel::Aport1ch8to15),
            6 => Some(Input16to23sel::Aport1ch16to23),
            7 => Some(Input16to23sel::Aport1ch24to31),
            8 => Some(Input16to23sel::Aport2ch0to7),
            9 => Some(Input16to23sel::Aport2ch8to15),
            10 => Some(Input16to23sel::Aport2ch16to23),
            11 => Some(Input16to23sel::Aport2ch24to31),
            12 => Some(Input16to23sel::Aport3ch0to7),
            13 => Some(Input16to23sel::Aport3ch8to15),
            14 => Some(Input16to23sel::Aport3ch16to23),
            15 => Some(Input16to23sel::Aport3ch24to31),
            16 => Some(Input16to23sel::Aport4ch0to7),
            17 => Some(Input16to23sel::Aport4ch8to15),
            18 => Some(Input16to23sel::Aport4ch16to23),
            19 => Some(Input16to23sel::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == Input16to23sel::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == Input16to23sel::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input16to23sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input16to23sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input16to23sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input16to23sel::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == Input16to23sel::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == Input16to23sel::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == Input16to23sel::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == Input16to23sel::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input16to23sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input16to23sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input16to23sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input16to23sel::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == Input16to23sel::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == Input16to23sel::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == Input16to23sel::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == Input16to23sel::Aport4ch24to31
    }
}
#[doc = "Field `INPUT16TO23SEL` writer - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
pub type Input16to23selW<'a, REG> = crate::FieldWriter<'a, REG, 5, Input16to23sel>;
impl<'a, REG> Input16to23selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input16to23sel::Aport4ch24to31)
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input24to31sel {
    #[doc = "0: `0`"]
    Aport0ch0to7 = 0,
    #[doc = "1: `1`"]
    Aport0ch8to15 = 1,
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "8: `1000`"]
    Aport2ch0to7 = 8,
    #[doc = "9: `1001`"]
    Aport2ch8to15 = 9,
    #[doc = "10: `1010`"]
    Aport2ch16to23 = 10,
    #[doc = "11: `1011`"]
    Aport2ch24to31 = 11,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
    #[doc = "16: `10000`"]
    Aport4ch0to7 = 16,
    #[doc = "17: `10001`"]
    Aport4ch8to15 = 17,
    #[doc = "18: `10010`"]
    Aport4ch16to23 = 18,
    #[doc = "19: `10011`"]
    Aport4ch24to31 = 19,
}
impl From<Input24to31sel> for u8 {
    #[inline(always)]
    fn from(variant: Input24to31sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input24to31sel {
    type Ux = u8;
}
impl crate::IsEnum for Input24to31sel {}
#[doc = "Field `INPUT24TO31SEL` reader - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
pub type Input24to31selR = crate::FieldReader<Input24to31sel>;
impl Input24to31selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input24to31sel> {
        match self.bits {
            0 => Some(Input24to31sel::Aport0ch0to7),
            1 => Some(Input24to31sel::Aport0ch8to15),
            4 => Some(Input24to31sel::Aport1ch0to7),
            5 => Some(Input24to31sel::Aport1ch8to15),
            6 => Some(Input24to31sel::Aport1ch16to23),
            7 => Some(Input24to31sel::Aport1ch24to31),
            8 => Some(Input24to31sel::Aport2ch0to7),
            9 => Some(Input24to31sel::Aport2ch8to15),
            10 => Some(Input24to31sel::Aport2ch16to23),
            11 => Some(Input24to31sel::Aport2ch24to31),
            12 => Some(Input24to31sel::Aport3ch0to7),
            13 => Some(Input24to31sel::Aport3ch8to15),
            14 => Some(Input24to31sel::Aport3ch16to23),
            15 => Some(Input24to31sel::Aport3ch24to31),
            16 => Some(Input24to31sel::Aport4ch0to7),
            17 => Some(Input24to31sel::Aport4ch8to15),
            18 => Some(Input24to31sel::Aport4ch16to23),
            19 => Some(Input24to31sel::Aport4ch24to31),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == Input24to31sel::Aport0ch0to7
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == Input24to31sel::Aport0ch8to15
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input24to31sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input24to31sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input24to31sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input24to31sel::Aport1ch24to31
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == Input24to31sel::Aport2ch0to7
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == Input24to31sel::Aport2ch8to15
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == Input24to31sel::Aport2ch16to23
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == Input24to31sel::Aport2ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input24to31sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input24to31sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input24to31sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input24to31sel::Aport3ch24to31
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == Input24to31sel::Aport4ch0to7
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == Input24to31sel::Aport4ch8to15
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == Input24to31sel::Aport4ch16to23
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == Input24to31sel::Aport4ch24to31
    }
}
#[doc = "Field `INPUT24TO31SEL` writer - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
pub type Input24to31selW<'a, REG> = crate::FieldWriter<'a, REG, 5, Input24to31sel>;
impl<'a, REG> Input24to31selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport0ch0to7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport0ch8to15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport1ch24to31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport2ch0to7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport2ch8to15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport2ch16to23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport2ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport3ch24to31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport4ch0to7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport4ch8to15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport4ch16to23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input24to31sel::Aport4ch24to31)
    }
}
impl R {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&self) -> Input0to7selR {
        Input0to7selR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&self) -> Input8to15selR {
        Input8to15selR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&self) -> Input16to23selR {
        Input16to23selR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&self) -> Input24to31selR {
        Input24to31selR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&mut self) -> Input0to7selW<'_, ScaninputselSpec> {
        Input0to7selW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&mut self) -> Input8to15selW<'_, ScaninputselSpec> {
        Input8to15selW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&mut self) -> Input16to23selW<'_, ScaninputselSpec> {
        Input16to23selW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&mut self) -> Input24to31selW<'_, ScaninputselSpec> {
        Input24to31selW::new(self, 24)
    }
}
#[doc = "Input Selection Register for Scan Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`scaninputsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScaninputselSpec;
impl crate::RegisterSpec for ScaninputselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scaninputsel::R`](R) reader structure"]
impl crate::Readable for ScaninputselSpec {}
#[doc = "`write(|w| ..)` method takes [`scaninputsel::W`](W) writer structure"]
impl crate::Writable for ScaninputselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANINPUTSEL to value 0"]
impl crate::Resettable for ScaninputselSpec {}
