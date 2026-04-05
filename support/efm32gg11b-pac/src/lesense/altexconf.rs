#[doc = "Register `ALTEXCONF` reader"]
pub type R = crate::R<AltexconfSpec>;
#[doc = "Register `ALTEXCONF` writer"]
pub type W = crate::W<AltexconfSpec>;
#[doc = "ALTEX0 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf0 {
    #[doc = "0: ALTEX0 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX0 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX0 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf0> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf0 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf0 {}
#[doc = "Field `IDLECONF0` reader - ALTEX0 Idle Phase Configuration"]
pub type Idleconf0R = crate::FieldReader<Idleconf0>;
impl Idleconf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf0> {
        match self.bits {
            0 => Some(Idleconf0::Disable),
            1 => Some(Idleconf0::High),
            2 => Some(Idleconf0::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf0::Disable
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf0::High
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf0::Low
    }
}
#[doc = "Field `IDLECONF0` writer - ALTEX0 Idle Phase Configuration"]
pub type Idleconf0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf0>;
impl<'a, REG> Idleconf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf0::Disable)
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf0::High)
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf0::Low)
    }
}
#[doc = "ALTEX1 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf1 {
    #[doc = "0: ALTEX1 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX1 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX1 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf1> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf1 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf1 {}
#[doc = "Field `IDLECONF1` reader - ALTEX1 Idle Phase Configuration"]
pub type Idleconf1R = crate::FieldReader<Idleconf1>;
impl Idleconf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf1> {
        match self.bits {
            0 => Some(Idleconf1::Disable),
            1 => Some(Idleconf1::High),
            2 => Some(Idleconf1::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf1::Disable
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf1::High
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf1::Low
    }
}
#[doc = "Field `IDLECONF1` writer - ALTEX1 Idle Phase Configuration"]
pub type Idleconf1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf1>;
impl<'a, REG> Idleconf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf1::Disable)
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf1::High)
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf1::Low)
    }
}
#[doc = "ALTEX2 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf2 {
    #[doc = "0: ALTEX2 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX2 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX2 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf2> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf2 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf2 {}
#[doc = "Field `IDLECONF2` reader - ALTEX2 Idle Phase Configuration"]
pub type Idleconf2R = crate::FieldReader<Idleconf2>;
impl Idleconf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf2> {
        match self.bits {
            0 => Some(Idleconf2::Disable),
            1 => Some(Idleconf2::High),
            2 => Some(Idleconf2::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf2::Disable
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf2::High
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf2::Low
    }
}
#[doc = "Field `IDLECONF2` writer - ALTEX2 Idle Phase Configuration"]
pub type Idleconf2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf2>;
impl<'a, REG> Idleconf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf2::Disable)
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf2::High)
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf2::Low)
    }
}
#[doc = "ALTEX3 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf3 {
    #[doc = "0: ALTEX3 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX3 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX3 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf3> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf3 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf3 {}
#[doc = "Field `IDLECONF3` reader - ALTEX3 Idle Phase Configuration"]
pub type Idleconf3R = crate::FieldReader<Idleconf3>;
impl Idleconf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf3> {
        match self.bits {
            0 => Some(Idleconf3::Disable),
            1 => Some(Idleconf3::High),
            2 => Some(Idleconf3::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf3::Disable
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf3::High
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf3::Low
    }
}
#[doc = "Field `IDLECONF3` writer - ALTEX3 Idle Phase Configuration"]
pub type Idleconf3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf3>;
impl<'a, REG> Idleconf3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf3::Disable)
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf3::High)
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf3::Low)
    }
}
#[doc = "ALTEX4 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf4 {
    #[doc = "0: ALTEX4 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX4 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX4 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf4> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf4 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf4 {}
#[doc = "Field `IDLECONF4` reader - ALTEX4 Idle Phase Configuration"]
pub type Idleconf4R = crate::FieldReader<Idleconf4>;
impl Idleconf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf4> {
        match self.bits {
            0 => Some(Idleconf4::Disable),
            1 => Some(Idleconf4::High),
            2 => Some(Idleconf4::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf4::Disable
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf4::High
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf4::Low
    }
}
#[doc = "Field `IDLECONF4` writer - ALTEX4 Idle Phase Configuration"]
pub type Idleconf4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf4>;
impl<'a, REG> Idleconf4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf4::Disable)
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf4::High)
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf4::Low)
    }
}
#[doc = "ALTEX5 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf5 {
    #[doc = "0: ALTEX5 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX5 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX5 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf5> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf5 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf5 {}
