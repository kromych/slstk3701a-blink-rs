#[doc = "Register `LFBPRESC0` reader"]
pub type R = crate::R<Lfbpresc0Spec>;
#[doc = "Register `LFBPRESC0` writer"]
pub type W = crate::W<Lfbpresc0Spec>;
#[doc = "Low Energy UART 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leuart0 {
    #[doc = "0: LFBCLKLEUART0 = LFBCLK"]
    Div1 = 0,
    #[doc = "1: LFBCLKLEUART0 = LFBCLK/2"]
    Div2 = 1,
    #[doc = "2: LFBCLKLEUART0 = LFBCLK/4"]
    Div4 = 2,
    #[doc = "3: LFBCLKLEUART0 = LFBCLK/8"]
    Div8 = 3,
}
impl From<Leuart0> for u8 {
    #[inline(always)]
    fn from(variant: Leuart0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leuart0 {
    type Ux = u8;
}
impl crate::IsEnum for Leuart0 {}
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Prescaler"]
pub type Leuart0R = crate::FieldReader<Leuart0>;
impl Leuart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leuart0 {
        match self.bits {
            0 => Leuart0::Div1,
            1 => Leuart0::Div2,
            2 => Leuart0::Div4,
            3 => Leuart0::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Leuart0::Div1
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Leuart0::Div2
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Leuart0::Div4
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Leuart0::Div8
    }
}
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Prescaler"]
pub type Leuart0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Leuart0, crate::Safe>;
impl<'a, REG> Leuart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart0::Div8)
    }
}
#[doc = "Low Energy UART 1 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leuart1 {
    #[doc = "0: LFBCLKLEUART1 = LFBCLK"]
    Div1 = 0,
    #[doc = "1: LFBCLKLEUART1 = LFBCLK/2"]
    Div2 = 1,
    #[doc = "2: LFBCLKLEUART1 = LFBCLK/4"]
    Div4 = 2,
    #[doc = "3: LFBCLKLEUART1 = LFBCLK/8"]
    Div8 = 3,
}
impl From<Leuart1> for u8 {
    #[inline(always)]
    fn from(variant: Leuart1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leuart1 {
    type Ux = u8;
}
impl crate::IsEnum for Leuart1 {}
#[doc = "Field `LEUART1` reader - Low Energy UART 1 Prescaler"]
pub type Leuart1R = crate::FieldReader<Leuart1>;
impl Leuart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leuart1 {
        match self.bits {
            0 => Leuart1::Div1,
            1 => Leuart1::Div2,
            2 => Leuart1::Div4,
            3 => Leuart1::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Leuart1::Div1
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Leuart1::Div2
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Leuart1::Div4
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Leuart1::Div8
    }
}
#[doc = "Field `LEUART1` writer - Low Energy UART 1 Prescaler"]
pub type Leuart1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Leuart1, crate::Safe>;
impl<'a, REG> Leuart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart1::Div1)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart1::Div2)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart1::Div4)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Leuart1::Div8)
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Systick {
    #[doc = "0: LFBCLKSYSTICK = LFBCLK"]
    Div1 = 0,
}
impl From<Systick> for u8 {
    #[inline(always)]
    fn from(variant: Systick) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Systick {
    type Ux = u8;
}
impl crate::IsEnum for Systick {}
#[doc = "Field `SYSTICK` reader - Prescaler"]
pub type SystickR = crate::FieldReader<Systick>;
impl SystickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Systick> {
        match self.bits {
            0 => Some(Systick::Div1),
            _ => None,
        }
    }
    #[doc = "LFBCLKSYSTICK = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Systick::Div1
    }
}
#[doc = "Capacitive touch sense module Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csen {
    #[doc = "0: LFBCLKCSEN = LFBCLK/16"]
    Div16 = 0,
    #[doc = "1: LFBCLKCSEN = LFBCLK/32"]
    Div32 = 1,
    #[doc = "2: LFBCLKCSEN = LFBCLK/64"]
    Div64 = 2,
    #[doc = "3: LFBCLKCSEN = LFBCLK/128"]
    Div128 = 3,
}
impl From<Csen> for u8 {
    #[inline(always)]
    fn from(variant: Csen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csen {
    type Ux = u8;
}
impl crate::IsEnum for Csen {}
#[doc = "Field `CSEN` reader - Capacitive touch sense module Prescaler"]
pub type CsenR = crate::FieldReader<Csen>;
impl CsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csen {
        match self.bits {
            0 => Csen::Div16,
            1 => Csen::Div32,
            2 => Csen::Div64,
            3 => Csen::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Csen::Div16
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Csen::Div32
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Csen::Div64
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Csen::Div128
    }
}
#[doc = "Field `CSEN` writer - Capacitive touch sense module Prescaler"]
pub type CsenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csen, crate::Safe>;
impl<'a, REG> CsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Csen::Div16)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Csen::Div32)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Csen::Div64)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Csen::Div128)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> Leuart0R {
        Leuart0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline(always)]
    pub fn leuart1(&self) -> Leuart1R {
        Leuart1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn systick(&self) -> SystickR {
        SystickR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    pub fn csen(&self) -> CsenR {
        CsenR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> Leuart0W<'_, Lfbpresc0Spec> {
        Leuart0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline(always)]
    pub fn leuart1(&mut self) -> Leuart1W<'_, Lfbpresc0Spec> {
        Leuart1W::new(self, 4)
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    pub fn csen(&mut self) -> CsenW<'_, Lfbpresc0Spec> {
        CsenW::new(self, 12)
    }
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfbpresc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfbpresc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfbpresc0Spec;
impl crate::RegisterSpec for Lfbpresc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbpresc0::R`](R) reader structure"]
impl crate::Readable for Lfbpresc0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfbpresc0::W`](W) writer structure"]
impl crate::Writable for Lfbpresc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFBPRESC0 to value 0"]
impl crate::Resettable for Lfbpresc0Spec {}
