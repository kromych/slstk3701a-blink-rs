#[doc = "Register `HFXOTIMEOUTCTRL` reader"]
pub type R = crate::R<HfxotimeoutctrlSpec>;
#[doc = "Register `HFXOTIMEOUTCTRL` writer"]
pub type W = crate::W<HfxotimeoutctrlSpec>;
#[doc = "Wait Duration in HFXO Startup Enable Wait State\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startuptimeout {
    #[doc = "0: Timeout period of 2 cycles"]
    _2cycles = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4cycles = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16cycles = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32cycles = 3,
    #[doc = "4: Timeout period of 64 cycles"]
    _64cycles = 4,
    #[doc = "5: Timeout period of 128 cycles"]
    _128cycles = 5,
    #[doc = "6: Timeout period of 256 cycles"]
    _256cycles = 6,
    #[doc = "7: Timeout period of 1024 cycles"]
    _1kcycles = 7,
    #[doc = "8: Timeout period of 2048 cycles"]
    _2kcycles = 8,
    #[doc = "9: Timeout period of 4096 cycles"]
    _4kcycles = 9,
    #[doc = "10: Timeout period of 8192 cycles"]
    _8kcycles = 10,
    #[doc = "11: Timeout period of 16384 cycles"]
    _16kcycles = 11,
    #[doc = "12: Timeout period of 32768 cycles"]
    _32kcycles = 12,
    #[doc = "13: Timeout period of 65536 cycles"]
    _64kcycles = 13,
    #[doc = "14: Timeout period of 131072 cycles"]
    _128kcycles = 14,
}
impl From<Startuptimeout> for u8 {
    #[inline(always)]
    fn from(variant: Startuptimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startuptimeout {
    type Ux = u8;
}
impl crate::IsEnum for Startuptimeout {}
#[doc = "Field `STARTUPTIMEOUT` reader - Wait Duration in HFXO Startup Enable Wait State"]
pub type StartuptimeoutR = crate::FieldReader<Startuptimeout>;
impl StartuptimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startuptimeout> {
        match self.bits {
            0 => Some(Startuptimeout::_2cycles),
            1 => Some(Startuptimeout::_4cycles),
            2 => Some(Startuptimeout::_16cycles),
            3 => Some(Startuptimeout::_32cycles),
            4 => Some(Startuptimeout::_64cycles),
            5 => Some(Startuptimeout::_128cycles),
            6 => Some(Startuptimeout::_256cycles),
            7 => Some(Startuptimeout::_1kcycles),
            8 => Some(Startuptimeout::_2kcycles),
            9 => Some(Startuptimeout::_4kcycles),
            10 => Some(Startuptimeout::_8kcycles),
            11 => Some(Startuptimeout::_16kcycles),
            12 => Some(Startuptimeout::_32kcycles),
            13 => Some(Startuptimeout::_64kcycles),
            14 => Some(Startuptimeout::_128kcycles),
            _ => None,
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Startuptimeout::_2cycles
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Startuptimeout::_4cycles
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Startuptimeout::_16cycles
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Startuptimeout::_32cycles
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Startuptimeout::_64cycles
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Startuptimeout::_128cycles
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Startuptimeout::_256cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Startuptimeout::_1kcycles
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == Startuptimeout::_2kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == Startuptimeout::_4kcycles
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == Startuptimeout::_8kcycles
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == Startuptimeout::_16kcycles
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == Startuptimeout::_32kcycles
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline(always)]
    pub fn is_64kcycles(&self) -> bool {
        *self == Startuptimeout::_64kcycles
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline(always)]
    pub fn is_128kcycles(&self) -> bool {
        *self == Startuptimeout::_128kcycles
    }
}
#[doc = "Field `STARTUPTIMEOUT` writer - Wait Duration in HFXO Startup Enable Wait State"]
pub type StartuptimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startuptimeout>;
impl<'a, REG> StartuptimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_2cycles)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_4cycles)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_16cycles)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_32cycles)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_64cycles)
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_128cycles)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_256cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_1kcycles)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_2kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_4kcycles)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_8kcycles)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_16kcycles)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_32kcycles)
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline(always)]
    pub fn _64kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_64kcycles)
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline(always)]
    pub fn _128kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startuptimeout::_128kcycles)
    }
}
#[doc = "Wait Duration in HFXO Startup Steady Wait State\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Steadytimeout {
    #[doc = "0: Timeout period of 2 cycles"]
    _2cycles = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4cycles = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16cycles = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32cycles = 3,
    #[doc = "4: Timeout period of 64 cycles"]
    _64cycles = 4,
    #[doc = "5: Timeout period of 128 cycles"]
    _128cycles = 5,
    #[doc = "6: Timeout period of 256 cycles"]
    _256cycles = 6,
    #[doc = "7: Timeout period of 1024 cycles"]
    _1kcycles = 7,
    #[doc = "8: Timeout period of 2048 cycles"]
    _2kcycles = 8,
    #[doc = "9: Timeout period of 4096 cycles"]
    _4kcycles = 9,
    #[doc = "10: Timeout period of 8192 cycles"]
    _8kcycles = 10,
    #[doc = "11: Timeout period of 16384 cycles"]
    _16kcycles = 11,
    #[doc = "12: Timeout period of 32768 cycles"]
    _32kcycles = 12,
    #[doc = "13: Timeout period of 65536 cycles"]
    _64kcycles = 13,
    #[doc = "14: Timeout period of 131072 cycles"]
    _128kcycles = 14,
}
impl From<Steadytimeout> for u8 {
    #[inline(always)]
    fn from(variant: Steadytimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Steadytimeout {
    type Ux = u8;
}
impl crate::IsEnum for Steadytimeout {}
#[doc = "Field `STEADYTIMEOUT` reader - Wait Duration in HFXO Startup Steady Wait State"]
pub type SteadytimeoutR = crate::FieldReader<Steadytimeout>;
impl SteadytimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Steadytimeout> {
        match self.bits {
            0 => Some(Steadytimeout::_2cycles),
            1 => Some(Steadytimeout::_4cycles),
            2 => Some(Steadytimeout::_16cycles),
            3 => Some(Steadytimeout::_32cycles),
            4 => Some(Steadytimeout::_64cycles),
            5 => Some(Steadytimeout::_128cycles),
            6 => Some(Steadytimeout::_256cycles),
            7 => Some(Steadytimeout::_1kcycles),
            8 => Some(Steadytimeout::_2kcycles),
            9 => Some(Steadytimeout::_4kcycles),
            10 => Some(Steadytimeout::_8kcycles),
            11 => Some(Steadytimeout::_16kcycles),
            12 => Some(Steadytimeout::_32kcycles),
            13 => Some(Steadytimeout::_64kcycles),
            14 => Some(Steadytimeout::_128kcycles),
            _ => None,
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Steadytimeout::_2cycles
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Steadytimeout::_4cycles
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Steadytimeout::_16cycles
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Steadytimeout::_32cycles
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Steadytimeout::_64cycles
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Steadytimeout::_128cycles
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Steadytimeout::_256cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Steadytimeout::_1kcycles
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == Steadytimeout::_2kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == Steadytimeout::_4kcycles
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == Steadytimeout::_8kcycles
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == Steadytimeout::_16kcycles
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == Steadytimeout::_32kcycles
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline(always)]
    pub fn is_64kcycles(&self) -> bool {
        *self == Steadytimeout::_64kcycles
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline(always)]
    pub fn is_128kcycles(&self) -> bool {
        *self == Steadytimeout::_128kcycles
    }
}
#[doc = "Field `STEADYTIMEOUT` writer - Wait Duration in HFXO Startup Steady Wait State"]
pub type SteadytimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, Steadytimeout>;
impl<'a, REG> SteadytimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_2cycles)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_4cycles)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_16cycles)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_32cycles)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_64cycles)
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_128cycles)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_256cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_1kcycles)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_2kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_4kcycles)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_8kcycles)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_16kcycles)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_32kcycles)
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline(always)]
    pub fn _64kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_64kcycles)
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline(always)]
    pub fn _128kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Steadytimeout::_128kcycles)
    }
}
#[doc = "Wait Duration in HFXO Peak Detection Wait State\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Peakdettimeout {
    #[doc = "0: Timeout period of 2 cycles"]
    _2cycles = 0,
    #[doc = "1: Timeout period of 4 cycles"]
    _4cycles = 1,
    #[doc = "2: Timeout period of 16 cycles"]
    _16cycles = 2,
    #[doc = "3: Timeout period of 32 cycles"]
    _32cycles = 3,
    #[doc = "4: Timeout period of 64 cycles"]
    _64cycles = 4,
    #[doc = "5: Timeout period of 128 cycles"]
    _128cycles = 5,
    #[doc = "6: Timeout period of 256 cycles"]
    _256cycles = 6,
    #[doc = "7: Timeout period of 1024 cycles"]
    _1kcycles = 7,
    #[doc = "8: Timeout period of 2048 cycles"]
    _2kcycles = 8,
    #[doc = "9: Timeout period of 4096 cycles"]
    _4kcycles = 9,
    #[doc = "10: Timeout period of 8192 cycles"]
    _8kcycles = 10,
    #[doc = "11: Timeout period of 16384 cycles"]
    _16kcycles = 11,
    #[doc = "12: Timeout period of 32768 cycles"]
    _32kcycles = 12,
    #[doc = "13: Timeout period of 65536 cycles"]
    _64kcycles = 13,
    #[doc = "14: Timeout period of 131072 cycles"]
    _128kcycles = 14,
}
impl From<Peakdettimeout> for u8 {
    #[inline(always)]
    fn from(variant: Peakdettimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peakdettimeout {
    type Ux = u8;
}
impl crate::IsEnum for Peakdettimeout {}
#[doc = "Field `PEAKDETTIMEOUT` reader - Wait Duration in HFXO Peak Detection Wait State"]
pub type PeakdettimeoutR = crate::FieldReader<Peakdettimeout>;
impl PeakdettimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Peakdettimeout> {
        match self.bits {
            0 => Some(Peakdettimeout::_2cycles),
            1 => Some(Peakdettimeout::_4cycles),
            2 => Some(Peakdettimeout::_16cycles),
            3 => Some(Peakdettimeout::_32cycles),
            4 => Some(Peakdettimeout::_64cycles),
            5 => Some(Peakdettimeout::_128cycles),
            6 => Some(Peakdettimeout::_256cycles),
            7 => Some(Peakdettimeout::_1kcycles),
            8 => Some(Peakdettimeout::_2kcycles),
            9 => Some(Peakdettimeout::_4kcycles),
            10 => Some(Peakdettimeout::_8kcycles),
            11 => Some(Peakdettimeout::_16kcycles),
            12 => Some(Peakdettimeout::_32kcycles),
            13 => Some(Peakdettimeout::_64kcycles),
            14 => Some(Peakdettimeout::_128kcycles),
            _ => None,
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Peakdettimeout::_2cycles
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Peakdettimeout::_4cycles
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Peakdettimeout::_16cycles
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Peakdettimeout::_32cycles
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == Peakdettimeout::_64cycles
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == Peakdettimeout::_128cycles
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == Peakdettimeout::_256cycles
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == Peakdettimeout::_1kcycles
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn is_2kcycles(&self) -> bool {
        *self == Peakdettimeout::_2kcycles
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_4kcycles(&self) -> bool {
        *self == Peakdettimeout::_4kcycles
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn is_8kcycles(&self) -> bool {
        *self == Peakdettimeout::_8kcycles
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == Peakdettimeout::_16kcycles
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == Peakdettimeout::_32kcycles
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline(always)]
    pub fn is_64kcycles(&self) -> bool {
        *self == Peakdettimeout::_64kcycles
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline(always)]
    pub fn is_128kcycles(&self) -> bool {
        *self == Peakdettimeout::_128kcycles
    }
}
#[doc = "Field `PEAKDETTIMEOUT` writer - Wait Duration in HFXO Peak Detection Wait State"]
pub type PeakdettimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 4, Peakdettimeout>;
impl<'a, REG> PeakdettimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_2cycles)
    }
    #[doc = "Timeout period of 4 cycles"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_4cycles)
    }
    #[doc = "Timeout period of 16 cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_16cycles)
    }
    #[doc = "Timeout period of 32 cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_32cycles)
    }
    #[doc = "Timeout period of 64 cycles"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_64cycles)
    }
    #[doc = "Timeout period of 128 cycles"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_128cycles)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_256cycles)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_1kcycles)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn _2kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_2kcycles)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn _4kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_4kcycles)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn _8kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_8kcycles)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_16kcycles)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_32kcycles)
    }
    #[doc = "Timeout period of 65536 cycles"]
    #[inline(always)]
    pub fn _64kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_64kcycles)
    }
    #[doc = "Timeout period of 131072 cycles"]
    #[inline(always)]
    pub fn _128kcycles(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdettimeout::_128kcycles)
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline(always)]
    pub fn startuptimeout(&self) -> StartuptimeoutR {
        StartuptimeoutR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline(always)]
    pub fn steadytimeout(&self) -> SteadytimeoutR {
        SteadytimeoutR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline(always)]
    pub fn peakdettimeout(&self) -> PeakdettimeoutR {
        PeakdettimeoutR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait Duration in HFXO Startup Enable Wait State"]
    #[inline(always)]
    pub fn startuptimeout(&mut self) -> StartuptimeoutW<'_, HfxotimeoutctrlSpec> {
        StartuptimeoutW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Wait Duration in HFXO Startup Steady Wait State"]
    #[inline(always)]
    pub fn steadytimeout(&mut self) -> SteadytimeoutW<'_, HfxotimeoutctrlSpec> {
        SteadytimeoutW::new(self, 4)
    }
    #[doc = "Bits 12:15 - Wait Duration in HFXO Peak Detection Wait State"]
    #[inline(always)]
    pub fn peakdettimeout(&mut self) -> PeakdettimeoutW<'_, HfxotimeoutctrlSpec> {
        PeakdettimeoutW::new(self, 12)
    }
}
#[doc = "HFXO Timeout Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxotimeoutctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxotimeoutctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxotimeoutctrlSpec;
impl crate::RegisterSpec for HfxotimeoutctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxotimeoutctrl::R`](R) reader structure"]
impl crate::Readable for HfxotimeoutctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxotimeoutctrl::W`](W) writer structure"]
impl crate::Writable for HfxotimeoutctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFXOTIMEOUTCTRL to value 0xd08e"]
impl crate::Resettable for HfxotimeoutctrlSpec {
    const RESET_VALUE: u32 = 0xd08e;
}
