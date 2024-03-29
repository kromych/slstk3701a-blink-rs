#[doc = "Register `HYSTERESIS0` reader"]
pub type R = crate::R<HYSTERESIS0_SPEC>;
#[doc = "Register `HYSTERESIS0` writer"]
pub type W = crate::W<HYSTERESIS0_SPEC>;
#[doc = "Field `HYST` reader - Hysteresis Select When ACMPOUT=0"]
pub type HYST_R = crate::FieldReader<HYST_A>;
#[doc = "Hysteresis Select When ACMPOUT=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: No hysteresis"]
    HYST0 = 0,
    #[doc = "1: 14 mV hysteresis"]
    HYST1 = 1,
    #[doc = "2: 25 mV hysteresis"]
    HYST2 = 2,
    #[doc = "3: 30 mV hysteresis"]
    HYST3 = 3,
    #[doc = "4: 35 mV hysteresis"]
    HYST4 = 4,
    #[doc = "5: 39 mV hysteresis"]
    HYST5 = 5,
    #[doc = "6: 42 mV hysteresis"]
    HYST6 = 6,
    #[doc = "7: 45 mV hysteresis"]
    HYST7 = 7,
    #[doc = "8: No hysteresis"]
    HYST8 = 8,
    #[doc = "9: -14 mV hysteresis"]
    HYST9 = 9,
    #[doc = "10: -25 mV hysteresis"]
    HYST10 = 10,
    #[doc = "11: -30 mV hysteresis"]
    HYST11 = 11,
    #[doc = "12: -35 mV hysteresis"]
    HYST12 = 12,
    #[doc = "13: -39 mV hysteresis"]
    HYST13 = 13,
    #[doc = "14: -42 mV hysteresis"]
    HYST14 = 14,
    #[doc = "15: -45 mV hysteresis"]
    HYST15 = 15,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYST_A {
    type Ux = u8;
}
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::HYST0,
            1 => HYST_A::HYST1,
            2 => HYST_A::HYST2,
            3 => HYST_A::HYST3,
            4 => HYST_A::HYST4,
            5 => HYST_A::HYST5,
            6 => HYST_A::HYST6,
            7 => HYST_A::HYST7,
            8 => HYST_A::HYST8,
            9 => HYST_A::HYST9,
            10 => HYST_A::HYST10,
            11 => HYST_A::HYST11,
            12 => HYST_A::HYST12,
            13 => HYST_A::HYST13,
            14 => HYST_A::HYST14,
            15 => HYST_A::HYST15,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_hyst0(&self) -> bool {
        *self == HYST_A::HYST0
    }
    #[doc = "14 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst1(&self) -> bool {
        *self == HYST_A::HYST1
    }
    #[doc = "25 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst2(&self) -> bool {
        *self == HYST_A::HYST2
    }
    #[doc = "30 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst3(&self) -> bool {
        *self == HYST_A::HYST3
    }
    #[doc = "35 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst4(&self) -> bool {
        *self == HYST_A::HYST4
    }
    #[doc = "39 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst5(&self) -> bool {
        *self == HYST_A::HYST5
    }
    #[doc = "42 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst6(&self) -> bool {
        *self == HYST_A::HYST6
    }
    #[doc = "45 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst7(&self) -> bool {
        *self == HYST_A::HYST7
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_hyst8(&self) -> bool {
        *self == HYST_A::HYST8
    }
    #[doc = "-14 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst9(&self) -> bool {
        *self == HYST_A::HYST9
    }
    #[doc = "-25 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst10(&self) -> bool {
        *self == HYST_A::HYST10
    }
    #[doc = "-30 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst11(&self) -> bool {
        *self == HYST_A::HYST11
    }
    #[doc = "-35 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst12(&self) -> bool {
        *self == HYST_A::HYST12
    }
    #[doc = "-39 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst13(&self) -> bool {
        *self == HYST_A::HYST13
    }
    #[doc = "-42 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst14(&self) -> bool {
        *self == HYST_A::HYST14
    }
    #[doc = "-45 mV hysteresis"]
    #[inline(always)]
    pub fn is_hyst15(&self) -> bool {
        *self == HYST_A::HYST15
    }
}
#[doc = "Field `HYST` writer - Hysteresis Select When ACMPOUT=0"]
pub type HYST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, HYST_A>;
impl<'a, REG> HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn hyst0(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST0)
    }
    #[doc = "14 mV hysteresis"]
    #[inline(always)]
    pub fn hyst1(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST1)
    }
    #[doc = "25 mV hysteresis"]
    #[inline(always)]
    pub fn hyst2(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST2)
    }
    #[doc = "30 mV hysteresis"]
    #[inline(always)]
    pub fn hyst3(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST3)
    }
    #[doc = "35 mV hysteresis"]
    #[inline(always)]
    pub fn hyst4(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST4)
    }
    #[doc = "39 mV hysteresis"]
    #[inline(always)]
    pub fn hyst5(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST5)
    }
    #[doc = "42 mV hysteresis"]
    #[inline(always)]
    pub fn hyst6(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST6)
    }
    #[doc = "45 mV hysteresis"]
    #[inline(always)]
    pub fn hyst7(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST7)
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn hyst8(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST8)
    }
    #[doc = "-14 mV hysteresis"]
    #[inline(always)]
    pub fn hyst9(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST9)
    }
    #[doc = "-25 mV hysteresis"]
    #[inline(always)]
    pub fn hyst10(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST10)
    }
    #[doc = "-30 mV hysteresis"]
    #[inline(always)]
    pub fn hyst11(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST11)
    }
    #[doc = "-35 mV hysteresis"]
    #[inline(always)]
    pub fn hyst12(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST12)
    }
    #[doc = "-39 mV hysteresis"]
    #[inline(always)]
    pub fn hyst13(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST13)
    }
    #[doc = "-42 mV hysteresis"]
    #[inline(always)]
    pub fn hyst14(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST14)
    }
    #[doc = "-45 mV hysteresis"]
    #[inline(always)]
    pub fn hyst15(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::HYST15)
    }
}
#[doc = "Field `DIVVA` reader - Divider for VA Voltage When ACMPOUT=0"]
pub type DIVVA_R = crate::FieldReader;
#[doc = "Field `DIVVA` writer - Divider for VA Voltage When ACMPOUT=0"]
pub type DIVVA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DIVVB` reader - Divider for VB Voltage When ACMPOUT=0"]
pub type DIVVB_R = crate::FieldReader;
#[doc = "Field `DIVVB` writer - Divider for VB Voltage When ACMPOUT=0"]
pub type DIVVB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=0"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=0"]
    #[inline(always)]
    pub fn divva(&self) -> DIVVA_R {
        DIVVA_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=0"]
    #[inline(always)]
    pub fn divvb(&self) -> DIVVB_R {
        DIVVB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=0"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<HYSTERESIS0_SPEC> {
        HYST_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=0"]
    #[inline(always)]
    #[must_use]
    pub fn divva(&mut self) -> DIVVA_W<HYSTERESIS0_SPEC> {
        DIVVA_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=0"]
    #[inline(always)]
    #[must_use]
    pub fn divvb(&mut self) -> DIVVB_W<HYSTERESIS0_SPEC> {
        DIVVB_W::new(self, 24)
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
#[doc = "Hysteresis 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hysteresis0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hysteresis0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HYSTERESIS0_SPEC;
impl crate::RegisterSpec for HYSTERESIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hysteresis0::R`](R) reader structure"]
impl crate::Readable for HYSTERESIS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hysteresis0::W`](W) writer structure"]
impl crate::Writable for HYSTERESIS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSTERESIS0 to value 0"]
impl crate::Resettable for HYSTERESIS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
