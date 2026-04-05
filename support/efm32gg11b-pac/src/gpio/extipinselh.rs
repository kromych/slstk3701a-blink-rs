#[doc = "Register `EXTIPINSELH` reader"]
pub type R = crate::R<ExtipinselhSpec>;
#[doc = "Register `EXTIPINSELH` writer"]
pub type W = crate::W<ExtipinselhSpec>;
#[doc = "External Interrupt 8 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel8 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<Extipinsel8> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel8 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel8 {}
#[doc = "Field `EXTIPINSEL8` reader - External Interrupt 8 Pin Select"]
pub type Extipinsel8R = crate::FieldReader<Extipinsel8>;
impl Extipinsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel8 {
        match self.bits {
            0 => Extipinsel8::Pin8,
            1 => Extipinsel8::Pin9,
            2 => Extipinsel8::Pin10,
            3 => Extipinsel8::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel8::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel8::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel8::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel8::Pin11
    }
}
#[doc = "Field `EXTIPINSEL8` writer - External Interrupt 8 Pin Select"]
pub type Extipinsel8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel8, crate::Safe>;
impl<'a, REG> Extipinsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel8::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel8::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel8::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel8::Pin11)
    }
}
#[doc = "External Interrupt 9 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel9 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<Extipinsel9> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel9 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel9 {}
#[doc = "Field `EXTIPINSEL9` reader - External Interrupt 9 Pin Select"]
pub type Extipinsel9R = crate::FieldReader<Extipinsel9>;
impl Extipinsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel9 {
        match self.bits {
            0 => Extipinsel9::Pin8,
            1 => Extipinsel9::Pin9,
            2 => Extipinsel9::Pin10,
            3 => Extipinsel9::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel9::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel9::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel9::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel9::Pin11
    }
}
#[doc = "Field `EXTIPINSEL9` writer - External Interrupt 9 Pin Select"]
pub type Extipinsel9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel9, crate::Safe>;
impl<'a, REG> Extipinsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel9::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel9::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel9::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel9::Pin11)
    }
}
#[doc = "External Interrupt 10 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel10 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<Extipinsel10> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel10 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel10 {}
#[doc = "Field `EXTIPINSEL10` reader - External Interrupt 10 Pin Select"]
pub type Extipinsel10R = crate::FieldReader<Extipinsel10>;
impl Extipinsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel10 {
        match self.bits {
            0 => Extipinsel10::Pin8,
            1 => Extipinsel10::Pin9,
            2 => Extipinsel10::Pin10,
            3 => Extipinsel10::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel10::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel10::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel10::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel10::Pin11
    }
}
#[doc = "Field `EXTIPINSEL10` writer - External Interrupt 10 Pin Select"]
pub type Extipinsel10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel10, crate::Safe>;
impl<'a, REG> Extipinsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel10::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel10::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel10::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel10::Pin11)
    }
}
#[doc = "External Interrupt 11 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel11 {
    #[doc = "0: Pin 8"]
    Pin8 = 0,
    #[doc = "1: Pin 9"]
    Pin9 = 1,
    #[doc = "2: Pin 10"]
    Pin10 = 2,
    #[doc = "3: Pin 11"]
    Pin11 = 3,
}
impl From<Extipinsel11> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel11 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel11 {}
#[doc = "Field `EXTIPINSEL11` reader - External Interrupt 11 Pin Select"]
pub type Extipinsel11R = crate::FieldReader<Extipinsel11>;
impl Extipinsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel11 {
        match self.bits {
            0 => Extipinsel11::Pin8,
            1 => Extipinsel11::Pin9,
            2 => Extipinsel11::Pin10,
            3 => Extipinsel11::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel11::Pin8
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel11::Pin9
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel11::Pin10
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel11::Pin11
    }
}
#[doc = "Field `EXTIPINSEL11` writer - External Interrupt 11 Pin Select"]
pub type Extipinsel11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel11, crate::Safe>;
impl<'a, REG> Extipinsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel11::Pin8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel11::Pin9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel11::Pin10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel11::Pin11)
    }
}
#[doc = "External Interrupt 12 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel12 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<Extipinsel12> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel12 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel12 {}
#[doc = "Field `EXTIPINSEL12` reader - External Interrupt 12 Pin Select"]
pub type Extipinsel12R = crate::FieldReader<Extipinsel12>;
impl Extipinsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel12 {
        match self.bits {
            0 => Extipinsel12::Pin12,
            1 => Extipinsel12::Pin13,
            2 => Extipinsel12::Pin14,
            3 => Extipinsel12::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Extipinsel12::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Extipinsel12::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Extipinsel12::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Extipinsel12::Pin15
    }
}
#[doc = "Field `EXTIPINSEL12` writer - External Interrupt 12 Pin Select"]
pub type Extipinsel12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel12, crate::Safe>;
impl<'a, REG> Extipinsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel12::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel12::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel12::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel12::Pin15)
    }
}
#[doc = "External Interrupt 13 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel13 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<Extipinsel13> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel13 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel13 {}
#[doc = "Field `EXTIPINSEL13` reader - External Interrupt 13 Pin Select"]
pub type Extipinsel13R = crate::FieldReader<Extipinsel13>;
impl Extipinsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel13 {
        match self.bits {
            0 => Extipinsel13::Pin12,
            1 => Extipinsel13::Pin13,
            2 => Extipinsel13::Pin14,
            3 => Extipinsel13::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Extipinsel13::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Extipinsel13::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Extipinsel13::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Extipinsel13::Pin15
    }
}
#[doc = "Field `EXTIPINSEL13` writer - External Interrupt 13 Pin Select"]
pub type Extipinsel13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel13, crate::Safe>;
impl<'a, REG> Extipinsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel13::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel13::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel13::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel13::Pin15)
    }
}
#[doc = "External Interrupt 14 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel14 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<Extipinsel14> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel14 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel14 {}
#[doc = "Field `EXTIPINSEL14` reader - External Interrupt 14 Pin Select"]
pub type Extipinsel14R = crate::FieldReader<Extipinsel14>;
impl Extipinsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel14 {
        match self.bits {
            0 => Extipinsel14::Pin12,
            1 => Extipinsel14::Pin13,
            2 => Extipinsel14::Pin14,
            3 => Extipinsel14::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Extipinsel14::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Extipinsel14::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Extipinsel14::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Extipinsel14::Pin15
    }
}
#[doc = "Field `EXTIPINSEL14` writer - External Interrupt 14 Pin Select"]
pub type Extipinsel14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel14, crate::Safe>;
impl<'a, REG> Extipinsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel14::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel14::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel14::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel14::Pin15)
    }
}
#[doc = "External Interrupt 15 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel15 {
    #[doc = "0: Pin 12"]
    Pin12 = 0,
    #[doc = "1: Pin 13"]
    Pin13 = 1,
    #[doc = "2: Pin 14"]
    Pin14 = 2,
    #[doc = "3: Pin 15"]
    Pin15 = 3,
}
impl From<Extipinsel15> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel15 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel15 {}
#[doc = "Field `EXTIPINSEL15` reader - External Interrupt 15 Pin Select"]
pub type Extipinsel15R = crate::FieldReader<Extipinsel15>;
impl Extipinsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel15 {
        match self.bits {
            0 => Extipinsel15::Pin12,
            1 => Extipinsel15::Pin13,
            2 => Extipinsel15::Pin14,
            3 => Extipinsel15::Pin15,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == Extipinsel15::Pin12
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == Extipinsel15::Pin13
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == Extipinsel15::Pin14
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == Extipinsel15::Pin15
    }
}
#[doc = "Field `EXTIPINSEL15` writer - External Interrupt 15 Pin Select"]
pub type Extipinsel15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel15, crate::Safe>;
impl<'a, REG> Extipinsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel15::Pin12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel15::Pin13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel15::Pin14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel15::Pin15)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&self) -> Extipinsel8R {
        Extipinsel8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&self) -> Extipinsel9R {
        Extipinsel9R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&self) -> Extipinsel10R {
        Extipinsel10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&self) -> Extipinsel11R {
        Extipinsel11R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&self) -> Extipinsel12R {
        Extipinsel12R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&self) -> Extipinsel13R {
        Extipinsel13R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&self) -> Extipinsel14R {
        Extipinsel14R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&self) -> Extipinsel15R {
        Extipinsel15R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&mut self) -> Extipinsel8W<'_, ExtipinselhSpec> {
        Extipinsel8W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&mut self) -> Extipinsel9W<'_, ExtipinselhSpec> {
        Extipinsel9W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&mut self) -> Extipinsel10W<'_, ExtipinselhSpec> {
        Extipinsel10W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&mut self) -> Extipinsel11W<'_, ExtipinselhSpec> {
        Extipinsel11W::new(self, 12)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&mut self) -> Extipinsel12W<'_, ExtipinselhSpec> {
        Extipinsel12W::new(self, 16)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&mut self) -> Extipinsel13W<'_, ExtipinselhSpec> {
        Extipinsel13W::new(self, 20)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&mut self) -> Extipinsel14W<'_, ExtipinselhSpec> {
        Extipinsel14W::new(self, 24)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&mut self) -> Extipinsel15W<'_, ExtipinselhSpec> {
        Extipinsel15W::new(self, 28)
    }
}
#[doc = "External Interrupt Pin Select High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtipinselhSpec;
impl crate::RegisterSpec for ExtipinselhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipinselh::R`](R) reader structure"]
impl crate::Readable for ExtipinselhSpec {}
#[doc = "`write(|w| ..)` method takes [`extipinselh::W`](W) writer structure"]
impl crate::Writable for ExtipinselhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIPINSELH to value 0x3210_3210"]
impl crate::Resettable for ExtipinselhSpec {
    const RESET_VALUE: u32 = 0x3210_3210;
}
