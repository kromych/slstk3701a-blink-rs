#[doc = "Register `OPA1_CTRL` reader"]
pub type R = crate::R<OPA1_CTRL_SPEC>;
#[doc = "Register `OPA1_CTRL` writer"]
pub type W = crate::W<OPA1_CTRL_SPEC>;
#[doc = "Field `DRIVESTRENGTH` reader - OPAx Operation Mode"]
pub type DRIVESTRENGTH_R = crate::FieldReader<DRIVESTRENGTH_A>;
#[doc = "OPAx Operation Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRIVESTRENGTH_A {
    #[doc = "0: Lower accuracy with Low drive strength."]
    _0 = 0,
    #[doc = "1: Low accuracy with Low drive strength."]
    _1 = 1,
    #[doc = "2: High accuracy with High drive strength."]
    _2 = 2,
    #[doc = "3: Higher accuracy with High drive strength."]
    _3 = 3,
}
impl From<DRIVESTRENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVESTRENGTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRIVESTRENGTH_A {
    type Ux = u8;
}
impl DRIVESTRENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVESTRENGTH_A {
        match self.bits {
            0 => DRIVESTRENGTH_A::_0,
            1 => DRIVESTRENGTH_A::_1,
            2 => DRIVESTRENGTH_A::_2,
            3 => DRIVESTRENGTH_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lower accuracy with Low drive strength."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRIVESTRENGTH_A::_0
    }
    #[doc = "Low accuracy with Low drive strength."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRIVESTRENGTH_A::_1
    }
    #[doc = "High accuracy with High drive strength."]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == DRIVESTRENGTH_A::_2
    }
    #[doc = "Higher accuracy with High drive strength."]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == DRIVESTRENGTH_A::_3
    }
}
#[doc = "Field `DRIVESTRENGTH` writer - OPAx Operation Mode"]
pub type DRIVESTRENGTH_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, DRIVESTRENGTH_A>;
impl<'a, REG, const O: u8> DRIVESTRENGTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lower accuracy with Low drive strength."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVESTRENGTH_A::_0)
    }
    #[doc = "Low accuracy with Low drive strength."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVESTRENGTH_A::_1)
    }
    #[doc = "High accuracy with High drive strength."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVESTRENGTH_A::_2)
    }
    #[doc = "Higher accuracy with High drive strength."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVESTRENGTH_A::_3)
    }
}
#[doc = "Field `INCBW` reader - OPAx Unity Gain Bandwidth Scale"]
pub type INCBW_R = crate::BitReader;
#[doc = "Field `INCBW` writer - OPAx Unity Gain Bandwidth Scale"]
pub type INCBW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HCMDIS` reader - High Common Mode Disable"]
pub type HCMDIS_R = crate::BitReader;
#[doc = "Field `HCMDIS` writer - High Common Mode Disable"]
pub type HCMDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTSCALE` reader - Scale OPAx Output Driving Strength"]
pub type OUTSCALE_R = crate::BitReader;
#[doc = "Field `OUTSCALE` writer - Scale OPAx Output Driving Strength"]
pub type OUTSCALE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSEN` reader - OPAx PRS Trigger Enable"]
pub type PRSEN_R = crate::BitReader;
#[doc = "Field `PRSEN` writer - OPAx PRS Trigger Enable"]
pub type PRSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSMODE` reader - OPAx PRS Trigger Mode"]
pub type PRSMODE_R = crate::BitReader;
#[doc = "Field `PRSMODE` writer - OPAx PRS Trigger Mode"]
pub type PRSMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSSEL` reader - OPAx PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<PRSSEL_A>;
#[doc = "OPAx PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers OPA."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers OPA."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers OPA."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers OPA."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers OPA."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers OPA."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers OPA."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers OPA."]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers OPA."]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers OPA."]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers OPA."]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers OPA."]
    PRSCH11 = 11,
    #[doc = "12: PRS ch 12 triggers OPA."]
    PRSCH12 = 12,
    #[doc = "13: PRS ch 13 triggers OPA."]
    PRSCH13 = 13,
    #[doc = "14: PRS ch 14 triggers OPA."]
    PRSCH14 = 14,
    #[doc = "15: PRS ch 15 triggers OPA."]
    PRSCH15 = 15,
    #[doc = "16: PRS ch 16 triggers OPA."]
    PRSCH16 = 16,
    #[doc = "17: PRS ch 17 triggers OPA."]
    PRSCH17 = 17,
    #[doc = "18: PRS ch 18 triggers OPA."]
    PRSCH18 = 18,
    #[doc = "19: PRS ch 19 triggers OPA."]
    PRSCH19 = 19,
    #[doc = "20: PRS ch 20 triggers OPA."]
    PRSCH20 = 20,
    #[doc = "21: PRS ch 21 triggers OPA."]
    PRSCH21 = 21,
    #[doc = "22: PRS ch 22 triggers OPA."]
    PRSCH22 = 22,
    #[doc = "23: PRS ch 23 triggers OPA."]
    PRSCH23 = 23,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL_A {
    type Ux = u8;
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            12 => Some(PRSSEL_A::PRSCH12),
            13 => Some(PRSSEL_A::PRSCH13),
            14 => Some(PRSSEL_A::PRSCH14),
            15 => Some(PRSSEL_A::PRSCH15),
            16 => Some(PRSSEL_A::PRSCH16),
            17 => Some(PRSSEL_A::PRSCH17),
            18 => Some(PRSSEL_A::PRSCH18),
            19 => Some(PRSSEL_A::PRSCH19),
            20 => Some(PRSSEL_A::PRSCH20),
            21 => Some(PRSSEL_A::PRSCH21),
            22 => Some(PRSSEL_A::PRSCH22),
            23 => Some(PRSSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "PRS ch 0 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "PRS ch 1 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "PRS ch 2 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "PRS ch 3 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "PRS ch 4 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "PRS ch 5 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "PRS ch 6 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "PRS ch 7 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "PRS ch 8 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "PRS ch 9 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "PRS ch 10 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "PRS ch 11 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
    #[doc = "PRS ch 12 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == PRSSEL_A::PRSCH12
    }
    #[doc = "PRS ch 13 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == PRSSEL_A::PRSCH13
    }
    #[doc = "PRS ch 14 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == PRSSEL_A::PRSCH14
    }
    #[doc = "PRS ch 15 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == PRSSEL_A::PRSCH15
    }
    #[doc = "PRS ch 16 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == PRSSEL_A::PRSCH16
    }
    #[doc = "PRS ch 17 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == PRSSEL_A::PRSCH17
    }
    #[doc = "PRS ch 18 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == PRSSEL_A::PRSCH18
    }
    #[doc = "PRS ch 19 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == PRSSEL_A::PRSCH19
    }
    #[doc = "PRS ch 20 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == PRSSEL_A::PRSCH20
    }
    #[doc = "PRS ch 21 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == PRSSEL_A::PRSCH21
    }
    #[doc = "PRS ch 22 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == PRSSEL_A::PRSCH22
    }
    #[doc = "PRS ch 23 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == PRSSEL_A::PRSCH23
    }
}
#[doc = "Field `PRSSEL` writer - OPAx PRS Trigger Select"]
pub type PRSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, PRSSEL_A>;
impl<'a, REG, const O: u8> PRSSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers OPA."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers OPA."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers OPA."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers OPA."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers OPA."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers OPA."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers OPA."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers OPA."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers OPA."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers OPA."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers OPA."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers OPA."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH11)
    }
    #[doc = "PRS ch 12 triggers OPA."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH12)
    }
    #[doc = "PRS ch 13 triggers OPA."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH13)
    }
    #[doc = "PRS ch 14 triggers OPA."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH14)
    }
    #[doc = "PRS ch 15 triggers OPA."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH15)
    }
    #[doc = "PRS ch 16 triggers OPA."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH16)
    }
    #[doc = "PRS ch 17 triggers OPA."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH17)
    }
    #[doc = "PRS ch 18 triggers OPA."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH18)
    }
    #[doc = "PRS ch 19 triggers OPA."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH19)
    }
    #[doc = "PRS ch 20 triggers OPA."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH20)
    }
    #[doc = "PRS ch 21 triggers OPA."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH21)
    }
    #[doc = "PRS ch 22 triggers OPA."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH22)
    }
    #[doc = "PRS ch 23 triggers OPA."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL_A::PRSCH23)
    }
}
#[doc = "Field `PRSOUTMODE` reader - OPAx PRS Output Select"]
pub type PRSOUTMODE_R = crate::BitReader;
#[doc = "Field `PRSOUTMODE` writer - OPAx PRS Output Select"]
pub type PRSOUTMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORTXMASTERDIS` reader - APORT Bus Master Disable"]
pub type APORTXMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORTXMASTERDIS` writer - APORT Bus Master Disable"]
pub type APORTXMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APORTYMASTERDIS` reader - APORT Bus Master Disable"]
pub type APORTYMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORTYMASTERDIS` writer - APORT Bus Master Disable"]
pub type APORTYMASTERDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline(always)]
    pub fn incbw(&self) -> INCBW_R {
        INCBW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline(always)]
    pub fn hcmdis(&self) -> HCMDIS_R {
        HCMDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline(always)]
    pub fn outscale(&self) -> OUTSCALE_R {
        OUTSCALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PRSMODE_R {
        PRSMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - OPAx PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline(always)]
    pub fn prsoutmode(&self) -> PRSOUTMODE_R {
        PRSOUTMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&self) -> APORTXMASTERDIS_R {
        APORTXMASTERDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&self) -> APORTYMASTERDIS_R {
        APORTYMASTERDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W<OPA1_CTRL_SPEC, 0> {
        DRIVESTRENGTH_W::new(self)
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline(always)]
    #[must_use]
    pub fn incbw(&mut self) -> INCBW_W<OPA1_CTRL_SPEC, 2> {
        INCBW_W::new(self)
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmdis(&mut self) -> HCMDIS_W<OPA1_CTRL_SPEC, 3> {
        HCMDIS_W::new(self)
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline(always)]
    #[must_use]
    pub fn outscale(&mut self) -> OUTSCALE_W<OPA1_CTRL_SPEC, 4> {
        OUTSCALE_W::new(self)
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsen(&mut self) -> PRSEN_W<OPA1_CTRL_SPEC, 8> {
        PRSEN_W::new(self)
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prsmode(&mut self) -> PRSMODE_W<OPA1_CTRL_SPEC, 9> {
        PRSMODE_W::new(self)
    }
    #[doc = "Bits 10:14 - OPAx PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<OPA1_CTRL_SPEC, 10> {
        PRSSEL_W::new(self)
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn prsoutmode(&mut self) -> PRSOUTMODE_W<OPA1_CTRL_SPEC, 16> {
        PRSOUTMODE_W::new(self)
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportxmasterdis(&mut self) -> APORTXMASTERDIS_W<OPA1_CTRL_SPEC, 20> {
        APORTXMASTERDIS_W::new(self)
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportymasterdis(&mut self) -> APORTYMASTERDIS_W<OPA1_CTRL_SPEC, 21> {
        APORTYMASTERDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Operational Amplifier Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPA1_CTRL_SPEC;
impl crate::RegisterSpec for OPA1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa1_ctrl::R`](R) reader structure"]
impl crate::Readable for OPA1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opa1_ctrl::W`](W) writer structure"]
impl crate::Writable for OPA1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA1_CTRL to value 0x0e"]
impl crate::Resettable for OPA1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
