#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0_SPEC>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0_SPEC>;
#[doc = "Field `S0INLOC` reader - I/O Location"]
pub type S0INLOC_R = crate::FieldReader<S0INLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0INLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<S0INLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: S0INLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0INLOC_A {
    type Ux = u8;
}
impl S0INLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0INLOC_A> {
        match self.bits {
            0 => Some(S0INLOC_A::LOC0),
            1 => Some(S0INLOC_A::LOC1),
            2 => Some(S0INLOC_A::LOC2),
            3 => Some(S0INLOC_A::LOC3),
            4 => Some(S0INLOC_A::LOC4),
            5 => Some(S0INLOC_A::LOC5),
            6 => Some(S0INLOC_A::LOC6),
            7 => Some(S0INLOC_A::LOC7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S0INLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S0INLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S0INLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S0INLOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S0INLOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S0INLOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S0INLOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S0INLOC_A::LOC7
    }
}
#[doc = "Field `S0INLOC` writer - I/O Location"]
pub type S0INLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, S0INLOC_A>;
impl<'a, REG> S0INLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC_A::LOC7)
    }
}
#[doc = "Field `S1INLOC` reader - I/O Location"]
pub type S1INLOC_R = crate::FieldReader<S1INLOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1INLOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<S1INLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: S1INLOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1INLOC_A {
    type Ux = u8;
}
impl S1INLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1INLOC_A> {
        match self.bits {
            0 => Some(S1INLOC_A::LOC0),
            1 => Some(S1INLOC_A::LOC1),
            2 => Some(S1INLOC_A::LOC2),
            3 => Some(S1INLOC_A::LOC3),
            4 => Some(S1INLOC_A::LOC4),
            5 => Some(S1INLOC_A::LOC5),
            6 => Some(S1INLOC_A::LOC6),
            7 => Some(S1INLOC_A::LOC7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S1INLOC_A::LOC0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S1INLOC_A::LOC1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S1INLOC_A::LOC2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S1INLOC_A::LOC3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S1INLOC_A::LOC4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S1INLOC_A::LOC5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S1INLOC_A::LOC6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S1INLOC_A::LOC7
    }
}
#[doc = "Field `S1INLOC` writer - I/O Location"]
pub type S1INLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, S1INLOC_A>;
impl<'a, REG> S1INLOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC_A::LOC7)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn s0inloc(&self) -> S0INLOC_R {
        S0INLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn s1inloc(&self) -> S1INLOC_R {
        S1INLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn s0inloc(&mut self) -> S0INLOC_W<ROUTELOC0_SPEC> {
        S0INLOC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn s1inloc(&mut self) -> S1INLOC_W<ROUTELOC0_SPEC> {
        S1INLOC_W::new(self, 8)
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
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
