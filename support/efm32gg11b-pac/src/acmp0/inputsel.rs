#[doc = "Register `INPUTSEL` reader"]
pub type R = crate::R<InputselSpec>;
#[doc = "Register `INPUTSEL` writer"]
pub type W = crate::W<InputselSpec>;
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub type PosselR = crate::FieldReader;
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub type PosselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub type NegselR = crate::FieldReader;
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub type NegselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "VA Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vasel {
    #[doc = "0: ACMPVDD"]
    Vdd = 0,
    #[doc = "1: APORT2Y Channel 0"]
    Aport2ych0 = 1,
    #[doc = "3: APORT2Y Channel 2"]
    Aport2ych2 = 3,
    #[doc = "5: APORT2Y Channel 4"]
    Aport2ych4 = 5,
    #[doc = "7: APORT2Y Channel 6"]
    Aport2ych6 = 7,
    #[doc = "9: APORT2Y Channel 8"]
    Aport2ych8 = 9,
    #[doc = "11: APORT2Y Channel 10"]
    Aport2ych10 = 11,
    #[doc = "13: APORT2Y Channel 12"]
    Aport2ych12 = 13,
    #[doc = "15: APORT2Y Channel 14"]
    Aport2ych14 = 15,
    #[doc = "17: APORT2Y Channel 16"]
    Aport2ych16 = 17,
    #[doc = "19: APORT2Y Channel 18"]
    Aport2ych18 = 19,
    #[doc = "21: APORT2Y Channel 20"]
    Aport2ych20 = 21,
    #[doc = "23: APORT2Y Channel 22"]
    Aport2ych22 = 23,
    #[doc = "25: APORT2Y Channel 24"]
    Aport2ych24 = 25,
    #[doc = "27: APORT2Y Channel 26"]
    Aport2ych26 = 27,
    #[doc = "29: APORT2Y Channel 28"]
    Aport2ych28 = 29,
    #[doc = "31: APORT2Y Channel 30"]
    Aport2ych30 = 31,
    #[doc = "32: APORT1X Channel 0"]
    Aport1xch0 = 32,
    #[doc = "33: APORT1Y Channel 1"]
    Aport1ych1 = 33,
    #[doc = "34: APORT1X Channel 2"]
    Aport1xch2 = 34,
    #[doc = "35: APORT1Y Channel 3"]
    Aport1ych3 = 35,
    #[doc = "36: APORT1X Channel 4"]
    Aport1xch4 = 36,
    #[doc = "37: APORT1Y Channel 5"]
    Aport1ych5 = 37,
    #[doc = "38: APORT1X Channel 6"]
    Aport1xch6 = 38,
    #[doc = "39: APORT1Y Channel 7"]
    Aport1ych7 = 39,
    #[doc = "40: APORT1X Channel 8"]
    Aport1xch8 = 40,
    #[doc = "41: APORT1Y Channel 9"]
    Aport1ych9 = 41,
    #[doc = "42: APORT1X Channel 10"]
    Aport1xch10 = 42,
    #[doc = "43: APORT1Y Channel 11"]
    Aport1ych11 = 43,
    #[doc = "44: APORT1X Channel 12"]
    Aport1xch12 = 44,
    #[doc = "45: APORT1Y Channel 13"]
    Aport1ych13 = 45,
    #[doc = "46: APORT1X Channel 14"]
    Aport1xch14 = 46,
    #[doc = "47: APORT1Y Channel 15"]
    Aport1ych15 = 47,
    #[doc = "48: APORT1X Channel 16"]
    Aport1xch16 = 48,
    #[doc = "49: APORT1Y Channel 17"]
    Aport1ych17 = 49,
    #[doc = "50: APORT1X Channel 18"]
    Aport1xch18 = 50,
    #[doc = "51: APORT1Y Channel 19"]
    Aport1ych19 = 51,
    #[doc = "52: APORT1X Channel 20"]
    Aport1xch20 = 52,
    #[doc = "53: APORT1Y Channel 21"]
    Aport1ych21 = 53,
    #[doc = "54: APORT1X Channel 22"]
    Aport1xch22 = 54,
    #[doc = "55: APORT1Y Channel 23"]
    Aport1ych23 = 55,
    #[doc = "56: APORT1X Channel 24"]
    Aport1xch24 = 56,
    #[doc = "57: APORT1Y Channel 25"]
    Aport1ych25 = 57,
    #[doc = "58: APORT1X Channel 26"]
    Aport1xch26 = 58,
    #[doc = "59: APORT1Y Channel 27"]
    Aport1ych27 = 59,
    #[doc = "60: APORT1X Channel 28"]
    Aport1xch28 = 60,
    #[doc = "61: APORT1Y Channel 29"]
    Aport1ych29 = 61,
    #[doc = "62: APORT1X Channel 30"]
    Aport1xch30 = 62,
    #[doc = "63: APORT1Y Channel 31"]
    Aport1ych31 = 63,
}
impl From<Vasel> for u8 {
    #[inline(always)]
    fn from(variant: Vasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vasel {
    type Ux = u8;
}
impl crate::IsEnum for Vasel {}
#[doc = "Field `VASEL` reader - VA Selection"]
pub type VaselR = crate::FieldReader<Vasel>;
impl VaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vasel> {
        match self.bits {
            0 => Some(Vasel::Vdd),
            1 => Some(Vasel::Aport2ych0),
            3 => Some(Vasel::Aport2ych2),
            5 => Some(Vasel::Aport2ych4),
            7 => Some(Vasel::Aport2ych6),
            9 => Some(Vasel::Aport2ych8),
            11 => Some(Vasel::Aport2ych10),
            13 => Some(Vasel::Aport2ych12),
            15 => Some(Vasel::Aport2ych14),
            17 => Some(Vasel::Aport2ych16),
            19 => Some(Vasel::Aport2ych18),
            21 => Some(Vasel::Aport2ych20),
            23 => Some(Vasel::Aport2ych22),
            25 => Some(Vasel::Aport2ych24),
            27 => Some(Vasel::Aport2ych26),
            29 => Some(Vasel::Aport2ych28),
            31 => Some(Vasel::Aport2ych30),
            32 => Some(Vasel::Aport1xch0),
            33 => Some(Vasel::Aport1ych1),
            34 => Some(Vasel::Aport1xch2),
            35 => Some(Vasel::Aport1ych3),
            36 => Some(Vasel::Aport1xch4),
            37 => Some(Vasel::Aport1ych5),
            38 => Some(Vasel::Aport1xch6),
            39 => Some(Vasel::Aport1ych7),
            40 => Some(Vasel::Aport1xch8),
            41 => Some(Vasel::Aport1ych9),
            42 => Some(Vasel::Aport1xch10),
            43 => Some(Vasel::Aport1ych11),
            44 => Some(Vasel::Aport1xch12),
            45 => Some(Vasel::Aport1ych13),
            46 => Some(Vasel::Aport1xch14),
            47 => Some(Vasel::Aport1ych15),
            48 => Some(Vasel::Aport1xch16),
            49 => Some(Vasel::Aport1ych17),
            50 => Some(Vasel::Aport1xch18),
            51 => Some(Vasel::Aport1ych19),
            52 => Some(Vasel::Aport1xch20),
            53 => Some(Vasel::Aport1ych21),
            54 => Some(Vasel::Aport1xch22),
            55 => Some(Vasel::Aport1ych23),
            56 => Some(Vasel::Aport1xch24),
            57 => Some(Vasel::Aport1ych25),
            58 => Some(Vasel::Aport1xch26),
            59 => Some(Vasel::Aport1ych27),
            60 => Some(Vasel::Aport1xch28),
            61 => Some(Vasel::Aport1ych29),
            62 => Some(Vasel::Aport1xch30),
            63 => Some(Vasel::Aport1ych31),
            _ => None,
        }
    }
    #[doc = "ACMPVDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Vasel::Vdd
    }
    #[doc = "APORT2Y Channel 0"]
    #[inline(always)]
    pub fn is_aport2ych0(&self) -> bool {
        *self == Vasel::Aport2ych0
    }
    #[doc = "APORT2Y Channel 2"]
    #[inline(always)]
    pub fn is_aport2ych2(&self) -> bool {
        *self == Vasel::Aport2ych2
    }
    #[doc = "APORT2Y Channel 4"]
    #[inline(always)]
    pub fn is_aport2ych4(&self) -> bool {
        *self == Vasel::Aport2ych4
    }
    #[doc = "APORT2Y Channel 6"]
    #[inline(always)]
    pub fn is_aport2ych6(&self) -> bool {
        *self == Vasel::Aport2ych6
    }
    #[doc = "APORT2Y Channel 8"]
    #[inline(always)]
    pub fn is_aport2ych8(&self) -> bool {
        *self == Vasel::Aport2ych8
    }
    #[doc = "APORT2Y Channel 10"]
    #[inline(always)]
    pub fn is_aport2ych10(&self) -> bool {
        *self == Vasel::Aport2ych10
    }
    #[doc = "APORT2Y Channel 12"]
    #[inline(always)]
    pub fn is_aport2ych12(&self) -> bool {
        *self == Vasel::Aport2ych12
    }
    #[doc = "APORT2Y Channel 14"]
    #[inline(always)]
    pub fn is_aport2ych14(&self) -> bool {
        *self == Vasel::Aport2ych14
    }
    #[doc = "APORT2Y Channel 16"]
    #[inline(always)]
    pub fn is_aport2ych16(&self) -> bool {
        *self == Vasel::Aport2ych16
    }
    #[doc = "APORT2Y Channel 18"]
    #[inline(always)]
    pub fn is_aport2ych18(&self) -> bool {
        *self == Vasel::Aport2ych18
    }
    #[doc = "APORT2Y Channel 20"]
    #[inline(always)]
    pub fn is_aport2ych20(&self) -> bool {
        *self == Vasel::Aport2ych20
    }
    #[doc = "APORT2Y Channel 22"]
    #[inline(always)]
    pub fn is_aport2ych22(&self) -> bool {
        *self == Vasel::Aport2ych22
    }
    #[doc = "APORT2Y Channel 24"]
    #[inline(always)]
    pub fn is_aport2ych24(&self) -> bool {
        *self == Vasel::Aport2ych24
    }
    #[doc = "APORT2Y Channel 26"]
    #[inline(always)]
    pub fn is_aport2ych26(&self) -> bool {
        *self == Vasel::Aport2ych26
    }
    #[doc = "APORT2Y Channel 28"]
    #[inline(always)]
    pub fn is_aport2ych28(&self) -> bool {
        *self == Vasel::Aport2ych28
    }
    #[doc = "APORT2Y Channel 30"]
    #[inline(always)]
    pub fn is_aport2ych30(&self) -> bool {
        *self == Vasel::Aport2ych30
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == Vasel::Aport1xch0
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == Vasel::Aport1ych1
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == Vasel::Aport1xch2
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == Vasel::Aport1ych3
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == Vasel::Aport1xch4
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == Vasel::Aport1ych5
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == Vasel::Aport1xch6
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == Vasel::Aport1ych7
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == Vasel::Aport1xch8
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == Vasel::Aport1ych9
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == Vasel::Aport1xch10
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == Vasel::Aport1ych11
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == Vasel::Aport1xch12
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == Vasel::Aport1ych13
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == Vasel::Aport1xch14
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == Vasel::Aport1ych15
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == Vasel::Aport1xch16
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == Vasel::Aport1ych17
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == Vasel::Aport1xch18
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == Vasel::Aport1ych19
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == Vasel::Aport1xch20
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == Vasel::Aport1ych21
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == Vasel::Aport1xch22
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == Vasel::Aport1ych23
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == Vasel::Aport1xch24
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == Vasel::Aport1ych25
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == Vasel::Aport1xch26
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == Vasel::Aport1ych27
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == Vasel::Aport1xch28
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == Vasel::Aport1ych29
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == Vasel::Aport1xch30
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == Vasel::Aport1ych31
    }
}
#[doc = "Field `VASEL` writer - VA Selection"]
pub type VaselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Vasel>;
impl<'a, REG> VaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACMPVDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Vdd)
    }
    #[doc = "APORT2Y Channel 0"]
    #[inline(always)]
    pub fn aport2ych0(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych0)
    }
    #[doc = "APORT2Y Channel 2"]
    #[inline(always)]
    pub fn aport2ych2(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych2)
    }
    #[doc = "APORT2Y Channel 4"]
    #[inline(always)]
    pub fn aport2ych4(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych4)
    }
    #[doc = "APORT2Y Channel 6"]
    #[inline(always)]
    pub fn aport2ych6(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych6)
    }
    #[doc = "APORT2Y Channel 8"]
    #[inline(always)]
    pub fn aport2ych8(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych8)
    }
    #[doc = "APORT2Y Channel 10"]
    #[inline(always)]
    pub fn aport2ych10(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych10)
    }
    #[doc = "APORT2Y Channel 12"]
    #[inline(always)]
    pub fn aport2ych12(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych12)
    }
    #[doc = "APORT2Y Channel 14"]
    #[inline(always)]
    pub fn aport2ych14(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych14)
    }
    #[doc = "APORT2Y Channel 16"]
    #[inline(always)]
    pub fn aport2ych16(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych16)
    }
    #[doc = "APORT2Y Channel 18"]
    #[inline(always)]
    pub fn aport2ych18(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych18)
    }
    #[doc = "APORT2Y Channel 20"]
    #[inline(always)]
    pub fn aport2ych20(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych20)
    }
    #[doc = "APORT2Y Channel 22"]
    #[inline(always)]
    pub fn aport2ych22(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych22)
    }
    #[doc = "APORT2Y Channel 24"]
    #[inline(always)]
    pub fn aport2ych24(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych24)
    }
    #[doc = "APORT2Y Channel 26"]
    #[inline(always)]
    pub fn aport2ych26(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych26)
    }
    #[doc = "APORT2Y Channel 28"]
    #[inline(always)]
    pub fn aport2ych28(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych28)
    }
    #[doc = "APORT2Y Channel 30"]
    #[inline(always)]
    pub fn aport2ych30(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport2ych30)
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1xch30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut crate::W<REG> {
        self.variant(Vasel::Aport1ych31)
    }
}
#[doc = "Field `VBSEL` reader - VB Selection"]
pub type VbselR = crate::BitReader;
#[doc = "Field `VBSEL` writer - VB Selection"]
pub type VbselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLPSEL` reader - Low-Power Sampled Voltage Selection"]
pub type VlpselR = crate::BitReader;
#[doc = "Field `VLPSEL` writer - Low-Power Sampled Voltage Selection"]
pub type VlpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRESEN` reader - Capacitive Sense Mode Internal Resistor Enable"]
pub type CsresenR = crate::BitReader;
#[doc = "Field `CSRESEN` writer - Capacitive Sense Mode Internal Resistor Enable"]
pub type CsresenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csressel {
    #[doc = "0: Internal capacitive sense resistor value 0"]
    Res0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1"]
    Res1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2"]
    Res2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3"]
    Res3 = 3,
    #[doc = "4: Internal capacitive sense resistor value 4"]
    Res4 = 4,
    #[doc = "5: Internal capacitive sense resistor value 5"]
    Res5 = 5,
    #[doc = "6: Internal capacitive sense resistor value 6"]
    Res6 = 6,
    #[doc = "7: Internal capacitive sense resistor value 7"]
    Res7 = 7,
}
impl From<Csressel> for u8 {
    #[inline(always)]
    fn from(variant: Csressel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csressel {
    type Ux = u8;
}
impl crate::IsEnum for Csressel {}
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor Select"]
pub type CsresselR = crate::FieldReader<Csressel>;
impl CsresselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csressel {
        match self.bits {
            0 => Csressel::Res0,
            1 => Csressel::Res1,
            2 => Csressel::Res2,
            3 => Csressel::Res3,
            4 => Csressel::Res4,
            5 => Csressel::Res5,
            6 => Csressel::Res6,
            7 => Csressel::Res7,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == Csressel::Res0
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == Csressel::Res1
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == Csressel::Res2
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == Csressel::Res3
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == Csressel::Res4
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == Csressel::Res5
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == Csressel::Res6
    }
    #[doc = "Internal capacitive sense resistor value 7"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == Csressel::Res7
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor Select"]
pub type CsresselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Csressel, crate::Safe>;
impl<'a, REG> CsresselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res0)
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res1)
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res2)
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res3)
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res4)
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res5)
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res6)
    }
    #[doc = "Internal capacitive sense resistor value 7"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res7)
    }
}
impl R {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> PosselR {
        PosselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NegselR {
        NegselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline(always)]
    pub fn vasel(&self) -> VaselR {
        VaselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline(always)]
    pub fn vbsel(&self) -> VbselR {
        VbselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline(always)]
    pub fn vlpsel(&self) -> VlpselR {
        VlpselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CsresenR {
        CsresenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CsresselR {
        CsresselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&mut self) -> PosselW<'_, InputselSpec> {
        PosselW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NegselW<'_, InputselSpec> {
        NegselW::new(self, 8)
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline(always)]
    pub fn vasel(&mut self) -> VaselW<'_, InputselSpec> {
        VaselW::new(self, 16)
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline(always)]
    pub fn vbsel(&mut self) -> VbselW<'_, InputselSpec> {
        VbselW::new(self, 22)
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline(always)]
    pub fn vlpsel(&mut self) -> VlpselW<'_, InputselSpec> {
        VlpselW::new(self, 24)
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&mut self) -> CsresenW<'_, InputselSpec> {
        CsresenW::new(self, 26)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&mut self) -> CsresselW<'_, InputselSpec> {
        CsresselW::new(self, 28)
    }
}
#[doc = "Input Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inputsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputselSpec;
impl crate::RegisterSpec for InputselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputsel::R`](R) reader structure"]
impl crate::Readable for InputselSpec {}
#[doc = "`write(|w| ..)` method takes [`inputsel::W`](W) writer structure"]
impl crate::Writable for InputselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTSEL to value 0"]
impl crate::Resettable for InputselSpec {}
