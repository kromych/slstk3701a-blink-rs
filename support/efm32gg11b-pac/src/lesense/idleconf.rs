#[doc = "Register `IDLECONF` reader"]
pub type R = crate::R<IdleconfSpec>;
#[doc = "Register `IDLECONF` writer"]
pub type W = crate::W<IdleconfSpec>;
#[doc = "Channel 0 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0 {
    #[doc = "0: CH0 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH0 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH0 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch0> for u8 {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0 {
    type Ux = u8;
}
impl crate::IsEnum for Ch0 {}
#[doc = "Field `CH0` reader - Channel 0 Idle Phase Configuration"]
pub type Ch0R = crate::FieldReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            0 => Ch0::Disable,
            1 => Ch0::High,
            2 => Ch0::Low,
            3 => Ch0::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch0::Disable
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch0::High
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch0::Low
    }
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch0::Dac
    }
}
#[doc = "Field `CH0` writer - Channel 0 Idle Phase Configuration"]
pub type Ch0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch0, crate::Safe>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Disable)
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::High)
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Low)
    }
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Dac)
    }
}
#[doc = "Channel 1 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1 {
    #[doc = "0: CH1 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH1 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH1 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch1> for u8 {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch1 {
    type Ux = u8;
}
impl crate::IsEnum for Ch1 {}
#[doc = "Field `CH1` reader - Channel 1 Idle Phase Configuration"]
pub type Ch1R = crate::FieldReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            0 => Ch1::Disable,
            1 => Ch1::High,
            2 => Ch1::Low,
            3 => Ch1::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch1::Disable
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch1::High
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch1::Low
    }
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch1::Dac
    }
}
#[doc = "Field `CH1` writer - Channel 1 Idle Phase Configuration"]
pub type Ch1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch1, crate::Safe>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Disable)
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::High)
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Low)
    }
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Dac)
    }
}
#[doc = "Channel 2 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch2 {
    #[doc = "0: CH2 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH2 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH2 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch2> for u8 {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch2 {
    type Ux = u8;
}
impl crate::IsEnum for Ch2 {}
#[doc = "Field `CH2` reader - Channel 2 Idle Phase Configuration"]
pub type Ch2R = crate::FieldReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            0 => Ch2::Disable,
            1 => Ch2::High,
            2 => Ch2::Low,
            3 => Ch2::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch2::Disable
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch2::High
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch2::Low
    }
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch2::Dac
    }
}
#[doc = "Field `CH2` writer - Channel 2 Idle Phase Configuration"]
pub type Ch2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch2, crate::Safe>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Disable)
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::High)
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Low)
    }
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Dac)
    }
}
#[doc = "Channel 3 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3 {
    #[doc = "0: CH3 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH3 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH3 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch3> for u8 {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3 {
    type Ux = u8;
}
impl crate::IsEnum for Ch3 {}
#[doc = "Field `CH3` reader - Channel 3 Idle Phase Configuration"]
pub type Ch3R = crate::FieldReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            0 => Ch3::Disable,
            1 => Ch3::High,
            2 => Ch3::Low,
            3 => Ch3::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch3::Disable
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch3::High
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch3::Low
    }
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch3::Dac
    }
}
#[doc = "Field `CH3` writer - Channel 3 Idle Phase Configuration"]
pub type Ch3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch3, crate::Safe>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Disable)
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::High)
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Low)
    }
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Dac)
    }
}
#[doc = "Channel 4 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch4 {
    #[doc = "0: CH4 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH4 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH4 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch4> for u8 {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch4 {
    type Ux = u8;
}
impl crate::IsEnum for Ch4 {}
#[doc = "Field `CH4` reader - Channel 4 Idle Phase Configuration"]
pub type Ch4R = crate::FieldReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            0 => Ch4::Disable,
            1 => Ch4::High,
            2 => Ch4::Low,
            3 => Ch4::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch4::Disable
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch4::High
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch4::Low
    }
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch4::Dac
    }
}
#[doc = "Field `CH4` writer - Channel 4 Idle Phase Configuration"]
pub type Ch4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch4, crate::Safe>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Disable)
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::High)
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Low)
    }
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Dac)
    }
}
#[doc = "Channel 5 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch5 {
    #[doc = "0: CH5 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH5 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH5 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch5> for u8 {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch5 {
    type Ux = u8;
}
impl crate::IsEnum for Ch5 {}
#[doc = "Field `CH5` reader - Channel 5 Idle Phase Configuration"]
pub type Ch5R = crate::FieldReader<Ch5>;
impl Ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5 {
        match self.bits {
            0 => Ch5::Disable,
            1 => Ch5::High,
            2 => Ch5::Low,
            3 => Ch5::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch5::Disable
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch5::High
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch5::Low
    }
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch5::Dac
    }
}
#[doc = "Field `CH5` writer - Channel 5 Idle Phase Configuration"]
pub type Ch5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch5, crate::Safe>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Disable)
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::High)
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Low)
    }
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Dac)
    }
}
#[doc = "Channel 6 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch6 {
    #[doc = "0: CH6 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH6 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH6 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch6> for u8 {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch6 {
    type Ux = u8;
}
impl crate::IsEnum for Ch6 {}
#[doc = "Field `CH6` reader - Channel 6 Idle Phase Configuration"]
pub type Ch6R = crate::FieldReader<Ch6>;
impl Ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6 {
        match self.bits {
            0 => Ch6::Disable,
            1 => Ch6::High,
            2 => Ch6::Low,
            3 => Ch6::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch6::Disable
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch6::High
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch6::Low
    }
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch6::Dac
    }
}
#[doc = "Field `CH6` writer - Channel 6 Idle Phase Configuration"]
pub type Ch6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch6, crate::Safe>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Disable)
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::High)
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Low)
    }
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Dac)
    }
}
#[doc = "Channel 7 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch7 {
    #[doc = "0: CH7 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH7 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH7 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch7> for u8 {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch7 {
    type Ux = u8;
}
impl crate::IsEnum for Ch7 {}
#[doc = "Field `CH7` reader - Channel 7 Idle Phase Configuration"]
pub type Ch7R = crate::FieldReader<Ch7>;
impl Ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7 {
        match self.bits {
            0 => Ch7::Disable,
            1 => Ch7::High,
            2 => Ch7::Low,
            3 => Ch7::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch7::Disable
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch7::High
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch7::Low
    }
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch7::Dac
    }
}
#[doc = "Field `CH7` writer - Channel 7 Idle Phase Configuration"]
pub type Ch7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch7, crate::Safe>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Disable)
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::High)
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Low)
    }
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Dac)
    }
}
#[doc = "Channel 8 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch8 {
    #[doc = "0: CH8 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH8 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH8 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch8> for u8 {
    #[inline(always)]
    fn from(variant: Ch8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch8 {
    type Ux = u8;
}
impl crate::IsEnum for Ch8 {}
#[doc = "Field `CH8` reader - Channel 8 Idle Phase Configuration"]
pub type Ch8R = crate::FieldReader<Ch8>;
impl Ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch8 {
        match self.bits {
            0 => Ch8::Disable,
            1 => Ch8::High,
            2 => Ch8::Low,
            3 => Ch8::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch8::Disable
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch8::High
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch8::Low
    }
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch8::Dac
    }
}
#[doc = "Field `CH8` writer - Channel 8 Idle Phase Configuration"]
pub type Ch8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch8, crate::Safe>;
impl<'a, REG> Ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Disable)
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::High)
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Low)
    }
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Dac)
    }
}
#[doc = "Channel 9 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch9 {
    #[doc = "0: CH9 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH9 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH9 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch9> for u8 {
    #[inline(always)]
    fn from(variant: Ch9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch9 {
    type Ux = u8;
}
impl crate::IsEnum for Ch9 {}
#[doc = "Field `CH9` reader - Channel 9 Idle Phase Configuration"]
pub type Ch9R = crate::FieldReader<Ch9>;
impl Ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch9 {
        match self.bits {
            0 => Ch9::Disable,
            1 => Ch9::High,
            2 => Ch9::Low,
            3 => Ch9::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch9::Disable
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch9::High
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch9::Low
    }
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch9::Dac
    }
}
#[doc = "Field `CH9` writer - Channel 9 Idle Phase Configuration"]
pub type Ch9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch9, crate::Safe>;
impl<'a, REG> Ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Disable)
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::High)
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Low)
    }
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Dac)
    }
}
#[doc = "Channel 10 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch10 {
    #[doc = "0: CH10 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH10 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH10 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch10> for u8 {
    #[inline(always)]
    fn from(variant: Ch10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch10 {
    type Ux = u8;
}
impl crate::IsEnum for Ch10 {}
#[doc = "Field `CH10` reader - Channel 10 Idle Phase Configuration"]
pub type Ch10R = crate::FieldReader<Ch10>;
impl Ch10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch10 {
        match self.bits {
            0 => Ch10::Disable,
            1 => Ch10::High,
            2 => Ch10::Low,
            3 => Ch10::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch10::Disable
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch10::High
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch10::Low
    }
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch10::Dac
    }
}
#[doc = "Field `CH10` writer - Channel 10 Idle Phase Configuration"]
pub type Ch10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch10, crate::Safe>;
impl<'a, REG> Ch10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Disable)
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::High)
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Low)
    }
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Dac)
    }
}
#[doc = "Channel 11 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch11 {
    #[doc = "0: CH11 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH11 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH11 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch11> for u8 {
    #[inline(always)]
    fn from(variant: Ch11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch11 {
    type Ux = u8;
}
impl crate::IsEnum for Ch11 {}
#[doc = "Field `CH11` reader - Channel 11 Idle Phase Configuration"]
pub type Ch11R = crate::FieldReader<Ch11>;
impl Ch11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch11 {
        match self.bits {
            0 => Ch11::Disable,
            1 => Ch11::High,
            2 => Ch11::Low,
            3 => Ch11::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch11::Disable
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch11::High
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch11::Low
    }
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch11::Dac
    }
}
#[doc = "Field `CH11` writer - Channel 11 Idle Phase Configuration"]
pub type Ch11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch11, crate::Safe>;
impl<'a, REG> Ch11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Disable)
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::High)
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Low)
    }
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Dac)
    }
}
#[doc = "Channel 12 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch12 {
    #[doc = "0: CH12 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH12 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH12 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch12> for u8 {
    #[inline(always)]
    fn from(variant: Ch12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch12 {
    type Ux = u8;
}
impl crate::IsEnum for Ch12 {}
#[doc = "Field `CH12` reader - Channel 12 Idle Phase Configuration"]
pub type Ch12R = crate::FieldReader<Ch12>;
impl Ch12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch12 {
        match self.bits {
            0 => Ch12::Disable,
            1 => Ch12::High,
            2 => Ch12::Low,
            3 => Ch12::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch12::Disable
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch12::High
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch12::Low
    }
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch12::Dac
    }
}
#[doc = "Field `CH12` writer - Channel 12 Idle Phase Configuration"]
pub type Ch12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch12, crate::Safe>;
impl<'a, REG> Ch12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Disable)
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::High)
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Low)
    }
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Dac)
    }
}
#[doc = "Channel 13 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch13 {
    #[doc = "0: CH13 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH13 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH13 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch13> for u8 {
    #[inline(always)]
    fn from(variant: Ch13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch13 {
    type Ux = u8;
}
impl crate::IsEnum for Ch13 {}
#[doc = "Field `CH13` reader - Channel 13 Idle Phase Configuration"]
pub type Ch13R = crate::FieldReader<Ch13>;
impl Ch13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch13 {
        match self.bits {
            0 => Ch13::Disable,
            1 => Ch13::High,
            2 => Ch13::Low,
            3 => Ch13::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch13::Disable
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch13::High
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch13::Low
    }
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch13::Dac
    }
}
#[doc = "Field `CH13` writer - Channel 13 Idle Phase Configuration"]
pub type Ch13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch13, crate::Safe>;
impl<'a, REG> Ch13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Disable)
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::High)
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Low)
    }
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Dac)
    }
}
#[doc = "Channel 14 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch14 {
    #[doc = "0: CH14 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH14 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH14 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch14> for u8 {
    #[inline(always)]
    fn from(variant: Ch14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch14 {
    type Ux = u8;
}
impl crate::IsEnum for Ch14 {}
#[doc = "Field `CH14` reader - Channel 14 Idle Phase Configuration"]
pub type Ch14R = crate::FieldReader<Ch14>;
impl Ch14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch14 {
        match self.bits {
            0 => Ch14::Disable,
            1 => Ch14::High,
            2 => Ch14::Low,
            3 => Ch14::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch14::Disable
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch14::High
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch14::Low
    }
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch14::Dac
    }
}
#[doc = "Field `CH14` writer - Channel 14 Idle Phase Configuration"]
pub type Ch14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch14, crate::Safe>;
impl<'a, REG> Ch14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Disable)
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::High)
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Low)
    }
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Dac)
    }
}
#[doc = "Channel 15 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch15 {
    #[doc = "0: CH15 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: CH15 output is high in idle phase"]
    High = 1,
    #[doc = "2: CH15 output is low in idle phase"]
    Low = 2,
    #[doc = "3: CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    Dac = 3,
}
impl From<Ch15> for u8 {
    #[inline(always)]
    fn from(variant: Ch15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch15 {
    type Ux = u8;
}
impl crate::IsEnum for Ch15 {}
#[doc = "Field `CH15` reader - Channel 15 Idle Phase Configuration"]
pub type Ch15R = crate::FieldReader<Ch15>;
impl Ch15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch15 {
        match self.bits {
            0 => Ch15::Disable,
            1 => Ch15::High,
            2 => Ch15::Low,
            3 => Ch15::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ch15::Disable
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch15::High
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch15::Low
    }
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Ch15::Dac
    }
}
#[doc = "Field `CH15` writer - Channel 15 Idle Phase Configuration"]
pub type Ch15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch15, crate::Safe>;
impl<'a, REG> Ch15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Disable)
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::High)
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Low)
    }
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Dac)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, IdleconfSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, IdleconfSpec> {
        Ch1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, IdleconfSpec> {
        Ch2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, IdleconfSpec> {
        Ch3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, IdleconfSpec> {
        Ch4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, IdleconfSpec> {
        Ch5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, IdleconfSpec> {
        Ch6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, IdleconfSpec> {
        Ch7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, IdleconfSpec> {
        Ch8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, IdleconfSpec> {
        Ch9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, IdleconfSpec> {
        Ch10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, IdleconfSpec> {
        Ch11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, IdleconfSpec> {
        Ch12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, IdleconfSpec> {
        Ch13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, IdleconfSpec> {
        Ch14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, IdleconfSpec> {
        Ch15W::new(self, 30)
    }
}
#[doc = "GPIO Idle Phase Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idleconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idleconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleconfSpec;
impl crate::RegisterSpec for IdleconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idleconf::R`](R) reader structure"]
impl crate::Readable for IdleconfSpec {}
#[doc = "`write(|w| ..)` method takes [`idleconf::W`](W) writer structure"]
impl crate::Writable for IdleconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDLECONF to value 0"]
impl crate::Resettable for IdleconfSpec {}
