#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: The module is disabled."]
    Disable = 0,
    #[doc = "1: Single input LFACLK oversampling mode (available in EM0-EM3)."]
    Ovssingle = 1,
    #[doc = "2: Externally clocked single input counter mode (available in EM0-EM3)."]
    Extclksingle = 2,
    #[doc = "3: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    Extclkquad = 3,
    #[doc = "4: LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    Ovsquad1x = 4,
    #[doc = "5: LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    Ovsquad2x = 5,
    #[doc = "6: LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    Ovsquad4x = 6,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Mode Select"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Disable),
            1 => Some(Mode::Ovssingle),
            2 => Some(Mode::Extclksingle),
            3 => Some(Mode::Extclkquad),
            4 => Some(Mode::Ovsquad1x),
            5 => Some(Mode::Ovsquad2x),
            6 => Some(Mode::Ovsquad4x),
            _ => None,
        }
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mode::Disable
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == Mode::Ovssingle
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == Mode::Extclksingle
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == Mode::Extclkquad
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == Mode::Ovsquad1x
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == Mode::Ovsquad2x
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == Mode::Ovsquad4x
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disable)
    }
    #[doc = "Single input LFACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovssingle)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclksingle)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclkquad)
    }
    #[doc = "LFACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovsquad1x)
    }
    #[doc = "LFACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovsquad2x)
    }
    #[doc = "LFACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovsquad4x)
    }
}
#[doc = "Field `FILT` reader - Enable Digital Pulse Width Filter"]
pub type FiltR = crate::BitReader;
#[doc = "Field `FILT` writer - Enable Digital Pulse Width Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTEN` reader - Enable PCNT Clock Domain Reset"]
pub type RstenR = crate::BitReader;
#[doc = "Field `RSTEN` writer - Enable PCNT Clock Domain Reset"]
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTRSTEN` reader - Enable CNT Reset"]
pub type CntrstenR = crate::BitReader;
#[doc = "Field `CNTRSTEN` writer - Enable CNT Reset"]
pub type CntrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXCNTRSTEN` reader - Enable AUXCNT Reset"]
pub type AuxcntrstenR = crate::BitReader;
#[doc = "Field `AUXCNTRSTEN` writer - Enable AUXCNT Reset"]
pub type AuxcntrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGHALT` reader - Debug Mode Halt Enable"]
pub type DebughaltR = crate::BitReader;
#[doc = "Field `DEBUGHALT` writer - Debug Mode Halt Enable"]
pub type DebughaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1CDIR` reader - Count Direction Determined By S1"]
pub type S1cdirR = crate::BitReader;
#[doc = "Field `S1CDIR` writer - Count Direction Determined By S1"]
pub type S1cdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntev {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    Both = 0,
    #[doc = "1: Only counts up on up-count events."]
    Up = 1,
    #[doc = "2: Only counts down on down-count events."]
    Down = 2,
    #[doc = "3: Never counts."]
    None = 3,
}
impl From<Cntev> for u8 {
    #[inline(always)]
    fn from(variant: Cntev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntev {
    type Ux = u8;
}
impl crate::IsEnum for Cntev {}
#[doc = "Field `CNTEV` reader - Controls When the Counter Counts"]
pub type CntevR = crate::FieldReader<Cntev>;
impl CntevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntev {
        match self.bits {
            0 => Cntev::Both,
            1 => Cntev::Up,
            2 => Cntev::Down,
            3 => Cntev::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Cntev::Both
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Cntev::Up
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Cntev::Down
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cntev::None
    }
}
#[doc = "Field `CNTEV` writer - Controls When the Counter Counts"]
pub type CntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntev, crate::Safe>;
impl<'a, REG> CntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Both)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Up)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Down)
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::None)
    }
}
#[doc = "Controls When the Auxiliary Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Auxcntev {
    #[doc = "0: Never counts."]
    None = 0,
    #[doc = "1: Counts up on up-count events."]
    Up = 1,
    #[doc = "2: Counts up on down-count events."]
    Down = 2,
    #[doc = "3: Counts up on both up-count and down-count events."]
    Both = 3,
}
impl From<Auxcntev> for u8 {
    #[inline(always)]
    fn from(variant: Auxcntev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Auxcntev {
    type Ux = u8;
}
impl crate::IsEnum for Auxcntev {}
#[doc = "Field `AUXCNTEV` reader - Controls When the Auxiliary Counter Counts"]
pub type AuxcntevR = crate::FieldReader<Auxcntev>;
impl AuxcntevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Auxcntev {
        match self.bits {
            0 => Auxcntev::None,
            1 => Auxcntev::Up,
            2 => Auxcntev::Down,
            3 => Auxcntev::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Auxcntev::None
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Auxcntev::Up
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Auxcntev::Down
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Auxcntev::Both
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls When the Auxiliary Counter Counts"]
pub type AuxcntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Auxcntev, crate::Safe>;
impl<'a, REG> AuxcntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never counts."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::None)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Up)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Down)
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Both)
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Control"]
pub type CntdirR = crate::BitReader;
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Control"]
pub type CntdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EdgeR = crate::BitReader;
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets the Mode for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tccmode {
    #[doc = "0: Triggered compare and clear not enabled."]
    Disabled = 0,
    #[doc = "1: Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    Lfa = 1,
    #[doc = "2: Compare and clear performed on positive PRS edges."]
    Prs = 2,
}
impl From<Tccmode> for u8 {
    #[inline(always)]
    fn from(variant: Tccmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tccmode {
    type Ux = u8;
}
impl crate::IsEnum for Tccmode {}
#[doc = "Field `TCCMODE` reader - Sets the Mode for Triggered Compare and Clear"]
pub type TccmodeR = crate::FieldReader<Tccmode>;
impl TccmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tccmode> {
        match self.bits {
            0 => Some(Tccmode::Disabled),
            1 => Some(Tccmode::Lfa),
            2 => Some(Tccmode::Prs),
            _ => None,
        }
    }
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tccmode::Disabled
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn is_lfa(&self) -> bool {
        *self == Tccmode::Lfa
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Tccmode::Prs
    }
}
#[doc = "Field `TCCMODE` writer - Sets the Mode for Triggered Compare and Clear"]
pub type TccmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tccmode>;
impl<'a, REG> TccmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triggered compare and clear not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tccmode::Disabled)
    }
    #[doc = "Compare and clear performed on each (optionally prescaled) LFA clock cycle."]
    #[inline(always)]
    pub fn lfa(self) -> &'a mut crate::W<REG> {
        self.variant(Tccmode::Lfa)
    }
    #[doc = "Compare and clear performed on positive PRS edges."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Tccmode::Prs)
    }
}
#[doc = "Set the LFA Prescaler for Triggered Compare and Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tccpresc {
    #[doc = "0: Compare and clear event each LFA cycle."]
    Div1 = 0,
    #[doc = "1: Compare and clear performed on every other LFA cycle."]
    Div2 = 1,
    #[doc = "2: Compare and clear performed on every 4th LFA cycle."]
    Div4 = 2,
    #[doc = "3: Compare and clear performed on every 8th LFA cycle."]
    Div8 = 3,
}
impl From<Tccpresc> for u8 {
    #[inline(always)]
    fn from(variant: Tccpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tccpresc {
    type Ux = u8;
}
impl crate::IsEnum for Tccpresc {}
#[doc = "Field `TCCPRESC` reader - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TccprescR = crate::FieldReader<Tccpresc>;
impl TccprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tccpresc {
        match self.bits {
            0 => Tccpresc::Div1,
            1 => Tccpresc::Div2,
            2 => Tccpresc::Div4,
            3 => Tccpresc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Tccpresc::Div1
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Tccpresc::Div2
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Tccpresc::Div4
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Tccpresc::Div8
    }
}
#[doc = "Field `TCCPRESC` writer - Set the LFA Prescaler for Triggered Compare and Clear"]
pub type TccprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tccpresc, crate::Safe>;
impl<'a, REG> TccprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare and clear event each LFA cycle."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div1)
    }
    #[doc = "Compare and clear performed on every other LFA cycle."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div2)
    }
    #[doc = "Compare and clear performed on every 4th LFA cycle."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div4)
    }
    #[doc = "Compare and clear performed on every 8th LFA cycle."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Tccpresc::Div8)
    }
}
#[doc = "Triggered Compare and Clear Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcccomp {
    #[doc = "0: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    Ltoe = 0,
    #[doc = "1: Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    Gtoe = 1,
    #[doc = "2: Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    Range = 2,
}
impl From<Tcccomp> for u8 {
    #[inline(always)]
    fn from(variant: Tcccomp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcccomp {
    type Ux = u8;
}
impl crate::IsEnum for Tcccomp {}
#[doc = "Field `TCCCOMP` reader - Triggered Compare and Clear Compare Mode"]
pub type TcccompR = crate::FieldReader<Tcccomp>;
impl TcccompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcccomp> {
        match self.bits {
            0 => Some(Tcccomp::Ltoe),
            1 => Some(Tcccomp::Gtoe),
            2 => Some(Tcccomp::Range),
            _ => None,
        }
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_ltoe(&self) -> bool {
        *self == Tcccomp::Ltoe
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn is_gtoe(&self) -> bool {
        *self == Tcccomp::Gtoe
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == Tcccomp::Range
    }
}
#[doc = "Field `TCCCOMP` writer - Triggered Compare and Clear Compare Mode"]
pub type TcccompW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcccomp>;
impl<'a, REG> TcccompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn ltoe(self) -> &'a mut crate::W<REG> {
        self.variant(Tcccomp::Ltoe)
    }
    #[doc = "Compare match if PCNT_CNT is greater than or equal to PCNT_TOP."]
    #[inline(always)]
    pub fn gtoe(self) -> &'a mut crate::W<REG> {
        self.variant(Tcccomp::Gtoe)
    }
    #[doc = "Compare match if PCNT_CNT is less than, or equal to PCNT_TOP\\[15:8\\]\\], and greater than, or equal to PCNT_TOP\\[7:0\\]."]
    #[inline(always)]
    pub fn range(self) -> &'a mut crate::W<REG> {
        self.variant(Tcccomp::Range)
    }
}
#[doc = "Field `PRSGATEEN` reader - PRS Gate Enable"]
pub type PrsgateenR = crate::BitReader;
#[doc = "Field `PRSGATEEN` writer - PRS Gate Enable"]
pub type PrsgateenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCPRSPOL` reader - TCC PRS Polarity Select"]
pub type TccprspolR = crate::BitReader;
#[doc = "Field `TCCPRSPOL` writer - TCC PRS Polarity Select"]
pub type TccprspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TCC PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tccprssel {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected."]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected."]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected."]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected."]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected."]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected."]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected."]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected."]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected."]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected."]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected."]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected."]
    Prsch23 = 23,
}
impl From<Tccprssel> for u8 {
    #[inline(always)]
    fn from(variant: Tccprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tccprssel {
    type Ux = u8;
}
impl crate::IsEnum for Tccprssel {}
#[doc = "Field `TCCPRSSEL` reader - TCC PRS Channel Select"]
pub type TccprsselR = crate::FieldReader<Tccprssel>;
impl TccprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tccprssel> {
        match self.bits {
            0 => Some(Tccprssel::Prsch0),
            1 => Some(Tccprssel::Prsch1),
            2 => Some(Tccprssel::Prsch2),
            3 => Some(Tccprssel::Prsch3),
            4 => Some(Tccprssel::Prsch4),
            5 => Some(Tccprssel::Prsch5),
            6 => Some(Tccprssel::Prsch6),
            7 => Some(Tccprssel::Prsch7),
            8 => Some(Tccprssel::Prsch8),
            9 => Some(Tccprssel::Prsch9),
            10 => Some(Tccprssel::Prsch10),
            11 => Some(Tccprssel::Prsch11),
            12 => Some(Tccprssel::Prsch12),
            13 => Some(Tccprssel::Prsch13),
            14 => Some(Tccprssel::Prsch14),
            15 => Some(Tccprssel::Prsch15),
            16 => Some(Tccprssel::Prsch16),
            17 => Some(Tccprssel::Prsch17),
            18 => Some(Tccprssel::Prsch18),
            19 => Some(Tccprssel::Prsch19),
            20 => Some(Tccprssel::Prsch20),
            21 => Some(Tccprssel::Prsch21),
            22 => Some(Tccprssel::Prsch22),
            23 => Some(Tccprssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Tccprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Tccprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Tccprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Tccprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Tccprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Tccprssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Tccprssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Tccprssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Tccprssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Tccprssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Tccprssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Tccprssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Tccprssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Tccprssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Tccprssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Tccprssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Tccprssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Tccprssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Tccprssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Tccprssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Tccprssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Tccprssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Tccprssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Tccprssel::Prsch23
    }
}
#[doc = "Field `TCCPRSSEL` writer - TCC PRS Channel Select"]
pub type TccprsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Tccprssel>;
impl<'a, REG> TccprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Tccprssel::Prsch23)
    }
}
#[doc = "Field `TOPBHFSEL` reader - TOPB High Frequency Value Select"]
pub type TopbhfselR = crate::BitReader;
#[doc = "Field `TOPBHFSEL` writer - TOPB High Frequency Value Select"]
pub type TopbhfselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&self) -> RstenR {
        RstenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&self) -> CntrstenR {
        CntrstenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&self) -> AuxcntrstenR {
        AuxcntrstenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DebughaltR {
        DebughaltR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1cdirR {
        S1cdirR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CntevR {
        CntevR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AuxcntevR {
        AuxcntevR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&self) -> CntdirR {
        CntdirR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&self) -> TccmodeR {
        TccmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&self) -> TccprescR {
        TccprescR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&self) -> TcccompR {
        TcccompR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&self) -> PrsgateenR {
        PrsgateenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&self) -> TccprspolR {
        TccprspolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&self) -> TccprsselR {
        TccprsselR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&self) -> TopbhfselR {
        TopbhfselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FiltW<'_, CtrlSpec> {
        FiltW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RstenW<'_, CtrlSpec> {
        RstenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable CNT Reset"]
    #[inline(always)]
    pub fn cntrsten(&mut self) -> CntrstenW<'_, CtrlSpec> {
        CntrstenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrsten(&mut self) -> AuxcntrstenW<'_, CtrlSpec> {
        AuxcntrstenW::new(self, 6)
    }
    #[doc = "Bit 7 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&mut self) -> DebughaltW<'_, CtrlSpec> {
        DebughaltW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, CtrlSpec> {
        HystW::new(self, 8)
    }
    #[doc = "Bit 9 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&mut self) -> S1cdirW<'_, CtrlSpec> {
        S1cdirW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&mut self) -> CntevW<'_, CtrlSpec> {
        CntevW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Controls When the Auxiliary Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&mut self) -> AuxcntevW<'_, CtrlSpec> {
        AuxcntevW::new(self, 12)
    }
    #[doc = "Bit 14 - Non-Quadrature Mode Counter Direction Control"]
    #[inline(always)]
    pub fn cntdir(&mut self) -> CntdirW<'_, CtrlSpec> {
        CntdirW::new(self, 14)
    }
    #[doc = "Bit 15 - Edge Select"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<'_, CtrlSpec> {
        EdgeW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Sets the Mode for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccmode(&mut self) -> TccmodeW<'_, CtrlSpec> {
        TccmodeW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Set the LFA Prescaler for Triggered Compare and Clear"]
    #[inline(always)]
    pub fn tccpresc(&mut self) -> TccprescW<'_, CtrlSpec> {
        TccprescW::new(self, 19)
    }
    #[doc = "Bits 22:23 - Triggered Compare and Clear Compare Mode"]
    #[inline(always)]
    pub fn tcccomp(&mut self) -> TcccompW<'_, CtrlSpec> {
        TcccompW::new(self, 22)
    }
    #[doc = "Bit 24 - PRS Gate Enable"]
    #[inline(always)]
    pub fn prsgateen(&mut self) -> PrsgateenW<'_, CtrlSpec> {
        PrsgateenW::new(self, 24)
    }
    #[doc = "Bit 25 - TCC PRS Polarity Select"]
    #[inline(always)]
    pub fn tccprspol(&mut self) -> TccprspolW<'_, CtrlSpec> {
        TccprspolW::new(self, 25)
    }
    #[doc = "Bits 26:30 - TCC PRS Channel Select"]
    #[inline(always)]
    pub fn tccprssel(&mut self) -> TccprsselW<'_, CtrlSpec> {
        TccprsselW::new(self, 26)
    }
    #[doc = "Bit 31 - TOPB High Frequency Value Select"]
    #[inline(always)]
    pub fn topbhfsel(&mut self) -> TopbhfselW<'_, CtrlSpec> {
        TopbhfselW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
