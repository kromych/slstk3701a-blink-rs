#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0_SPEC>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0_SPEC>;
#[doc = "Field `DATLOC` reader - I/O Location for D0-7 Pins"]
pub type DATLOC_R = crate::FieldReader<DATLOC_A>;
#[doc = "I/O Location for D0-7 Pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<DATLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATLOC_A {
    type Ux = u8;
}
impl DATLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATLOC_A> {
        match self.bits {
            0 => Some(DATLOC_A::LOC0),
            1 => Some(DATLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == DATLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == DATLOC_A::LOC1
    }
}
#[doc = "Field `DATLOC` writer - I/O Location for D0-7 Pins"]
pub type DATLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, DATLOC_A>;
impl<'a, REG> DATLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(DATLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(DATLOC_A::LOC1)
    }
}
#[doc = "Field `CDLOC` reader - I/O Location for CD"]
pub type CDLOC_R = crate::FieldReader<CDLOC_A>;
#[doc = "I/O Location for CD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<CDLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDLOC_A {
    type Ux = u8;
}
impl CDLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CDLOC_A> {
        match self.bits {
            0 => Some(CDLOC_A::LOC0),
            1 => Some(CDLOC_A::LOC1),
            2 => Some(CDLOC_A::LOC2),
            3 => Some(CDLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDLOC_A::LOC3
    }
}
#[doc = "Field `CDLOC` writer - I/O Location for CD"]
pub type CDLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CDLOC_A>;
impl<'a, REG> CDLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CDLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CDLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CDLOC_A::LOC3)
    }
}
#[doc = "Field `WPLOC` reader - I/O Location for WP"]
pub type WPLOC_R = crate::FieldReader<WPLOC_A>;
#[doc = "I/O Location for WP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WPLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<WPLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: WPLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPLOC_A {
    type Ux = u8;
}
impl WPLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WPLOC_A> {
        match self.bits {
            0 => Some(WPLOC_A::LOC0),
            1 => Some(WPLOC_A::LOC1),
            2 => Some(WPLOC_A::LOC2),
            3 => Some(WPLOC_A::LOC3),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == WPLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == WPLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == WPLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == WPLOC_A::LOC3
    }
}
#[doc = "Field `WPLOC` writer - I/O Location for WP"]
pub type WPLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, WPLOC_A>;
impl<'a, REG> WPLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(WPLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(WPLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(WPLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(WPLOC_A::LOC3)
    }
}
#[doc = "Field `CLKLOC` reader - I/O Location for CLK"]
pub type CLKLOC_R = crate::FieldReader<CLKLOC_A>;
#[doc = "I/O Location for CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<CLKLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKLOC_A {
    type Ux = u8;
}
impl CLKLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKLOC_A> {
        match self.bits {
            0 => Some(CLKLOC_A::LOC0),
            1 => Some(CLKLOC_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKLOC_A::LOC1
    }
}
#[doc = "Field `CLKLOC` writer - I/O Location for CLK"]
pub type CLKLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CLKLOC_A>;
impl<'a, REG> CLKLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC_A::LOC1)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    pub fn datloc(&self) -> DATLOC_R {
        DATLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    pub fn cdloc(&self) -> CDLOC_R {
        CDLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    pub fn wploc(&self) -> WPLOC_R {
        WPLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    pub fn clkloc(&self) -> CLKLOC_R {
        CLKLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location for D0-7 Pins"]
    #[inline(always)]
    #[must_use]
    pub fn datloc(&mut self) -> DATLOC_W<ROUTELOC0_SPEC> {
        DATLOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location for CD"]
    #[inline(always)]
    #[must_use]
    pub fn cdloc(&mut self) -> CDLOC_W<ROUTELOC0_SPEC> {
        CDLOC_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location for WP"]
    #[inline(always)]
    #[must_use]
    pub fn wploc(&mut self) -> WPLOC_W<ROUTELOC0_SPEC> {
        WPLOC_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location for CLK"]
    #[inline(always)]
    #[must_use]
    pub fn clkloc(&mut self) -> CLKLOC_W<ROUTELOC0_SPEC> {
        CLKLOC_W::new(self, 24)
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
#[doc = "I/O LOCATION Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC0_SPEC;
impl crate::RegisterSpec for ROUTELOC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for ROUTELOC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for ROUTELOC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0_SPEC {
    const RESET_VALUE: u32 = 0;
}