#[doc = "Field `IDLECONF5` reader - ALTEX5 Idle Phase Configuration"]
pub type Idleconf5R = crate::FieldReader<Idleconf5>;
impl Idleconf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf5> {
        match self.bits {
            0 => Some(Idleconf5::Disable),
            1 => Some(Idleconf5::High),
            2 => Some(Idleconf5::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf5::Disable
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf5::High
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf5::Low
    }
}
#[doc = "Field `IDLECONF5` writer - ALTEX5 Idle Phase Configuration"]
pub type Idleconf5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf5>;
impl<'a, REG> Idleconf5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf5::Disable)
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf5::High)
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf5::Low)
    }
}
#[doc = "ALTEX6 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf6 {
    #[doc = "0: ALTEX6 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX6 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX6 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf6> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf6 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf6 {}
#[doc = "Field `IDLECONF6` reader - ALTEX6 Idle Phase Configuration"]
pub type Idleconf6R = crate::FieldReader<Idleconf6>;
impl Idleconf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf6> {
        match self.bits {
            0 => Some(Idleconf6::Disable),
            1 => Some(Idleconf6::High),
            2 => Some(Idleconf6::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf6::Disable
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf6::High
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf6::Low
    }
}
#[doc = "Field `IDLECONF6` writer - ALTEX6 Idle Phase Configuration"]
pub type Idleconf6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf6>;
impl<'a, REG> Idleconf6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf6::Disable)
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf6::High)
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf6::Low)
    }
}
#[doc = "ALTEX7 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idleconf7 {
    #[doc = "0: ALTEX7 output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: ALTEX7 output is high in idle phase"]
    High = 1,
    #[doc = "2: ALTEX7 output is low in idle phase"]
    Low = 2,
}
impl From<Idleconf7> for u8 {
    #[inline(always)]
    fn from(variant: Idleconf7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idleconf7 {
    type Ux = u8;
}
impl crate::IsEnum for Idleconf7 {}
#[doc = "Field `IDLECONF7` reader - ALTEX7 Idle Phase Configuration"]
pub type Idleconf7R = crate::FieldReader<Idleconf7>;
impl Idleconf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Idleconf7> {
        match self.bits {
            0 => Some(Idleconf7::Disable),
            1 => Some(Idleconf7::High),
            2 => Some(Idleconf7::Low),
            _ => None,
        }
    }
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleconf7::Disable
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Idleconf7::High
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Idleconf7::Low
    }
}
#[doc = "Field `IDLECONF7` writer - ALTEX7 Idle Phase Configuration"]
pub type Idleconf7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Idleconf7>;
impl<'a, REG> Idleconf7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf7::Disable)
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf7::High)
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Idleconf7::Low)
    }
}
#[doc = "Field `AEX0` reader - ALTEX0 Always Excite Enable"]
pub type Aex0R = crate::BitReader;
#[doc = "Field `AEX0` writer - ALTEX0 Always Excite Enable"]
pub type Aex0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX1` reader - ALTEX1 Always Excite Enable"]
pub type Aex1R = crate::BitReader;
#[doc = "Field `AEX1` writer - ALTEX1 Always Excite Enable"]
pub type Aex1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX2` reader - ALTEX2 Always Excite Enable"]
pub type Aex2R = crate::BitReader;
#[doc = "Field `AEX2` writer - ALTEX2 Always Excite Enable"]
pub type Aex2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX3` reader - ALTEX3 Always Excite Enable"]
pub type Aex3R = crate::BitReader;
#[doc = "Field `AEX3` writer - ALTEX3 Always Excite Enable"]
pub type Aex3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX4` reader - ALTEX4 Always Excite Enable"]
pub type Aex4R = crate::BitReader;
#[doc = "Field `AEX4` writer - ALTEX4 Always Excite Enable"]
pub type Aex4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX5` reader - ALTEX5 Always Excite Enable"]
pub type Aex5R = crate::BitReader;
#[doc = "Field `AEX5` writer - ALTEX5 Always Excite Enable"]
pub type Aex5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX6` reader - ALTEX6 Always Excite Enable"]
pub type Aex6R = crate::BitReader;
#[doc = "Field `AEX6` writer - ALTEX6 Always Excite Enable"]
pub type Aex6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEX7` reader - ALTEX7 Always Excite Enable"]
pub type Aex7R = crate::BitReader;
#[doc = "Field `AEX7` writer - ALTEX7 Always Excite Enable"]
pub type Aex7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - ALTEX0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf0(&self) -> Idleconf0R {
        Idleconf0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ALTEX1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf1(&self) -> Idleconf1R {
        Idleconf1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ALTEX2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf2(&self) -> Idleconf2R {
        Idleconf2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ALTEX3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf3(&self) -> Idleconf3R {
        Idleconf3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ALTEX4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf4(&self) -> Idleconf4R {
        Idleconf4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ALTEX5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf5(&self) -> Idleconf5R {
        Idleconf5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ALTEX6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf6(&self) -> Idleconf6R {
        Idleconf6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ALTEX7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf7(&self) -> Idleconf7R {
        Idleconf7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - ALTEX0 Always Excite Enable"]
    #[inline(always)]
    pub fn aex0(&self) -> Aex0R {
        Aex0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ALTEX1 Always Excite Enable"]
    #[inline(always)]
    pub fn aex1(&self) -> Aex1R {
        Aex1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALTEX2 Always Excite Enable"]
    #[inline(always)]
    pub fn aex2(&self) -> Aex2R {
        Aex2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ALTEX3 Always Excite Enable"]
    #[inline(always)]
    pub fn aex3(&self) -> Aex3R {
        Aex3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ALTEX4 Always Excite Enable"]
    #[inline(always)]
    pub fn aex4(&self) -> Aex4R {
        Aex4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ALTEX5 Always Excite Enable"]
    #[inline(always)]
    pub fn aex5(&self) -> Aex5R {
        Aex5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ALTEX6 Always Excite Enable"]
    #[inline(always)]
    pub fn aex6(&self) -> Aex6R {
        Aex6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ALTEX7 Always Excite Enable"]
    #[inline(always)]
    pub fn aex7(&self) -> Aex7R {
        Aex7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ALTEX0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf0(&mut self) -> Idleconf0W<'_, AltexconfSpec> {
        Idleconf0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - ALTEX1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf1(&mut self) -> Idleconf1W<'_, AltexconfSpec> {
        Idleconf1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - ALTEX2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf2(&mut self) -> Idleconf2W<'_, AltexconfSpec> {
        Idleconf2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - ALTEX3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf3(&mut self) -> Idleconf3W<'_, AltexconfSpec> {
        Idleconf3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - ALTEX4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf4(&mut self) -> Idleconf4W<'_, AltexconfSpec> {
        Idleconf4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - ALTEX5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf5(&mut self) -> Idleconf5W<'_, AltexconfSpec> {
        Idleconf5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - ALTEX6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf6(&mut self) -> Idleconf6W<'_, AltexconfSpec> {
        Idleconf6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - ALTEX7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf7(&mut self) -> Idleconf7W<'_, AltexconfSpec> {
        Idleconf7W::new(self, 14)
    }
    #[doc = "Bit 16 - ALTEX0 Always Excite Enable"]
    #[inline(always)]
    pub fn aex0(&mut self) -> Aex0W<'_, AltexconfSpec> {
        Aex0W::new(self, 16)
    }
    #[doc = "Bit 17 - ALTEX1 Always Excite Enable"]
    #[inline(always)]
    pub fn aex1(&mut self) -> Aex1W<'_, AltexconfSpec> {
        Aex1W::new(self, 17)
    }
    #[doc = "Bit 18 - ALTEX2 Always Excite Enable"]
    #[inline(always)]
    pub fn aex2(&mut self) -> Aex2W<'_, AltexconfSpec> {
        Aex2W::new(self, 18)
    }
    #[doc = "Bit 19 - ALTEX3 Always Excite Enable"]
    #[inline(always)]
    pub fn aex3(&mut self) -> Aex3W<'_, AltexconfSpec> {
        Aex3W::new(self, 19)
    }
    #[doc = "Bit 20 - ALTEX4 Always Excite Enable"]
    #[inline(always)]
    pub fn aex4(&mut self) -> Aex4W<'_, AltexconfSpec> {
        Aex4W::new(self, 20)
    }
    #[doc = "Bit 21 - ALTEX5 Always Excite Enable"]
    #[inline(always)]
    pub fn aex5(&mut self) -> Aex5W<'_, AltexconfSpec> {
        Aex5W::new(self, 21)
    }
    #[doc = "Bit 22 - ALTEX6 Always Excite Enable"]
    #[inline(always)]
    pub fn aex6(&mut self) -> Aex6W<'_, AltexconfSpec> {
        Aex6W::new(self, 22)
    }
    #[doc = "Bit 23 - ALTEX7 Always Excite Enable"]
    #[inline(always)]
    pub fn aex7(&mut self) -> Aex7W<'_, AltexconfSpec> {
        Aex7W::new(self, 23)
    }
}
#[doc = "Alternative Excite Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`altexconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altexconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltexconfSpec;
impl crate::RegisterSpec for AltexconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altexconf::R`](R) reader structure"]
impl crate::Readable for AltexconfSpec {}
#[doc = "`write(|w| ..)` method takes [`altexconf::W`](W) writer structure"]
impl crate::Writable for AltexconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALTEXCONF to value 0"]
impl crate::Resettable for AltexconfSpec {}
