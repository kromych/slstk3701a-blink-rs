#[doc = "Register `OPA2_CTRL` reader"]
pub type R = crate::R<Opa2CtrlSpec>;
#[doc = "Register `OPA2_CTRL` writer"]
pub type W = crate::W<Opa2CtrlSpec>;
#[doc = "OPAx Operation Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drivestrength {
    #[doc = "0: Lower accuracy with Low drive strength."]
    _0 = 0,
    #[doc = "1: Low accuracy with Low drive strength."]
    _1 = 1,
    #[doc = "2: High accuracy with High drive strength."]
    _2 = 2,
    #[doc = "3: Higher accuracy with High drive strength."]
    _3 = 3,
}
impl From<Drivestrength> for u8 {
    #[inline(always)]
    fn from(variant: Drivestrength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drivestrength {
    type Ux = u8;
}
impl crate::IsEnum for Drivestrength {}
#[doc = "Field `DRIVESTRENGTH` reader - OPAx Operation Mode"]
pub type DrivestrengthR = crate::FieldReader<Drivestrength>;
impl DrivestrengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drivestrength {
        match self.bits {
            0 => Drivestrength::_0,
            1 => Drivestrength::_1,
            2 => Drivestrength::_2,
            3 => Drivestrength::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lower accuracy with Low drive strength."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drivestrength::_0
    }
    #[doc = "Low accuracy with Low drive strength."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drivestrength::_1
    }
    #[doc = "High accuracy with High drive strength."]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Drivestrength::_2
    }
    #[doc = "Higher accuracy with High drive strength."]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Drivestrength::_3
    }
}
#[doc = "Field `DRIVESTRENGTH` writer - OPAx Operation Mode"]
pub type DrivestrengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Drivestrength, crate::Safe>;
impl<'a, REG> DrivestrengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lower accuracy with Low drive strength."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drivestrength::_0)
    }
    #[doc = "Low accuracy with Low drive strength."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drivestrength::_1)
    }
    #[doc = "High accuracy with High drive strength."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Drivestrength::_2)
    }
    #[doc = "Higher accuracy with High drive strength."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Drivestrength::_3)
    }
}
#[doc = "Field `INCBW` reader - OPAx Unity Gain Bandwidth Scale"]
pub type IncbwR = crate::BitReader;
#[doc = "Field `INCBW` writer - OPAx Unity Gain Bandwidth Scale"]
pub type IncbwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMDIS` reader - High Common Mode Disable"]
pub type HcmdisR = crate::BitReader;
#[doc = "Field `HCMDIS` writer - High Common Mode Disable"]
pub type HcmdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSCALE` reader - Scale OPAx Output Driving Strength"]
pub type OutscaleR = crate::BitReader;
#[doc = "Field `OUTSCALE` writer - Scale OPAx Output Driving Strength"]
pub type OutscaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSEN` reader - OPAx PRS Trigger Enable"]
pub type PrsenR = crate::BitReader;
#[doc = "Field `PRSEN` writer - OPAx PRS Trigger Enable"]
pub type PrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSMODE` reader - OPAx PRS Trigger Mode"]
pub type PrsmodeR = crate::BitReader;
#[doc = "Field `PRSMODE` writer - OPAx PRS Trigger Mode"]
pub type PrsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "OPAx PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS ch 0 triggers OPA."]
    Prsch0 = 0,
    #[doc = "1: PRS ch 1 triggers OPA."]
    Prsch1 = 1,
    #[doc = "2: PRS ch 2 triggers OPA."]
    Prsch2 = 2,
    #[doc = "3: PRS ch 3 triggers OPA."]
    Prsch3 = 3,
    #[doc = "4: PRS ch 4 triggers OPA."]
    Prsch4 = 4,
    #[doc = "5: PRS ch 5 triggers OPA."]
    Prsch5 = 5,
    #[doc = "6: PRS ch 6 triggers OPA."]
    Prsch6 = 6,
    #[doc = "7: PRS ch 7 triggers OPA."]
    Prsch7 = 7,
    #[doc = "8: PRS ch 8 triggers OPA."]
    Prsch8 = 8,
    #[doc = "9: PRS ch 9 triggers OPA."]
    Prsch9 = 9,
    #[doc = "10: PRS ch 10 triggers OPA."]
    Prsch10 = 10,
    #[doc = "11: PRS ch 11 triggers OPA."]
    Prsch11 = 11,
    #[doc = "12: PRS ch 12 triggers OPA."]
    Prsch12 = 12,
    #[doc = "13: PRS ch 13 triggers OPA."]
    Prsch13 = 13,
    #[doc = "14: PRS ch 14 triggers OPA."]
    Prsch14 = 14,
    #[doc = "15: PRS ch 15 triggers OPA."]
    Prsch15 = 15,
    #[doc = "16: PRS ch 16 triggers OPA."]
    Prsch16 = 16,
    #[doc = "17: PRS ch 17 triggers OPA."]
    Prsch17 = 17,
    #[doc = "18: PRS ch 18 triggers OPA."]
    Prsch18 = 18,
    #[doc = "19: PRS ch 19 triggers OPA."]
    Prsch19 = 19,
    #[doc = "20: PRS ch 20 triggers OPA."]
    Prsch20 = 20,
    #[doc = "21: PRS ch 21 triggers OPA."]
    Prsch21 = 21,
    #[doc = "22: PRS ch 22 triggers OPA."]
    Prsch22 = 22,
    #[doc = "23: PRS ch 23 triggers OPA."]
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
#[doc = "Field `PRSSEL` reader - OPAx PRS Trigger Select"]
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
    #[doc = "PRS ch 0 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS ch 1 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS ch 2 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS ch 3 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS ch 4 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS ch 5 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
    #[doc = "PRS ch 6 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel::Prsch6
    }
    #[doc = "PRS ch 7 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel::Prsch7
    }
    #[doc = "PRS ch 8 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel::Prsch8
    }
    #[doc = "PRS ch 9 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel::Prsch9
    }
    #[doc = "PRS ch 10 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel::Prsch10
    }
    #[doc = "PRS ch 11 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel::Prsch11
    }
    #[doc = "PRS ch 12 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel::Prsch12
    }
    #[doc = "PRS ch 13 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel::Prsch13
    }
    #[doc = "PRS ch 14 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel::Prsch14
    }
    #[doc = "PRS ch 15 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel::Prsch15
    }
    #[doc = "PRS ch 16 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel::Prsch16
    }
    #[doc = "PRS ch 17 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel::Prsch17
    }
    #[doc = "PRS ch 18 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel::Prsch18
    }
    #[doc = "PRS ch 19 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel::Prsch19
    }
    #[doc = "PRS ch 20 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel::Prsch20
    }
    #[doc = "PRS ch 21 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel::Prsch21
    }
    #[doc = "PRS ch 22 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel::Prsch22
    }
    #[doc = "PRS ch 23 triggers OPA."]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel::Prsch23
    }
}
#[doc = "Field `PRSSEL` writer - OPAx PRS Trigger Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS ch 0 triggers OPA."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS ch 1 triggers OPA."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS ch 2 triggers OPA."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS ch 3 triggers OPA."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS ch 4 triggers OPA."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS ch 5 triggers OPA."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
    #[doc = "PRS ch 6 triggers OPA."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch6)
    }
    #[doc = "PRS ch 7 triggers OPA."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch7)
    }
    #[doc = "PRS ch 8 triggers OPA."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch8)
    }
    #[doc = "PRS ch 9 triggers OPA."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch9)
    }
    #[doc = "PRS ch 10 triggers OPA."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch10)
    }
    #[doc = "PRS ch 11 triggers OPA."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch11)
    }
    #[doc = "PRS ch 12 triggers OPA."]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch12)
    }
    #[doc = "PRS ch 13 triggers OPA."]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch13)
    }
    #[doc = "PRS ch 14 triggers OPA."]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch14)
    }
    #[doc = "PRS ch 15 triggers OPA."]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch15)
    }
    #[doc = "PRS ch 16 triggers OPA."]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch16)
    }
    #[doc = "PRS ch 17 triggers OPA."]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch17)
    }
    #[doc = "PRS ch 18 triggers OPA."]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch18)
    }
    #[doc = "PRS ch 19 triggers OPA."]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch19)
    }
    #[doc = "PRS ch 20 triggers OPA."]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch20)
    }
    #[doc = "PRS ch 21 triggers OPA."]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch21)
    }
    #[doc = "PRS ch 22 triggers OPA."]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch22)
    }
    #[doc = "PRS ch 23 triggers OPA."]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch23)
    }
}
#[doc = "Field `PRSOUTMODE` reader - OPAx PRS Output Select"]
pub type PrsoutmodeR = crate::BitReader;
#[doc = "Field `PRSOUTMODE` writer - OPAx PRS Output Select"]
pub type PrsoutmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTXMASTERDIS` reader - APORT Bus Master Disable"]
pub type AportxmasterdisR = crate::BitReader;
#[doc = "Field `APORTXMASTERDIS` writer - APORT Bus Master Disable"]
pub type AportxmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTYMASTERDIS` reader - APORT Bus Master Disable"]
pub type AportymasterdisR = crate::BitReader;
#[doc = "Field `APORTYMASTERDIS` writer - APORT Bus Master Disable"]
pub type AportymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DrivestrengthR {
        DrivestrengthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline(always)]
    pub fn incbw(&self) -> IncbwR {
        IncbwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline(always)]
    pub fn hcmdis(&self) -> HcmdisR {
        HcmdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline(always)]
    pub fn outscale(&self) -> OutscaleR {
        OutscaleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PrsenR {
        PrsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&self) -> PrsmodeR {
        PrsmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - OPAx PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline(always)]
    pub fn prsoutmode(&self) -> PrsoutmodeR {
        PrsoutmodeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&self) -> AportxmasterdisR {
        AportxmasterdisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&self) -> AportymasterdisR {
        AportymasterdisR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - OPAx Operation Mode"]
    #[inline(always)]
    pub fn drivestrength(&mut self) -> DrivestrengthW<'_, Opa2CtrlSpec> {
        DrivestrengthW::new(self, 0)
    }
    #[doc = "Bit 2 - OPAx Unity Gain Bandwidth Scale"]
    #[inline(always)]
    pub fn incbw(&mut self) -> IncbwW<'_, Opa2CtrlSpec> {
        IncbwW::new(self, 2)
    }
    #[doc = "Bit 3 - High Common Mode Disable"]
    #[inline(always)]
    pub fn hcmdis(&mut self) -> HcmdisW<'_, Opa2CtrlSpec> {
        HcmdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Scale OPAx Output Driving Strength"]
    #[inline(always)]
    pub fn outscale(&mut self) -> OutscaleW<'_, Opa2CtrlSpec> {
        OutscaleW::new(self, 4)
    }
    #[doc = "Bit 8 - OPAx PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PrsenW<'_, Opa2CtrlSpec> {
        PrsenW::new(self, 8)
    }
    #[doc = "Bit 9 - OPAx PRS Trigger Mode"]
    #[inline(always)]
    pub fn prsmode(&mut self) -> PrsmodeW<'_, Opa2CtrlSpec> {
        PrsmodeW::new(self, 9)
    }
    #[doc = "Bits 10:14 - OPAx PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, Opa2CtrlSpec> {
        PrsselW::new(self, 10)
    }
    #[doc = "Bit 16 - OPAx PRS Output Select"]
    #[inline(always)]
    pub fn prsoutmode(&mut self) -> PrsoutmodeW<'_, Opa2CtrlSpec> {
        PrsoutmodeW::new(self, 16)
    }
    #[doc = "Bit 20 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&mut self) -> AportxmasterdisW<'_, Opa2CtrlSpec> {
        AportxmasterdisW::new(self, 20)
    }
    #[doc = "Bit 21 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&mut self) -> AportymasterdisW<'_, Opa2CtrlSpec> {
        AportymasterdisW::new(self, 21)
    }
}
#[doc = "Operational Amplifier Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opa2CtrlSpec;
impl crate::RegisterSpec for Opa2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa2_ctrl::R`](R) reader structure"]
impl crate::Readable for Opa2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`opa2_ctrl::W`](W) writer structure"]
impl crate::Writable for Opa2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPA2_CTRL to value 0x0e"]
impl crate::Resettable for Opa2CtrlSpec {
    const RESET_VALUE: u32 = 0x0e;
}
