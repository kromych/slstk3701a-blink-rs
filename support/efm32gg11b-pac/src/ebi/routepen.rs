#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPEN_SPEC>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPEN_SPEC>;
#[doc = "Field `EBIPEN` reader - EBI Pin Enable"]
pub type EBIPEN_R = crate::BitReader;
#[doc = "Field `EBIPEN` writer - EBI Pin Enable"]
pub type EBIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS0PEN` reader - EBI_CS0 Pin Enable"]
pub type CS0PEN_R = crate::BitReader;
#[doc = "Field `CS0PEN` writer - EBI_CS0 Pin Enable"]
pub type CS0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1PEN` reader - EBI_CS1 Pin Enable"]
pub type CS1PEN_R = crate::BitReader;
#[doc = "Field `CS1PEN` writer - EBI_CS1 Pin Enable"]
pub type CS1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2PEN` reader - EBI_CS2 Pin Enable"]
pub type CS2PEN_R = crate::BitReader;
#[doc = "Field `CS2PEN` writer - EBI_CS2 Pin Enable"]
pub type CS2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS3PEN` reader - EBI_CS3 Pin Enable"]
pub type CS3PEN_R = crate::BitReader;
#[doc = "Field `CS3PEN` writer - EBI_CS3 Pin Enable"]
pub type CS3PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALEPEN` reader - EBI_ALE Pin Enable"]
pub type ALEPEN_R = crate::BitReader;
#[doc = "Field `ALEPEN` writer - EBI_ALE Pin Enable"]
pub type ALEPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYPEN` reader - EBI_ARDY Pin Enable"]
pub type ARDYPEN_R = crate::BitReader;
#[doc = "Field `ARDYPEN` writer - EBI_ARDY Pin Enable"]
pub type ARDYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLPEN` reader - EBI_BL\\[1:0\\]
Pin Enable"]
pub type BLPEN_R = crate::BitReader;
#[doc = "Field `BLPEN` writer - EBI_BL\\[1:0\\]
Pin Enable"]
pub type BLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NANDPEN` reader - NANDRE and NANDWE Pin Enable"]
pub type NANDPEN_R = crate::BitReader;
#[doc = "Field `NANDPEN` writer - NANDRE and NANDWE Pin Enable"]
pub type NANDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALB` reader - Sets the Lower Bound for EBI_A Enabling"]
pub type ALB_R = crate::FieldReader<ALB_A>;
#[doc = "Sets the Lower Bound for EBI_A Enabling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALB_A {
    #[doc = "0: Address lines from EBI_A\\[0\\]
and upwards can be enabled via APEN."]
    A0 = 0,
    #[doc = "1: Address lines from EBI_A\\[8\\]
and upwards can be enabled via APEN."]
    A8 = 1,
    #[doc = "2: Address lines from EBI_A\\[16\\]
and upwards can be enabled via APEN."]
    A16 = 2,
    #[doc = "3: Address lines from EBI_A\\[24\\]
and upwards can be enabled via APEN."]
    A24 = 3,
}
impl From<ALB_A> for u8 {
    #[inline(always)]
    fn from(variant: ALB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALB_A {
    type Ux = u8;
}
impl ALB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALB_A {
        match self.bits {
            0 => ALB_A::A0,
            1 => ALB_A::A8,
            2 => ALB_A::A16,
            3 => ALB_A::A24,
            _ => unreachable!(),
        }
    }
    #[doc = "Address lines from EBI_A\\[0\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == ALB_A::A0
    }
    #[doc = "Address lines from EBI_A\\[8\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == ALB_A::A8
    }
    #[doc = "Address lines from EBI_A\\[16\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == ALB_A::A16
    }
    #[doc = "Address lines from EBI_A\\[24\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == ALB_A::A24
    }
}
#[doc = "Field `ALB` writer - Sets the Lower Bound for EBI_A Enabling"]
pub type ALB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ALB_A>;
impl<'a, REG> ALB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address lines from EBI_A\\[0\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(ALB_A::A0)
    }
    #[doc = "Address lines from EBI_A\\[8\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut crate::W<REG> {
        self.variant(ALB_A::A8)
    }
    #[doc = "Address lines from EBI_A\\[16\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut crate::W<REG> {
        self.variant(ALB_A::A16)
    }
    #[doc = "Address lines from EBI_A\\[24\\]
and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut crate::W<REG> {
        self.variant(ALB_A::A24)
    }
}
#[doc = "Field `APEN` reader - EBI_A Pin Enable"]
pub type APEN_R = crate::FieldReader<APEN_A>;
#[doc = "EBI_A Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APEN_A {
    #[doc = "0: All EBI_A pins are disabled."]
    A0 = 0,
    #[doc = "5: EBI_A\\[4:L\\]
pins enabled."]
    A5 = 5,
    #[doc = "6: EBI_A\\[5:L\\]
pins enabled."]
    A6 = 6,
    #[doc = "7: EBI_A\\[6:L\\]
pins enabled."]
    A7 = 7,
    #[doc = "8: EBI_A\\[7:L\\]
pins enabled."]
    A8 = 8,
    #[doc = "9: EBI_A\\[8:L\\]
pins enabled."]
    A9 = 9,
    #[doc = "10: EBI_A\\[9:L\\]
pins enabled."]
    A10 = 10,
    #[doc = "11: EBI_A\\[10:L\\]
pins enabled."]
    A11 = 11,
    #[doc = "12: EBI_A\\[11:L\\]
pins enabled."]
    A12 = 12,
    #[doc = "13: EBI_A\\[12:L\\]
pins enabled."]
    A13 = 13,
    #[doc = "14: EBI_A\\[13:L\\]
pins enabled."]
    A14 = 14,
    #[doc = "15: EBI_A\\[14:L\\]
pins enabled."]
    A15 = 15,
    #[doc = "16: EBI_A\\[15:L\\]
pins enabled."]
    A16 = 16,
    #[doc = "17: EBI_A\\[16:L\\]
pins enabled."]
    A17 = 17,
    #[doc = "18: EBI_A\\[17:L\\]
pins enabled."]
    A18 = 18,
    #[doc = "19: EBI_A\\[18:L\\]
pins enabled."]
    A19 = 19,
    #[doc = "20: EBI_A\\[19:L\\]
pins enabled."]
    A20 = 20,
    #[doc = "21: EBI_A\\[20:L\\]
pins enabled."]
    A21 = 21,
    #[doc = "22: EBI_A\\[21:L\\]
pins enabled."]
    A22 = 22,
    #[doc = "23: EBI_A\\[22:L\\]
pins enabled."]
    A23 = 23,
    #[doc = "24: EBI_A\\[23:L\\]
pins enabled."]
    A24 = 24,
    #[doc = "25: EBI_A\\[24:L\\]
pins enabled."]
    A25 = 25,
    #[doc = "26: EBI_A\\[25:L\\]
pins enabled."]
    A26 = 26,
    #[doc = "27: EBI_A\\[26:L\\]
pins enabled."]
    A27 = 27,
    #[doc = "28: EBI_A\\[27:L\\]
pins enabled."]
    A28 = 28,
}
impl From<APEN_A> for u8 {
    #[inline(always)]
    fn from(variant: APEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APEN_A {
    type Ux = u8;
}
impl APEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<APEN_A> {
        match self.bits {
            0 => Some(APEN_A::A0),
            5 => Some(APEN_A::A5),
            6 => Some(APEN_A::A6),
            7 => Some(APEN_A::A7),
            8 => Some(APEN_A::A8),
            9 => Some(APEN_A::A9),
            10 => Some(APEN_A::A10),
            11 => Some(APEN_A::A11),
            12 => Some(APEN_A::A12),
            13 => Some(APEN_A::A13),
            14 => Some(APEN_A::A14),
            15 => Some(APEN_A::A15),
            16 => Some(APEN_A::A16),
            17 => Some(APEN_A::A17),
            18 => Some(APEN_A::A18),
            19 => Some(APEN_A::A19),
            20 => Some(APEN_A::A20),
            21 => Some(APEN_A::A21),
            22 => Some(APEN_A::A22),
            23 => Some(APEN_A::A23),
            24 => Some(APEN_A::A24),
            25 => Some(APEN_A::A25),
            26 => Some(APEN_A::A26),
            27 => Some(APEN_A::A27),
            28 => Some(APEN_A::A28),
            _ => None,
        }
    }
    #[doc = "All EBI_A pins are disabled."]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == APEN_A::A0
    }
    #[doc = "EBI_A\\[4:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a5(&self) -> bool {
        *self == APEN_A::A5
    }
    #[doc = "EBI_A\\[5:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a6(&self) -> bool {
        *self == APEN_A::A6
    }
    #[doc = "EBI_A\\[6:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a7(&self) -> bool {
        *self == APEN_A::A7
    }
    #[doc = "EBI_A\\[7:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == APEN_A::A8
    }
    #[doc = "EBI_A\\[8:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a9(&self) -> bool {
        *self == APEN_A::A9
    }
    #[doc = "EBI_A\\[9:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a10(&self) -> bool {
        *self == APEN_A::A10
    }
    #[doc = "EBI_A\\[10:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a11(&self) -> bool {
        *self == APEN_A::A11
    }
    #[doc = "EBI_A\\[11:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a12(&self) -> bool {
        *self == APEN_A::A12
    }
    #[doc = "EBI_A\\[12:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a13(&self) -> bool {
        *self == APEN_A::A13
    }
    #[doc = "EBI_A\\[13:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a14(&self) -> bool {
        *self == APEN_A::A14
    }
    #[doc = "EBI_A\\[14:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a15(&self) -> bool {
        *self == APEN_A::A15
    }
    #[doc = "EBI_A\\[15:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == APEN_A::A16
    }
    #[doc = "EBI_A\\[16:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a17(&self) -> bool {
        *self == APEN_A::A17
    }
    #[doc = "EBI_A\\[17:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a18(&self) -> bool {
        *self == APEN_A::A18
    }
    #[doc = "EBI_A\\[18:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a19(&self) -> bool {
        *self == APEN_A::A19
    }
    #[doc = "EBI_A\\[19:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a20(&self) -> bool {
        *self == APEN_A::A20
    }
    #[doc = "EBI_A\\[20:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a21(&self) -> bool {
        *self == APEN_A::A21
    }
    #[doc = "EBI_A\\[21:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a22(&self) -> bool {
        *self == APEN_A::A22
    }
    #[doc = "EBI_A\\[22:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a23(&self) -> bool {
        *self == APEN_A::A23
    }
    #[doc = "EBI_A\\[23:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == APEN_A::A24
    }
    #[doc = "EBI_A\\[24:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a25(&self) -> bool {
        *self == APEN_A::A25
    }
    #[doc = "EBI_A\\[25:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a26(&self) -> bool {
        *self == APEN_A::A26
    }
    #[doc = "EBI_A\\[26:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a27(&self) -> bool {
        *self == APEN_A::A27
    }
    #[doc = "EBI_A\\[27:L\\]
pins enabled."]
    #[inline(always)]
    pub fn is_a28(&self) -> bool {
        *self == APEN_A::A28
    }
}
#[doc = "Field `APEN` writer - EBI_A Pin Enable"]
pub type APEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5, APEN_A>;
impl<'a, REG> APEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All EBI_A pins are disabled."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A0)
    }
    #[doc = "EBI_A\\[4:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a5(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A5)
    }
    #[doc = "EBI_A\\[5:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a6(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A6)
    }
    #[doc = "EBI_A\\[6:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a7(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A7)
    }
    #[doc = "EBI_A\\[7:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A8)
    }
    #[doc = "EBI_A\\[8:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a9(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A9)
    }
    #[doc = "EBI_A\\[9:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a10(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A10)
    }
    #[doc = "EBI_A\\[10:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a11(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A11)
    }
    #[doc = "EBI_A\\[11:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a12(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A12)
    }
    #[doc = "EBI_A\\[12:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a13(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A13)
    }
    #[doc = "EBI_A\\[13:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a14(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A14)
    }
    #[doc = "EBI_A\\[14:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a15(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A15)
    }
    #[doc = "EBI_A\\[15:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A16)
    }
    #[doc = "EBI_A\\[16:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a17(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A17)
    }
    #[doc = "EBI_A\\[17:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a18(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A18)
    }
    #[doc = "EBI_A\\[18:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a19(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A19)
    }
    #[doc = "EBI_A\\[19:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a20(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A20)
    }
    #[doc = "EBI_A\\[20:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a21(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A21)
    }
    #[doc = "EBI_A\\[21:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a22(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A22)
    }
    #[doc = "EBI_A\\[22:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a23(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A23)
    }
    #[doc = "EBI_A\\[23:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A24)
    }
    #[doc = "EBI_A\\[24:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a25(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A25)
    }
    #[doc = "EBI_A\\[25:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a26(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A26)
    }
    #[doc = "EBI_A\\[26:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a27(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A27)
    }
    #[doc = "EBI_A\\[27:L\\]
pins enabled."]
    #[inline(always)]
    pub fn a28(self) -> &'a mut crate::W<REG> {
        self.variant(APEN_A::A28)
    }
}
#[doc = "Field `TFTPEN` reader - EBI_TFT Pin Enable"]
pub type TFTPEN_R = crate::BitReader;
#[doc = "Field `TFTPEN` writer - EBI_TFT Pin Enable"]
pub type TFTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENPEN` reader - EBI_DATA Pin Enable"]
pub type DATAENPEN_R = crate::BitReader;
#[doc = "Field `DATAENPEN` writer - EBI_DATA Pin Enable"]
pub type DATAENPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTFTPEN` reader - EBI_CSTFT Pin Enable"]
pub type CSTFTPEN_R = crate::BitReader;
#[doc = "Field `CSTFTPEN` writer - EBI_CSTFT Pin Enable"]
pub type CSTFTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&self) -> EBIPEN_R {
        EBIPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> CS0PEN_R {
        CS0PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> CS1PEN_R {
        CS1PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&self) -> CS2PEN_R {
        CS2PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&self) -> CS3PEN_R {
        CS3PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&self) -> ALEPEN_R {
        ALEPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&self) -> ARDYPEN_R {
        ARDYPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\]
Pin Enable"]
    #[inline(always)]
    pub fn blpen(&self) -> BLPEN_R {
        BLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&self) -> NANDPEN_R {
        NANDPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Lower Bound for EBI_A Enabling"]
    #[inline(always)]
    pub fn alb(&self) -> ALB_R {
        ALB_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&self) -> APEN_R {
        APEN_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&self) -> TFTPEN_R {
        TFTPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EBI_DATA Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&self) -> DATAENPEN_R {
        DATAENPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&self) -> CSTFTPEN_R {
        CSTFTPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ebipen(&mut self) -> EBIPEN_W<ROUTEPEN_SPEC> {
        EBIPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0pen(&mut self) -> CS0PEN_W<ROUTEPEN_SPEC> {
        CS0PEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1pen(&mut self) -> CS1PEN_W<ROUTEPEN_SPEC> {
        CS1PEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs2pen(&mut self) -> CS2PEN_W<ROUTEPEN_SPEC> {
        CS2PEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs3pen(&mut self) -> CS3PEN_W<ROUTEPEN_SPEC> {
        CS3PEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alepen(&mut self) -> ALEPEN_W<ROUTEPEN_SPEC> {
        ALEPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ardypen(&mut self) -> ARDYPEN_W<ROUTEPEN_SPEC> {
        ARDYPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\]
Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blpen(&mut self) -> BLPEN_W<ROUTEPEN_SPEC> {
        BLPEN_W::new(self, 7)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nandpen(&mut self) -> NANDPEN_W<ROUTEPEN_SPEC> {
        NANDPEN_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Sets the Lower Bound for EBI_A Enabling"]
    #[inline(always)]
    #[must_use]
    pub fn alb(&mut self) -> ALB_W<ROUTEPEN_SPEC> {
        ALB_W::new(self, 16)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn apen(&mut self) -> APEN_W<ROUTEPEN_SPEC> {
        APEN_W::new(self, 18)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tftpen(&mut self) -> TFTPEN_W<ROUTEPEN_SPEC> {
        TFTPEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - EBI_DATA Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataenpen(&mut self) -> DATAENPEN_W<ROUTEPEN_SPEC> {
        DATAENPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cstftpen(&mut self) -> CSTFTPEN_W<ROUTEPEN_SPEC> {
        CSTFTPEN_W::new(self, 26)
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
#[doc = "I/O Routing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
