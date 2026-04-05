#[doc = "Register `ROUTELOC2` reader"]
pub type R = crate::R<Routeloc2Spec>;
#[doc = "Register `ROUTELOC2` writer"]
pub type W = crate::W<Routeloc2Spec>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdti0loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
}
impl From<Cdti0loc> for u8 {
    #[inline(always)]
    fn from(variant: Cdti0loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdti0loc {
    type Ux = u8;
}
impl crate::IsEnum for Cdti0loc {}
#[doc = "Field `CDTI0LOC` reader - I/O Location"]
pub type Cdti0locR = crate::FieldReader<Cdti0loc>;
impl Cdti0locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdti0loc> {
        match self.bits {
            0 => Some(Cdti0loc::Loc0),
            1 => Some(Cdti0loc::Loc1),
            2 => Some(Cdti0loc::Loc2),
            3 => Some(Cdti0loc::Loc3),
            4 => Some(Cdti0loc::Loc4),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Cdti0loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Cdti0loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Cdti0loc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Cdti0loc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Cdti0loc::Loc4
    }
}
#[doc = "Field `CDTI0LOC` writer - I/O Location"]
pub type Cdti0locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cdti0loc>;
impl<'a, REG> Cdti0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti0loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti0loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti0loc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti0loc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti0loc::Loc4)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdti1loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
}
impl From<Cdti1loc> for u8 {
    #[inline(always)]
    fn from(variant: Cdti1loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdti1loc {
    type Ux = u8;
}
impl crate::IsEnum for Cdti1loc {}
#[doc = "Field `CDTI1LOC` reader - I/O Location"]
pub type Cdti1locR = crate::FieldReader<Cdti1loc>;
impl Cdti1locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdti1loc> {
        match self.bits {
            0 => Some(Cdti1loc::Loc0),
            1 => Some(Cdti1loc::Loc1),
            2 => Some(Cdti1loc::Loc2),
            3 => Some(Cdti1loc::Loc3),
            4 => Some(Cdti1loc::Loc4),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Cdti1loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Cdti1loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Cdti1loc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Cdti1loc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Cdti1loc::Loc4
    }
}
#[doc = "Field `CDTI1LOC` writer - I/O Location"]
pub type Cdti1locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cdti1loc>;
impl<'a, REG> Cdti1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti1loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti1loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti1loc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti1loc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti1loc::Loc4)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdti2loc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
}
impl From<Cdti2loc> for u8 {
    #[inline(always)]
    fn from(variant: Cdti2loc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdti2loc {
    type Ux = u8;
}
impl crate::IsEnum for Cdti2loc {}
#[doc = "Field `CDTI2LOC` reader - I/O Location"]
pub type Cdti2locR = crate::FieldReader<Cdti2loc>;
impl Cdti2locR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdti2loc> {
        match self.bits {
            0 => Some(Cdti2loc::Loc0),
            1 => Some(Cdti2loc::Loc1),
            2 => Some(Cdti2loc::Loc2),
            3 => Some(Cdti2loc::Loc3),
            4 => Some(Cdti2loc::Loc4),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Cdti2loc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Cdti2loc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Cdti2loc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Cdti2loc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Cdti2loc::Loc4
    }
}
#[doc = "Field `CDTI2LOC` writer - I/O Location"]
pub type Cdti2locW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cdti2loc>;
impl<'a, REG> Cdti2locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti2loc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti2loc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti2loc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti2loc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Cdti2loc::Loc4)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&self) -> Cdti0locR {
        Cdti0locR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&self) -> Cdti1locR {
        Cdti1locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&self) -> Cdti2locR {
        Cdti2locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&mut self) -> Cdti0locW<'_, Routeloc2Spec> {
        Cdti0locW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&mut self) -> Cdti1locW<'_, Routeloc2Spec> {
        Cdti1locW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&mut self) -> Cdti2locW<'_, Routeloc2Spec> {
        Cdti2locW::new(self, 16)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Routeloc2Spec;
impl crate::RegisterSpec for Routeloc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc2::R`](R) reader structure"]
impl crate::Readable for Routeloc2Spec {}
#[doc = "`write(|w| ..)` method takes [`routeloc2::W`](W) writer structure"]
impl crate::Writable for Routeloc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for Routeloc2Spec {}
