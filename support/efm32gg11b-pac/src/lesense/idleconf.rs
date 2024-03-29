#[doc = "Register `IDLECONF` reader"]
pub type R = crate::R<IDLECONF_SPEC>;
#[doc = "Register `IDLECONF` writer"]
pub type W = crate::W<IDLECONF_SPEC>;
#[doc = "Field `CH0` reader - Channel 0 Idle Phase Configuration"]
pub type CH0_R = crate::FieldReader<CH0_A>;
#[doc = "Channel 0 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH0_A {
    #[doc = "0: CH0 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH0 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH0 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH0_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH0_A {
    type Ux = u8;
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH0_A {
        match self.bits {
            0 => CH0_A::DISABLE,
            1 => CH0_A::HIGH,
            2 => CH0_A::LOW,
            3 => CH0_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH0_A::DISABLE
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH0_A::HIGH
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH0_A::LOW
    }
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH0_A::DAC
    }
}
#[doc = "Field `CH0` writer - Channel 0 Idle Phase Configuration"]
pub type CH0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH0_A>;
impl<'a, REG> CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::DISABLE)
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::HIGH)
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::LOW)
    }
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::DAC)
    }
}
#[doc = "Field `CH1` reader - Channel 1 Idle Phase Configuration"]
pub type CH1_R = crate::FieldReader<CH1_A>;
#[doc = "Channel 1 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH1_A {
    #[doc = "0: CH1 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH1 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH1 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH1_A> for u8 {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH1_A {
    type Ux = u8;
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1_A {
        match self.bits {
            0 => CH1_A::DISABLE,
            1 => CH1_A::HIGH,
            2 => CH1_A::LOW,
            3 => CH1_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH1_A::DISABLE
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH1_A::HIGH
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH1_A::LOW
    }
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH1_A::DAC
    }
}
#[doc = "Field `CH1` writer - Channel 1 Idle Phase Configuration"]
pub type CH1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH1_A>;
impl<'a, REG> CH1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::DISABLE)
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::HIGH)
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::LOW)
    }
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::DAC)
    }
}
#[doc = "Field `CH2` reader - Channel 2 Idle Phase Configuration"]
pub type CH2_R = crate::FieldReader<CH2_A>;
#[doc = "Channel 2 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH2_A {
    #[doc = "0: CH2 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH2 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH2 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH2_A> for u8 {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH2_A {
    type Ux = u8;
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2_A {
        match self.bits {
            0 => CH2_A::DISABLE,
            1 => CH2_A::HIGH,
            2 => CH2_A::LOW,
            3 => CH2_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH2_A::DISABLE
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH2_A::HIGH
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH2_A::LOW
    }
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH2_A::DAC
    }
}
#[doc = "Field `CH2` writer - Channel 2 Idle Phase Configuration"]
pub type CH2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH2_A>;
impl<'a, REG> CH2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::DISABLE)
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::HIGH)
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::LOW)
    }
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::DAC)
    }
}
#[doc = "Field `CH3` reader - Channel 3 Idle Phase Configuration"]
pub type CH3_R = crate::FieldReader<CH3_A>;
#[doc = "Channel 3 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH3_A {
    #[doc = "0: CH3 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH3 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH3 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH3_A> for u8 {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH3_A {
    type Ux = u8;
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH3_A {
        match self.bits {
            0 => CH3_A::DISABLE,
            1 => CH3_A::HIGH,
            2 => CH3_A::LOW,
            3 => CH3_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH3_A::DISABLE
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH3_A::HIGH
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH3_A::LOW
    }
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH3_A::DAC
    }
}
#[doc = "Field `CH3` writer - Channel 3 Idle Phase Configuration"]
pub type CH3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH3_A>;
impl<'a, REG> CH3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::DISABLE)
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::HIGH)
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::LOW)
    }
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::DAC)
    }
}
#[doc = "Field `CH4` reader - Channel 4 Idle Phase Configuration"]
pub type CH4_R = crate::FieldReader<CH4_A>;
#[doc = "Channel 4 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH4_A {
    #[doc = "0: CH4 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH4 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH4 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH4_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH4_A {
    type Ux = u8;
}
impl CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH4_A {
        match self.bits {
            0 => CH4_A::DISABLE,
            1 => CH4_A::HIGH,
            2 => CH4_A::LOW,
            3 => CH4_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH4_A::DISABLE
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH4_A::HIGH
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH4_A::LOW
    }
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH4_A::DAC
    }
}
#[doc = "Field `CH4` writer - Channel 4 Idle Phase Configuration"]
pub type CH4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH4_A>;
impl<'a, REG> CH4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH4_A::DISABLE)
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH4_A::HIGH)
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH4_A::LOW)
    }
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH4_A::DAC)
    }
}
#[doc = "Field `CH5` reader - Channel 5 Idle Phase Configuration"]
pub type CH5_R = crate::FieldReader<CH5_A>;
#[doc = "Channel 5 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH5_A {
    #[doc = "0: CH5 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH5 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH5 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH5_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH5_A {
    type Ux = u8;
}
impl CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH5_A {
        match self.bits {
            0 => CH5_A::DISABLE,
            1 => CH5_A::HIGH,
            2 => CH5_A::LOW,
            3 => CH5_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH5_A::DISABLE
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH5_A::HIGH
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH5_A::LOW
    }
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH5_A::DAC
    }
}
#[doc = "Field `CH5` writer - Channel 5 Idle Phase Configuration"]
pub type CH5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH5_A>;
impl<'a, REG> CH5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH5_A::DISABLE)
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH5_A::HIGH)
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH5_A::LOW)
    }
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH5_A::DAC)
    }
}
#[doc = "Field `CH6` reader - Channel 6 Idle Phase Configuration"]
pub type CH6_R = crate::FieldReader<CH6_A>;
#[doc = "Channel 6 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH6_A {
    #[doc = "0: CH6 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH6 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH6 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH6_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH6_A {
    type Ux = u8;
}
impl CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH6_A {
        match self.bits {
            0 => CH6_A::DISABLE,
            1 => CH6_A::HIGH,
            2 => CH6_A::LOW,
            3 => CH6_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH6_A::DISABLE
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH6_A::HIGH
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH6_A::LOW
    }
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH6_A::DAC
    }
}
#[doc = "Field `CH6` writer - Channel 6 Idle Phase Configuration"]
pub type CH6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH6_A>;
impl<'a, REG> CH6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH6_A::DISABLE)
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH6_A::HIGH)
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH6_A::LOW)
    }
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH6_A::DAC)
    }
}
#[doc = "Field `CH7` reader - Channel 7 Idle Phase Configuration"]
pub type CH7_R = crate::FieldReader<CH7_A>;
#[doc = "Channel 7 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH7_A {
    #[doc = "0: CH7 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH7 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH7 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH7_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH7_A {
    type Ux = u8;
}
impl CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH7_A {
        match self.bits {
            0 => CH7_A::DISABLE,
            1 => CH7_A::HIGH,
            2 => CH7_A::LOW,
            3 => CH7_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH7_A::DISABLE
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH7_A::HIGH
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH7_A::LOW
    }
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH7_A::DAC
    }
}
#[doc = "Field `CH7` writer - Channel 7 Idle Phase Configuration"]
pub type CH7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH7_A>;
impl<'a, REG> CH7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH7_A::DISABLE)
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH7_A::HIGH)
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH7_A::LOW)
    }
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH7_A::DAC)
    }
}
#[doc = "Field `CH8` reader - Channel 8 Idle Phase Configuration"]
pub type CH8_R = crate::FieldReader<CH8_A>;
#[doc = "Channel 8 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH8_A {
    #[doc = "0: CH8 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH8 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH8 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH8_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH8_A {
    type Ux = u8;
}
impl CH8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH8_A {
        match self.bits {
            0 => CH8_A::DISABLE,
            1 => CH8_A::HIGH,
            2 => CH8_A::LOW,
            3 => CH8_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH8_A::DISABLE
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH8_A::HIGH
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH8_A::LOW
    }
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH8_A::DAC
    }
}
#[doc = "Field `CH8` writer - Channel 8 Idle Phase Configuration"]
pub type CH8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH8_A>;
impl<'a, REG> CH8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH8_A::DISABLE)
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH8_A::HIGH)
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH8_A::LOW)
    }
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH8_A::DAC)
    }
}
#[doc = "Field `CH9` reader - Channel 9 Idle Phase Configuration"]
pub type CH9_R = crate::FieldReader<CH9_A>;
#[doc = "Channel 9 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH9_A {
    #[doc = "0: CH9 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH9 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH9 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH9_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH9_A {
    type Ux = u8;
}
impl CH9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH9_A {
        match self.bits {
            0 => CH9_A::DISABLE,
            1 => CH9_A::HIGH,
            2 => CH9_A::LOW,
            3 => CH9_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH9_A::DISABLE
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH9_A::HIGH
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH9_A::LOW
    }
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH9_A::DAC
    }
}
#[doc = "Field `CH9` writer - Channel 9 Idle Phase Configuration"]
pub type CH9_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH9_A>;
impl<'a, REG> CH9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH9_A::DISABLE)
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH9_A::HIGH)
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH9_A::LOW)
    }
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH9_A::DAC)
    }
}
#[doc = "Field `CH10` reader - Channel 10 Idle Phase Configuration"]
pub type CH10_R = crate::FieldReader<CH10_A>;
#[doc = "Channel 10 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH10_A {
    #[doc = "0: CH10 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH10 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH10 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH10_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH10_A {
    type Ux = u8;
}
impl CH10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH10_A {
        match self.bits {
            0 => CH10_A::DISABLE,
            1 => CH10_A::HIGH,
            2 => CH10_A::LOW,
            3 => CH10_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH10_A::DISABLE
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH10_A::HIGH
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH10_A::LOW
    }
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH10_A::DAC
    }
}
#[doc = "Field `CH10` writer - Channel 10 Idle Phase Configuration"]
pub type CH10_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH10_A>;
impl<'a, REG> CH10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH10_A::DISABLE)
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH10_A::HIGH)
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH10_A::LOW)
    }
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH10_A::DAC)
    }
}
#[doc = "Field `CH11` reader - Channel 11 Idle Phase Configuration"]
pub type CH11_R = crate::FieldReader<CH11_A>;
#[doc = "Channel 11 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH11_A {
    #[doc = "0: CH11 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH11 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH11 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH11_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH11_A {
    type Ux = u8;
}
impl CH11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH11_A {
        match self.bits {
            0 => CH11_A::DISABLE,
            1 => CH11_A::HIGH,
            2 => CH11_A::LOW,
            3 => CH11_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH11_A::DISABLE
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH11_A::HIGH
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH11_A::LOW
    }
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH11_A::DAC
    }
}
#[doc = "Field `CH11` writer - Channel 11 Idle Phase Configuration"]
pub type CH11_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH11_A>;
impl<'a, REG> CH11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH11_A::DISABLE)
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH11_A::HIGH)
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH11_A::LOW)
    }
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH11_A::DAC)
    }
}
#[doc = "Field `CH12` reader - Channel 12 Idle Phase Configuration"]
pub type CH12_R = crate::FieldReader<CH12_A>;
#[doc = "Channel 12 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH12_A {
    #[doc = "0: CH12 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH12 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH12 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH12_A> for u8 {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH12_A {
    type Ux = u8;
}
impl CH12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH12_A {
        match self.bits {
            0 => CH12_A::DISABLE,
            1 => CH12_A::HIGH,
            2 => CH12_A::LOW,
            3 => CH12_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH12_A::DISABLE
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH12_A::HIGH
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH12_A::LOW
    }
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH12_A::DAC
    }
}
#[doc = "Field `CH12` writer - Channel 12 Idle Phase Configuration"]
pub type CH12_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH12_A>;
impl<'a, REG> CH12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH12_A::DISABLE)
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH12_A::HIGH)
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH12_A::LOW)
    }
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH12_A::DAC)
    }
}
#[doc = "Field `CH13` reader - Channel 13 Idle Phase Configuration"]
pub type CH13_R = crate::FieldReader<CH13_A>;
#[doc = "Channel 13 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH13_A {
    #[doc = "0: CH13 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH13 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH13 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH13_A> for u8 {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH13_A {
    type Ux = u8;
}
impl CH13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH13_A {
        match self.bits {
            0 => CH13_A::DISABLE,
            1 => CH13_A::HIGH,
            2 => CH13_A::LOW,
            3 => CH13_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH13_A::DISABLE
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH13_A::HIGH
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH13_A::LOW
    }
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH13_A::DAC
    }
}
#[doc = "Field `CH13` writer - Channel 13 Idle Phase Configuration"]
pub type CH13_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH13_A>;
impl<'a, REG> CH13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH13_A::DISABLE)
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH13_A::HIGH)
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH13_A::LOW)
    }
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH13_A::DAC)
    }
}
#[doc = "Field `CH14` reader - Channel 14 Idle Phase Configuration"]
pub type CH14_R = crate::FieldReader<CH14_A>;
#[doc = "Channel 14 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH14_A {
    #[doc = "0: CH14 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH14 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH14 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH14_A> for u8 {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH14_A {
    type Ux = u8;
}
impl CH14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH14_A {
        match self.bits {
            0 => CH14_A::DISABLE,
            1 => CH14_A::HIGH,
            2 => CH14_A::LOW,
            3 => CH14_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH14_A::DISABLE
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH14_A::HIGH
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH14_A::LOW
    }
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH14_A::DAC
    }
}
#[doc = "Field `CH14` writer - Channel 14 Idle Phase Configuration"]
pub type CH14_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH14_A>;
impl<'a, REG> CH14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH14_A::DISABLE)
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH14_A::HIGH)
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH14_A::LOW)
    }
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH14_A::DAC)
    }
}
#[doc = "Field `CH15` reader - Channel 15 Idle Phase Configuration"]
pub type CH15_R = crate::FieldReader<CH15_A>;
#[doc = "Channel 15 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH15_A {
    #[doc = "0: CH15 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: CH15 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: CH15 output is low in idle phase"]
    LOW = 2,
    #[doc = "3: CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC = 3,
}
impl From<CH15_A> for u8 {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH15_A {
    type Ux = u8;
}
impl CH15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH15_A {
        match self.bits {
            0 => CH15_A::DISABLE,
            1 => CH15_A::HIGH,
            2 => CH15_A::LOW,
            3 => CH15_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH15_A::DISABLE
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH15_A::HIGH
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH15_A::LOW
    }
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH15_A::DAC
    }
}
#[doc = "Field `CH15` writer - Channel 15 Idle Phase Configuration"]
pub type CH15_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CH15_A>;
impl<'a, REG> CH15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH15_A::DISABLE)
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CH15_A::HIGH)
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CH15_A::LOW)
    }
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(CH15_A::DAC)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<IDLECONF_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<IDLECONF_SPEC> {
        CH1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<IDLECONF_SPEC> {
        CH2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<IDLECONF_SPEC> {
        CH3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<IDLECONF_SPEC> {
        CH4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<IDLECONF_SPEC> {
        CH5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<IDLECONF_SPEC> {
        CH6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<IDLECONF_SPEC> {
        CH7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<IDLECONF_SPEC> {
        CH8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<IDLECONF_SPEC> {
        CH9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<IDLECONF_SPEC> {
        CH10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<IDLECONF_SPEC> {
        CH11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<IDLECONF_SPEC> {
        CH12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<IDLECONF_SPEC> {
        CH13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<IDLECONF_SPEC> {
        CH14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<IDLECONF_SPEC> {
        CH15_W::new(self, 30)
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
#[doc = "GPIO Idle Phase Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idleconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idleconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLECONF_SPEC;
impl crate::RegisterSpec for IDLECONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idleconf::R`](R) reader structure"]
impl crate::Readable for IDLECONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idleconf::W`](W) writer structure"]
impl crate::Writable for IDLECONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLECONF to value 0"]
impl crate::Resettable for IDLECONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
