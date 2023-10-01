#[doc = "Register `LFBPRESC0` reader"]
pub type R = crate::R<LFBPRESC0_SPEC>;
#[doc = "Register `LFBPRESC0` writer"]
pub type W = crate::W<LFBPRESC0_SPEC>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Prescaler"]
pub type LEUART0_R = crate::FieldReader<LEUART0_A>;
#[doc = "Low Energy UART 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEUART0_A {
    #[doc = "0: LFBCLKLEUART0 = LFBCLK"]
    DIV1 = 0,
    #[doc = "1: LFBCLKLEUART0 = LFBCLK/2"]
    DIV2 = 1,
    #[doc = "2: LFBCLKLEUART0 = LFBCLK/4"]
    DIV4 = 2,
    #[doc = "3: LFBCLKLEUART0 = LFBCLK/8"]
    DIV8 = 3,
}
impl From<LEUART0_A> for u8 {
    #[inline(always)]
    fn from(variant: LEUART0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEUART0_A {
    type Ux = u8;
}
impl LEUART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEUART0_A {
        match self.bits {
            0 => LEUART0_A::DIV1,
            1 => LEUART0_A::DIV2,
            2 => LEUART0_A::DIV4,
            3 => LEUART0_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART0_A::DIV1
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART0_A::DIV2
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART0_A::DIV4
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART0_A::DIV8
    }
}
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Prescaler"]
pub type LEUART0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LEUART0_A>;
impl<'a, REG, const O: u8> LEUART0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0_A::DIV1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0_A::DIV2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0_A::DIV4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0_A::DIV8)
    }
}
#[doc = "Field `LEUART1` reader - Low Energy UART 1 Prescaler"]
pub type LEUART1_R = crate::FieldReader<LEUART1_A>;
#[doc = "Low Energy UART 1 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEUART1_A {
    #[doc = "0: LFBCLKLEUART1 = LFBCLK"]
    DIV1 = 0,
    #[doc = "1: LFBCLKLEUART1 = LFBCLK/2"]
    DIV2 = 1,
    #[doc = "2: LFBCLKLEUART1 = LFBCLK/4"]
    DIV4 = 2,
    #[doc = "3: LFBCLKLEUART1 = LFBCLK/8"]
    DIV8 = 3,
}
impl From<LEUART1_A> for u8 {
    #[inline(always)]
    fn from(variant: LEUART1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEUART1_A {
    type Ux = u8;
}
impl LEUART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEUART1_A {
        match self.bits {
            0 => LEUART1_A::DIV1,
            1 => LEUART1_A::DIV2,
            2 => LEUART1_A::DIV4,
            3 => LEUART1_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART1_A::DIV1
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART1_A::DIV2
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART1_A::DIV4
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART1_A::DIV8
    }
}
#[doc = "Field `LEUART1` writer - Low Energy UART 1 Prescaler"]
pub type LEUART1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LEUART1_A>;
impl<'a, REG, const O: u8> LEUART1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART1_A::DIV1)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART1_A::DIV2)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART1_A::DIV4)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART1_A::DIV8)
    }
}
#[doc = "Field `SYSTICK` reader - Prescaler"]
pub type SYSTICK_R = crate::FieldReader<SYSTICK_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSTICK_A {
    #[doc = "0: LFBCLKSYSTICK = LFBCLK"]
    DIV1 = 0,
}
impl From<SYSTICK_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSTICK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSTICK_A {
    type Ux = u8;
}
impl SYSTICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSTICK_A> {
        match self.bits {
            0 => Some(SYSTICK_A::DIV1),
            _ => None,
        }
    }
    #[doc = "LFBCLKSYSTICK = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYSTICK_A::DIV1
    }
}
#[doc = "Field `CSEN` reader - Capacitive touch sense module Prescaler"]
pub type CSEN_R = crate::FieldReader<CSEN_A>;
#[doc = "Capacitive touch sense module Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEN_A {
    #[doc = "0: LFBCLKCSEN = LFBCLK/16"]
    DIV16 = 0,
    #[doc = "1: LFBCLKCSEN = LFBCLK/32"]
    DIV32 = 1,
    #[doc = "2: LFBCLKCSEN = LFBCLK/64"]
    DIV64 = 2,
    #[doc = "3: LFBCLKCSEN = LFBCLK/128"]
    DIV128 = 3,
}
impl From<CSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSEN_A {
    type Ux = u8;
}
impl CSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEN_A {
        match self.bits {
            0 => CSEN_A::DIV16,
            1 => CSEN_A::DIV32,
            2 => CSEN_A::DIV64,
            3 => CSEN_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CSEN_A::DIV16
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CSEN_A::DIV32
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CSEN_A::DIV64
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CSEN_A::DIV128
    }
}
#[doc = "Field `CSEN` writer - Capacitive touch sense module Prescaler"]
pub type CSEN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CSEN_A>;
impl<'a, REG, const O: u8> CSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CSEN_A::DIV16)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(CSEN_A::DIV32)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(CSEN_A::DIV64)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CSEN_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn systick(&self) -> SYSTICK_R {
        SYSTICK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<LFBPRESC0_SPEC, 0> {
        LEUART0_W::new(self)
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn leuart1(&mut self) -> LEUART1_W<LFBPRESC0_SPEC, 4> {
        LEUART1_W::new(self)
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<LFBPRESC0_SPEC, 12> {
        CSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbpresc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbpresc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFBPRESC0_SPEC;
impl crate::RegisterSpec for LFBPRESC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbpresc0::R`](R) reader structure"]
impl crate::Readable for LFBPRESC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfbpresc0::W`](W) writer structure"]
impl crate::Writable for LFBPRESC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFBPRESC0 to value 0"]
impl crate::Resettable for LFBPRESC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
