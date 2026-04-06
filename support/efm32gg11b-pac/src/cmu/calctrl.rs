#[doc = "Register `CALCTRL` reader"]
pub type R = crate::R<CalctrlSpec>;
#[doc = "Register `CALCTRL` writer"]
pub type W = crate::W<CalctrlSpec>;
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upsel {
    #[doc = "0: Select HFXO as up-counter"]
    Hfxo = 0,
    #[doc = "1: Select LFXO as up-counter"]
    Lfxo = 1,
    #[doc = "2: Select HFRCO as up-counter"]
    Hfrco = 2,
    #[doc = "3: Select LFRCO as up-counter"]
    Lfrco = 3,
    #[doc = "4: Select AUXHFRCO as up-counter"]
    Auxhfrco = 4,
    #[doc = "5: Select PRS input selected by PRSUPSEL as up-counter"]
    Prs = 5,
    #[doc = "7: Select USHFRCO as up-counter"]
    Ushfrco = 7,
}
impl From<Upsel> for u8 {
    #[inline(always)]
    fn from(variant: Upsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upsel {
    type Ux = u8;
}
impl crate::IsEnum for Upsel {}
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub type UpselR = crate::FieldReader<Upsel>;
impl UpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Upsel> {
        match self.bits {
            0 => Some(Upsel::Hfxo),
            1 => Some(Upsel::Lfxo),
            2 => Some(Upsel::Hfrco),
            3 => Some(Upsel::Lfrco),
            4 => Some(Upsel::Auxhfrco),
            5 => Some(Upsel::Prs),
            7 => Some(Upsel::Ushfrco),
            _ => None,
        }
    }
    #[doc = "Select HFXO as up-counter"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Upsel::Hfxo
    }
    #[doc = "Select LFXO as up-counter"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Upsel::Lfxo
    }
    #[doc = "Select HFRCO as up-counter"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Upsel::Hfrco
    }
    #[doc = "Select LFRCO as up-counter"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Upsel::Lfrco
    }
    #[doc = "Select AUXHFRCO as up-counter"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Upsel::Auxhfrco
    }
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Upsel::Prs
    }
    #[doc = "Select USHFRCO as up-counter"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Upsel::Ushfrco
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub type UpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Upsel>;
impl<'a, REG> UpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFXO as up-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfxo)
    }
    #[doc = "Select LFXO as up-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Lfxo)
    }
    #[doc = "Select HFRCO as up-counter"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfrco)
    }
    #[doc = "Select LFRCO as up-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Lfrco)
    }
    #[doc = "Select AUXHFRCO as up-counter"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Auxhfrco)
    }
    #[doc = "Select PRS input selected by PRSUPSEL as up-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Prs)
    }
    #[doc = "Select USHFRCO as up-counter"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Ushfrco)
    }
}
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Downsel {
    #[doc = "0: Select HFCLK for down-counter"]
    Hfclk = 0,
    #[doc = "1: Select HFXO for down-counter"]
    Hfxo = 1,
    #[doc = "2: Select LFXO for down-counter"]
    Lfxo = 2,
    #[doc = "3: Select HFRCO for down-counter"]
    Hfrco = 3,
    #[doc = "4: Select LFRCO for down-counter"]
    Lfrco = 4,
    #[doc = "5: Select AUXHFRCO for down-counter"]
    Auxhfrco = 5,
    #[doc = "6: Select PRS input selected by PRSDOWNSEL as down-counter"]
    Prs = 6,
    #[doc = "8: Select USHFRCO for down-counter"]
    Ushfrco = 8,
}
impl From<Downsel> for u8 {
    #[inline(always)]
    fn from(variant: Downsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Downsel {
    type Ux = u8;
}
impl crate::IsEnum for Downsel {}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub type DownselR = crate::FieldReader<Downsel>;
impl DownselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Downsel> {
        match self.bits {
            0 => Some(Downsel::Hfclk),
            1 => Some(Downsel::Hfxo),
            2 => Some(Downsel::Lfxo),
            3 => Some(Downsel::Hfrco),
            4 => Some(Downsel::Lfrco),
            5 => Some(Downsel::Auxhfrco),
            6 => Some(Downsel::Prs),
            8 => Some(Downsel::Ushfrco),
            _ => None,
        }
    }
    #[doc = "Select HFCLK for down-counter"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Downsel::Hfclk
    }
    #[doc = "Select HFXO for down-counter"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Downsel::Hfxo
    }
    #[doc = "Select LFXO for down-counter"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Downsel::Lfxo
    }
    #[doc = "Select HFRCO for down-counter"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Downsel::Hfrco
    }
    #[doc = "Select LFRCO for down-counter"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Downsel::Lfrco
    }
    #[doc = "Select AUXHFRCO for down-counter"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Downsel::Auxhfrco
    }
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Downsel::Prs
    }
    #[doc = "Select USHFRCO for down-counter"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Downsel::Ushfrco
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub type DownselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Downsel>;
impl<'a, REG> DownselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HFCLK for down-counter"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfclk)
    }
    #[doc = "Select HFXO for down-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfxo)
    }
    #[doc = "Select LFXO for down-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Lfxo)
    }
    #[doc = "Select HFRCO for down-counter"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfrco)
    }
    #[doc = "Select LFRCO for down-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Lfrco)
    }
    #[doc = "Select AUXHFRCO for down-counter"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Auxhfrco)
    }
    #[doc = "Select PRS input selected by PRSDOWNSEL as down-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Prs)
    }
    #[doc = "Select USHFRCO for down-counter"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Ushfrco)
    }
}
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PRS Select for PRS Input When Selected in UPSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsupsel {
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
impl From<Prsupsel> for u8 {
    #[inline(always)]
    fn from(variant: Prsupsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsupsel {
    type Ux = u8;
}
impl crate::IsEnum for Prsupsel {}
#[doc = "Field `PRSUPSEL` reader - PRS Select for PRS Input When Selected in UPSEL"]
pub type PrsupselR = crate::FieldReader<Prsupsel>;
impl PrsupselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prsupsel> {
        match self.bits {
            0 => Some(Prsupsel::Prsch0),
            1 => Some(Prsupsel::Prsch1),
            2 => Some(Prsupsel::Prsch2),
            3 => Some(Prsupsel::Prsch3),
            4 => Some(Prsupsel::Prsch4),
            5 => Some(Prsupsel::Prsch5),
            6 => Some(Prsupsel::Prsch6),
            7 => Some(Prsupsel::Prsch7),
            8 => Some(Prsupsel::Prsch8),
            9 => Some(Prsupsel::Prsch9),
            10 => Some(Prsupsel::Prsch10),
            11 => Some(Prsupsel::Prsch11),
            12 => Some(Prsupsel::Prsch12),
            13 => Some(Prsupsel::Prsch13),
            14 => Some(Prsupsel::Prsch14),
            15 => Some(Prsupsel::Prsch15),
            16 => Some(Prsupsel::Prsch16),
            17 => Some(Prsupsel::Prsch17),
            18 => Some(Prsupsel::Prsch18),
            19 => Some(Prsupsel::Prsch19),
            20 => Some(Prsupsel::Prsch20),
            21 => Some(Prsupsel::Prsch21),
            22 => Some(Prsupsel::Prsch22),
            23 => Some(Prsupsel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prsupsel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prsupsel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prsupsel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prsupsel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prsupsel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prsupsel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prsupsel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prsupsel::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prsupsel::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prsupsel::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prsupsel::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prsupsel::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prsupsel::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prsupsel::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prsupsel::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prsupsel::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prsupsel::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prsupsel::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prsupsel::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prsupsel::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prsupsel::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prsupsel::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prsupsel::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prsupsel::Prsch23
    }
}
#[doc = "Field `PRSUPSEL` writer - PRS Select for PRS Input When Selected in UPSEL"]
pub type PrsupselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prsupsel>;
impl<'a, REG> PrsupselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prsupsel::Prsch23)
    }
}
#[doc = "PRS Select for PRS Input When Selected in DOWNSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsdownsel {
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
impl From<Prsdownsel> for u8 {
    #[inline(always)]
    fn from(variant: Prsdownsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsdownsel {
    type Ux = u8;
}
impl crate::IsEnum for Prsdownsel {}
#[doc = "Field `PRSDOWNSEL` reader - PRS Select for PRS Input When Selected in DOWNSEL"]
pub type PrsdownselR = crate::FieldReader<Prsdownsel>;
impl PrsdownselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prsdownsel> {
        match self.bits {
            0 => Some(Prsdownsel::Prsch0),
            1 => Some(Prsdownsel::Prsch1),
            2 => Some(Prsdownsel::Prsch2),
            3 => Some(Prsdownsel::Prsch3),
            4 => Some(Prsdownsel::Prsch4),
            5 => Some(Prsdownsel::Prsch5),
            6 => Some(Prsdownsel::Prsch6),
            7 => Some(Prsdownsel::Prsch7),
            8 => Some(Prsdownsel::Prsch8),
            9 => Some(Prsdownsel::Prsch9),
            10 => Some(Prsdownsel::Prsch10),
            11 => Some(Prsdownsel::Prsch11),
            12 => Some(Prsdownsel::Prsch12),
            13 => Some(Prsdownsel::Prsch13),
            14 => Some(Prsdownsel::Prsch14),
            15 => Some(Prsdownsel::Prsch15),
            16 => Some(Prsdownsel::Prsch16),
            17 => Some(Prsdownsel::Prsch17),
            18 => Some(Prsdownsel::Prsch18),
            19 => Some(Prsdownsel::Prsch19),
            20 => Some(Prsdownsel::Prsch20),
            21 => Some(Prsdownsel::Prsch21),
            22 => Some(Prsdownsel::Prsch22),
            23 => Some(Prsdownsel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prsdownsel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prsdownsel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prsdownsel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prsdownsel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prsdownsel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prsdownsel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prsdownsel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prsdownsel::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prsdownsel::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prsdownsel::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prsdownsel::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prsdownsel::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prsdownsel::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prsdownsel::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prsdownsel::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prsdownsel::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prsdownsel::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prsdownsel::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prsdownsel::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prsdownsel::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prsdownsel::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prsdownsel::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prsdownsel::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prsdownsel::Prsch23
    }
}
#[doc = "Field `PRSDOWNSEL` writer - PRS Select for PRS Input When Selected in DOWNSEL"]
pub type PrsdownselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prsdownsel>;
impl<'a, REG> PrsdownselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prsdownsel::Prsch23)
    }
}
impl R {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UpselR {
        UpselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DownselR {
        DownselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline(always)]
    pub fn prsupsel(&self) -> PrsupselR {
        PrsupselR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline(always)]
    pub fn prsdownsel(&self) -> PrsdownselR {
        PrsdownselR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&mut self) -> UpselW<'_, CalctrlSpec> {
        UpselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&mut self) -> DownselW<'_, CalctrlSpec> {
        DownselW::new(self, 4)
    }
    #[doc = "Bit 8 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, CalctrlSpec> {
        ContW::new(self, 8)
    }
    #[doc = "Bits 16:20 - PRS Select for PRS Input When Selected in UPSEL"]
    #[inline(always)]
    pub fn prsupsel(&mut self) -> PrsupselW<'_, CalctrlSpec> {
        PrsupselW::new(self, 16)
    }
    #[doc = "Bits 24:28 - PRS Select for PRS Input When Selected in DOWNSEL"]
    #[inline(always)]
    pub fn prsdownsel(&mut self) -> PrsdownselW<'_, CalctrlSpec> {
        PrsdownselW::new(self, 24)
    }
}
#[doc = "Calibration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctrlSpec;
impl crate::RegisterSpec for CalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctrl::R`](R) reader structure"]
impl crate::Readable for CalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`calctrl::W`](W) writer structure"]
impl crate::Writable for CalctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CalctrlSpec {}
