#[doc = "Register `SCANNEGSEL` reader"]
pub type R = crate::R<ScannegselSpec>;
#[doc = "Register `SCANNEGSEL` writer"]
pub type W = crate::W<ScannegselSpec>;
#[doc = "Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input0negsel {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    Input1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    Input3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    Input5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    Input7 = 3,
}
impl From<Input0negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input0negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input0negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input0negsel {}
#[doc = "Field `INPUT0NEGSEL` reader - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
pub type Input0negselR = crate::FieldReader<Input0negsel>;
impl Input0negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input0negsel {
        match self.bits {
            0 => Input0negsel::Input1,
            1 => Input0negsel::Input3,
            2 => Input0negsel::Input5,
            3 => Input0negsel::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Input0negsel::Input1
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Input0negsel::Input3
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Input0negsel::Input5
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Input0negsel::Input7
    }
}
#[doc = "Field `INPUT0NEGSEL` writer - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
pub type Input0negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input0negsel, crate::Safe>;
impl<'a, REG> Input0negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Input0negsel::Input1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Input0negsel::Input3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Input0negsel::Input5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Input0negsel::Input7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input2negsel {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    Input1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    Input3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    Input5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    Input7 = 3,
}
impl From<Input2negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input2negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input2negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input2negsel {}
#[doc = "Field `INPUT2NEGSEL` reader - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
pub type Input2negselR = crate::FieldReader<Input2negsel>;
impl Input2negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input2negsel {
        match self.bits {
            0 => Input2negsel::Input1,
            1 => Input2negsel::Input3,
            2 => Input2negsel::Input5,
            3 => Input2negsel::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Input2negsel::Input1
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Input2negsel::Input3
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Input2negsel::Input5
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Input2negsel::Input7
    }
}
#[doc = "Field `INPUT2NEGSEL` writer - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
pub type Input2negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input2negsel, crate::Safe>;
impl<'a, REG> Input2negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Input2negsel::Input1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Input2negsel::Input3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Input2negsel::Input5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Input2negsel::Input7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input4negsel {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    Input1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    Input3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    Input5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    Input7 = 3,
}
impl From<Input4negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input4negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input4negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input4negsel {}
#[doc = "Field `INPUT4NEGSEL` reader - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
pub type Input4negselR = crate::FieldReader<Input4negsel>;
impl Input4negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input4negsel {
        match self.bits {
            0 => Input4negsel::Input1,
            1 => Input4negsel::Input3,
            2 => Input4negsel::Input5,
            3 => Input4negsel::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Input4negsel::Input1
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Input4negsel::Input3
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Input4negsel::Input5
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Input4negsel::Input7
    }
}
#[doc = "Field `INPUT4NEGSEL` writer - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
pub type Input4negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input4negsel, crate::Safe>;
impl<'a, REG> Input4negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Input4negsel::Input1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Input4negsel::Input3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Input4negsel::Input5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Input4negsel::Input7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input6negsel {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    Input1 = 0,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    Input3 = 1,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    Input5 = 2,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    Input7 = 3,
}
impl From<Input6negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input6negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input6negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input6negsel {}
#[doc = "Field `INPUT6NEGSEL` reader - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
pub type Input6negselR = crate::FieldReader<Input6negsel>;
impl Input6negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input6negsel {
        match self.bits {
            0 => Input6negsel::Input1,
            1 => Input6negsel::Input3,
            2 => Input6negsel::Input5,
            3 => Input6negsel::Input7,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == Input6negsel::Input1
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == Input6negsel::Input3
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == Input6negsel::Input5
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == Input6negsel::Input7
    }
}
#[doc = "Field `INPUT6NEGSEL` writer - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
pub type Input6negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input6negsel, crate::Safe>;
impl<'a, REG> Input6negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(Input6negsel::Input1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(Input6negsel::Input3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut crate::W<REG> {
        self.variant(Input6negsel::Input5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut crate::W<REG> {
        self.variant(Input6negsel::Input7)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input9negsel {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    Input8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    Input10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    Input12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    Input14 = 3,
}
impl From<Input9negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input9negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input9negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input9negsel {}
#[doc = "Field `INPUT9NEGSEL` reader - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
pub type Input9negselR = crate::FieldReader<Input9negsel>;
impl Input9negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input9negsel {
        match self.bits {
            0 => Input9negsel::Input8,
            1 => Input9negsel::Input10,
            2 => Input9negsel::Input12,
            3 => Input9negsel::Input14,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == Input9negsel::Input8
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == Input9negsel::Input10
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == Input9negsel::Input12
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == Input9negsel::Input14
    }
}
#[doc = "Field `INPUT9NEGSEL` writer - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
pub type Input9negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input9negsel, crate::Safe>;
impl<'a, REG> Input9negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(Input9negsel::Input8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(Input9negsel::Input10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(Input9negsel::Input12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(Input9negsel::Input14)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input11negsel {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    Input8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    Input10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    Input12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    Input14 = 3,
}
impl From<Input11negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input11negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input11negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input11negsel {}
#[doc = "Field `INPUT11NEGSEL` reader - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
pub type Input11negselR = crate::FieldReader<Input11negsel>;
impl Input11negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input11negsel {
        match self.bits {
            0 => Input11negsel::Input8,
            1 => Input11negsel::Input10,
            2 => Input11negsel::Input12,
            3 => Input11negsel::Input14,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == Input11negsel::Input8
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == Input11negsel::Input10
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == Input11negsel::Input12
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == Input11negsel::Input14
    }
}
#[doc = "Field `INPUT11NEGSEL` writer - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
pub type Input11negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input11negsel, crate::Safe>;
impl<'a, REG> Input11negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(Input11negsel::Input8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(Input11negsel::Input10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(Input11negsel::Input12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(Input11negsel::Input14)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input13negsel {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    Input8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    Input10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    Input12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    Input14 = 3,
}
impl From<Input13negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input13negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input13negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input13negsel {}
#[doc = "Field `INPUT13NEGSEL` reader - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
pub type Input13negselR = crate::FieldReader<Input13negsel>;
impl Input13negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input13negsel {
        match self.bits {
            0 => Input13negsel::Input8,
            1 => Input13negsel::Input10,
            2 => Input13negsel::Input12,
            3 => Input13negsel::Input14,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == Input13negsel::Input8
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == Input13negsel::Input10
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == Input13negsel::Input12
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == Input13negsel::Input14
    }
}
#[doc = "Field `INPUT13NEGSEL` writer - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
pub type Input13negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input13negsel, crate::Safe>;
impl<'a, REG> Input13negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(Input13negsel::Input8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(Input13negsel::Input10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(Input13negsel::Input12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(Input13negsel::Input14)
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Input15negsel {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    Input8 = 0,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    Input10 = 1,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    Input12 = 2,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    Input14 = 3,
}
impl From<Input15negsel> for u8 {
    #[inline(always)]
    fn from(variant: Input15negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Input15negsel {
    type Ux = u8;
}
impl crate::IsEnum for Input15negsel {}
#[doc = "Field `INPUT15NEGSEL` reader - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
pub type Input15negselR = crate::FieldReader<Input15negsel>;
impl Input15negselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Input15negsel {
        match self.bits {
            0 => Input15negsel::Input8,
            1 => Input15negsel::Input10,
            2 => Input15negsel::Input12,
            3 => Input15negsel::Input14,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == Input15negsel::Input8
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == Input15negsel::Input10
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == Input15negsel::Input12
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == Input15negsel::Input14
    }
}
#[doc = "Field `INPUT15NEGSEL` writer - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
pub type Input15negselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Input15negsel, crate::Safe>;
impl<'a, REG> Input15negselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut crate::W<REG> {
        self.variant(Input15negsel::Input8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut crate::W<REG> {
        self.variant(Input15negsel::Input10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut crate::W<REG> {
        self.variant(Input15negsel::Input12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut crate::W<REG> {
        self.variant(Input15negsel::Input14)
    }
}
impl R {
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input0negsel(&self) -> Input0negselR {
        Input0negselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input2negsel(&self) -> Input2negselR {
        Input2negselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input4negsel(&self) -> Input4negselR {
        Input4negselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input6negsel(&self) -> Input6negselR {
        Input6negselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input9negsel(&self) -> Input9negselR {
        Input9negselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input11negsel(&self) -> Input11negselR {
        Input11negselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input13negsel(&self) -> Input13negselR {
        Input13negselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input15negsel(&self) -> Input15negselR {
        Input15negselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input0negsel(&mut self) -> Input0negselW<'_, ScannegselSpec> {
        Input0negselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input2negsel(&mut self) -> Input2negselW<'_, ScannegselSpec> {
        Input2negselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input4negsel(&mut self) -> Input4negselW<'_, ScannegselSpec> {
        Input4negselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input6negsel(&mut self) -> Input6negselW<'_, ScannegselSpec> {
        Input6negselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input9negsel(&mut self) -> Input9negselW<'_, ScannegselSpec> {
        Input9negselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input11negsel(&mut self) -> Input11negselW<'_, ScannegselSpec> {
        Input11negselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input13negsel(&mut self) -> Input13negselW<'_, ScannegselSpec> {
        Input13negselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input15negsel(&mut self) -> Input15negselW<'_, ScannegselSpec> {
        Input15negselW::new(self, 14)
    }
}
#[doc = "Negative Input Select Register for Scan\n\nYou can [`read`](crate::Reg::read) this register and get [`scannegsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scannegsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScannegselSpec;
impl crate::RegisterSpec for ScannegselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scannegsel::R`](R) reader structure"]
impl crate::Readable for ScannegselSpec {}
#[doc = "`write(|w| ..)` method takes [`scannegsel::W`](W) writer structure"]
impl crate::Writable for ScannegselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANNEGSEL to value 0x39e4"]
impl crate::Resettable for ScannegselSpec {
    const RESET_VALUE: u32 = 0x39e4;
}
