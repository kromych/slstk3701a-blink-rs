#[doc = "Register `ROUTELOC5` reader"]
pub type R = crate::R<ROUTELOC5_SPEC>;
#[doc = "Register `ROUTELOC5` writer"]
pub type W = crate::W<ROUTELOC5_SPEC>;
#[doc = "Field `CH20LOC` reader - I/O Location"]
pub type CH20LOC_R = crate::FieldReader<CH20LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH20LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH20LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH20LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH20LOC_A {
    type Ux = u8;
}
impl CH20LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH20LOC_A> {
        match self.bits {
            0 => Some(CH20LOC_A::LOC0),
            1 => Some(CH20LOC_A::LOC1),
            2 => Some(CH20LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH20LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH20LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH20LOC_A::LOC2
    }
}
#[doc = "Field `CH20LOC` writer - I/O Location"]
pub type CH20LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH20LOC_A>;
impl<'a, REG, const O: u8> CH20LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH20LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH20LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH20LOC_A::LOC2)
    }
}
#[doc = "Field `CH21LOC` reader - I/O Location"]
pub type CH21LOC_R = crate::FieldReader<CH21LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH21LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH21LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH21LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH21LOC_A {
    type Ux = u8;
}
impl CH21LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH21LOC_A> {
        match self.bits {
            0 => Some(CH21LOC_A::LOC0),
            1 => Some(CH21LOC_A::LOC1),
            2 => Some(CH21LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH21LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH21LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH21LOC_A::LOC2
    }
}
#[doc = "Field `CH21LOC` writer - I/O Location"]
pub type CH21LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH21LOC_A>;
impl<'a, REG, const O: u8> CH21LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH21LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH21LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH21LOC_A::LOC2)
    }
}
#[doc = "Field `CH22LOC` reader - I/O Location"]
pub type CH22LOC_R = crate::FieldReader<CH22LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH22LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH22LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH22LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH22LOC_A {
    type Ux = u8;
}
impl CH22LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH22LOC_A> {
        match self.bits {
            0 => Some(CH22LOC_A::LOC0),
            1 => Some(CH22LOC_A::LOC1),
            2 => Some(CH22LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH22LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH22LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH22LOC_A::LOC2
    }
}
#[doc = "Field `CH22LOC` writer - I/O Location"]
pub type CH22LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH22LOC_A>;
impl<'a, REG, const O: u8> CH22LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH22LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH22LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH22LOC_A::LOC2)
    }
}
#[doc = "Field `CH23LOC` reader - I/O Location"]
pub type CH23LOC_R = crate::FieldReader<CH23LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH23LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH23LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH23LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH23LOC_A {
    type Ux = u8;
}
impl CH23LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH23LOC_A> {
        match self.bits {
            0 => Some(CH23LOC_A::LOC0),
            1 => Some(CH23LOC_A::LOC1),
            2 => Some(CH23LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH23LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH23LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH23LOC_A::LOC2
    }
}
#[doc = "Field `CH23LOC` writer - I/O Location"]
pub type CH23LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH23LOC_A>;
impl<'a, REG, const O: u8> CH23LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH23LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH23LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH23LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch20loc(&self) -> CH20LOC_R {
        CH20LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch21loc(&self) -> CH21LOC_R {
        CH21LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch22loc(&self) -> CH22LOC_R {
        CH22LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch23loc(&self) -> CH23LOC_R {
        CH23LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch20loc(&mut self) -> CH20LOC_W<ROUTELOC5_SPEC, 0> {
        CH20LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch21loc(&mut self) -> CH21LOC_W<ROUTELOC5_SPEC, 8> {
        CH21LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch22loc(&mut self) -> CH22LOC_W<ROUTELOC5_SPEC, 16> {
        CH22LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch23loc(&mut self) -> CH23LOC_W<ROUTELOC5_SPEC, 24> {
        CH23LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC5_SPEC;
impl crate::RegisterSpec for ROUTELOC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc5::R`](R) reader structure"]
impl crate::Readable for ROUTELOC5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc5::W`](W) writer structure"]
impl crate::Writable for ROUTELOC5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC5 to value 0"]
impl crate::Resettable for ROUTELOC5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
