#[doc = "Register `SINGLECTRL` reader"]
pub type R = crate::R<SinglectrlSpec>;
#[doc = "Register `SINGLECTRL` writer"]
pub type W = crate::W<SinglectrlSpec>;
#[doc = "Single Channel Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Singlesel {
    #[doc = "32: `100000`"]
    Aport1xch0 = 32,
    #[doc = "33: `100001`"]
    Aport1ych1 = 33,
    #[doc = "34: `100010`"]
    Aport1xch2 = 34,
    #[doc = "35: `100011`"]
    Aport1ych3 = 35,
    #[doc = "36: `100100`"]
    Aport1xch4 = 36,
    #[doc = "37: `100101`"]
    Aport1ych5 = 37,
    #[doc = "38: `100110`"]
    Aport1xch6 = 38,
    #[doc = "39: `100111`"]
    Aport1ych7 = 39,
    #[doc = "40: `101000`"]
    Aport1xch8 = 40,
    #[doc = "41: `101001`"]
    Aport1ych9 = 41,
    #[doc = "42: `101010`"]
    Aport1xch10 = 42,
    #[doc = "43: `101011`"]
    Aport1ych11 = 43,
    #[doc = "44: `101100`"]
    Aport1xch12 = 44,
    #[doc = "45: `101101`"]
    Aport1ych13 = 45,
    #[doc = "46: `101110`"]
    Aport1xch14 = 46,
    #[doc = "47: `101111`"]
    Aport1ych15 = 47,
    #[doc = "48: `110000`"]
    Aport1xch16 = 48,
    #[doc = "49: `110001`"]
    Aport1ych17 = 49,
    #[doc = "50: `110010`"]
    Aport1xch18 = 50,
    #[doc = "51: `110011`"]
    Aport1ych19 = 51,
    #[doc = "52: `110100`"]
    Aport1xch20 = 52,
    #[doc = "53: `110101`"]
    Aport1ych21 = 53,
    #[doc = "54: `110110`"]
    Aport1xch22 = 54,
    #[doc = "55: `110111`"]
    Aport1ych23 = 55,
    #[doc = "56: `111000`"]
    Aport1xch24 = 56,
    #[doc = "57: `111001`"]
    Aport1ych25 = 57,
    #[doc = "58: `111010`"]
    Aport1xch26 = 58,
    #[doc = "59: `111011`"]
    Aport1ych27 = 59,
    #[doc = "60: `111100`"]
    Aport1xch28 = 60,
    #[doc = "61: `111101`"]
    Aport1ych29 = 61,
    #[doc = "62: `111110`"]
    Aport1xch30 = 62,
    #[doc = "63: `111111`"]
    Aport1ych31 = 63,
    #[doc = "96: `1100000`"]
    Aport3xch0 = 96,
    #[doc = "97: `1100001`"]
    Aport3ych1 = 97,
    #[doc = "98: `1100010`"]
    Aport3xch2 = 98,
    #[doc = "99: `1100011`"]
    Aport3ych3 = 99,
    #[doc = "100: `1100100`"]
    Aport3xch4 = 100,
    #[doc = "101: `1100101`"]
    Aport3ych5 = 101,
    #[doc = "102: `1100110`"]
    Aport3xch6 = 102,
    #[doc = "103: `1100111`"]
    Aport3ych7 = 103,
    #[doc = "104: `1101000`"]
    Aport3xch8 = 104,
    #[doc = "105: `1101001`"]
    Aport3ych9 = 105,
    #[doc = "106: `1101010`"]
    Aport3xch10 = 106,
    #[doc = "107: `1101011`"]
    Aport3ych11 = 107,
    #[doc = "108: `1101100`"]
    Aport3xch12 = 108,
    #[doc = "109: `1101101`"]
    Aport3ych13 = 109,
    #[doc = "110: `1101110`"]
    Aport3xch14 = 110,
    #[doc = "111: `1101111`"]
    Aport3ych15 = 111,
    #[doc = "112: `1110000`"]
    Aport3xch16 = 112,
    #[doc = "113: `1110001`"]
    Aport3ych17 = 113,
    #[doc = "114: `1110010`"]
    Aport3xch18 = 114,
    #[doc = "115: `1110011`"]
    Aport3ych19 = 115,
    #[doc = "116: `1110100`"]
    Aport3xch20 = 116,
    #[doc = "117: `1110101`"]
    Aport3ych21 = 117,
    #[doc = "118: `1110110`"]
    Aport3xch22 = 118,
    #[doc = "119: `1110111`"]
    Aport3ych23 = 119,
    #[doc = "120: `1111000`"]
    Aport3xch24 = 120,
    #[doc = "121: `1111001`"]
    Aport3ych25 = 121,
    #[doc = "122: `1111010`"]
    Aport3xch26 = 122,
    #[doc = "123: `1111011`"]
    Aport3ych27 = 123,
    #[doc = "124: `1111100`"]
    Aport3xch28 = 124,
    #[doc = "125: `1111101`"]
    Aport3ych29 = 125,
    #[doc = "126: `1111110`"]
    Aport3xch30 = 126,
    #[doc = "127: `1111111`"]
    Aport3ych31 = 127,
}
impl From<Singlesel> for u8 {
    #[inline(always)]
    fn from(variant: Singlesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Singlesel {
    type Ux = u8;
}
impl crate::IsEnum for Singlesel {}
#[doc = "Field `SINGLESEL` reader - Single Channel Input Select"]
pub type SingleselR = crate::FieldReader<Singlesel>;
impl SingleselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Singlesel> {
        match self.bits {
            32 => Some(Singlesel::Aport1xch0),
            33 => Some(Singlesel::Aport1ych1),
            34 => Some(Singlesel::Aport1xch2),
            35 => Some(Singlesel::Aport1ych3),
            36 => Some(Singlesel::Aport1xch4),
            37 => Some(Singlesel::Aport1ych5),
            38 => Some(Singlesel::Aport1xch6),
            39 => Some(Singlesel::Aport1ych7),
            40 => Some(Singlesel::Aport1xch8),
            41 => Some(Singlesel::Aport1ych9),
            42 => Some(Singlesel::Aport1xch10),
            43 => Some(Singlesel::Aport1ych11),
            44 => Some(Singlesel::Aport1xch12),
            45 => Some(Singlesel::Aport1ych13),
            46 => Some(Singlesel::Aport1xch14),
            47 => Some(Singlesel::Aport1ych15),
            48 => Some(Singlesel::Aport1xch16),
            49 => Some(Singlesel::Aport1ych17),
            50 => Some(Singlesel::Aport1xch18),
            51 => Some(Singlesel::Aport1ych19),
            52 => Some(Singlesel::Aport1xch20),
            53 => Some(Singlesel::Aport1ych21),
            54 => Some(Singlesel::Aport1xch22),
            55 => Some(Singlesel::Aport1ych23),
            56 => Some(Singlesel::Aport1xch24),
            57 => Some(Singlesel::Aport1ych25),
            58 => Some(Singlesel::Aport1xch26),
            59 => Some(Singlesel::Aport1ych27),
            60 => Some(Singlesel::Aport1xch28),
            61 => Some(Singlesel::Aport1ych29),
            62 => Some(Singlesel::Aport1xch30),
            63 => Some(Singlesel::Aport1ych31),
            96 => Some(Singlesel::Aport3xch0),
            97 => Some(Singlesel::Aport3ych1),
            98 => Some(Singlesel::Aport3xch2),
            99 => Some(Singlesel::Aport3ych3),
            100 => Some(Singlesel::Aport3xch4),
            101 => Some(Singlesel::Aport3ych5),
            102 => Some(Singlesel::Aport3xch6),
            103 => Some(Singlesel::Aport3ych7),
            104 => Some(Singlesel::Aport3xch8),
            105 => Some(Singlesel::Aport3ych9),
            106 => Some(Singlesel::Aport3xch10),
            107 => Some(Singlesel::Aport3ych11),
            108 => Some(Singlesel::Aport3xch12),
            109 => Some(Singlesel::Aport3ych13),
            110 => Some(Singlesel::Aport3xch14),
            111 => Some(Singlesel::Aport3ych15),
            112 => Some(Singlesel::Aport3xch16),
            113 => Some(Singlesel::Aport3ych17),
            114 => Some(Singlesel::Aport3xch18),
            115 => Some(Singlesel::Aport3ych19),
            116 => Some(Singlesel::Aport3xch20),
            117 => Some(Singlesel::Aport3ych21),
            118 => Some(Singlesel::Aport3xch22),
            119 => Some(Singlesel::Aport3ych23),
            120 => Some(Singlesel::Aport3xch24),
            121 => Some(Singlesel::Aport3ych25),
            122 => Some(Singlesel::Aport3xch26),
            123 => Some(Singlesel::Aport3ych27),
            124 => Some(Singlesel::Aport3xch28),
            125 => Some(Singlesel::Aport3ych29),
            126 => Some(Singlesel::Aport3xch30),
            127 => Some(Singlesel::Aport3ych31),
            _ => None,
        }
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == Singlesel::Aport1xch0
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == Singlesel::Aport1ych1
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == Singlesel::Aport1xch2
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == Singlesel::Aport1ych3
    }
    #[doc = "`100100`"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == Singlesel::Aport1xch4
    }
    #[doc = "`100101`"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == Singlesel::Aport1ych5
    }
    #[doc = "`100110`"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == Singlesel::Aport1xch6
    }
    #[doc = "`100111`"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == Singlesel::Aport1ych7
    }
    #[doc = "`101000`"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == Singlesel::Aport1xch8
    }
    #[doc = "`101001`"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == Singlesel::Aport1ych9
    }
    #[doc = "`101010`"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == Singlesel::Aport1xch10
    }
    #[doc = "`101011`"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == Singlesel::Aport1ych11
    }
    #[doc = "`101100`"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == Singlesel::Aport1xch12
    }
    #[doc = "`101101`"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == Singlesel::Aport1ych13
    }
    #[doc = "`101110`"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == Singlesel::Aport1xch14
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == Singlesel::Aport1ych15
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == Singlesel::Aport1xch16
    }
    #[doc = "`110001`"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == Singlesel::Aport1ych17
    }
    #[doc = "`110010`"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == Singlesel::Aport1xch18
    }
    #[doc = "`110011`"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == Singlesel::Aport1ych19
    }
    #[doc = "`110100`"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == Singlesel::Aport1xch20
    }
    #[doc = "`110101`"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == Singlesel::Aport1ych21
    }
    #[doc = "`110110`"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == Singlesel::Aport1xch22
    }
    #[doc = "`110111`"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == Singlesel::Aport1ych23
    }
    #[doc = "`111000`"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == Singlesel::Aport1xch24
    }
    #[doc = "`111001`"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == Singlesel::Aport1ych25
    }
    #[doc = "`111010`"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == Singlesel::Aport1xch26
    }
    #[doc = "`111011`"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == Singlesel::Aport1ych27
    }
    #[doc = "`111100`"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == Singlesel::Aport1xch28
    }
    #[doc = "`111101`"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == Singlesel::Aport1ych29
    }
    #[doc = "`111110`"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == Singlesel::Aport1xch30
    }
    #[doc = "`111111`"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == Singlesel::Aport1ych31
    }
    #[doc = "`1100000`"]
    #[inline(always)]
    pub fn is_aport3xch0(&self) -> bool {
        *self == Singlesel::Aport3xch0
    }
    #[doc = "`1100001`"]
    #[inline(always)]
    pub fn is_aport3ych1(&self) -> bool {
        *self == Singlesel::Aport3ych1
    }
    #[doc = "`1100010`"]
    #[inline(always)]
    pub fn is_aport3xch2(&self) -> bool {
        *self == Singlesel::Aport3xch2
    }
    #[doc = "`1100011`"]
    #[inline(always)]
    pub fn is_aport3ych3(&self) -> bool {
        *self == Singlesel::Aport3ych3
    }
    #[doc = "`1100100`"]
    #[inline(always)]
    pub fn is_aport3xch4(&self) -> bool {
        *self == Singlesel::Aport3xch4
    }
    #[doc = "`1100101`"]
    #[inline(always)]
    pub fn is_aport3ych5(&self) -> bool {
        *self == Singlesel::Aport3ych5
    }
    #[doc = "`1100110`"]
    #[inline(always)]
    pub fn is_aport3xch6(&self) -> bool {
        *self == Singlesel::Aport3xch6
    }
    #[doc = "`1100111`"]
    #[inline(always)]
    pub fn is_aport3ych7(&self) -> bool {
        *self == Singlesel::Aport3ych7
    }
    #[doc = "`1101000`"]
    #[inline(always)]
    pub fn is_aport3xch8(&self) -> bool {
        *self == Singlesel::Aport3xch8
    }
    #[doc = "`1101001`"]
    #[inline(always)]
    pub fn is_aport3ych9(&self) -> bool {
        *self == Singlesel::Aport3ych9
    }
    #[doc = "`1101010`"]
    #[inline(always)]
    pub fn is_aport3xch10(&self) -> bool {
        *self == Singlesel::Aport3xch10
    }
    #[doc = "`1101011`"]
    #[inline(always)]
    pub fn is_aport3ych11(&self) -> bool {
        *self == Singlesel::Aport3ych11
    }
    #[doc = "`1101100`"]
    #[inline(always)]
    pub fn is_aport3xch12(&self) -> bool {
        *self == Singlesel::Aport3xch12
    }
    #[doc = "`1101101`"]
    #[inline(always)]
    pub fn is_aport3ych13(&self) -> bool {
        *self == Singlesel::Aport3ych13
    }
    #[doc = "`1101110`"]
    #[inline(always)]
    pub fn is_aport3xch14(&self) -> bool {
        *self == Singlesel::Aport3xch14
    }
    #[doc = "`1101111`"]
    #[inline(always)]
    pub fn is_aport3ych15(&self) -> bool {
        *self == Singlesel::Aport3ych15
    }
    #[doc = "`1110000`"]
    #[inline(always)]
    pub fn is_aport3xch16(&self) -> bool {
        *self == Singlesel::Aport3xch16
    }
    #[doc = "`1110001`"]
    #[inline(always)]
    pub fn is_aport3ych17(&self) -> bool {
        *self == Singlesel::Aport3ych17
    }
    #[doc = "`1110010`"]
    #[inline(always)]
    pub fn is_aport3xch18(&self) -> bool {
        *self == Singlesel::Aport3xch18
    }
    #[doc = "`1110011`"]
    #[inline(always)]
    pub fn is_aport3ych19(&self) -> bool {
        *self == Singlesel::Aport3ych19
    }
    #[doc = "`1110100`"]
    #[inline(always)]
    pub fn is_aport3xch20(&self) -> bool {
        *self == Singlesel::Aport3xch20
    }
    #[doc = "`1110101`"]
    #[inline(always)]
    pub fn is_aport3ych21(&self) -> bool {
        *self == Singlesel::Aport3ych21
    }
    #[doc = "`1110110`"]
    #[inline(always)]
    pub fn is_aport3xch22(&self) -> bool {
        *self == Singlesel::Aport3xch22
    }
    #[doc = "`1110111`"]
    #[inline(always)]
    pub fn is_aport3ych23(&self) -> bool {
        *self == Singlesel::Aport3ych23
    }
    #[doc = "`1111000`"]
    #[inline(always)]
    pub fn is_aport3xch24(&self) -> bool {
        *self == Singlesel::Aport3xch24
    }
    #[doc = "`1111001`"]
    #[inline(always)]
    pub fn is_aport3ych25(&self) -> bool {
        *self == Singlesel::Aport3ych25
    }
    #[doc = "`1111010`"]
    #[inline(always)]
    pub fn is_aport3xch26(&self) -> bool {
        *self == Singlesel::Aport3xch26
    }
    #[doc = "`1111011`"]
    #[inline(always)]
    pub fn is_aport3ych27(&self) -> bool {
        *self == Singlesel::Aport3ych27
    }
    #[doc = "`1111100`"]
    #[inline(always)]
    pub fn is_aport3xch28(&self) -> bool {
        *self == Singlesel::Aport3xch28
    }
    #[doc = "`1111101`"]
    #[inline(always)]
    pub fn is_aport3ych29(&self) -> bool {
        *self == Singlesel::Aport3ych29
    }
    #[doc = "`1111110`"]
    #[inline(always)]
    pub fn is_aport3xch30(&self) -> bool {
        *self == Singlesel::Aport3xch30
    }
    #[doc = "`1111111`"]
    #[inline(always)]
    pub fn is_aport3ych31(&self) -> bool {
        *self == Singlesel::Aport3ych31
    }
}
#[doc = "Field `SINGLESEL` writer - Single Channel Input Select"]
pub type SingleselW<'a, REG> = crate::FieldWriter<'a, REG, 7, Singlesel>;
impl<'a, REG> SingleselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch0)
    }
    #[doc = "`100001`"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych1)
    }
    #[doc = "`100010`"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch2)
    }
    #[doc = "`100011`"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych3)
    }
    #[doc = "`100100`"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch4)
    }
    #[doc = "`100101`"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych5)
    }
    #[doc = "`100110`"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch6)
    }
    #[doc = "`100111`"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych7)
    }
    #[doc = "`101000`"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch8)
    }
    #[doc = "`101001`"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych9)
    }
    #[doc = "`101010`"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch10)
    }
    #[doc = "`101011`"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych11)
    }
    #[doc = "`101100`"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch12)
    }
    #[doc = "`101101`"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych13)
    }
    #[doc = "`101110`"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch14)
    }
    #[doc = "`101111`"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych15)
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch16)
    }
    #[doc = "`110001`"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych17)
    }
    #[doc = "`110010`"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch18)
    }
    #[doc = "`110011`"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych19)
    }
    #[doc = "`110100`"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch20)
    }
    #[doc = "`110101`"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych21)
    }
    #[doc = "`110110`"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch22)
    }
    #[doc = "`110111`"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych23)
    }
    #[doc = "`111000`"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch24)
    }
    #[doc = "`111001`"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych25)
    }
    #[doc = "`111010`"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch26)
    }
    #[doc = "`111011`"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych27)
    }
    #[doc = "`111100`"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch28)
    }
    #[doc = "`111101`"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych29)
    }
    #[doc = "`111110`"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1xch30)
    }
    #[doc = "`111111`"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport1ych31)
    }
    #[doc = "`1100000`"]
    #[inline(always)]
    pub fn aport3xch0(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch0)
    }
    #[doc = "`1100001`"]
    #[inline(always)]
    pub fn aport3ych1(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych1)
    }
    #[doc = "`1100010`"]
    #[inline(always)]
    pub fn aport3xch2(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch2)
    }
    #[doc = "`1100011`"]
    #[inline(always)]
    pub fn aport3ych3(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych3)
    }
    #[doc = "`1100100`"]
    #[inline(always)]
    pub fn aport3xch4(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch4)
    }
    #[doc = "`1100101`"]
    #[inline(always)]
    pub fn aport3ych5(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych5)
    }
    #[doc = "`1100110`"]
    #[inline(always)]
    pub fn aport3xch6(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch6)
    }
    #[doc = "`1100111`"]
    #[inline(always)]
    pub fn aport3ych7(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych7)
    }
    #[doc = "`1101000`"]
    #[inline(always)]
    pub fn aport3xch8(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch8)
    }
    #[doc = "`1101001`"]
    #[inline(always)]
    pub fn aport3ych9(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych9)
    }
    #[doc = "`1101010`"]
    #[inline(always)]
    pub fn aport3xch10(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch10)
    }
    #[doc = "`1101011`"]
    #[inline(always)]
    pub fn aport3ych11(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych11)
    }
    #[doc = "`1101100`"]
    #[inline(always)]
    pub fn aport3xch12(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch12)
    }
    #[doc = "`1101101`"]
    #[inline(always)]
    pub fn aport3ych13(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych13)
    }
    #[doc = "`1101110`"]
    #[inline(always)]
    pub fn aport3xch14(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch14)
    }
    #[doc = "`1101111`"]
    #[inline(always)]
    pub fn aport3ych15(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych15)
    }
    #[doc = "`1110000`"]
    #[inline(always)]
    pub fn aport3xch16(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch16)
    }
    #[doc = "`1110001`"]
    #[inline(always)]
    pub fn aport3ych17(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych17)
    }
    #[doc = "`1110010`"]
    #[inline(always)]
    pub fn aport3xch18(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch18)
    }
    #[doc = "`1110011`"]
    #[inline(always)]
    pub fn aport3ych19(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych19)
    }
    #[doc = "`1110100`"]
    #[inline(always)]
    pub fn aport3xch20(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch20)
    }
    #[doc = "`1110101`"]
    #[inline(always)]
    pub fn aport3ych21(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych21)
    }
    #[doc = "`1110110`"]
    #[inline(always)]
    pub fn aport3xch22(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch22)
    }
    #[doc = "`1110111`"]
    #[inline(always)]
    pub fn aport3ych23(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych23)
    }
    #[doc = "`1111000`"]
    #[inline(always)]
    pub fn aport3xch24(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch24)
    }
    #[doc = "`1111001`"]
    #[inline(always)]
    pub fn aport3ych25(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych25)
    }
    #[doc = "`1111010`"]
    #[inline(always)]
    pub fn aport3xch26(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch26)
    }
    #[doc = "`1111011`"]
    #[inline(always)]
    pub fn aport3ych27(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych27)
    }
    #[doc = "`1111100`"]
    #[inline(always)]
    pub fn aport3xch28(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch28)
    }
    #[doc = "`1111101`"]
    #[inline(always)]
    pub fn aport3ych29(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych29)
    }
    #[doc = "`1111110`"]
    #[inline(always)]
    pub fn aport3xch30(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3xch30)
    }
    #[doc = "`1111111`"]
    #[inline(always)]
    pub fn aport3ych31(self) -> &'a mut crate::W<REG> {
        self.variant(Singlesel::Aport3ych31)
    }
}
impl R {
    #[doc = "Bits 4:10 - Single Channel Input Select"]
    #[inline(always)]
    pub fn singlesel(&self) -> SingleselR {
        SingleselR::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:10 - Single Channel Input Select"]
    #[inline(always)]
    pub fn singlesel(&mut self) -> SingleselW<'_, SinglectrlSpec> {
        SingleselW::new(self, 4)
    }
}
#[doc = "Single Conversion Control\n\nYou can [`read`](crate::Reg::read) this register and get [`singlectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglectrlSpec;
impl crate::RegisterSpec for SinglectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlectrl::R`](R) reader structure"]
impl crate::Readable for SinglectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`singlectrl::W`](W) writer structure"]
impl crate::Writable for SinglectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLECTRL to value 0"]
impl crate::Resettable for SinglectrlSpec {}
