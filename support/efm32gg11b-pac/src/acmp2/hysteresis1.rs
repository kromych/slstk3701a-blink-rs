#[doc = "Register `HYSTERESIS1` reader"]
pub type R = crate::R<Hysteresis1Spec>;
#[doc = "Register `HYSTERESIS1` writer"]
pub type W = crate::W<Hysteresis1Spec>;
#[doc = "Hysteresis Select When ACMPOUT=1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hyst {
    #[doc = "0: No hysteresis"]
    Hyst0 = 0,
    #[doc = "1: 14 mV hysteresis"]
    Hyst1 = 1,
    #[doc = "2: 25 mV hysteresis"]
    Hyst2 = 2,
    #[doc = "3: 30 mV hysteresis"]
    Hyst3 = 3,
    #[doc = "4: 35 mV hysteresis"]
    Hyst4 = 4,
    #[doc = "5: 39 mV hysteresis"]
    Hyst5 = 5,
    #[doc = "6: 42 mV hysteresis"]
    Hyst6 = 6,
    #[doc = "7: 45 mV hysteresis"]
    Hyst7 = 7,
    #[doc = "8: No hysteresis"]
    Hyst8 = 8,
    #[doc = "9: -14 mV hysteresis"]
    Hyst9 = 9,
    #[doc = "10: -25 mV hysteresis"]
    Hyst10 = 10,
    #[doc = "11: -30 mV hysteresis"]
    Hyst11 = 11,
    #[doc = "12: -35 mV hysteresis"]
    Hyst12 = 12,
    #[doc = "13: -39 mV hysteresis"]
    Hyst13 = 13,
    #[doc = "14: -42 mV hysteresis"]
    Hyst14 = 14,
    #[doc = "15: -45 mV hysteresis"]
    Hyst15 = 15,
}
impl From<Hyst> for u8 {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hyst {
    type Ux = u8;
}
impl crate::IsEnum for Hyst {}
#[doc = "Field `HYST` reader - Hysteresis Select When ACMPOUT=1"]
pub type HystR = crate::FieldReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hyst {
        match self.bits {
            0 => Hyst::Hyst0,
            1 => Hyst::Hyst1,
            2 => Hyst::Hyst2,
            3 => Hyst::Hyst3,
            4 => Hyst::Hyst4,
            5 => Hyst::Hyst5,
            6 => Hyst::Hyst6,
            7 => Hyst::Hyst7,
            8 => Hyst::Hyst8,
            9 => Hyst::Hyst9,
            10 => Hyst::Hyst10,
            11 => Hyst::Hyst11,
            12 => Hyst::Hyst12,
            13 => Hyst::Hyst13,
            14 => Hyst::Hyst14,
            15 => Hyst::Hyst15,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_hyst0(&self) -> bool {
        *self == Hyst::Hyst0
    }
    #[doc = "14 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst1(&self) -> bool {
        *self == Hyst::Hyst1
    }
    #[doc = "25 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst2(&self) -> bool {
        *self == Hyst::Hyst2
    }
    #[doc = "30 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst3(&self) -> bool {
        *self == Hyst::Hyst3
    }
    #[doc = "35 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst4(&self) -> bool {
        *self == Hyst::Hyst4
    }
    #[doc = "39 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst5(&self) -> bool {
        *self == Hyst::Hyst5
    }
    #[doc = "42 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst6(&self) -> bool {
        *self == Hyst::Hyst6
    }
    #[doc = "45 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst7(&self) -> bool {
        *self == Hyst::Hyst7
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_hyst8(&self) -> bool {
        *self == Hyst::Hyst8
    }
    #[doc = "-14 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst9(&self) -> bool {
        *self == Hyst::Hyst9
    }
    #[doc = "-25 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst10(&self) -> bool {
        *self == Hyst::Hyst10
    }
    #[doc = "-30 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst11(&self) -> bool {
        *self == Hyst::Hyst11
    }
    #[doc = "-35 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst12(&self) -> bool {
        *self == Hyst::Hyst12
    }
    #[doc = "-39 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst13(&self) -> bool {
        *self == Hyst::Hyst13
    }
    #[doc = "-42 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst14(&self) -> bool {
        *self == Hyst::Hyst14
    }
    #[doc = "-45 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst15(&self) -> bool {
        *self == Hyst::Hyst15
    }
}
#[doc = "Field `HYST` writer - Hysteresis Select When ACMPOUT=1"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hyst, crate::Safe>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn hyst0(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst0)
    }
    #[doc = "14 mV hysteresis"]
    #[inline(always)]
    pub fn hyst1(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst1)
    }
    #[doc = "25 mV hysteresis"]
    #[inline(always)]
    pub fn hyst2(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst2)
    }
    #[doc = "30 mV hysteresis"]
    #[inline(always)]
    pub fn hyst3(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst3)
    }
    #[doc = "35 mV hysteresis"]
    #[inline(always)]
    pub fn hyst4(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst4)
    }
    #[doc = "39 mV hysteresis"]
    #[inline(always)]
    pub fn hyst5(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst5)
    }
    #[doc = "42 mV hysteresis"]
    #[inline(always)]
    pub fn hyst6(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst6)
    }
    #[doc = "45 mV hysteresis"]
    #[inline(always)]
    pub fn hyst7(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst7)
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn hyst8(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst8)
    }
    #[doc = "-14 mV hysteresis"]
    #[inline(always)]
    pub fn hyst9(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst9)
    }
    #[doc = "-25 mV hysteresis"]
    #[inline(always)]
    pub fn hyst10(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst10)
    }
    #[doc = "-30 mV hysteresis"]
    #[inline(always)]
    pub fn hyst11(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst11)
    }
    #[doc = "-35 mV hysteresis"]
    #[inline(always)]
    pub fn hyst12(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst12)
    }
    #[doc = "-39 mV hysteresis"]
    #[inline(always)]
    pub fn hyst13(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst13)
    }
    #[doc = "-42 mV hysteresis"]
    #[inline(always)]
    pub fn hyst14(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst14)
    }
    #[doc = "-45 mV hysteresis"]
    #[inline(always)]
    pub fn hyst15(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Hyst15)
    }
}
#[doc = "Field `DIVVA` reader - Divider for VA Voltage When ACMPOUT=1"]
pub type DivvaR = crate::FieldReader;
#[doc = "Field `DIVVA` writer - Divider for VA Voltage When ACMPOUT=1"]
pub type DivvaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DIVVB` reader - Divider for VB Voltage When ACMPOUT=1"]
pub type DivvbR = crate::FieldReader;
#[doc = "Field `DIVVB` writer - Divider for VB Voltage When ACMPOUT=1"]
pub type DivvbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=1"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=1"]
    #[inline(always)]
    pub fn divva(&self) -> DivvaR {
        DivvaR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=1"]
    #[inline(always)]
    pub fn divvb(&self) -> DivvbR {
        DivvbR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=1"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, Hysteresis1Spec> {
        HystW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=1"]
    #[inline(always)]
    pub fn divva(&mut self) -> DivvaW<'_, Hysteresis1Spec> {
        DivvaW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=1"]
    #[inline(always)]
    pub fn divvb(&mut self) -> DivvbW<'_, Hysteresis1Spec> {
        DivvbW::new(self, 24)
    }
}
#[doc = "Hysteresis 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hysteresis1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hysteresis1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hysteresis1Spec;
impl crate::RegisterSpec for Hysteresis1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hysteresis1::R`](R) reader structure"]
impl crate::Readable for Hysteresis1Spec {}
#[doc = "`write(|w| ..)` method takes [`hysteresis1::W`](W) writer structure"]
impl crate::Writable for Hysteresis1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HYSTERESIS1 to value 0"]
impl crate::Resettable for Hysteresis1Spec {}
