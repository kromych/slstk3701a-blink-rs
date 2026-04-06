#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DtctrlSpec>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DtctrlSpec>;
#[doc = "Field `DTEN` reader - DTI Enable"]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - DTI Enable"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub type DtdasR = crate::BitReader;
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub type DtdasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub type DtipolR = crate::BitReader;
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub type DtipolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert"]
pub type DtcinvR = crate::BitReader;
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert"]
pub type DtcinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DTI PRS Source Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtprssel {
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
impl From<Dtprssel> for u8 {
    #[inline(always)]
    fn from(variant: Dtprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtprssel {
    type Ux = u8;
}
impl crate::IsEnum for Dtprssel {}
#[doc = "Field `DTPRSSEL` reader - DTI PRS Source Channel Select"]
pub type DtprsselR = crate::FieldReader<Dtprssel>;
impl DtprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtprssel> {
        match self.bits {
            0 => Some(Dtprssel::Prsch0),
            1 => Some(Dtprssel::Prsch1),
            2 => Some(Dtprssel::Prsch2),
            3 => Some(Dtprssel::Prsch3),
            4 => Some(Dtprssel::Prsch4),
            5 => Some(Dtprssel::Prsch5),
            6 => Some(Dtprssel::Prsch6),
            7 => Some(Dtprssel::Prsch7),
            8 => Some(Dtprssel::Prsch8),
            9 => Some(Dtprssel::Prsch9),
            10 => Some(Dtprssel::Prsch10),
            11 => Some(Dtprssel::Prsch11),
            12 => Some(Dtprssel::Prsch12),
            13 => Some(Dtprssel::Prsch13),
            14 => Some(Dtprssel::Prsch14),
            15 => Some(Dtprssel::Prsch15),
            16 => Some(Dtprssel::Prsch16),
            17 => Some(Dtprssel::Prsch17),
            18 => Some(Dtprssel::Prsch18),
            19 => Some(Dtprssel::Prsch19),
            20 => Some(Dtprssel::Prsch20),
            21 => Some(Dtprssel::Prsch21),
            22 => Some(Dtprssel::Prsch22),
            23 => Some(Dtprssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Dtprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Dtprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Dtprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Dtprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Dtprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Dtprssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Dtprssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Dtprssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Dtprssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Dtprssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Dtprssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Dtprssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Dtprssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Dtprssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Dtprssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Dtprssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Dtprssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Dtprssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Dtprssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Dtprssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Dtprssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Dtprssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Dtprssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Dtprssel::Prsch23
    }
}
#[doc = "Field `DTPRSSEL` writer - DTI PRS Source Channel Select"]
pub type DtprsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dtprssel>;
impl<'a, REG> DtprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as input"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as input"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as input"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as input"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as input"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as input"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as input"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as input"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as input"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as input"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as input"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as input"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as input"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as input"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as input"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as input"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprssel::Prsch23)
    }
}
#[doc = "Field `DTAR` reader - DTI Always Run"]
pub type DtarR = crate::BitReader;
#[doc = "Field `DTAR` writer - DTI Always Run"]
pub type DtarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFATS` reader - DTI Fault Action on Timer Stop"]
pub type DtfatsR = crate::BitReader;
#[doc = "Field `DTFATS` writer - DTI Fault Action on Timer Stop"]
pub type DtfatsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub type DtprsenR = crate::BitReader;
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub type DtprsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DtdasR {
        DtdasR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DtipolR {
        DtipolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&self) -> DtcinvR {
        DtcinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DtprsselR {
        DtprsselR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&self) -> DtarR {
        DtarR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&self) -> DtfatsR {
        DtfatsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DtprsenR {
        DtprsenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DtenW<'_, DtctrlSpec> {
        DtenW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DtdasW<'_, DtctrlSpec> {
        DtdasW::new(self, 1)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DtipolW<'_, DtctrlSpec> {
        DtipolW::new(self, 2)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DtcinvW<'_, DtctrlSpec> {
        DtcinvW::new(self, 3)
    }
    #[doc = "Bits 4:8 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&mut self) -> DtprsselW<'_, DtctrlSpec> {
        DtprsselW::new(self, 4)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&mut self) -> DtarW<'_, DtctrlSpec> {
        DtarW::new(self, 9)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&mut self) -> DtfatsW<'_, DtctrlSpec> {
        DtfatsW::new(self, 10)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DtprsenW<'_, DtctrlSpec> {
        DtprsenW::new(self, 24)
    }
}
#[doc = "DTI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtctrlSpec;
impl crate::RegisterSpec for DtctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DtctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DtctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DtctrlSpec {}
