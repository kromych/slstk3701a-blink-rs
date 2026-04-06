#[doc = "Register `CC1_CTRL` reader"]
pub type R = crate::R<Cc1CtrlSpec>;
#[doc = "Register `CC1_CTRL` writer"]
pub type W = crate::W<Cc1CtrlSpec>;
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Compare/Capture channel turned off"]
    Off = 0,
    #[doc = "1: Input capture"]
    Inputcapture = 1,
    #[doc = "2: Output compare"]
    Outputcompare = 2,
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
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Off),
            1 => Some(Mode::Inputcapture),
            2 => Some(Mode::Outputcompare),
            _ => None,
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == Mode::Inputcapture
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == Mode::Outputcompare
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Inputcapture)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Outputcompare)
    }
}
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmoa {
    #[doc = "0: A single clock cycle pulse is generated on output"]
    Pulse = 0,
    #[doc = "1: Toggle output on compare match"]
    Toggle = 1,
    #[doc = "2: Clear output on compare match"]
    Clear = 2,
    #[doc = "3: Set output on compare match"]
    Set = 3,
}
impl From<Cmoa> for u8 {
    #[inline(always)]
    fn from(variant: Cmoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmoa {
    type Ux = u8;
}
impl crate::IsEnum for Cmoa {}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CmoaR = crate::FieldReader<Cmoa>;
impl CmoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmoa {
        match self.bits {
            0 => Cmoa::Pulse,
            1 => Cmoa::Toggle,
            2 => Cmoa::Clear,
            3 => Cmoa::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Cmoa::Pulse
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cmoa::Toggle
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cmoa::Clear
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cmoa::Set
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CmoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmoa, crate::Safe>;
impl<'a, REG> CmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Pulse)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Toggle)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Clear)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Set)
    }
}
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icedge {
    #[doc = "0: Rising edges detected"]
    Rising = 0,
    #[doc = "1: Falling edges detected"]
    Falling = 1,
    #[doc = "2: Both edges detected"]
    Both = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    None = 3,
}
impl From<Icedge> for u8 {
    #[inline(always)]
    fn from(variant: Icedge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Icedge {
    type Ux = u8;
}
impl crate::IsEnum for Icedge {}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type IcedgeR = crate::FieldReader<Icedge>;
impl IcedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icedge {
        match self.bits {
            0 => Icedge::Rising,
            1 => Icedge::Falling,
            2 => Icedge::Both,
            3 => Icedge::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Icedge::Rising
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Icedge::Falling
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Icedge::Both
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Icedge::None
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type IcedgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Icedge, crate::Safe>;
impl<'a, REG> IcedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Rising)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Falling)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Both)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::None)
    }
}
#[doc = "Compare/Capture Channel PRS Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS Channel 0 selected as input"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as input"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as input"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as input"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as input"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as input"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as input"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as input"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as input"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as input"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as input"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as input"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as input"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as input"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as input"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as input"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as input"]
    Prsch23 = 23,
}
impl From<Prssel> for u8 {
    #[inline(always)]
    fn from(variant: Prssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prssel {
    type Ux = u8;
}
impl crate::IsEnum for Prssel {}
#[doc = "Field `PRSSEL` reader - Compare/Capture Channel PRS Input Channel Selection"]
pub type PrsselR = crate::FieldReader<Prssel>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prssel> {
        match self.bits {
            0 => Some(Prssel::Prsch0),
            1 => Some(Prssel::Prsch1),
            2 => Some(Prssel::Prsch2),
            3 => Some(Prssel::Prsch3),
            4 => Some(Prssel::Prsch4),
            5 => Some(Prssel::Prsch5),
            6 => Some(Prssel::Prsch6),
            7 => Some(Prssel::Prsch7),
            8 => Some(Prssel::Prsch8),
            9 => Some(Prssel::Prsch9),
            10 => Some(Prssel::Prsch10),
            11 => Some(Prssel::Prsch11),
            12 => Some(Prssel::Prsch12),
            13 => Some(Prssel::Prsch13),
            14 => Some(Prssel::Prsch14),
            15 => Some(Prssel::Prsch15),
            16 => Some(Prssel::Prsch16),
            17 => Some(Prssel::Prsch17),
            18 => Some(Prssel::Prsch18),
            19 => Some(Prssel::Prsch19),
            20 => Some(Prssel::Prsch20),
            21 => Some(Prssel::Prsch21),
            22 => Some(Prssel::Prsch22),
            23 => Some(Prssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel::Prsch23
    }
}
#[doc = "Field `PRSSEL` writer - Compare/Capture Channel PRS Input Channel Selection"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch23)
    }
}
#[doc = "Field `COMPBASE` reader - Capture Compare Channel Comparison Base"]
pub type CompbaseR = crate::BitReader;
#[doc = "Field `COMPBASE` writer - Capture Compare Channel Comparison Base"]
pub type CompbaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPMASK` reader - Capture Compare Channel Comparison Mask"]
pub type CompmaskR = crate::FieldReader;
#[doc = "Field `COMPMASK` writer - Capture Compare Channel Comparison Mask"]
pub type CompmaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAYCC` reader - Day Capture/Compare Selection"]
pub type DayccR = crate::BitReader;
#[doc = "Field `DAYCC` writer - Day Capture/Compare Selection"]
pub type DayccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CmoaR {
        CmoaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> IcedgeR {
        IcedgeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:10 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Capture Compare Channel Comparison Base"]
    #[inline(always)]
    pub fn compbase(&self) -> CompbaseR {
        CompbaseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - Capture Compare Channel Comparison Mask"]
    #[inline(always)]
    pub fn compmask(&self) -> CompmaskR {
        CompmaskR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Day Capture/Compare Selection"]
    #[inline(always)]
    pub fn daycc(&self) -> DayccR {
        DayccR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cc1CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&mut self) -> CmoaW<'_, Cc1CtrlSpec> {
        CmoaW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&mut self) -> IcedgeW<'_, Cc1CtrlSpec> {
        IcedgeW::new(self, 4)
    }
    #[doc = "Bits 6:10 - Compare/Capture Channel PRS Input Channel Selection"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, Cc1CtrlSpec> {
        PrsselW::new(self, 6)
    }
    #[doc = "Bit 11 - Capture Compare Channel Comparison Base"]
    #[inline(always)]
    pub fn compbase(&mut self) -> CompbaseW<'_, Cc1CtrlSpec> {
        CompbaseW::new(self, 11)
    }
    #[doc = "Bits 12:16 - Capture Compare Channel Comparison Mask"]
    #[inline(always)]
    pub fn compmask(&mut self) -> CompmaskW<'_, Cc1CtrlSpec> {
        CompmaskW::new(self, 12)
    }
    #[doc = "Bit 17 - Day Capture/Compare Selection"]
    #[inline(always)]
    pub fn daycc(&mut self) -> DayccW<'_, Cc1CtrlSpec> {
        DayccW::new(self, 17)
    }
}
#[doc = "CC Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1CtrlSpec;
impl crate::RegisterSpec for Cc1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ctrl::R`](R) reader structure"]
impl crate::Readable for Cc1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cc1_ctrl::W`](W) writer structure"]
impl crate::Writable for Cc1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC1_CTRL to value 0"]
impl crate::Resettable for Cc1CtrlSpec {}
