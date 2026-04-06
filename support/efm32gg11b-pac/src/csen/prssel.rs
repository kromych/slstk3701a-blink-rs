#[doc = "Register `PRSSEL` reader"]
pub type R = crate::R<PrsselSpec>;
#[doc = "Register `PRSSEL` writer"]
pub type W = crate::W<PrsselSpec>;
#[doc = "PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prssel {
    #[doc = "0: PRS Channel 0 selected as the start trigger"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as the start trigger"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as the start trigger"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as the start trigger"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as the start trigger"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as the start trigger"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as the start trigger"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as the start trigger"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected as the start trigger"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected as the start trigger"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected as the start trigger"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected as the start trigger"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected as the start trigger"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected as the start trigger"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected as the start trigger"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected as the start trigger"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected as the start trigger"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected as the start trigger"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected as the start trigger"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected as the start trigger"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected as the start trigger"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected as the start trigger"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected as the start trigger"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected as the start trigger"]
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
#[doc = "Field `PRSSEL` reader - PRS Channel Select"]
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
    #[doc = "PRS Channel 0 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Prssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Prssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Prssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Prssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Prssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Prssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Prssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Prssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Prssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Prssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Prssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Prssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Prssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Prssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Prssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Prssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Prssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Prssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Prssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Prssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Prssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Prssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Prssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected as the start trigger"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Prssel::Prsch23
    }
}
#[doc = "Field `PRSSEL` writer - PRS Channel Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Prssel>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected as the start trigger"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Prssel::Prsch23)
    }
}
impl R {
    #[doc = "Bits 0:4 - PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, PrsselSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "PRS Select\n\nYou can [`read`](crate::Reg::read) this register and get [`prssel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prssel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrsselSpec;
impl crate::RegisterSpec for PrsselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prssel::R`](R) reader structure"]
impl crate::Readable for PrsselSpec {}
#[doc = "`write(|w| ..)` method takes [`prssel::W`](W) writer structure"]
impl crate::Writable for PrsselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRSSEL to value 0"]
impl crate::Resettable for PrsselSpec {}
