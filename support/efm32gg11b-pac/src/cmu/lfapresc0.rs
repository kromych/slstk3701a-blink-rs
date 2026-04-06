#[doc = "Register `LFAPRESC0` reader"]
pub type R = crate::R<Lfapresc0Spec>;
#[doc = "Register `LFAPRESC0` writer"]
pub type W = crate::W<Lfapresc0Spec>;
#[doc = "Low Energy Timer 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Letimer0 {
    #[doc = "0: LFACLKLETIMER0 = LFACLK"]
    Div1 = 0,
    #[doc = "1: LFACLKLETIMER0 = LFACLK/2"]
    Div2 = 1,
    #[doc = "2: LFACLKLETIMER0 = LFACLK/4"]
    Div4 = 2,
    #[doc = "3: LFACLKLETIMER0 = LFACLK/8"]
    Div8 = 3,
    #[doc = "4: LFACLKLETIMER0 = LFACLK/16"]
    Div16 = 4,
    #[doc = "5: LFACLKLETIMER0 = LFACLK/32"]
    Div32 = 5,
    #[doc = "6: LFACLKLETIMER0 = LFACLK/64"]
    Div64 = 6,
    #[doc = "7: LFACLKLETIMER0 = LFACLK/128"]
    Div128 = 7,
    #[doc = "8: LFACLKLETIMER0 = LFACLK/256"]
    Div256 = 8,
    #[doc = "9: LFACLKLETIMER0 = LFACLK/512"]
    Div512 = 9,
    #[doc = "10: LFACLKLETIMER0 = LFACLK/1024"]
    Div1024 = 10,
    #[doc = "11: LFACLKLETIMER0 = LFACLK/2048"]
    Div2048 = 11,
    #[doc = "12: LFACLKLETIMER0 = LFACLK/4096"]
    Div4096 = 12,
    #[doc = "13: LFACLKLETIMER0 = LFACLK/8192"]
    Div8192 = 13,
    #[doc = "14: LFACLKLETIMER0 = LFACLK/16384"]
    Div16384 = 14,
    #[doc = "15: LFACLKLETIMER0 = LFACLK/32768"]
    Div32768 = 15,
}
impl From<Letimer0> for u8 {
    #[inline(always)]
    fn from(variant: Letimer0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Letimer0 {
    type Ux = u8;
}
impl crate::IsEnum for Letimer0 {}
#[doc = "Field `LETIMER0` reader - Low Energy Timer 0 Prescaler"]
pub type Letimer0R = crate::FieldReader<Letimer0>;
impl Letimer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Letimer0 {
        match self.bits {
            0 => Letimer0::Div1,
            1 => Letimer0::Div2,
            2 => Letimer0::Div4,
            3 => Letimer0::Div8,
            4 => Letimer0::Div16,
            5 => Letimer0::Div32,
            6 => Letimer0::Div64,
            7 => Letimer0::Div128,
            8 => Letimer0::Div256,
            9 => Letimer0::Div512,
            10 => Letimer0::Div1024,
            11 => Letimer0::Div2048,
            12 => Letimer0::Div4096,
            13 => Letimer0::Div8192,
            14 => Letimer0::Div16384,
            15 => Letimer0::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Letimer0::Div1
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Letimer0::Div2
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Letimer0::Div4
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Letimer0::Div8
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Letimer0::Div16
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Letimer0::Div32
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Letimer0::Div64
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Letimer0::Div128
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Letimer0::Div256
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Letimer0::Div512
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Letimer0::Div1024
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Letimer0::Div2048
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Letimer0::Div4096
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Letimer0::Div8192
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Letimer0::Div16384
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Letimer0::Div32768
    }
}
#[doc = "Field `LETIMER0` writer - Low Energy Timer 0 Prescaler"]
pub type Letimer0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Letimer0, crate::Safe>;
impl<'a, REG> Letimer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLKLETIMER0 = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div1)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div2)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div4)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div8)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div16)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div32)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div64)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div128)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div256)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div512)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div1024)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div2048)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div4096)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div8192)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div16384)
    }
    #[doc = "LFACLKLETIMER0 = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer0::Div32768)
    }
}
#[doc = "Low Energy Timer 1 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Letimer1 {
    #[doc = "0: LFACLKLETIMER1 = LFACLK"]
    Div1 = 0,
    #[doc = "1: LFACLKLETIMER1 = LFACLK/2"]
    Div2 = 1,
    #[doc = "2: LFACLKLETIMER1 = LFACLK/4"]
    Div4 = 2,
    #[doc = "3: LFACLKLETIMER1 = LFACLK/8"]
    Div8 = 3,
    #[doc = "4: LFACLKLETIMER1 = LFACLK/16"]
    Div16 = 4,
    #[doc = "5: LFACLKLETIMER1 = LFACLK/32"]
    Div32 = 5,
    #[doc = "6: LFACLKLETIMER1 = LFACLK/64"]
    Div64 = 6,
    #[doc = "7: LFACLKLETIMER1 = LFACLK/128"]
    Div128 = 7,
    #[doc = "8: LFACLKLETIMER1 = LFACLK/256"]
    Div256 = 8,
    #[doc = "9: LFACLKLETIMER1 = LFACLK/512"]
    Div512 = 9,
    #[doc = "10: LFACLKLETIMER1 = LFACLK/1024"]
    Div1024 = 10,
    #[doc = "11: LFACLKLETIMER1 = LFACLK/2048"]
    Div2048 = 11,
    #[doc = "12: LFACLKLETIMER1 = LFACLK/4096"]
    Div4096 = 12,
    #[doc = "13: LFACLKLETIMER1 = LFACLK/8192"]
    Div8192 = 13,
    #[doc = "14: LFACLKLETIMER1 = LFACLK/16384"]
    Div16384 = 14,
    #[doc = "15: LFACLKLETIMER1 = LFACLK/32768"]
    Div32768 = 15,
}
impl From<Letimer1> for u8 {
    #[inline(always)]
    fn from(variant: Letimer1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Letimer1 {
    type Ux = u8;
}
impl crate::IsEnum for Letimer1 {}
#[doc = "Field `LETIMER1` reader - Low Energy Timer 1 Prescaler"]
pub type Letimer1R = crate::FieldReader<Letimer1>;
impl Letimer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Letimer1 {
        match self.bits {
            0 => Letimer1::Div1,
            1 => Letimer1::Div2,
            2 => Letimer1::Div4,
            3 => Letimer1::Div8,
            4 => Letimer1::Div16,
            5 => Letimer1::Div32,
            6 => Letimer1::Div64,
            7 => Letimer1::Div128,
            8 => Letimer1::Div256,
            9 => Letimer1::Div512,
            10 => Letimer1::Div1024,
            11 => Letimer1::Div2048,
            12 => Letimer1::Div4096,
            13 => Letimer1::Div8192,
            14 => Letimer1::Div16384,
            15 => Letimer1::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLKLETIMER1 = LFACLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Letimer1::Div1
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Letimer1::Div2
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Letimer1::Div4
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Letimer1::Div8
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Letimer1::Div16
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Letimer1::Div32
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Letimer1::Div64
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Letimer1::Div128
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Letimer1::Div256
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Letimer1::Div512
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Letimer1::Div1024
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Letimer1::Div2048
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Letimer1::Div4096
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Letimer1::Div8192
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Letimer1::Div16384
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Letimer1::Div32768
    }
}
#[doc = "Field `LETIMER1` writer - Low Energy Timer 1 Prescaler"]
pub type Letimer1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Letimer1, crate::Safe>;
impl<'a, REG> Letimer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLKLETIMER1 = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div1)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div2)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div4)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div8)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div16)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div32)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div64)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div128)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div256)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div512)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div1024)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div2048)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div4096)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div8192)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div16384)
    }
    #[doc = "LFACLKLETIMER1 = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Letimer1::Div32768)
    }
}
#[doc = "Low Energy Sensor Interface Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lesense {
    #[doc = "0: LFACLKLESENSE = LFACLK"]
    Div1 = 0,
    #[doc = "1: LFACLKLESENSE = LFACLK/2"]
    Div2 = 1,
    #[doc = "2: LFACLKLESENSE = LFACLK/4"]
    Div4 = 2,
    #[doc = "3: LFACLKLESENSE = LFACLK/8"]
    Div8 = 3,
}
impl From<Lesense> for u8 {
    #[inline(always)]
    fn from(variant: Lesense) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lesense {
    type Ux = u8;
}
impl crate::IsEnum for Lesense {}
#[doc = "Field `LESENSE` reader - Low Energy Sensor Interface Prescaler"]
pub type LesenseR = crate::FieldReader<Lesense>;
impl LesenseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lesense {
        match self.bits {
            0 => Lesense::Div1,
            1 => Lesense::Div2,
            2 => Lesense::Div4,
            3 => Lesense::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLKLESENSE = LFACLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Lesense::Div1
    }
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Lesense::Div2
    }
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Lesense::Div4
    }
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Lesense::Div8
    }
}
#[doc = "Field `LESENSE` writer - Low Energy Sensor Interface Prescaler"]
pub type LesenseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lesense, crate::Safe>;
impl<'a, REG> LesenseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLKLESENSE = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Lesense::Div1)
    }
    #[doc = "LFACLKLESENSE = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Lesense::Div2)
    }
    #[doc = "LFACLKLESENSE = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Lesense::Div4)
    }
    #[doc = "LFACLKLESENSE = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Lesense::Div8)
    }
}
#[doc = "Liquid Crystal Display Controller Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcd {
    #[doc = "0: LFACLKLCD = LFACLK"]
    Div1 = 0,
    #[doc = "1: LFACLKLCD = LFACLK/2"]
    Div2 = 1,
    #[doc = "2: LFACLKLCD = LFACLK/4"]
    Div4 = 2,
    #[doc = "3: LFACLKLCD = LFACLK/8"]
    Div8 = 3,
    #[doc = "4: LFACLKLCD = LFACLK/16"]
    Div16 = 4,
    #[doc = "5: LFACLKLCD = LFACLK/32"]
    Div32 = 5,
    #[doc = "6: LFACLKLCD = LFACLK/64"]
    Div64 = 6,
    #[doc = "7: LFACLKLCD = LFACLK/128"]
    Div128 = 7,
}
impl From<Lcd> for u8 {
    #[inline(always)]
    fn from(variant: Lcd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcd {
    type Ux = u8;
}
impl crate::IsEnum for Lcd {}
#[doc = "Field `LCD` reader - Liquid Crystal Display Controller Prescaler"]
pub type LcdR = crate::FieldReader<Lcd>;
impl LcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcd {
        match self.bits {
            0 => Lcd::Div1,
            1 => Lcd::Div2,
            2 => Lcd::Div4,
            3 => Lcd::Div8,
            4 => Lcd::Div16,
            5 => Lcd::Div32,
            6 => Lcd::Div64,
            7 => Lcd::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLKLCD = LFACLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Lcd::Div1
    }
    #[doc = "LFACLKLCD = LFACLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Lcd::Div2
    }
    #[doc = "LFACLKLCD = LFACLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Lcd::Div4
    }
    #[doc = "LFACLKLCD = LFACLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Lcd::Div8
    }
    #[doc = "LFACLKLCD = LFACLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Lcd::Div16
    }
    #[doc = "LFACLKLCD = LFACLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Lcd::Div32
    }
    #[doc = "LFACLKLCD = LFACLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Lcd::Div64
    }
    #[doc = "LFACLKLCD = LFACLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Lcd::Div128
    }
}
#[doc = "Field `LCD` writer - Liquid Crystal Display Controller Prescaler"]
pub type LcdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcd, crate::Safe>;
impl<'a, REG> LcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLKLCD = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div1)
    }
    #[doc = "LFACLKLCD = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div2)
    }
    #[doc = "LFACLKLCD = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div4)
    }
    #[doc = "LFACLKLCD = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div8)
    }
    #[doc = "LFACLKLCD = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div16)
    }
    #[doc = "LFACLKLCD = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div32)
    }
    #[doc = "LFACLKLCD = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div64)
    }
    #[doc = "LFACLKLCD = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Lcd::Div128)
    }
}
#[doc = "Real-Time Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtc {
    #[doc = "0: LFACLKRTC = LFACLK"]
    Div1 = 0,
    #[doc = "1: LFACLKRTC = LFACLK/2"]
    Div2 = 1,
    #[doc = "2: LFACLKRTC = LFACLK/4"]
    Div4 = 2,
    #[doc = "3: LFACLKRTC = LFACLK/8"]
    Div8 = 3,
    #[doc = "4: LFACLKRTC = LFACLK/16"]
    Div16 = 4,
    #[doc = "5: LFACLKRTC = LFACLK/32"]
    Div32 = 5,
    #[doc = "6: LFACLKRTC = LFACLK/64"]
    Div64 = 6,
    #[doc = "7: LFACLKRTC = LFACLK/128"]
    Div128 = 7,
    #[doc = "8: LFACLKRTC = LFACLK/256"]
    Div256 = 8,
    #[doc = "9: LFACLKRTC = LFACLK/512"]
    Div512 = 9,
    #[doc = "10: LFACLKRTC = LFACLK/1024"]
    Div1024 = 10,
    #[doc = "11: LFACLKRTC = LFACLK/2048"]
    Div2048 = 11,
    #[doc = "12: LFACLKRTC = LFACLK/4096"]
    Div4096 = 12,
    #[doc = "13: LFACLKRTC = LFACLK/8192"]
    Div8192 = 13,
    #[doc = "14: LFACLKRTC = LFACLK/16384"]
    Div16384 = 14,
    #[doc = "15: LFACLKRTC = LFACLK/32768"]
    Div32768 = 15,
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtc {
    type Ux = u8;
}
impl crate::IsEnum for Rtc {}
#[doc = "Field `RTC` reader - Real-Time Counter Prescaler"]
pub type RtcR = crate::FieldReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            0 => Rtc::Div1,
            1 => Rtc::Div2,
            2 => Rtc::Div4,
            3 => Rtc::Div8,
            4 => Rtc::Div16,
            5 => Rtc::Div32,
            6 => Rtc::Div64,
            7 => Rtc::Div128,
            8 => Rtc::Div256,
            9 => Rtc::Div512,
            10 => Rtc::Div1024,
            11 => Rtc::Div2048,
            12 => Rtc::Div4096,
            13 => Rtc::Div8192,
            14 => Rtc::Div16384,
            15 => Rtc::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Rtc::Div1
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Rtc::Div2
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Rtc::Div4
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Rtc::Div8
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Rtc::Div16
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Rtc::Div32
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Rtc::Div64
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Rtc::Div128
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Rtc::Div256
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Rtc::Div512
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Rtc::Div1024
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Rtc::Div2048
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Rtc::Div4096
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Rtc::Div8192
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Rtc::Div16384
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Rtc::Div32768
    }
}
#[doc = "Field `RTC` writer - Real-Time Counter Prescaler"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rtc, crate::Safe>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFACLKRTC = LFACLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div1)
    }
    #[doc = "LFACLKRTC = LFACLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div2)
    }
    #[doc = "LFACLKRTC = LFACLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div4)
    }
    #[doc = "LFACLKRTC = LFACLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div8)
    }
    #[doc = "LFACLKRTC = LFACLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div16)
    }
    #[doc = "LFACLKRTC = LFACLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div32)
    }
    #[doc = "LFACLKRTC = LFACLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div64)
    }
    #[doc = "LFACLKRTC = LFACLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div128)
    }
    #[doc = "LFACLKRTC = LFACLK/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div256)
    }
    #[doc = "LFACLKRTC = LFACLK/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div512)
    }
    #[doc = "LFACLKRTC = LFACLK/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div1024)
    }
    #[doc = "LFACLKRTC = LFACLK/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div2048)
    }
    #[doc = "LFACLKRTC = LFACLK/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div4096)
    }
    #[doc = "LFACLKRTC = LFACLK/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div8192)
    }
    #[doc = "LFACLKRTC = LFACLK/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div16384)
    }
    #[doc = "LFACLKRTC = LFACLK/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Div32768)
    }
}
impl R {
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&self) -> Letimer0R {
        Letimer0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low Energy Timer 1 Prescaler"]
    #[inline(always)]
    pub fn letimer1(&self) -> Letimer1R {
        Letimer1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Low Energy Sensor Interface Prescaler"]
    #[inline(always)]
    pub fn lesense(&self) -> LesenseR {
        LesenseR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Liquid Crystal Display Controller Prescaler"]
    #[inline(always)]
    pub fn lcd(&self) -> LcdR {
        LcdR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Low Energy Timer 0 Prescaler"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> Letimer0W<'_, Lfapresc0Spec> {
        Letimer0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Low Energy Timer 1 Prescaler"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> Letimer1W<'_, Lfapresc0Spec> {
        Letimer1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Low Energy Sensor Interface Prescaler"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LesenseW<'_, Lfapresc0Spec> {
        LesenseW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Liquid Crystal Display Controller Prescaler"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LcdW<'_, Lfapresc0Spec> {
        LcdW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Real-Time Counter Prescaler"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Lfapresc0Spec> {
        RtcW::new(self, 16)
    }
}
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfapresc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfapresc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfapresc0Spec;
impl crate::RegisterSpec for Lfapresc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfapresc0::R`](R) reader structure"]
impl crate::Readable for Lfapresc0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfapresc0::W`](W) writer structure"]
impl crate::Writable for Lfapresc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFAPRESC0 to value 0"]
impl crate::Resettable for Lfapresc0Spec {}
