#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `EBIPEN` reader - EBI Pin Enable"]
pub type EbipenR = crate::BitReader;
#[doc = "Field `EBIPEN` writer - EBI Pin Enable"]
pub type EbipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS0PEN` reader - EBI_CS0 Pin Enable"]
pub type Cs0penR = crate::BitReader;
#[doc = "Field `CS0PEN` writer - EBI_CS0 Pin Enable"]
pub type Cs0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1PEN` reader - EBI_CS1 Pin Enable"]
pub type Cs1penR = crate::BitReader;
#[doc = "Field `CS1PEN` writer - EBI_CS1 Pin Enable"]
pub type Cs1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2PEN` reader - EBI_CS2 Pin Enable"]
pub type Cs2penR = crate::BitReader;
#[doc = "Field `CS2PEN` writer - EBI_CS2 Pin Enable"]
pub type Cs2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS3PEN` reader - EBI_CS3 Pin Enable"]
pub type Cs3penR = crate::BitReader;
#[doc = "Field `CS3PEN` writer - EBI_CS3 Pin Enable"]
pub type Cs3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALEPEN` reader - EBI_ALE Pin Enable"]
pub type AlepenR = crate::BitReader;
#[doc = "Field `ALEPEN` writer - EBI_ALE Pin Enable"]
pub type AlepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARDYPEN` reader - EBI_ARDY Pin Enable"]
pub type ArdypenR = crate::BitReader;
#[doc = "Field `ARDYPEN` writer - EBI_ARDY Pin Enable"]
pub type ArdypenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLPEN` reader - EBI_BL\\[1:0\\] Pin Enable"]
pub type BlpenR = crate::BitReader;
#[doc = "Field `BLPEN` writer - EBI_BL\\[1:0\\] Pin Enable"]
pub type BlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NANDPEN` reader - NANDRE and NANDWE Pin Enable"]
pub type NandpenR = crate::BitReader;
#[doc = "Field `NANDPEN` writer - NANDRE and NANDWE Pin Enable"]
pub type NandpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets the Lower Bound for EBI_A Enabling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Alb {
    #[doc = "0: Address lines from EBI_A\\[0\\] and upwards can be enabled via APEN."]
    A0 = 0,
    #[doc = "1: Address lines from EBI_A\\[8\\] and upwards can be enabled via APEN."]
    A8 = 1,
    #[doc = "2: Address lines from EBI_A\\[16\\] and upwards can be enabled via APEN."]
    A16 = 2,
    #[doc = "3: Address lines from EBI_A\\[24\\] and upwards can be enabled via APEN."]
    A24 = 3,
}
impl From<Alb> for u8 {
    #[inline(always)]
    fn from(variant: Alb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Alb {
    type Ux = u8;
}
impl crate::IsEnum for Alb {}
#[doc = "Field `ALB` reader - Sets the Lower Bound for EBI_A Enabling"]
pub type AlbR = crate::FieldReader<Alb>;
impl AlbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alb {
        match self.bits {
            0 => Alb::A0,
            1 => Alb::A8,
            2 => Alb::A16,
            3 => Alb::A24,
            _ => unreachable!(),
        }
    }
    #[doc = "Address lines from EBI_A\\[0\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == Alb::A0
    }
    #[doc = "Address lines from EBI_A\\[8\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == Alb::A8
    }
    #[doc = "Address lines from EBI_A\\[16\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == Alb::A16
    }
    #[doc = "Address lines from EBI_A\\[24\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == Alb::A24
    }
}
#[doc = "Field `ALB` writer - Sets the Lower Bound for EBI_A Enabling"]
pub type AlbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Alb, crate::Safe>;
impl<'a, REG> AlbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address lines from EBI_A\\[0\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(Alb::A0)
    }
    #[doc = "Address lines from EBI_A\\[8\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut crate::W<REG> {
        self.variant(Alb::A8)
    }
    #[doc = "Address lines from EBI_A\\[16\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut crate::W<REG> {
        self.variant(Alb::A16)
    }
    #[doc = "Address lines from EBI_A\\[24\\] and upwards can be enabled via APEN."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut crate::W<REG> {
        self.variant(Alb::A24)
    }
}
#[doc = "EBI_A Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apen {
    #[doc = "0: All EBI_A pins are disabled."]
    A0 = 0,
    #[doc = "5: EBI_A\\[4:L\\] pins enabled."]
    A5 = 5,
    #[doc = "6: EBI_A\\[5:L\\] pins enabled."]
    A6 = 6,
    #[doc = "7: EBI_A\\[6:L\\] pins enabled."]
    A7 = 7,
    #[doc = "8: EBI_A\\[7:L\\] pins enabled."]
    A8 = 8,
    #[doc = "9: EBI_A\\[8:L\\] pins enabled."]
    A9 = 9,
    #[doc = "10: EBI_A\\[9:L\\] pins enabled."]
    A10 = 10,
    #[doc = "11: EBI_A\\[10:L\\] pins enabled."]
    A11 = 11,
    #[doc = "12: EBI_A\\[11:L\\] pins enabled."]
    A12 = 12,
    #[doc = "13: EBI_A\\[12:L\\] pins enabled."]
    A13 = 13,
    #[doc = "14: EBI_A\\[13:L\\] pins enabled."]
    A14 = 14,
    #[doc = "15: EBI_A\\[14:L\\] pins enabled."]
    A15 = 15,
    #[doc = "16: EBI_A\\[15:L\\] pins enabled."]
    A16 = 16,
    #[doc = "17: EBI_A\\[16:L\\] pins enabled."]
    A17 = 17,
    #[doc = "18: EBI_A\\[17:L\\] pins enabled."]
    A18 = 18,
    #[doc = "19: EBI_A\\[18:L\\] pins enabled."]
    A19 = 19,
    #[doc = "20: EBI_A\\[19:L\\] pins enabled."]
    A20 = 20,
    #[doc = "21: EBI_A\\[20:L\\] pins enabled."]
    A21 = 21,
    #[doc = "22: EBI_A\\[21:L\\] pins enabled."]
    A22 = 22,
    #[doc = "23: EBI_A\\[22:L\\] pins enabled."]
    A23 = 23,
    #[doc = "24: EBI_A\\[23:L\\] pins enabled."]
    A24 = 24,
    #[doc = "25: EBI_A\\[24:L\\] pins enabled."]
    A25 = 25,
    #[doc = "26: EBI_A\\[25:L\\] pins enabled."]
    A26 = 26,
    #[doc = "27: EBI_A\\[26:L\\] pins enabled."]
    A27 = 27,
    #[doc = "28: EBI_A\\[27:L\\] pins enabled."]
    A28 = 28,
}
impl From<Apen> for u8 {
    #[inline(always)]
    fn from(variant: Apen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apen {
    type Ux = u8;
}
impl crate::IsEnum for Apen {}
#[doc = "Field `APEN` reader - EBI_A Pin Enable"]
pub type ApenR = crate::FieldReader<Apen>;
impl ApenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Apen> {
        match self.bits {
            0 => Some(Apen::A0),
            5 => Some(Apen::A5),
            6 => Some(Apen::A6),
            7 => Some(Apen::A7),
            8 => Some(Apen::A8),
            9 => Some(Apen::A9),
            10 => Some(Apen::A10),
            11 => Some(Apen::A11),
            12 => Some(Apen::A12),
            13 => Some(Apen::A13),
            14 => Some(Apen::A14),
            15 => Some(Apen::A15),
            16 => Some(Apen::A16),
            17 => Some(Apen::A17),
            18 => Some(Apen::A18),
            19 => Some(Apen::A19),
            20 => Some(Apen::A20),
            21 => Some(Apen::A21),
            22 => Some(Apen::A22),
            23 => Some(Apen::A23),
            24 => Some(Apen::A24),
            25 => Some(Apen::A25),
            26 => Some(Apen::A26),
            27 => Some(Apen::A27),
            28 => Some(Apen::A28),
            _ => None,
        }
    }
    #[doc = "All EBI_A pins are disabled."]
    #[inline(always)]
    pub fn is_a0(&self) -> bool {
        *self == Apen::A0
    }
    #[doc = "EBI_A\\[4:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a5(&self) -> bool {
        *self == Apen::A5
    }
    #[doc = "EBI_A\\[5:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a6(&self) -> bool {
        *self == Apen::A6
    }
    #[doc = "EBI_A\\[6:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a7(&self) -> bool {
        *self == Apen::A7
    }
    #[doc = "EBI_A\\[7:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == Apen::A8
    }
    #[doc = "EBI_A\\[8:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a9(&self) -> bool {
        *self == Apen::A9
    }
    #[doc = "EBI_A\\[9:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a10(&self) -> bool {
        *self == Apen::A10
    }
    #[doc = "EBI_A\\[10:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a11(&self) -> bool {
        *self == Apen::A11
    }
    #[doc = "EBI_A\\[11:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a12(&self) -> bool {
        *self == Apen::A12
    }
    #[doc = "EBI_A\\[12:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a13(&self) -> bool {
        *self == Apen::A13
    }
    #[doc = "EBI_A\\[13:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a14(&self) -> bool {
        *self == Apen::A14
    }
    #[doc = "EBI_A\\[14:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a15(&self) -> bool {
        *self == Apen::A15
    }
    #[doc = "EBI_A\\[15:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a16(&self) -> bool {
        *self == Apen::A16
    }
    #[doc = "EBI_A\\[16:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a17(&self) -> bool {
        *self == Apen::A17
    }
    #[doc = "EBI_A\\[17:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a18(&self) -> bool {
        *self == Apen::A18
    }
    #[doc = "EBI_A\\[18:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a19(&self) -> bool {
        *self == Apen::A19
    }
    #[doc = "EBI_A\\[19:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a20(&self) -> bool {
        *self == Apen::A20
    }
    #[doc = "EBI_A\\[20:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a21(&self) -> bool {
        *self == Apen::A21
    }
    #[doc = "EBI_A\\[21:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a22(&self) -> bool {
        *self == Apen::A22
    }
    #[doc = "EBI_A\\[22:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a23(&self) -> bool {
        *self == Apen::A23
    }
    #[doc = "EBI_A\\[23:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a24(&self) -> bool {
        *self == Apen::A24
    }
    #[doc = "EBI_A\\[24:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a25(&self) -> bool {
        *self == Apen::A25
    }
    #[doc = "EBI_A\\[25:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a26(&self) -> bool {
        *self == Apen::A26
    }
    #[doc = "EBI_A\\[26:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a27(&self) -> bool {
        *self == Apen::A27
    }
    #[doc = "EBI_A\\[27:L\\] pins enabled."]
    #[inline(always)]
    pub fn is_a28(&self) -> bool {
        *self == Apen::A28
    }
}
#[doc = "Field `APEN` writer - EBI_A Pin Enable"]
pub type ApenW<'a, REG> = crate::FieldWriter<'a, REG, 5, Apen>;
impl<'a, REG> ApenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All EBI_A pins are disabled."]
    #[inline(always)]
    pub fn a0(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A0)
    }
    #[doc = "EBI_A\\[4:L\\] pins enabled."]
    #[inline(always)]
    pub fn a5(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A5)
    }
    #[doc = "EBI_A\\[5:L\\] pins enabled."]
    #[inline(always)]
    pub fn a6(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A6)
    }
    #[doc = "EBI_A\\[6:L\\] pins enabled."]
    #[inline(always)]
    pub fn a7(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A7)
    }
    #[doc = "EBI_A\\[7:L\\] pins enabled."]
    #[inline(always)]
    pub fn a8(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A8)
    }
    #[doc = "EBI_A\\[8:L\\] pins enabled."]
    #[inline(always)]
    pub fn a9(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A9)
    }
    #[doc = "EBI_A\\[9:L\\] pins enabled."]
    #[inline(always)]
    pub fn a10(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A10)
    }
    #[doc = "EBI_A\\[10:L\\] pins enabled."]
    #[inline(always)]
    pub fn a11(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A11)
    }
    #[doc = "EBI_A\\[11:L\\] pins enabled."]
    #[inline(always)]
    pub fn a12(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A12)
    }
    #[doc = "EBI_A\\[12:L\\] pins enabled."]
    #[inline(always)]
    pub fn a13(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A13)
    }
    #[doc = "EBI_A\\[13:L\\] pins enabled."]
    #[inline(always)]
    pub fn a14(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A14)
    }
    #[doc = "EBI_A\\[14:L\\] pins enabled."]
    #[inline(always)]
    pub fn a15(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A15)
    }
    #[doc = "EBI_A\\[15:L\\] pins enabled."]
    #[inline(always)]
    pub fn a16(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A16)
    }
    #[doc = "EBI_A\\[16:L\\] pins enabled."]
    #[inline(always)]
    pub fn a17(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A17)
    }
    #[doc = "EBI_A\\[17:L\\] pins enabled."]
    #[inline(always)]
    pub fn a18(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A18)
    }
    #[doc = "EBI_A\\[18:L\\] pins enabled."]
    #[inline(always)]
    pub fn a19(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A19)
    }
    #[doc = "EBI_A\\[19:L\\] pins enabled."]
    #[inline(always)]
    pub fn a20(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A20)
    }
    #[doc = "EBI_A\\[20:L\\] pins enabled."]
    #[inline(always)]
    pub fn a21(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A21)
    }
    #[doc = "EBI_A\\[21:L\\] pins enabled."]
    #[inline(always)]
    pub fn a22(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A22)
    }
    #[doc = "EBI_A\\[22:L\\] pins enabled."]
    #[inline(always)]
    pub fn a23(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A23)
    }
    #[doc = "EBI_A\\[23:L\\] pins enabled."]
    #[inline(always)]
    pub fn a24(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A24)
    }
    #[doc = "EBI_A\\[24:L\\] pins enabled."]
    #[inline(always)]
    pub fn a25(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A25)
    }
    #[doc = "EBI_A\\[25:L\\] pins enabled."]
    #[inline(always)]
    pub fn a26(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A26)
    }
    #[doc = "EBI_A\\[26:L\\] pins enabled."]
    #[inline(always)]
    pub fn a27(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A27)
    }
    #[doc = "EBI_A\\[27:L\\] pins enabled."]
    #[inline(always)]
    pub fn a28(self) -> &'a mut crate::W<REG> {
        self.variant(Apen::A28)
    }
}
#[doc = "Field `TFTPEN` reader - EBI_TFT Pin Enable"]
pub type TftpenR = crate::BitReader;
#[doc = "Field `TFTPEN` writer - EBI_TFT Pin Enable"]
pub type TftpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENPEN` reader - EBI_DATA Pin Enable"]
pub type DataenpenR = crate::BitReader;
#[doc = "Field `DATAENPEN` writer - EBI_DATA Pin Enable"]
pub type DataenpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTFTPEN` reader - EBI_CSTFT Pin Enable"]
pub type CstftpenR = crate::BitReader;
#[doc = "Field `CSTFTPEN` writer - EBI_CSTFT Pin Enable"]
pub type CstftpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&self) -> EbipenR {
        EbipenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> Cs0penR {
        Cs0penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> Cs1penR {
        Cs1penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&self) -> Cs2penR {
        Cs2penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&self) -> Cs3penR {
        Cs3penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&self) -> AlepenR {
        AlepenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&self) -> ArdypenR {
        ArdypenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\] Pin Enable"]
    #[inline(always)]
    pub fn blpen(&self) -> BlpenR {
        BlpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&self) -> NandpenR {
        NandpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the Lower Bound for EBI_A Enabling"]
    #[inline(always)]
    pub fn alb(&self) -> AlbR {
        AlbR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&self) -> ApenR {
        ApenR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&self) -> TftpenR {
        TftpenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EBI_DATA Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&self) -> DataenpenR {
        DataenpenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&self) -> CstftpenR {
        CstftpenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&mut self) -> EbipenW<'_, RoutepenSpec> {
        EbipenW::new(self, 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&mut self) -> Cs0penW<'_, RoutepenSpec> {
        Cs0penW::new(self, 1)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&mut self) -> Cs1penW<'_, RoutepenSpec> {
        Cs1penW::new(self, 2)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&mut self) -> Cs2penW<'_, RoutepenSpec> {
        Cs2penW::new(self, 3)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&mut self) -> Cs3penW<'_, RoutepenSpec> {
        Cs3penW::new(self, 4)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&mut self) -> AlepenW<'_, RoutepenSpec> {
        AlepenW::new(self, 5)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&mut self) -> ArdypenW<'_, RoutepenSpec> {
        ArdypenW::new(self, 6)
    }
    #[doc = "Bit 7 - EBI_BL\\[1:0\\] Pin Enable"]
    #[inline(always)]
    pub fn blpen(&mut self) -> BlpenW<'_, RoutepenSpec> {
        BlpenW::new(self, 7)
    }
    #[doc = "Bit 12 - NANDRE and NANDWE Pin Enable"]
    #[inline(always)]
    pub fn nandpen(&mut self) -> NandpenW<'_, RoutepenSpec> {
        NandpenW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Sets the Lower Bound for EBI_A Enabling"]
    #[inline(always)]
    pub fn alb(&mut self) -> AlbW<'_, RoutepenSpec> {
        AlbW::new(self, 16)
    }
    #[doc = "Bits 18:22 - EBI_A Pin Enable"]
    #[inline(always)]
    pub fn apen(&mut self) -> ApenW<'_, RoutepenSpec> {
        ApenW::new(self, 18)
    }
    #[doc = "Bit 24 - EBI_TFT Pin Enable"]
    #[inline(always)]
    pub fn tftpen(&mut self) -> TftpenW<'_, RoutepenSpec> {
        TftpenW::new(self, 24)
    }
    #[doc = "Bit 25 - EBI_DATA Pin Enable"]
    #[inline(always)]
    pub fn dataenpen(&mut self) -> DataenpenW<'_, RoutepenSpec> {
        DataenpenW::new(self, 25)
    }
    #[doc = "Bit 26 - EBI_CSTFT Pin Enable"]
    #[inline(always)]
    pub fn cstftpen(&mut self) -> CstftpenW<'_, RoutepenSpec> {
        CstftpenW::new(self, 26)
    }
}
#[doc = "I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoutepenSpec;
impl crate::RegisterSpec for RoutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for RoutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for RoutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for RoutepenSpec {}
