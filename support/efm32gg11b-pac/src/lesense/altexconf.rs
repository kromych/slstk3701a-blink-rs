#[doc = "Register `ALTEXCONF` reader"]
pub type R = crate::R<ALTEXCONF_SPEC>;
#[doc = "Register `ALTEXCONF` writer"]
pub type W = crate::W<ALTEXCONF_SPEC>;
#[doc = "Field `IDLECONF0` reader - ALTEX0 Idle Phase Configuration"]
pub type IDLECONF0_R = crate::FieldReader<IDLECONF0_A>;
#[doc = "ALTEX0 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF0_A {
    #[doc = "0: ALTEX0 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX0 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX0 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF0_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF0_A {
    type Ux = u8;
}
impl IDLECONF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF0_A> {
        match self.bits {
            0 => Some(IDLECONF0_A::DISABLE),
            1 => Some(IDLECONF0_A::HIGH),
            2 => Some(IDLECONF0_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF0_A::DISABLE
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF0_A::HIGH
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF0_A::LOW
    }
}
#[doc = "Field `IDLECONF0` writer - ALTEX0 Idle Phase Configuration"]
pub type IDLECONF0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF0_A>;
impl<'a, REG, const O: u8> IDLECONF0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF0_A::DISABLE)
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF0_A::HIGH)
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF0_A::LOW)
    }
}
#[doc = "Field `IDLECONF1` reader - ALTEX1 Idle Phase Configuration"]
pub type IDLECONF1_R = crate::FieldReader<IDLECONF1_A>;
#[doc = "ALTEX1 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF1_A {
    #[doc = "0: ALTEX1 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX1 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX1 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF1_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF1_A {
    type Ux = u8;
}
impl IDLECONF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF1_A> {
        match self.bits {
            0 => Some(IDLECONF1_A::DISABLE),
            1 => Some(IDLECONF1_A::HIGH),
            2 => Some(IDLECONF1_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF1_A::DISABLE
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF1_A::HIGH
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF1_A::LOW
    }
}
#[doc = "Field `IDLECONF1` writer - ALTEX1 Idle Phase Configuration"]
pub type IDLECONF1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF1_A>;
impl<'a, REG, const O: u8> IDLECONF1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF1_A::DISABLE)
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF1_A::HIGH)
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF1_A::LOW)
    }
}
#[doc = "Field `IDLECONF2` reader - ALTEX2 Idle Phase Configuration"]
pub type IDLECONF2_R = crate::FieldReader<IDLECONF2_A>;
#[doc = "ALTEX2 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF2_A {
    #[doc = "0: ALTEX2 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX2 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX2 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF2_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF2_A {
    type Ux = u8;
}
impl IDLECONF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF2_A> {
        match self.bits {
            0 => Some(IDLECONF2_A::DISABLE),
            1 => Some(IDLECONF2_A::HIGH),
            2 => Some(IDLECONF2_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF2_A::DISABLE
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF2_A::HIGH
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF2_A::LOW
    }
}
#[doc = "Field `IDLECONF2` writer - ALTEX2 Idle Phase Configuration"]
pub type IDLECONF2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF2_A>;
impl<'a, REG, const O: u8> IDLECONF2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF2_A::DISABLE)
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF2_A::HIGH)
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF2_A::LOW)
    }
}
#[doc = "Field `IDLECONF3` reader - ALTEX3 Idle Phase Configuration"]
pub type IDLECONF3_R = crate::FieldReader<IDLECONF3_A>;
#[doc = "ALTEX3 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF3_A {
    #[doc = "0: ALTEX3 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX3 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX3 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF3_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF3_A {
    type Ux = u8;
}
impl IDLECONF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF3_A> {
        match self.bits {
            0 => Some(IDLECONF3_A::DISABLE),
            1 => Some(IDLECONF3_A::HIGH),
            2 => Some(IDLECONF3_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF3_A::DISABLE
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF3_A::HIGH
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF3_A::LOW
    }
}
#[doc = "Field `IDLECONF3` writer - ALTEX3 Idle Phase Configuration"]
pub type IDLECONF3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF3_A>;
impl<'a, REG, const O: u8> IDLECONF3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF3_A::DISABLE)
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF3_A::HIGH)
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF3_A::LOW)
    }
}
#[doc = "Field `IDLECONF4` reader - ALTEX4 Idle Phase Configuration"]
pub type IDLECONF4_R = crate::FieldReader<IDLECONF4_A>;
#[doc = "ALTEX4 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF4_A {
    #[doc = "0: ALTEX4 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX4 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX4 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF4_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF4_A {
    type Ux = u8;
}
impl IDLECONF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF4_A> {
        match self.bits {
            0 => Some(IDLECONF4_A::DISABLE),
            1 => Some(IDLECONF4_A::HIGH),
            2 => Some(IDLECONF4_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF4_A::DISABLE
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF4_A::HIGH
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF4_A::LOW
    }
}
#[doc = "Field `IDLECONF4` writer - ALTEX4 Idle Phase Configuration"]
pub type IDLECONF4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF4_A>;
impl<'a, REG, const O: u8> IDLECONF4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF4_A::DISABLE)
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF4_A::HIGH)
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF4_A::LOW)
    }
}
#[doc = "Field `IDLECONF5` reader - ALTEX5 Idle Phase Configuration"]
pub type IDLECONF5_R = crate::FieldReader<IDLECONF5_A>;
#[doc = "ALTEX5 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF5_A {
    #[doc = "0: ALTEX5 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX5 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX5 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF5_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF5_A {
    type Ux = u8;
}
impl IDLECONF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF5_A> {
        match self.bits {
            0 => Some(IDLECONF5_A::DISABLE),
            1 => Some(IDLECONF5_A::HIGH),
            2 => Some(IDLECONF5_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF5_A::DISABLE
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF5_A::HIGH
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF5_A::LOW
    }
}
#[doc = "Field `IDLECONF5` writer - ALTEX5 Idle Phase Configuration"]
pub type IDLECONF5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF5_A>;
impl<'a, REG, const O: u8> IDLECONF5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF5_A::DISABLE)
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF5_A::HIGH)
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF5_A::LOW)
    }
}
#[doc = "Field `IDLECONF6` reader - ALTEX6 Idle Phase Configuration"]
pub type IDLECONF6_R = crate::FieldReader<IDLECONF6_A>;
#[doc = "ALTEX6 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF6_A {
    #[doc = "0: ALTEX6 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX6 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX6 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF6_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF6_A {
    type Ux = u8;
}
impl IDLECONF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF6_A> {
        match self.bits {
            0 => Some(IDLECONF6_A::DISABLE),
            1 => Some(IDLECONF6_A::HIGH),
            2 => Some(IDLECONF6_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF6_A::DISABLE
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF6_A::HIGH
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF6_A::LOW
    }
}
#[doc = "Field `IDLECONF6` writer - ALTEX6 Idle Phase Configuration"]
pub type IDLECONF6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF6_A>;
impl<'a, REG, const O: u8> IDLECONF6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF6_A::DISABLE)
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF6_A::HIGH)
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF6_A::LOW)
    }
}
#[doc = "Field `IDLECONF7` reader - ALTEX7 Idle Phase Configuration"]
pub type IDLECONF7_R = crate::FieldReader<IDLECONF7_A>;
#[doc = "ALTEX7 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECONF7_A {
    #[doc = "0: ALTEX7 output is disabled in idle phase"]
    DISABLE = 0,
    #[doc = "1: ALTEX7 output is high in idle phase"]
    HIGH = 1,
    #[doc = "2: ALTEX7 output is low in idle phase"]
    LOW = 2,
}
impl From<IDLECONF7_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLECONF7_A {
    type Ux = u8;
}
impl IDLECONF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECONF7_A> {
        match self.bits {
            0 => Some(IDLECONF7_A::DISABLE),
            1 => Some(IDLECONF7_A::HIGH),
            2 => Some(IDLECONF7_A::LOW),
            _ => None,
        }
    }
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF7_A::DISABLE
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF7_A::HIGH
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF7_A::LOW
    }
}
#[doc = "Field `IDLECONF7` writer - ALTEX7 Idle Phase Configuration"]
pub type IDLECONF7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IDLECONF7_A>;
impl<'a, REG, const O: u8> IDLECONF7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF7_A::DISABLE)
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF7_A::HIGH)
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECONF7_A::LOW)
    }
}
#[doc = "Field `AEX0` reader - ALTEX0 Always Excite Enable"]
pub type AEX0_R = crate::BitReader;
#[doc = "Field `AEX0` writer - ALTEX0 Always Excite Enable"]
pub type AEX0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX1` reader - ALTEX1 Always Excite Enable"]
pub type AEX1_R = crate::BitReader;
#[doc = "Field `AEX1` writer - ALTEX1 Always Excite Enable"]
pub type AEX1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX2` reader - ALTEX2 Always Excite Enable"]
pub type AEX2_R = crate::BitReader;
#[doc = "Field `AEX2` writer - ALTEX2 Always Excite Enable"]
pub type AEX2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX3` reader - ALTEX3 Always Excite Enable"]
pub type AEX3_R = crate::BitReader;
#[doc = "Field `AEX3` writer - ALTEX3 Always Excite Enable"]
pub type AEX3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX4` reader - ALTEX4 Always Excite Enable"]
pub type AEX4_R = crate::BitReader;
#[doc = "Field `AEX4` writer - ALTEX4 Always Excite Enable"]
pub type AEX4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX5` reader - ALTEX5 Always Excite Enable"]
pub type AEX5_R = crate::BitReader;
#[doc = "Field `AEX5` writer - ALTEX5 Always Excite Enable"]
pub type AEX5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX6` reader - ALTEX6 Always Excite Enable"]
pub type AEX6_R = crate::BitReader;
#[doc = "Field `AEX6` writer - ALTEX6 Always Excite Enable"]
pub type AEX6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEX7` reader - ALTEX7 Always Excite Enable"]
pub type AEX7_R = crate::BitReader;
#[doc = "Field `AEX7` writer - ALTEX7 Always Excite Enable"]
pub type AEX7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - ALTEX0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf0(&self) -> IDLECONF0_R {
        IDLECONF0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ALTEX1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf1(&self) -> IDLECONF1_R {
        IDLECONF1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ALTEX2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf2(&self) -> IDLECONF2_R {
        IDLECONF2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ALTEX3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf3(&self) -> IDLECONF3_R {
        IDLECONF3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ALTEX4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf4(&self) -> IDLECONF4_R {
        IDLECONF4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ALTEX5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf5(&self) -> IDLECONF5_R {
        IDLECONF5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ALTEX6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf6(&self) -> IDLECONF6_R {
        IDLECONF6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ALTEX7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf7(&self) -> IDLECONF7_R {
        IDLECONF7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - ALTEX0 Always Excite Enable"]
    #[inline(always)]
    pub fn aex0(&self) -> AEX0_R {
        AEX0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ALTEX1 Always Excite Enable"]
    #[inline(always)]
    pub fn aex1(&self) -> AEX1_R {
        AEX1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALTEX2 Always Excite Enable"]
    #[inline(always)]
    pub fn aex2(&self) -> AEX2_R {
        AEX2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ALTEX3 Always Excite Enable"]
    #[inline(always)]
    pub fn aex3(&self) -> AEX3_R {
        AEX3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ALTEX4 Always Excite Enable"]
    #[inline(always)]
    pub fn aex4(&self) -> AEX4_R {
        AEX4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ALTEX5 Always Excite Enable"]
    #[inline(always)]
    pub fn aex5(&self) -> AEX5_R {
        AEX5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ALTEX6 Always Excite Enable"]
    #[inline(always)]
    pub fn aex6(&self) -> AEX6_R {
        AEX6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ALTEX7 Always Excite Enable"]
    #[inline(always)]
    pub fn aex7(&self) -> AEX7_R {
        AEX7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ALTEX0 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf0(&mut self) -> IDLECONF0_W<ALTEXCONF_SPEC, 0> {
        IDLECONF0_W::new(self)
    }
    #[doc = "Bits 2:3 - ALTEX1 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf1(&mut self) -> IDLECONF1_W<ALTEXCONF_SPEC, 2> {
        IDLECONF1_W::new(self)
    }
    #[doc = "Bits 4:5 - ALTEX2 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf2(&mut self) -> IDLECONF2_W<ALTEXCONF_SPEC, 4> {
        IDLECONF2_W::new(self)
    }
    #[doc = "Bits 6:7 - ALTEX3 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf3(&mut self) -> IDLECONF3_W<ALTEXCONF_SPEC, 6> {
        IDLECONF3_W::new(self)
    }
    #[doc = "Bits 8:9 - ALTEX4 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf4(&mut self) -> IDLECONF4_W<ALTEXCONF_SPEC, 8> {
        IDLECONF4_W::new(self)
    }
    #[doc = "Bits 10:11 - ALTEX5 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf5(&mut self) -> IDLECONF5_W<ALTEXCONF_SPEC, 10> {
        IDLECONF5_W::new(self)
    }
    #[doc = "Bits 12:13 - ALTEX6 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf6(&mut self) -> IDLECONF6_W<ALTEXCONF_SPEC, 12> {
        IDLECONF6_W::new(self)
    }
    #[doc = "Bits 14:15 - ALTEX7 Idle Phase Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idleconf7(&mut self) -> IDLECONF7_W<ALTEXCONF_SPEC, 14> {
        IDLECONF7_W::new(self)
    }
    #[doc = "Bit 16 - ALTEX0 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex0(&mut self) -> AEX0_W<ALTEXCONF_SPEC, 16> {
        AEX0_W::new(self)
    }
    #[doc = "Bit 17 - ALTEX1 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex1(&mut self) -> AEX1_W<ALTEXCONF_SPEC, 17> {
        AEX1_W::new(self)
    }
    #[doc = "Bit 18 - ALTEX2 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex2(&mut self) -> AEX2_W<ALTEXCONF_SPEC, 18> {
        AEX2_W::new(self)
    }
    #[doc = "Bit 19 - ALTEX3 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex3(&mut self) -> AEX3_W<ALTEXCONF_SPEC, 19> {
        AEX3_W::new(self)
    }
    #[doc = "Bit 20 - ALTEX4 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex4(&mut self) -> AEX4_W<ALTEXCONF_SPEC, 20> {
        AEX4_W::new(self)
    }
    #[doc = "Bit 21 - ALTEX5 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex5(&mut self) -> AEX5_W<ALTEXCONF_SPEC, 21> {
        AEX5_W::new(self)
    }
    #[doc = "Bit 22 - ALTEX6 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex6(&mut self) -> AEX6_W<ALTEXCONF_SPEC, 22> {
        AEX6_W::new(self)
    }
    #[doc = "Bit 23 - ALTEX7 Always Excite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aex7(&mut self) -> AEX7_W<ALTEXCONF_SPEC, 23> {
        AEX7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Alternative Excite Pin Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altexconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altexconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALTEXCONF_SPEC;
impl crate::RegisterSpec for ALTEXCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altexconf::R`](R) reader structure"]
impl crate::Readable for ALTEXCONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`altexconf::W`](W) writer structure"]
impl crate::Writable for ALTEXCONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALTEXCONF to value 0"]
impl crate::Resettable for ALTEXCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
