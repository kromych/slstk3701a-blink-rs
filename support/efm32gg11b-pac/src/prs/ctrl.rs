#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SEVONPRS` reader - Set Event on PRS"]
pub type SevonprsR = crate::BitReader;
#[doc = "Field `SEVONPRS` writer - Set Event on PRS"]
pub type SevonprsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SEVONPRS PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sevonprssel {
    #[doc = "0: PRS Channel 0 selected"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    Prsch11 = 11,
    #[doc = "12: PRS Channel 12 selected"]
    Prsch12 = 12,
    #[doc = "13: PRS Channel 13 selected"]
    Prsch13 = 13,
    #[doc = "14: PRS Channel 14 selected"]
    Prsch14 = 14,
    #[doc = "15: PRS Channel 15 selected"]
    Prsch15 = 15,
    #[doc = "16: PRS Channel 16 selected"]
    Prsch16 = 16,
    #[doc = "17: PRS Channel 17 selected"]
    Prsch17 = 17,
    #[doc = "18: PRS Channel 18 selected"]
    Prsch18 = 18,
    #[doc = "19: PRS Channel 19 selected"]
    Prsch19 = 19,
    #[doc = "20: PRS Channel 20 selected"]
    Prsch20 = 20,
    #[doc = "21: PRS Channel 21 selected"]
    Prsch21 = 21,
    #[doc = "22: PRS Channel 22 selected"]
    Prsch22 = 22,
    #[doc = "23: PRS Channel 23 selected"]
    Prsch23 = 23,
}
impl From<Sevonprssel> for u8 {
    #[inline(always)]
    fn from(variant: Sevonprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sevonprssel {
    type Ux = u8;
}
impl crate::IsEnum for Sevonprssel {}
#[doc = "Field `SEVONPRSSEL` reader - SEVONPRS PRS Channel Select"]
pub type SevonprsselR = crate::FieldReader<Sevonprssel>;
impl SevonprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sevonprssel> {
        match self.bits {
            0 => Some(Sevonprssel::Prsch0),
            1 => Some(Sevonprssel::Prsch1),
            2 => Some(Sevonprssel::Prsch2),
            3 => Some(Sevonprssel::Prsch3),
            4 => Some(Sevonprssel::Prsch4),
            5 => Some(Sevonprssel::Prsch5),
            6 => Some(Sevonprssel::Prsch6),
            7 => Some(Sevonprssel::Prsch7),
            8 => Some(Sevonprssel::Prsch8),
            9 => Some(Sevonprssel::Prsch9),
            10 => Some(Sevonprssel::Prsch10),
            11 => Some(Sevonprssel::Prsch11),
            12 => Some(Sevonprssel::Prsch12),
            13 => Some(Sevonprssel::Prsch13),
            14 => Some(Sevonprssel::Prsch14),
            15 => Some(Sevonprssel::Prsch15),
            16 => Some(Sevonprssel::Prsch16),
            17 => Some(Sevonprssel::Prsch17),
            18 => Some(Sevonprssel::Prsch18),
            19 => Some(Sevonprssel::Prsch19),
            20 => Some(Sevonprssel::Prsch20),
            21 => Some(Sevonprssel::Prsch21),
            22 => Some(Sevonprssel::Prsch22),
            23 => Some(Sevonprssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Sevonprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Sevonprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Sevonprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Sevonprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Sevonprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Sevonprssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Sevonprssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Sevonprssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Sevonprssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Sevonprssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Sevonprssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Sevonprssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Sevonprssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Sevonprssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Sevonprssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Sevonprssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Sevonprssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Sevonprssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Sevonprssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Sevonprssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Sevonprssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Sevonprssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Sevonprssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Sevonprssel::Prsch23
    }
}
#[doc = "Field `SEVONPRSSEL` writer - SEVONPRS PRS Channel Select"]
pub type SevonprsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Sevonprssel>;
impl<'a, REG> SevonprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonprssel::Prsch23)
    }
}
impl R {
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline(always)]
    pub fn sevonprs(&self) -> SevonprsR {
        SevonprsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - SEVONPRS PRS Channel Select"]
    #[inline(always)]
    pub fn sevonprssel(&self) -> SevonprsselR {
        SevonprsselR::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set Event on PRS"]
    #[inline(always)]
    pub fn sevonprs(&mut self) -> SevonprsW<'_, CtrlSpec> {
        SevonprsW::new(self, 0)
    }
    #[doc = "Bits 1:5 - SEVONPRS PRS Channel Select"]
    #[inline(always)]
    pub fn sevonprssel(&mut self) -> SevonprsselW<'_, CtrlSpec> {
        SevonprsselW::new(self, 1)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
