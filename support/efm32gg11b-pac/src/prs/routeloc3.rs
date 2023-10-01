#[doc = "Register `ROUTELOC3` reader"]
pub type R = crate::R<ROUTELOC3_SPEC>;
#[doc = "Register `ROUTELOC3` writer"]
pub type W = crate::W<ROUTELOC3_SPEC>;
#[doc = "Field `CH12LOC` reader - I/O Location"]
pub type CH12LOC_R = crate::FieldReader<CH12LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH12LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH12LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH12LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH12LOC_A {
    type Ux = u8;
}
impl CH12LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH12LOC_A> {
        match self.bits {
            0 => Some(CH12LOC_A::LOC0),
            1 => Some(CH12LOC_A::LOC1),
            2 => Some(CH12LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH12LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH12LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH12LOC_A::LOC2
    }
}
#[doc = "Field `CH12LOC` writer - I/O Location"]
pub type CH12LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH12LOC_A>;
impl<'a, REG, const O: u8> CH12LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH12LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH12LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH12LOC_A::LOC2)
    }
}
#[doc = "Field `CH13LOC` reader - I/O Location"]
pub type CH13LOC_R = crate::FieldReader<CH13LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH13LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH13LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH13LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH13LOC_A {
    type Ux = u8;
}
impl CH13LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH13LOC_A> {
        match self.bits {
            0 => Some(CH13LOC_A::LOC0),
            1 => Some(CH13LOC_A::LOC1),
            2 => Some(CH13LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH13LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH13LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH13LOC_A::LOC2
    }
}
#[doc = "Field `CH13LOC` writer - I/O Location"]
pub type CH13LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH13LOC_A>;
impl<'a, REG, const O: u8> CH13LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH13LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH13LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH13LOC_A::LOC2)
    }
}
#[doc = "Field `CH14LOC` reader - I/O Location"]
pub type CH14LOC_R = crate::FieldReader<CH14LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH14LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH14LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH14LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH14LOC_A {
    type Ux = u8;
}
impl CH14LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH14LOC_A> {
        match self.bits {
            0 => Some(CH14LOC_A::LOC0),
            1 => Some(CH14LOC_A::LOC1),
            2 => Some(CH14LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH14LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH14LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH14LOC_A::LOC2
    }
}
#[doc = "Field `CH14LOC` writer - I/O Location"]
pub type CH14LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH14LOC_A>;
impl<'a, REG, const O: u8> CH14LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH14LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH14LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH14LOC_A::LOC2)
    }
}
#[doc = "Field `CH15LOC` reader - I/O Location"]
pub type CH15LOC_R = crate::FieldReader<CH15LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH15LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH15LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH15LOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH15LOC_A {
    type Ux = u8;
}
impl CH15LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH15LOC_A> {
        match self.bits {
            0 => Some(CH15LOC_A::LOC0),
            1 => Some(CH15LOC_A::LOC1),
            2 => Some(CH15LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH15LOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH15LOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH15LOC_A::LOC2
    }
}
#[doc = "Field `CH15LOC` writer - I/O Location"]
pub type CH15LOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O, CH15LOC_A>;
impl<'a, REG, const O: u8> CH15LOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH15LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH15LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH15LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch12loc(&self) -> CH12LOC_R {
        CH12LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch13loc(&self) -> CH13LOC_R {
        CH13LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch14loc(&self) -> CH14LOC_R {
        CH14LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch15loc(&self) -> CH15LOC_R {
        CH15LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch12loc(&mut self) -> CH12LOC_W<ROUTELOC3_SPEC, 0> {
        CH12LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch13loc(&mut self) -> CH13LOC_W<ROUTELOC3_SPEC, 8> {
        CH13LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch14loc(&mut self) -> CH14LOC_W<ROUTELOC3_SPEC, 16> {
        CH14LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ch15loc(&mut self) -> CH15LOC_W<ROUTELOC3_SPEC, 24> {
        CH15LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC3_SPEC;
impl crate::RegisterSpec for ROUTELOC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc3::R`](R) reader structure"]
impl crate::Readable for ROUTELOC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc3::W`](W) writer structure"]
impl crate::Writable for ROUTELOC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC3 to value 0"]
impl crate::Resettable for ROUTELOC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
