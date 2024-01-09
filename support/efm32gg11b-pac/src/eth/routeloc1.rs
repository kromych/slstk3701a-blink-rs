#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<ROUTELOC1_SPEC>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<ROUTELOC1_SPEC>;
#[doc = "Field `TSUEXTCLKLOC` reader - I/O Location"]
pub type TSUEXTCLKLOC_R = crate::FieldReader<TSUEXTCLKLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSUEXTCLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<TSUEXTCLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUEXTCLKLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSUEXTCLKLOC_A {
    type Ux = u8;
}
impl TSUEXTCLKLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSUEXTCLKLOC_A> {
        match self.bits {
            0 => Some(TSUEXTCLKLOC_A::LOC0),
            1 => Some(TSUEXTCLKLOC_A::LOC1),
            2 => Some(TSUEXTCLKLOC_A::LOC2),
            3 => Some(TSUEXTCLKLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TSUEXTCLKLOC_A::LOC3
    }
}
#[doc = "Field `TSUEXTCLKLOC` writer - I/O Location"]
pub type TSUEXTCLKLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, TSUEXTCLKLOC_A>;
impl<'a, REG> TSUEXTCLKLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(TSUEXTCLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(TSUEXTCLKLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(TSUEXTCLKLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(TSUEXTCLKLOC_A::LOC3)
    }
}
#[doc = "Field `TSUTMRTOGLOC` reader - I/O Location"]
pub type TSUTMRTOGLOC_R = crate::FieldReader<TSUTMRTOGLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSUTMRTOGLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<TSUTMRTOGLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUTMRTOGLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSUTMRTOGLOC_A {
    type Ux = u8;
}
impl TSUTMRTOGLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSUTMRTOGLOC_A> {
        match self.bits {
            0 => Some(TSUTMRTOGLOC_A::LOC0),
            1 => Some(TSUTMRTOGLOC_A::LOC1),
            2 => Some(TSUTMRTOGLOC_A::LOC2),
            3 => Some(TSUTMRTOGLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TSUTMRTOGLOC_A::LOC3
    }
}
#[doc = "Field `TSUTMRTOGLOC` writer - I/O Location"]
pub type TSUTMRTOGLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, TSUTMRTOGLOC_A>;
impl<'a, REG> TSUTMRTOGLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(TSUTMRTOGLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(TSUTMRTOGLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(TSUTMRTOGLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(TSUTMRTOGLOC_A::LOC3)
    }
}
#[doc = "Field `MDIOLOC` reader - I/O Location"]
pub type MDIOLOC_R = crate::FieldReader<MDIOLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDIOLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<MDIOLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIOLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDIOLOC_A {
    type Ux = u8;
}
impl MDIOLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MDIOLOC_A> {
        match self.bits {
            0 => Some(MDIOLOC_A::LOC0),
            1 => Some(MDIOLOC_A::LOC1),
            2 => Some(MDIOLOC_A::LOC2),
            3 => Some(MDIOLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MDIOLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MDIOLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MDIOLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == MDIOLOC_A::LOC3
    }
}
#[doc = "Field `MDIOLOC` writer - I/O Location"]
pub type MDIOLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, MDIOLOC_A>;
impl<'a, REG> MDIOLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(MDIOLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(MDIOLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(MDIOLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(MDIOLOC_A::LOC3)
    }
}
#[doc = "Field `RMIILOC` reader - I/O Location"]
pub type RMIILOC_R = crate::FieldReader<RMIILOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RMIILOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<RMIILOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RMIILOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RMIILOC_A {
    type Ux = u8;
}
impl RMIILOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RMIILOC_A> {
        match self.bits {
            0 => Some(RMIILOC_A::LOC0),
            1 => Some(RMIILOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RMIILOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RMIILOC_A::LOC1
    }
}
#[doc = "Field `RMIILOC` writer - I/O Location"]
pub type RMIILOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, RMIILOC_A>;
impl<'a, REG> RMIILOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(RMIILOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(RMIILOC_A::LOC1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn tsuextclkloc(&self) -> TSUEXTCLKLOC_R {
        TSUEXTCLKLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn tsutmrtogloc(&self) -> TSUTMRTOGLOC_R {
        TSUTMRTOGLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn mdioloc(&self) -> MDIOLOC_R {
        MDIOLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn rmiiloc(&self) -> RMIILOC_R {
        RMIILOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn tsuextclkloc(&mut self) -> TSUEXTCLKLOC_W<ROUTELOC1_SPEC> {
        TSUEXTCLKLOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn tsutmrtogloc(&mut self) -> TSUTMRTOGLOC_W<ROUTELOC1_SPEC> {
        TSUTMRTOGLOC_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn mdioloc(&mut self) -> MDIOLOC_W<ROUTELOC1_SPEC> {
        MDIOLOC_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn rmiiloc(&mut self) -> RMIILOC_W<ROUTELOC1_SPEC> {
        RMIILOC_W::new(self, 24)
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
#[doc = "I/O Route Location Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc1::R`](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc1::W`](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    const RESET_VALUE: u32 = 0;
}
