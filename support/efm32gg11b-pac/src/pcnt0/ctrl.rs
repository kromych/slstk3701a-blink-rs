#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `MODE` reader - Mode Select"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: The module is disabled."]
    DISABLE = 0,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM3)."]
    OVSSINGLE = 1,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    EXTCLKSINGLE = 2,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    EXTCLKQUAD = 3,
    #[doc = "4: LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    OVSQUAD1X = 4,
    #[doc = "5: LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    OVSQUAD2X = 5,
    #[doc = "6: LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    OVSQUAD4X = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLE),
            1 => Some(MODE_A::OVSSINGLE),
            2 => Some(MODE_A::EXTCLKSINGLE),
            3 => Some(MODE_A::EXTCLKQUAD),
            4 => Some(MODE_A::OVSQUAD1X),
            5 => Some(MODE_A::OVSQUAD2X),
            6 => Some(MODE_A::OVSQUAD4X),
            _ => None,
        }
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == MODE_A::OVSSINGLE
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == MODE_A::EXTCLKSINGLE
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == MODE_A::EXTCLKQUAD
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == MODE_A::OVSQUAD1X
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == MODE_A::OVSQUAD2X
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == MODE_A::OVSQUAD4X
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OVSSINGLE)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::EXTCLKSINGLE)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::EXTCLKQUAD)
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OVSQUAD1X)
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OVSQUAD2X)
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::OVSQUAD4X)
    }
}
#[doc = "Field `FILT` reader - Enable Digital Pulse Width Filter"]
pub type FILT_R = crate::BitReader;
#[doc = "Field `FILT` writer - Enable Digital Pulse Width Filter"]
pub type FILT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Enable PCNT Clock Domain Reset"]
pub type RSTEN_R = crate::BitReader;
#[doc = "Field `RSTEN` writer - Enable PCNT Clock Domain Reset"]
pub type RSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTRSTEN` reader - Enable CNT Reset"]
pub type CNTRSTEN_R = crate::BitReader;
#[doc = "Field `CNTRSTEN` writer - Enable CNT Reset"]
pub type CNTRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_R = crate::BitReader;
#[doc = "Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset"]
pub type AUXCNTRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGHALT` reader - Debug Mode Halt Enable"]
pub type DEBUGHALT_R = crate::BitReader;
#[doc = "Field `DEBUGHALT` writer - Debug Mode Halt Enable"]
pub type DEBUGHALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HYST_R = crate::BitReader;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HYST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1CDIR` reader - Count Direction Determined By S1"]
pub type S1CDIR_R = crate::BitReader;
#[doc = "Field `S1CDIR` writer - Count Direction Determined By S1"]
pub type S1CDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTEV` reader - Controls When the Counter Counts"]
pub type CNTEV_R = crate::FieldReader<CNTEV_A>;
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTEV_A {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    BOTH = 0,
    #[doc = "1: Only counts up on up-count events."]
    UP = 1,
    #[doc = "2: Only counts down on down-count events."]
    DOWN = 2,
    #[doc = "3: Never counts."]
    NONE = 3,
}
impl From<CNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTEV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNTEV_A {
    type Ux = u8;
}
impl CNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTEV_A {
        match self.bits {
            0 => CNTEV_A::BOTH,
            1 => CNTEV_A::UP,
            2 => CNTEV_A::DOWN,
            3 => CNTEV_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CNTEV_A::BOTH
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CNTEV_A::UP
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == CNTEV_A::DOWN
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CNTEV_A::NONE
    }
}
#[doc = "Field `CNTEV` writer - Controls When the Counter Counts"]
pub type CNTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CNTEV_A>;
impl<'a, REG> CNTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::BOTH)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::UP)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::DOWN)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CNTEV_A::NONE)
    }
}
#[doc = "Field `AUXCNTEV` reader - Controls When the Auxiliary Counter Counts"]
pub type AUXCNTEV_R = crate::FieldReader<AUXCNTEV_A>;
#[doc = "Controls When the Auxiliary Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUXCNTEV_A {
    #[doc = "0: Never counts."]
    NONE = 0,
    #[doc = "1: Counts up on up-count events."]
    UP = 1,
    #[doc = "2: Counts up on down-count events."]
    DOWN = 2,
    #[doc = "3: Counts up on both up-count and down-count events."]
    BOTH = 3,
}
impl From<AUXCNTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCNTEV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AUXCNTEV_A {
    type Ux = u8;
}
impl AUXCNTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUXCNTEV_A {
        match self.bits {
            0 => AUXCNTEV_A::NONE,
            1 => AUXCNTEV_A::UP,
            2 => AUXCNTEV_A::DOWN,
            3 => AUXCNTEV_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AUXCNTEV_A::NONE
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == AUXCNTEV_A::UP
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == AUXCNTEV_A::DOWN
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == AUXCNTEV_A::BOTH
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls When the Auxiliary Counter Counts"]
pub type AUXCNTEV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AUXCNTEV_A>;
impl<'a, REG> AUXCNTEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::NONE)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::UP)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::DOWN)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(AUXCNTEV_A::BOTH)
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_R = crate::BitReader;
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control"]
pub type CNTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EDGE_R = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCMODE` reader - Sets the Mode for Triggered Compare and Clear"]
pub type TCCMODE_R = crate::FieldReader<TCCMODE_A>;
#[doc = "Sets the Mode for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCMODE_A {
    #[doc = "0: Triggered compare and clear not enabled."]
    DISABLED = 0,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    LFA = 1,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
    PRS = 2,
}
impl From<TCCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCMODE_A {
    type Ux = u8;
}
impl TCCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCMODE_A> {
        match self.bits {
            0 => Some(TCCMODE_A::DISABLED),
            1 => Some(TCCMODE_A::LFA),
            2 => Some(TCCMODE_A::PRS),
            _ => None,
        }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCCMODE_A::DISABLED
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == TCCMODE_A::LFA
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TCCMODE_A::PRS
    }
}
#[doc = "Field `TCCMODE` writer - Sets the Mode for Triggered Compare and Clear"]
pub type TCCMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCMODE_A>;
impl<'a, REG> TCCMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE_A::DISABLED)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE_A::LFA)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(TCCMODE_A::PRS)
    }
}
#[doc = "Field `TCCPRESC` reader - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TCCPRESC_R = crate::FieldReader<TCCPRESC_A>;
#[doc = "Set the LFA Prescaler for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRESC_A {
    #[doc = "0: Compare and clear event each LFA cycle."]
    DIV1 = 0,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    DIV2 = 1,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    DIV4 = 2,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
    DIV8 = 3,
}
impl From<TCCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCPRESC_A {
    type Ux = u8;
}
impl TCCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCCPRESC_A {
        match self.bits {
            0 => TCCPRESC_A::DIV1,
            1 => TCCPRESC_A::DIV2,
            2 => TCCPRESC_A::DIV4,
            3 => TCCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == TCCPRESC_A::DIV1
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == TCCPRESC_A::DIV2
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TCCPRESC_A::DIV4
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == TCCPRESC_A::DIV8
    }
}
#[doc = "Field `TCCPRESC` writer - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TCCPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TCCPRESC_A>;
impl<'a, REG> TCCPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRESC_A::DIV8)
    }
}
#[doc = "Field `TCCCOMP` reader - Triggered Compare and Clear Compare Mode"]
pub type TCCCOMP_R = crate::FieldReader<TCCCOMP_A>;
#[doc = "Triggered Compare and Clear Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCCOMP_A {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    LTOE = 0,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    GTOE = 1,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    RANGE = 2,
}
impl From<TCCCOMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCCOMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCCOMP_A {
    type Ux = u8;
}
impl TCCCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCCOMP_A> {
        match self.bits {
            0 => Some(TCCCOMP_A::LTOE),
            1 => Some(TCCCOMP_A::GTOE),
            2 => Some(TCCCOMP_A::RANGE),
            _ => None,
        }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == TCCCOMP_A::LTOE
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == TCCCOMP_A::GTOE
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == TCCCOMP_A::RANGE
    }
}
#[doc = "Field `TCCCOMP` writer - Triggered Compare and Clear Compare Mode"]
pub type TCCCOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCCCOMP_A>;
impl<'a, REG> TCCCOMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP_A::LTOE)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP_A::GTOE)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(TCCCOMP_A::RANGE)
    }
}
#[doc = "Field `PRSGATEEN` reader - PRS Gate Enable"]
pub type PRSGATEEN_R = crate::BitReader;
#[doc = "Field `PRSGATEEN` writer - PRS Gate Enable"]
pub type PRSGATEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSPOL` reader - TCC PRS Polarity Select"]
pub type TCCPRSPOL_R = crate::BitReader;
#[doc = "Field `TCCPRSPOL` writer - TCC PRS Polarity Select"]
pub type TCCPRSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSSEL` reader - TCC PRS Channel Select"]
pub type TCCPRSSEL_R = crate::FieldReader<TCCPRSSEL_A>;
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected."]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected."]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected."]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected."]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected."]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected."]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected."]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected."]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected."]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected."]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected."]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected."]
    PRSCH23 = 23,
}
impl From<TCCPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCPRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCPRSSEL_A {
    type Ux = u8;
}
impl TCCPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCCPRSSEL_A> {
        match self.bits {
            0 => Some(TCCPRSSEL_A::PRSCH0),
            1 => Some(TCCPRSSEL_A::PRSCH1),
            2 => Some(TCCPRSSEL_A::PRSCH2),
            3 => Some(TCCPRSSEL_A::PRSCH3),
            4 => Some(TCCPRSSEL_A::PRSCH4),
            5 => Some(TCCPRSSEL_A::PRSCH5),
            6 => Some(TCCPRSSEL_A::PRSCH6),
            7 => Some(TCCPRSSEL_A::PRSCH7),
            8 => Some(TCCPRSSEL_A::PRSCH8),
            9 => Some(TCCPRSSEL_A::PRSCH9),
            10 => Some(TCCPRSSEL_A::PRSCH10),
            11 => Some(TCCPRSSEL_A::PRSCH11),
            12 => Some(TCCPRSSEL_A::PRSCH12),
            13 => Some(TCCPRSSEL_A::PRSCH13),
            14 => Some(TCCPRSSEL_A::PRSCH14),
            15 => Some(TCCPRSSEL_A::PRSCH15),
            16 => Some(TCCPRSSEL_A::PRSCH16),
            17 => Some(TCCPRSSEL_A::PRSCH17),
            18 => Some(TCCPRSSEL_A::PRSCH18),
            19 => Some(TCCPRSSEL_A::PRSCH19),
            20 => Some(TCCPRSSEL_A::PRSCH20),
            21 => Some(TCCPRSSEL_A::PRSCH21),
            22 => Some(TCCPRSSEL_A::PRSCH22),
            23 => Some(TCCPRSSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH11
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH12
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH13
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH14
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH15
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH16
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH17
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH18
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH19
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH20
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH21
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH22
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == TCCPRSSEL_A::PRSCH23
    }
}
#[doc = "Field `TCCPRSSEL` writer - TCC PRS Channel Select"]
pub type TCCPRSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, TCCPRSSEL_A>;
impl<'a, REG> TCCPRSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(TCCPRSSEL_A::PRSCH23)
    }
}
#[doc = "Field `TOPBHFSEL` reader - TOPB High Frequency Value Select"]
pub type TOPBHFSEL_R = crate::BitReader;
#[doc = "Field `TOPBHFSEL` writer - TOPB High Frequency Value Select"]
pub type TOPBHFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&self) -> CNTRSTEN_R {
        CNTRSTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AUXCNTRSTEN_R {
        AUXCNTRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DEBUGHALT_R {
        DEBUGHALT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1CDIR_R {
        S1CDIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CNTEV_R {
        CNTEV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AUXCNTEV_R {
        AUXCNTEV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CNTDIR_R {
        CNTDIR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TCCMODE_R {
        TCCMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TCCPRESC_R {
        TCCPRESC_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TCCCOMP_R {
        TCCCOMP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PRSGATEEN_R {
        PRSGATEEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TCCPRSPOL_R {
        TCCPRSPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TCCPRSSEL_R {
        TCCPRSSEL_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&self) -> TOPBHFSEL_R {
        TOPBHFSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRL_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<CTRL_SPEC> {
        FILT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<CTRL_SPEC> {
        RSTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntrsten(&mut self) -> CNTRSTEN_W<CTRL_SPEC> {
        CNTRSTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntrsten(&mut self) -> AUXCNTRSTEN_W<CTRL_SPEC> {
        AUXCNTRSTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debughalt(&mut self) -> DEBUGHALT_W<CTRL_SPEC> {
        DEBUGHALT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<CTRL_SPEC> {
        HYST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    #[must_use]
    pub fn s1cdir(&mut self) -> S1CDIR_W<CTRL_SPEC> {
        S1CDIR_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    #[must_use]
    pub fn cntev(&mut self) -> CNTEV_W<CTRL_SPEC> {
        CNTEV_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    #[must_use]
    pub fn auxcntev(&mut self) -> AUXCNTEV_W<CTRL_SPEC> {
        AUXCNTEV_W::new(self, 12)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    #[must_use]
    pub fn cntdir(&mut self) -> CNTDIR_W<CTRL_SPEC> {
        CNTDIR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<CTRL_SPEC> {
        EDGE_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tccmode(&mut self) -> TCCMODE_W<CTRL_SPEC> {
        TCCMODE_W::new(self, 16)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tccpresc(&mut self) -> TCCPRESC_W<CTRL_SPEC> {
        TCCPRESC_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcccomp(&mut self) -> TCCCOMP_W<CTRL_SPEC> {
        TCCCOMP_W::new(self, 22)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsgateen(&mut self) -> PRSGATEEN_W<CTRL_SPEC> {
        PRSGATEEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn tccprspol(&mut self) -> TCCPRSPOL_W<CTRL_SPEC> {
        TCCPRSPOL_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - TCC PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn tccprssel(&mut self) -> TCCPRSSEL_W<CTRL_SPEC> {
        TCCPRSSEL_W::new(self, 26)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    #[must_use]
    pub fn topbhfsel(&mut self) -> TOPBHFSEL_W<CTRL_SPEC> {
        TOPBHFSEL_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
