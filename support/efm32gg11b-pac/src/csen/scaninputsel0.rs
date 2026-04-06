#[doc = "Register `SCANINPUTSEL0` reader"]
pub type R = crate::R<Scaninputsel0Spec>;
#[doc = "Register `SCANINPUTSEL0` writer"]
pub type W = crate::W<Scaninputsel0Spec>;
#[doc = "CSEN_INPUT0-7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input0to7sel {
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
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
#[doc = "Field `INPUT0TO7SEL` reader - CSEN_INPUT0-7 Select"]
pub type Input0to7selR = crate::FieldReader<Input0to7sel>;
impl Input0to7selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input0to7sel> {
        match self.bits {
            4 => Some(Input0to7sel::Aport1ch0to7),
            5 => Some(Input0to7sel::Aport1ch8to15),
            6 => Some(Input0to7sel::Aport1ch16to23),
            7 => Some(Input0to7sel::Aport1ch24to31),
            12 => Some(Input0to7sel::Aport3ch0to7),
            13 => Some(Input0to7sel::Aport3ch8to15),
            14 => Some(Input0to7sel::Aport3ch16to23),
            15 => Some(Input0to7sel::Aport3ch24to31),
            _ => None,
        }
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
}
#[doc = "Field `INPUT0TO7SEL` writer - CSEN_INPUT0-7 Select"]
pub type Input0to7selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input0to7sel>;
impl<'a, REG> Input0to7selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
}
#[doc = "CSEN_INPUT8-15 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input8to15sel {
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
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
#[doc = "Field `INPUT8TO15SEL` reader - CSEN_INPUT8-15 Select"]
pub type Input8to15selR = crate::FieldReader<Input8to15sel>;
impl Input8to15selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input8to15sel> {
        match self.bits {
            4 => Some(Input8to15sel::Aport1ch0to7),
            5 => Some(Input8to15sel::Aport1ch8to15),
            6 => Some(Input8to15sel::Aport1ch16to23),
            7 => Some(Input8to15sel::Aport1ch24to31),
            12 => Some(Input8to15sel::Aport3ch0to7),
            13 => Some(Input8to15sel::Aport3ch8to15),
            14 => Some(Input8to15sel::Aport3ch16to23),
            15 => Some(Input8to15sel::Aport3ch24to31),
            _ => None,
        }
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
}
#[doc = "Field `INPUT8TO15SEL` writer - CSEN_INPUT8-15 Select"]
pub type Input8to15selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input8to15sel>;
impl<'a, REG> Input8to15selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
}
#[doc = "CSEN_INPUT16-23 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input16to23sel {
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
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
#[doc = "Field `INPUT16TO23SEL` reader - CSEN_INPUT16-23 Select"]
pub type Input16to23selR = crate::FieldReader<Input16to23sel>;
impl Input16to23selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input16to23sel> {
        match self.bits {
            4 => Some(Input16to23sel::Aport1ch0to7),
            5 => Some(Input16to23sel::Aport1ch8to15),
            6 => Some(Input16to23sel::Aport1ch16to23),
            7 => Some(Input16to23sel::Aport1ch24to31),
            12 => Some(Input16to23sel::Aport3ch0to7),
            13 => Some(Input16to23sel::Aport3ch8to15),
            14 => Some(Input16to23sel::Aport3ch16to23),
            15 => Some(Input16to23sel::Aport3ch24to31),
            _ => None,
        }
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
}
#[doc = "Field `INPUT16TO23SEL` writer - CSEN_INPUT16-23 Select"]
pub type Input16to23selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input16to23sel>;
impl<'a, REG> Input16to23selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
}
#[doc = "CSEN_INPUT24-31 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input24to31sel {
    #[doc = "4: `100`"]
    Aport1ch0to7 = 4,
    #[doc = "5: `101`"]
    Aport1ch8to15 = 5,
    #[doc = "6: `110`"]
    Aport1ch16to23 = 6,
    #[doc = "7: `111`"]
    Aport1ch24to31 = 7,
    #[doc = "12: `1100`"]
    Aport3ch0to7 = 12,
    #[doc = "13: `1101`"]
    Aport3ch8to15 = 13,
    #[doc = "14: `1110`"]
    Aport3ch16to23 = 14,
    #[doc = "15: `1111`"]
    Aport3ch24to31 = 15,
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
#[doc = "Field `INPUT24TO31SEL` reader - CSEN_INPUT24-31 Select"]
pub type Input24to31selR = crate::FieldReader<Input24to31sel>;
impl Input24to31selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input24to31sel> {
        match self.bits {
            4 => Some(Input24to31sel::Aport1ch0to7),
            5 => Some(Input24to31sel::Aport1ch8to15),
            6 => Some(Input24to31sel::Aport1ch16to23),
            7 => Some(Input24to31sel::Aport1ch24to31),
            12 => Some(Input24to31sel::Aport3ch0to7),
            13 => Some(Input24to31sel::Aport3ch8to15),
            14 => Some(Input24to31sel::Aport3ch16to23),
            15 => Some(Input24to31sel::Aport3ch24to31),
            _ => None,
        }
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
}
#[doc = "Field `INPUT24TO31SEL` writer - CSEN_INPUT24-31 Select"]
pub type Input24to31selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input24to31sel>;
impl<'a, REG> Input24to31selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
}
impl R {
    #[doc = "Bits 0:3 - CSEN_INPUT0-7 Select"]
    #[inline(always)]
    pub fn input0to7sel(&self) -> Input0to7selR {
        Input0to7selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT8-15 Select"]
    #[inline(always)]
    pub fn input8to15sel(&self) -> Input8to15selR {
        Input8to15selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT16-23 Select"]
    #[inline(always)]
    pub fn input16to23sel(&self) -> Input16to23selR {
        Input16to23selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT24-31 Select"]
    #[inline(always)]
    pub fn input24to31sel(&self) -> Input24to31selR {
        Input24to31selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSEN_INPUT0-7 Select"]
    #[inline(always)]
    pub fn input0to7sel(&mut self) -> Input0to7selW<'_, Scaninputsel0Spec> {
        Input0to7selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT8-15 Select"]
    #[inline(always)]
    pub fn input8to15sel(&mut self) -> Input8to15selW<'_, Scaninputsel0Spec> {
        Input8to15selW::new(self, 8)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT16-23 Select"]
    #[inline(always)]
    pub fn input16to23sel(&mut self) -> Input16to23selW<'_, Scaninputsel0Spec> {
        Input16to23selW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT24-31 Select"]
    #[inline(always)]
    pub fn input24to31sel(&mut self) -> Input24to31selW<'_, Scaninputsel0Spec> {
        Input24to31selW::new(self, 24)
    }
}
#[doc = "Scan Input Selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scaninputsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scaninputsel0Spec;
impl crate::RegisterSpec for Scaninputsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scaninputsel0::R`](R) reader structure"]
impl crate::Readable for Scaninputsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`scaninputsel0::W`](W) writer structure"]
impl crate::Writable for Scaninputsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANINPUTSEL0 to value 0"]
impl crate::Resettable for Scaninputsel0Spec {}
