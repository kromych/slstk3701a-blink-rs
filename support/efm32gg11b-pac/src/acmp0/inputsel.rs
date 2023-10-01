#[doc = "Register `INPUTSEL` reader"]
pub type R = crate::R<INPUTSEL_SPEC>;
#[doc = "Register `INPUTSEL` writer"]
pub type W = crate::W<INPUTSEL_SPEC>;
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub type POSSEL_R = crate::FieldReader;
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub type POSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub type NEGSEL_R = crate::FieldReader;
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub type NEGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VASEL` reader - VA Selection"]
pub type VASEL_R = crate::FieldReader<VASEL_A>;
#[doc = "VA Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VASEL_A {
    #[doc = "0: ACMPVDD"]
    VDD = 0,
    #[doc = "1: APORT2Y Channel 0"]
    APORT2YCH0 = 1,
    #[doc = "3: APORT2Y Channel 2"]
    APORT2YCH2 = 3,
    #[doc = "5: APORT2Y Channel 4"]
    APORT2YCH4 = 5,
    #[doc = "7: APORT2Y Channel 6"]
    APORT2YCH6 = 7,
    #[doc = "9: APORT2Y Channel 8"]
    APORT2YCH8 = 9,
    #[doc = "11: APORT2Y Channel 10"]
    APORT2YCH10 = 11,
    #[doc = "13: APORT2Y Channel 12"]
    APORT2YCH12 = 13,
    #[doc = "15: APORT2Y Channel 14"]
    APORT2YCH14 = 15,
    #[doc = "17: APORT2Y Channel 16"]
    APORT2YCH16 = 17,
    #[doc = "19: APORT2Y Channel 18"]
    APORT2YCH18 = 19,
    #[doc = "21: APORT2Y Channel 20"]
    APORT2YCH20 = 21,
    #[doc = "23: APORT2Y Channel 22"]
    APORT2YCH22 = 23,
    #[doc = "25: APORT2Y Channel 24"]
    APORT2YCH24 = 25,
    #[doc = "27: APORT2Y Channel 26"]
    APORT2YCH26 = 27,
    #[doc = "29: APORT2Y Channel 28"]
    APORT2YCH28 = 29,
    #[doc = "31: APORT2Y Channel 30"]
    APORT2YCH30 = 31,
    #[doc = "32: APORT1X Channel 0"]
    APORT1XCH0 = 32,
    #[doc = "33: APORT1Y Channel 1"]
    APORT1YCH1 = 33,
    #[doc = "34: APORT1X Channel 2"]
    APORT1XCH2 = 34,
    #[doc = "35: APORT1Y Channel 3"]
    APORT1YCH3 = 35,
    #[doc = "36: APORT1X Channel 4"]
    APORT1XCH4 = 36,
    #[doc = "37: APORT1Y Channel 5"]
    APORT1YCH5 = 37,
    #[doc = "38: APORT1X Channel 6"]
    APORT1XCH6 = 38,
    #[doc = "39: APORT1Y Channel 7"]
    APORT1YCH7 = 39,
    #[doc = "40: APORT1X Channel 8"]
    APORT1XCH8 = 40,
    #[doc = "41: APORT1Y Channel 9"]
    APORT1YCH9 = 41,
    #[doc = "42: APORT1X Channel 10"]
    APORT1XCH10 = 42,
    #[doc = "43: APORT1Y Channel 11"]
    APORT1YCH11 = 43,
    #[doc = "44: APORT1X Channel 12"]
    APORT1XCH12 = 44,
    #[doc = "45: APORT1Y Channel 13"]
    APORT1YCH13 = 45,
    #[doc = "46: APORT1X Channel 14"]
    APORT1XCH14 = 46,
    #[doc = "47: APORT1Y Channel 15"]
    APORT1YCH15 = 47,
    #[doc = "48: APORT1X Channel 16"]
    APORT1XCH16 = 48,
    #[doc = "49: APORT1Y Channel 17"]
    APORT1YCH17 = 49,
    #[doc = "50: APORT1X Channel 18"]
    APORT1XCH18 = 50,
    #[doc = "51: APORT1Y Channel 19"]
    APORT1YCH19 = 51,
    #[doc = "52: APORT1X Channel 20"]
    APORT1XCH20 = 52,
    #[doc = "53: APORT1Y Channel 21"]
    APORT1YCH21 = 53,
    #[doc = "54: APORT1X Channel 22"]
    APORT1XCH22 = 54,
    #[doc = "55: APORT1Y Channel 23"]
    APORT1YCH23 = 55,
    #[doc = "56: APORT1X Channel 24"]
    APORT1XCH24 = 56,
    #[doc = "57: APORT1Y Channel 25"]
    APORT1YCH25 = 57,
    #[doc = "58: APORT1X Channel 26"]
    APORT1XCH26 = 58,
    #[doc = "59: APORT1Y Channel 27"]
    APORT1YCH27 = 59,
    #[doc = "60: APORT1X Channel 28"]
    APORT1XCH28 = 60,
    #[doc = "61: APORT1Y Channel 29"]
    APORT1YCH29 = 61,
    #[doc = "62: APORT1X Channel 30"]
    APORT1XCH30 = 62,
    #[doc = "63: APORT1Y Channel 31"]
    APORT1YCH31 = 63,
}
impl From<VASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VASEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VASEL_A {
    type Ux = u8;
}
impl VASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VASEL_A> {
        match self.bits {
            0 => Some(VASEL_A::VDD),
            1 => Some(VASEL_A::APORT2YCH0),
            3 => Some(VASEL_A::APORT2YCH2),
            5 => Some(VASEL_A::APORT2YCH4),
            7 => Some(VASEL_A::APORT2YCH6),
            9 => Some(VASEL_A::APORT2YCH8),
            11 => Some(VASEL_A::APORT2YCH10),
            13 => Some(VASEL_A::APORT2YCH12),
            15 => Some(VASEL_A::APORT2YCH14),
            17 => Some(VASEL_A::APORT2YCH16),
            19 => Some(VASEL_A::APORT2YCH18),
            21 => Some(VASEL_A::APORT2YCH20),
            23 => Some(VASEL_A::APORT2YCH22),
            25 => Some(VASEL_A::APORT2YCH24),
            27 => Some(VASEL_A::APORT2YCH26),
            29 => Some(VASEL_A::APORT2YCH28),
            31 => Some(VASEL_A::APORT2YCH30),
            32 => Some(VASEL_A::APORT1XCH0),
            33 => Some(VASEL_A::APORT1YCH1),
            34 => Some(VASEL_A::APORT1XCH2),
            35 => Some(VASEL_A::APORT1YCH3),
            36 => Some(VASEL_A::APORT1XCH4),
            37 => Some(VASEL_A::APORT1YCH5),
            38 => Some(VASEL_A::APORT1XCH6),
            39 => Some(VASEL_A::APORT1YCH7),
            40 => Some(VASEL_A::APORT1XCH8),
            41 => Some(VASEL_A::APORT1YCH9),
            42 => Some(VASEL_A::APORT1XCH10),
            43 => Some(VASEL_A::APORT1YCH11),
            44 => Some(VASEL_A::APORT1XCH12),
            45 => Some(VASEL_A::APORT1YCH13),
            46 => Some(VASEL_A::APORT1XCH14),
            47 => Some(VASEL_A::APORT1YCH15),
            48 => Some(VASEL_A::APORT1XCH16),
            49 => Some(VASEL_A::APORT1YCH17),
            50 => Some(VASEL_A::APORT1XCH18),
            51 => Some(VASEL_A::APORT1YCH19),
            52 => Some(VASEL_A::APORT1XCH20),
            53 => Some(VASEL_A::APORT1YCH21),
            54 => Some(VASEL_A::APORT1XCH22),
            55 => Some(VASEL_A::APORT1YCH23),
            56 => Some(VASEL_A::APORT1XCH24),
            57 => Some(VASEL_A::APORT1YCH25),
            58 => Some(VASEL_A::APORT1XCH26),
            59 => Some(VASEL_A::APORT1YCH27),
            60 => Some(VASEL_A::APORT1XCH28),
            61 => Some(VASEL_A::APORT1YCH29),
            62 => Some(VASEL_A::APORT1XCH30),
            63 => Some(VASEL_A::APORT1YCH31),
            _ => None,
        }
    }
    #[doc = "ACMPVDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == VASEL_A::VDD
    }
    #[doc = "APORT2Y Channel 0"]
    #[inline(always)]
    pub fn is_aport2ych0(&self) -> bool {
        *self == VASEL_A::APORT2YCH0
    }
    #[doc = "APORT2Y Channel 2"]
    #[inline(always)]
    pub fn is_aport2ych2(&self) -> bool {
        *self == VASEL_A::APORT2YCH2
    }
    #[doc = "APORT2Y Channel 4"]
    #[inline(always)]
    pub fn is_aport2ych4(&self) -> bool {
        *self == VASEL_A::APORT2YCH4
    }
    #[doc = "APORT2Y Channel 6"]
    #[inline(always)]
    pub fn is_aport2ych6(&self) -> bool {
        *self == VASEL_A::APORT2YCH6
    }
    #[doc = "APORT2Y Channel 8"]
    #[inline(always)]
    pub fn is_aport2ych8(&self) -> bool {
        *self == VASEL_A::APORT2YCH8
    }
    #[doc = "APORT2Y Channel 10"]
    #[inline(always)]
    pub fn is_aport2ych10(&self) -> bool {
        *self == VASEL_A::APORT2YCH10
    }
    #[doc = "APORT2Y Channel 12"]
    #[inline(always)]
    pub fn is_aport2ych12(&self) -> bool {
        *self == VASEL_A::APORT2YCH12
    }
    #[doc = "APORT2Y Channel 14"]
    #[inline(always)]
    pub fn is_aport2ych14(&self) -> bool {
        *self == VASEL_A::APORT2YCH14
    }
    #[doc = "APORT2Y Channel 16"]
    #[inline(always)]
    pub fn is_aport2ych16(&self) -> bool {
        *self == VASEL_A::APORT2YCH16
    }
    #[doc = "APORT2Y Channel 18"]
    #[inline(always)]
    pub fn is_aport2ych18(&self) -> bool {
        *self == VASEL_A::APORT2YCH18
    }
    #[doc = "APORT2Y Channel 20"]
    #[inline(always)]
    pub fn is_aport2ych20(&self) -> bool {
        *self == VASEL_A::APORT2YCH20
    }
    #[doc = "APORT2Y Channel 22"]
    #[inline(always)]
    pub fn is_aport2ych22(&self) -> bool {
        *self == VASEL_A::APORT2YCH22
    }
    #[doc = "APORT2Y Channel 24"]
    #[inline(always)]
    pub fn is_aport2ych24(&self) -> bool {
        *self == VASEL_A::APORT2YCH24
    }
    #[doc = "APORT2Y Channel 26"]
    #[inline(always)]
    pub fn is_aport2ych26(&self) -> bool {
        *self == VASEL_A::APORT2YCH26
    }
    #[doc = "APORT2Y Channel 28"]
    #[inline(always)]
    pub fn is_aport2ych28(&self) -> bool {
        *self == VASEL_A::APORT2YCH28
    }
    #[doc = "APORT2Y Channel 30"]
    #[inline(always)]
    pub fn is_aport2ych30(&self) -> bool {
        *self == VASEL_A::APORT2YCH30
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == VASEL_A::APORT1XCH0
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == VASEL_A::APORT1YCH1
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == VASEL_A::APORT1XCH2
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == VASEL_A::APORT1YCH3
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == VASEL_A::APORT1XCH4
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == VASEL_A::APORT1YCH5
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == VASEL_A::APORT1XCH6
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == VASEL_A::APORT1YCH7
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == VASEL_A::APORT1XCH8
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == VASEL_A::APORT1YCH9
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == VASEL_A::APORT1XCH10
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == VASEL_A::APORT1YCH11
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == VASEL_A::APORT1XCH12
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == VASEL_A::APORT1YCH13
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == VASEL_A::APORT1XCH14
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == VASEL_A::APORT1YCH15
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == VASEL_A::APORT1XCH16
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == VASEL_A::APORT1YCH17
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == VASEL_A::APORT1XCH18
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == VASEL_A::APORT1YCH19
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == VASEL_A::APORT1XCH20
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == VASEL_A::APORT1YCH21
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == VASEL_A::APORT1XCH22
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == VASEL_A::APORT1YCH23
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == VASEL_A::APORT1XCH24
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == VASEL_A::APORT1YCH25
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == VASEL_A::APORT1XCH26
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == VASEL_A::APORT1YCH27
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == VASEL_A::APORT1XCH28
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == VASEL_A::APORT1YCH29
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == VASEL_A::APORT1XCH30
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == VASEL_A::APORT1YCH31
    }
}
#[doc = "Field `VASEL` writer - VA Selection"]
pub type VASEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, VASEL_A>;
impl<'a, REG, const O: u8> VASEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACMPVDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::VDD)
    }
    #[doc = "APORT2Y Channel 0"]
    #[inline(always)]
    pub fn aport2ych0(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH0)
    }
    #[doc = "APORT2Y Channel 2"]
    #[inline(always)]
    pub fn aport2ych2(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH2)
    }
    #[doc = "APORT2Y Channel 4"]
    #[inline(always)]
    pub fn aport2ych4(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH4)
    }
    #[doc = "APORT2Y Channel 6"]
    #[inline(always)]
    pub fn aport2ych6(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH6)
    }
    #[doc = "APORT2Y Channel 8"]
    #[inline(always)]
    pub fn aport2ych8(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH8)
    }
    #[doc = "APORT2Y Channel 10"]
    #[inline(always)]
    pub fn aport2ych10(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH10)
    }
    #[doc = "APORT2Y Channel 12"]
    #[inline(always)]
    pub fn aport2ych12(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH12)
    }
    #[doc = "APORT2Y Channel 14"]
    #[inline(always)]
    pub fn aport2ych14(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH14)
    }
    #[doc = "APORT2Y Channel 16"]
    #[inline(always)]
    pub fn aport2ych16(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH16)
    }
    #[doc = "APORT2Y Channel 18"]
    #[inline(always)]
    pub fn aport2ych18(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH18)
    }
    #[doc = "APORT2Y Channel 20"]
    #[inline(always)]
    pub fn aport2ych20(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH20)
    }
    #[doc = "APORT2Y Channel 22"]
    #[inline(always)]
    pub fn aport2ych22(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH22)
    }
    #[doc = "APORT2Y Channel 24"]
    #[inline(always)]
    pub fn aport2ych24(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH24)
    }
    #[doc = "APORT2Y Channel 26"]
    #[inline(always)]
    pub fn aport2ych26(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH26)
    }
    #[doc = "APORT2Y Channel 28"]
    #[inline(always)]
    pub fn aport2ych28(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH28)
    }
    #[doc = "APORT2Y Channel 30"]
    #[inline(always)]
    pub fn aport2ych30(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT2YCH30)
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1XCH30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL_A::APORT1YCH31)
    }
}
#[doc = "Field `VBSEL` reader - VB Selection"]
pub type VBSEL_R = crate::BitReader;
#[doc = "Field `VBSEL` writer - VB Selection"]
pub type VBSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VLPSEL` reader - Low-Power Sampled Voltage Selection"]
pub type VLPSEL_R = crate::BitReader;
#[doc = "Field `VLPSEL` writer - Low-Power Sampled Voltage Selection"]
pub type VLPSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSRESEN` reader - Capacitive Sense Mode Internal Resistor Enable"]
pub type CSRESEN_R = crate::BitReader;
#[doc = "Field `CSRESEN` writer - Capacitive Sense Mode Internal Resistor Enable"]
pub type CSRESEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor Select"]
pub type CSRESSEL_R = crate::FieldReader<CSRESSEL_A>;
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSRESSEL_A {
    #[doc = "0: Internal capacitive sense resistor value 0"]
    RES0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1"]
    RES1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2"]
    RES2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3"]
    RES3 = 3,
    #[doc = "4: Internal capacitive sense resistor value 4"]
    RES4 = 4,
    #[doc = "5: Internal capacitive sense resistor value 5"]
    RES5 = 5,
    #[doc = "6: Internal capacitive sense resistor value 6"]
    RES6 = 6,
    #[doc = "7: Internal capacitive sense resistor value 7"]
    RES7 = 7,
}
impl From<CSRESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSRESSEL_A {
    type Ux = u8;
}
impl CSRESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRESSEL_A {
        match self.bits {
            0 => CSRESSEL_A::RES0,
            1 => CSRESSEL_A::RES1,
            2 => CSRESSEL_A::RES2,
            3 => CSRESSEL_A::RES3,
            4 => CSRESSEL_A::RES4,
            5 => CSRESSEL_A::RES5,
            6 => CSRESSEL_A::RES6,
            7 => CSRESSEL_A::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSEL_A::RES0
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSEL_A::RES1
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSEL_A::RES2
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSEL_A::RES3
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == CSRESSEL_A::RES4
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == CSRESSEL_A::RES5
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == CSRESSEL_A::RES6
    }
    #[doc = "Internal capacitive sense resistor value 7"]
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == CSRESSEL_A::RES7
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor Select"]
pub type CSRESSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CSRESSEL_A>;
impl<'a, REG, const O: u8> CSRESSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES3)
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES4)
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES5)
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES6)
    }
    #[doc = "Internal capacitive sense resistor value 7"]
    #[inline(always)]
    pub fn res7(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL_A::RES7)
    }
}
impl R {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline(always)]
    pub fn vasel(&self) -> VASEL_R {
        VASEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline(always)]
    pub fn vbsel(&self) -> VBSEL_R {
        VBSEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline(always)]
    pub fn vlpsel(&self) -> VLPSEL_R {
        VLPSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CSRESEN_R {
        CSRESEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CSRESSEL_R {
        CSRESSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> POSSEL_W<INPUTSEL_SPEC, 0> {
        POSSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NEGSEL_W<INPUTSEL_SPEC, 8> {
        NEGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - VA Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vasel(&mut self) -> VASEL_W<INPUTSEL_SPEC, 16> {
        VASEL_W::new(self)
    }
    #[doc = "Bit 22 - VB Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbsel(&mut self) -> VBSEL_W<INPUTSEL_SPEC, 22> {
        VBSEL_W::new(self)
    }
    #[doc = "Bit 24 - Low-Power Sampled Voltage Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vlpsel(&mut self) -> VLPSEL_W<INPUTSEL_SPEC, 24> {
        VLPSEL_W::new(self)
    }
    #[doc = "Bit 26 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csresen(&mut self) -> CSRESEN_W<INPUTSEL_SPEC, 26> {
        CSRESEN_W::new(self)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    #[must_use]
    pub fn csressel(&mut self) -> CSRESSEL_W<INPUTSEL_SPEC, 28> {
        CSRESSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inputsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTSEL_SPEC;
impl crate::RegisterSpec for INPUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputsel::R`](R) reader structure"]
impl crate::Readable for INPUTSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inputsel::W`](W) writer structure"]
impl crate::Writable for INPUTSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPUTSEL to value 0"]
impl crate::Resettable for INPUTSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
