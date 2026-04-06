#[doc = "Register `SCANINPUTSEL1` reader"]
pub type R = crate::R<Scaninputsel1Spec>;
#[doc = "Register `SCANINPUTSEL1` writer"]
pub type W = crate::W<Scaninputsel1Spec>;
#[doc = "CSEN_INPUT32-39 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input32to39sel {
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
impl From<Input32to39sel> for u8 {
    #[inline(always)]
    fn from(variant: Input32to39sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input32to39sel {
    type Ux = u8;
}
impl crate::IsEnum for Input32to39sel {}
#[doc = "Field `INPUT32TO39SEL` reader - CSEN_INPUT32-39 Select"]
pub type Input32to39selR = crate::FieldReader<Input32to39sel>;
impl Input32to39selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input32to39sel> {
        match self.bits {
            4 => Some(Input32to39sel::Aport1ch0to7),
            5 => Some(Input32to39sel::Aport1ch8to15),
            6 => Some(Input32to39sel::Aport1ch16to23),
            7 => Some(Input32to39sel::Aport1ch24to31),
            12 => Some(Input32to39sel::Aport3ch0to7),
            13 => Some(Input32to39sel::Aport3ch8to15),
            14 => Some(Input32to39sel::Aport3ch16to23),
            15 => Some(Input32to39sel::Aport3ch24to31),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input32to39sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input32to39sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input32to39sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input32to39sel::Aport1ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input32to39sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input32to39sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input32to39sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input32to39sel::Aport3ch24to31
    }
}
#[doc = "Field `INPUT32TO39SEL` writer - CSEN_INPUT32-39 Select"]
pub type Input32to39selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input32to39sel>;
impl<'a, REG> Input32to39selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport1ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input32to39sel::Aport3ch24to31)
    }
}
#[doc = "CSEN_INPUT40-47 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input40to47sel {
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
impl From<Input40to47sel> for u8 {
    #[inline(always)]
    fn from(variant: Input40to47sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input40to47sel {
    type Ux = u8;
}
impl crate::IsEnum for Input40to47sel {}
#[doc = "Field `INPUT40TO47SEL` reader - CSEN_INPUT40-47 Select"]
pub type Input40to47selR = crate::FieldReader<Input40to47sel>;
impl Input40to47selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input40to47sel> {
        match self.bits {
            4 => Some(Input40to47sel::Aport1ch0to7),
            5 => Some(Input40to47sel::Aport1ch8to15),
            6 => Some(Input40to47sel::Aport1ch16to23),
            7 => Some(Input40to47sel::Aport1ch24to31),
            12 => Some(Input40to47sel::Aport3ch0to7),
            13 => Some(Input40to47sel::Aport3ch8to15),
            14 => Some(Input40to47sel::Aport3ch16to23),
            15 => Some(Input40to47sel::Aport3ch24to31),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input40to47sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input40to47sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input40to47sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input40to47sel::Aport1ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input40to47sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input40to47sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input40to47sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input40to47sel::Aport3ch24to31
    }
}
#[doc = "Field `INPUT40TO47SEL` writer - CSEN_INPUT40-47 Select"]
pub type Input40to47selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input40to47sel>;
impl<'a, REG> Input40to47selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport1ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input40to47sel::Aport3ch24to31)
    }
}
#[doc = "CSEN_INPUT48-55 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input48to55sel {
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
impl From<Input48to55sel> for u8 {
    #[inline(always)]
    fn from(variant: Input48to55sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input48to55sel {
    type Ux = u8;
}
impl crate::IsEnum for Input48to55sel {}
#[doc = "Field `INPUT48TO55SEL` reader - CSEN_INPUT48-55 Select"]
pub type Input48to55selR = crate::FieldReader<Input48to55sel>;
impl Input48to55selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input48to55sel> {
        match self.bits {
            4 => Some(Input48to55sel::Aport1ch0to7),
            5 => Some(Input48to55sel::Aport1ch8to15),
            6 => Some(Input48to55sel::Aport1ch16to23),
            7 => Some(Input48to55sel::Aport1ch24to31),
            12 => Some(Input48to55sel::Aport3ch0to7),
            13 => Some(Input48to55sel::Aport3ch8to15),
            14 => Some(Input48to55sel::Aport3ch16to23),
            15 => Some(Input48to55sel::Aport3ch24to31),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input48to55sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input48to55sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input48to55sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input48to55sel::Aport1ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input48to55sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input48to55sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input48to55sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input48to55sel::Aport3ch24to31
    }
}
#[doc = "Field `INPUT48TO55SEL` writer - CSEN_INPUT48-55 Select"]
pub type Input48to55selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input48to55sel>;
impl<'a, REG> Input48to55selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport1ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input48to55sel::Aport3ch24to31)
    }
}
#[doc = "CSEN_INPUT56-63 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input56to63sel {
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
impl From<Input56to63sel> for u8 {
    #[inline(always)]
    fn from(variant: Input56to63sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input56to63sel {
    type Ux = u8;
}
impl crate::IsEnum for Input56to63sel {}
#[doc = "Field `INPUT56TO63SEL` reader - CSEN_INPUT56-63 Select"]
pub type Input56to63selR = crate::FieldReader<Input56to63sel>;
impl Input56to63selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Input56to63sel> {
        match self.bits {
            4 => Some(Input56to63sel::Aport1ch0to7),
            5 => Some(Input56to63sel::Aport1ch8to15),
            6 => Some(Input56to63sel::Aport1ch16to23),
            7 => Some(Input56to63sel::Aport1ch24to31),
            12 => Some(Input56to63sel::Aport3ch0to7),
            13 => Some(Input56to63sel::Aport3ch8to15),
            14 => Some(Input56to63sel::Aport3ch16to23),
            15 => Some(Input56to63sel::Aport3ch24to31),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == Input56to63sel::Aport1ch0to7
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == Input56to63sel::Aport1ch8to15
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == Input56to63sel::Aport1ch16to23
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == Input56to63sel::Aport1ch24to31
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == Input56to63sel::Aport3ch0to7
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == Input56to63sel::Aport3ch8to15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == Input56to63sel::Aport3ch16to23
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == Input56to63sel::Aport3ch24to31
    }
}
#[doc = "Field `INPUT56TO63SEL` writer - CSEN_INPUT56-63 Select"]
pub type Input56to63selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Input56to63sel>;
impl<'a, REG> Input56to63selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport1ch0to7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport1ch8to15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport1ch16to23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport1ch24to31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport3ch0to7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport3ch8to15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport3ch16to23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut crate::W<REG> {
        self.variant(Input56to63sel::Aport3ch24to31)
    }
}
impl R {
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline(always)]
    pub fn input32to39sel(&self) -> Input32to39selR {
        Input32to39selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline(always)]
    pub fn input40to47sel(&self) -> Input40to47selR {
        Input40to47selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline(always)]
    pub fn input48to55sel(&self) -> Input48to55selR {
        Input48to55selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline(always)]
    pub fn input56to63sel(&self) -> Input56to63selR {
        Input56to63selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline(always)]
    pub fn input32to39sel(&mut self) -> Input32to39selW<'_, Scaninputsel1Spec> {
        Input32to39selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline(always)]
    pub fn input40to47sel(&mut self) -> Input40to47selW<'_, Scaninputsel1Spec> {
        Input40to47selW::new(self, 8)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline(always)]
    pub fn input48to55sel(&mut self) -> Input48to55selW<'_, Scaninputsel1Spec> {
        Input48to55selW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline(always)]
    pub fn input56to63sel(&mut self) -> Input56to63selW<'_, Scaninputsel1Spec> {
        Input56to63selW::new(self, 24)
    }
}
#[doc = "Scan Input Selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scaninputsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaninputsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scaninputsel1Spec;
impl crate::RegisterSpec for Scaninputsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scaninputsel1::R`](R) reader structure"]
impl crate::Readable for Scaninputsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`scaninputsel1::W`](W) writer structure"]
impl crate::Writable for Scaninputsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANINPUTSEL1 to value 0"]
impl crate::Resettable for Scaninputsel1Spec {}
