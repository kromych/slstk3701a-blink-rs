#[doc = "Register `INPUT` reader"]
pub type R = crate::R<InputSpec>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<InputSpec>;
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxprssel {
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
impl From<Rxprssel> for u8 {
    #[inline(always)]
    fn from(variant: Rxprssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxprssel {
    type Ux = u8;
}
impl crate::IsEnum for Rxprssel {}
#[doc = "Field `RXPRSSEL` reader - RX PRS Channel Select"]
pub type RxprsselR = crate::FieldReader<Rxprssel>;
impl RxprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxprssel> {
        match self.bits {
            0 => Some(Rxprssel::Prsch0),
            1 => Some(Rxprssel::Prsch1),
            2 => Some(Rxprssel::Prsch2),
            3 => Some(Rxprssel::Prsch3),
            4 => Some(Rxprssel::Prsch4),
            5 => Some(Rxprssel::Prsch5),
            6 => Some(Rxprssel::Prsch6),
            7 => Some(Rxprssel::Prsch7),
            8 => Some(Rxprssel::Prsch8),
            9 => Some(Rxprssel::Prsch9),
            10 => Some(Rxprssel::Prsch10),
            11 => Some(Rxprssel::Prsch11),
            12 => Some(Rxprssel::Prsch12),
            13 => Some(Rxprssel::Prsch13),
            14 => Some(Rxprssel::Prsch14),
            15 => Some(Rxprssel::Prsch15),
            16 => Some(Rxprssel::Prsch16),
            17 => Some(Rxprssel::Prsch17),
            18 => Some(Rxprssel::Prsch18),
            19 => Some(Rxprssel::Prsch19),
            20 => Some(Rxprssel::Prsch20),
            21 => Some(Rxprssel::Prsch21),
            22 => Some(Rxprssel::Prsch22),
            23 => Some(Rxprssel::Prsch23),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Rxprssel::Prsch0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Rxprssel::Prsch1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Rxprssel::Prsch2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Rxprssel::Prsch3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Rxprssel::Prsch4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Rxprssel::Prsch5
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Rxprssel::Prsch6
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Rxprssel::Prsch7
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == Rxprssel::Prsch8
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == Rxprssel::Prsch9
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == Rxprssel::Prsch10
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == Rxprssel::Prsch11
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == Rxprssel::Prsch12
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == Rxprssel::Prsch13
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == Rxprssel::Prsch14
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == Rxprssel::Prsch15
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == Rxprssel::Prsch16
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == Rxprssel::Prsch17
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == Rxprssel::Prsch18
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == Rxprssel::Prsch19
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == Rxprssel::Prsch20
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == Rxprssel::Prsch21
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == Rxprssel::Prsch22
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == Rxprssel::Prsch23
    }
}
#[doc = "Field `RXPRSSEL` writer - RX PRS Channel Select"]
pub type RxprsselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Rxprssel>;
impl<'a, REG> RxprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch11)
    }
    #[doc = "PRS Channel 12 selected"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch12)
    }
    #[doc = "PRS Channel 13 selected"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch13)
    }
    #[doc = "PRS Channel 14 selected"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch14)
    }
    #[doc = "PRS Channel 15 selected"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch15)
    }
    #[doc = "PRS Channel 16 selected"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch16)
    }
    #[doc = "PRS Channel 17 selected"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch17)
    }
    #[doc = "PRS Channel 18 selected"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch18)
    }
    #[doc = "PRS Channel 19 selected"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch19)
    }
    #[doc = "PRS Channel 20 selected"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch20)
    }
    #[doc = "PRS Channel 21 selected"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch21)
    }
    #[doc = "PRS Channel 22 selected"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch22)
    }
    #[doc = "PRS Channel 23 selected"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut crate::W<REG> {
        self.variant(Rxprssel::Prsch23)
    }
}
#[doc = "Field `RXPRS` reader - PRS RX Enable"]
pub type RxprsR = crate::BitReader;
#[doc = "Field `RXPRS` writer - PRS RX Enable"]
pub type RxprsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RxprsselR {
        RxprsselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RxprsR {
        RxprsR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&mut self) -> RxprsselW<'_, InputSpec> {
        RxprsselW::new(self, 0)
    }
    #[doc = "Bit 5 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&mut self) -> RxprsW<'_, InputSpec> {
        RxprsW::new(self, 5)
    }
}
#[doc = "LEUART Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputSpec;
impl crate::RegisterSpec for InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for InputSpec {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for InputSpec {}
